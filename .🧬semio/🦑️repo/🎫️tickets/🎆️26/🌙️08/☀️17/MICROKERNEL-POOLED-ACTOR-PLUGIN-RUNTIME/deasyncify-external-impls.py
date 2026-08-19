#!/usr/bin/env python3
"""🧹 Undo the fleet codemod's damage to EXTERNAL-trait impls (repair codemod S1).

WHY
---
Two codemods ran over this repo. `asyncify-universal.py` was trait-aware: it first collected every
trait DECLARED in first-party code, then treated any `impl X for Y` whose `X` was absent from that
set as external and left its methods alone. `asyncify-fleet.py` was **blind** — a line regex with no
impl-stack tracking — so across `✏️s/🔌️plugins` it happily wrote `async fn` into impls of traits whose
signatures are fixed outside this repo. Measured damage: ~548 `Default::default`, ~600 serde
`serialize`/`deserialize`, ~53 `From::from`, ~31 `Display`/`Debug::fmt`.

Those cannot compile under any setting: you cannot change the signature of a trait you do not own.
They are exception class **E1** in `📌️important.md`. This script removes the `async` keyword from
exactly those methods and nothing else.

SAFETY PROPERTIES (each one exists because of a specific way this could go wrong)
--------------------------------------------------------------------------------
1. **Reuses the SAME local-trait census as the original codemod**, so "external" means precisely what
   it meant when the damage was done — not a hand-maintained name list that could drift.
2. **`FORCE_EXTERNAL` overrides the census.** If first-party code happens to declare a trait named
   `Default`/`From`/`Display`/... the census would wrongly mark those impls local and we would leave
   uncompilable code in place. The std/serde core names always win.
3. **Real brace-depth tracking**, not `line.startswith('}')`. The original's pop heuristic is fine for
   adding a keyword (a false negative just skips a fn) but not for removing one, where a mis-scoped
   stack would strip `async` from a first-party method and silently change its meaning.
4. **Only ever removes `async`; never adds, never reorders, never touches anything else on the line.**
5. **Idempotent** — safe to re-run; a fn already sync is left alone.
6. `--scan` reports without writing, and every rewrite is recorded with file, line, trait and fn name
   so the diff is reviewable rather than a leap of faith.

USAGE
-----
    python3 deasyncify-external-impls.py --scan  /Users/ueli/Documents/semio/✏️s
    python3 deasyncify-external-impls.py --apply /Users/ueli/Documents/semio/✏️s --report s1.json
"""
from __future__ import annotations

import json
import os
import re
import sys

SKIP_DIRS = ('/target', '/node_modules', '/.git', '/vendor', '/storybook-static', '/.🧬semio')
CENSUS_ROOTS = ['/Users/ueli/Documents/semio/🧰️framework', '/Users/ueli/Documents/semio/✏️s']

# Traits whose signatures are fixed outside this repo. These ALWAYS count as external, even if the
# first-party census happens to contain a same-named trait (safety property 2).
FORCE_EXTERNAL = {
    'Default', 'From', 'TryFrom', 'Into', 'TryInto', 'Display', 'Debug', 'Drop', 'Clone', 'Copy',
    'Iterator', 'IntoIterator', 'DoubleEndedIterator', 'ExactSizeIterator', 'Future', 'Deref',
    'DerefMut', 'Index', 'IndexMut', 'PartialEq', 'Eq', 'PartialOrd', 'Ord', 'Hash', 'FromStr',
    'AsRef', 'AsMut', 'Borrow', 'BorrowMut', 'Add', 'Sub', 'Mul', 'Div', 'Rem', 'Neg', 'Not',
    'AddAssign', 'SubAssign', 'MulAssign', 'DivAssign', 'BitAnd', 'BitOr', 'BitXor', 'Shl', 'Shr',
    'Serialize', 'Deserialize', 'Serializer', 'Deserializer', 'Visitor', 'Error', 'Write', 'Read',
    'Send', 'Sync', 'Sized', 'Unpin', 'Wake', 'Termination', 'FnOnce', 'FnMut', 'Fn',
}

FN_RE = re.compile(
    r'^(?P<indent>\s*)'
    r'(?P<vis>pub(?:\([^)]*\))?\s+)?'
    r'(?P<constkw>const\s+)?'
    r'(?P<unsafekw>unsafe\s+)?'
    r'(?P<externkw>extern\s+"[^"]*"\s+)?'
    r'(?P<asynckw>async\s+)'
    r'fn\s+(?P<name>[A-Za-z0-9_]+)'
)
TRAIT_DECL_RE = re.compile(r'^\s*(?:pub(?:\([^)]*\))?\s+)?(?:unsafe\s+)?trait\s+([A-Za-z0-9_]+)')
IMPL_FOR_RE = re.compile(
    r'^\s*impl(?:\s*<[^>]*>)?\s+(?P<trait>[A-Za-z0-9_:]+)(?:\s*<[^>]*>)?\s+for\s+'
)
IMPL_ANY_RE = re.compile(r'^\s*impl\b')


def iter_rs(paths):
    """📂 Every first-party .rs file under the given roots."""
    for root in paths:
        if os.path.isfile(root) and root.endswith('.rs'):
            yield root
            continue
        for dp, dn, fn in os.walk(root):
            if any(s in dp for s in SKIP_DIRS):
                continue
            dn[:] = [d for d in dn if not d.startswith('🎯️target')]
            for f in fn:
                if f.endswith('.rs'):
                    yield os.path.join(dp, f)


def collect_local_traits(paths):
    """🏷️ Traits DECLARED in first-party code — the same census the original codemod used."""
    local = set()
    for p in iter_rs(paths):
        try:
            text = open(p, encoding='utf-8', errors='replace').read()
        except OSError:
            continue
        for line in text.split('\n'):
            m = TRAIT_DECL_RE.match(line)
            if m:
                local.add(m.group(1))
    return local


def strip_strings_and_comments(line: str) -> str:
    """🧼 Blank out string literals and line comments so brace counting is not fooled by them."""
    out, i, n, in_str, esc = [], 0, len(line), False, False
    while i < n:
        c = line[i]
        if in_str:
            if esc:
                esc = False
            elif c == '\\':
                esc = True
            elif c == '"':
                in_str = False
            out.append(' ')
        else:
            if c == '"':
                in_str = True
                out.append(' ')
            elif c == '/' and i + 1 < n and line[i + 1] == '/':
                break
            else:
                out.append(c)
        i += 1
    return ''.join(out)


def process(path, local_traits, apply):
    """✂️ Remove `async` from fns inside external-trait impl blocks. Returns (stats, edits)."""
    try:
        text = open(path, encoding='utf-8', errors='replace').read()
    except OSError:
        return None, []
    lines = text.split('\n')
    out, edits = [], []
    stats = {'reverted': 0, 'kept_local': 0, 'scanned_fns': 0}

    depth = 0
    # stack of (depth_at_open, trait_name_or_None, is_external)
    impl_stack = []
    pending_impl = None  # an `impl ... for ...` header whose `{` has not been seen yet

    for lineno, line in enumerate(lines, 1):
        code = strip_strings_and_comments(line)

        if IMPL_ANY_RE.match(line):
            m = IMPL_FOR_RE.match(line)
            if m:
                name = m.group('trait').split('::')[-1]
                external = (name in FORCE_EXTERNAL) or (name not in local_traits)
                pending_impl = (name, external)
            else:
                pending_impl = (None, False)  # inherent impl

        opens = code.count('{')
        closes = code.count('}')

        if pending_impl is not None and opens > 0:
            impl_stack.append((depth, pending_impl[0], pending_impl[1]))
            pending_impl = None

        m = FN_RE.match(line)
        if m:
            stats['scanned_fns'] += 1
            in_external = any(ext for _, _, ext in impl_stack)
            if in_external:
                tname = next((t for _, t, e in reversed(impl_stack) if e), '?')
                start = m.start('asynckw')
                line = line[:start] + line[m.end('asynckw'):]
                stats['reverted'] += 1
                edits.append({'file': path, 'line': lineno, 'trait': tname, 'fn': m.group('name')})
            else:
                stats['kept_local'] += 1

        depth += opens - closes
        while impl_stack and depth <= impl_stack[-1][0]:
            impl_stack.pop()

        out.append(line)

    if apply and stats['reverted']:
        open(path, 'w', encoding='utf-8').write('\n'.join(out))
    return stats, edits


def main() -> int:
    if len(sys.argv) < 3 or sys.argv[1] not in ('--scan', '--apply'):
        print(__doc__)
        return 2
    apply = sys.argv[1] == '--apply'
    args = sys.argv[2:]
    report = None
    if '--report' in args:
        i = args.index('--report')
        report = args[i + 1]
        args = args[:i] + args[i + 2:]

    local = collect_local_traits(CENSUS_ROOTS)
    total = {'reverted': 0, 'kept_local': 0, 'scanned_fns': 0}
    all_edits, files, touched = [], 0, 0
    for p in iter_rs(args):
        s, edits = process(p, local, apply)
        if s is None:
            continue
        files += 1
        if s['reverted']:
            touched += 1
        for k in total:
            total[k] += s[k]
        all_edits.extend(edits)

    by_trait = {}
    for e in all_edits:
        by_trait[e['trait']] = by_trait.get(e['trait'], 0) + 1

    print(f"local traits known: {len(local)} | files scanned: {files} | files touched: {touched}")
    print(json.dumps(total, indent=2))
    print("\nreverted by trait:")
    for t, n in sorted(by_trait.items(), key=lambda x: -x[1]):
        print(f"  {t:28s} {n}")
    if report:
        with open(report, 'w', encoding='utf-8') as fh:
            json.dump({'totals': total, 'by_trait': by_trait, 'edits': all_edits},
                      fh, indent=1, ensure_ascii=False)
        print(f"\nreport -> {report}")
    if not apply:
        print("\n(scan only — nothing written; re-run with --apply)")
    return 0


if __name__ == '__main__':
    sys.exit(main())
