#!/usr/bin/env python3
"""🩹 Wider sibling of `fix-repeated-await.py` for the "repeated `.await` on an already-resolved
local" residue (R16 Mode 1).

WHY THIS EXISTS (packet `test-attr-restore`, 2026-08-20)
----------------------------------------------------------
`fix-repeated-await.py` only rewrites `IDENT.await.METHOD(...)` — a call immediately following the
bad `.await`. In practice `insert-await.py`'s span-keyed passes also produce THREE sibling shapes on
an already-`.await`-resolved local that the narrower tool intentionally leaves alone (documented in
its own header as "different shape, not this bug"):
  1. `IDENT.await,`               — bare use as a function/struct-literal argument
  2. `IDENT.await }`              — bare use as the tail expression of a block/struct literal
  3. `IDENT.await.FIELD = ...`    — a field ASSIGNMENT through the resolved value (no call parens)
  4. `IDENT.await.FIELD` (read, no call) — plain field access

All four are the exact same root bug as the narrow tool's target: `IDENT` was already resolved via
`.await` at its OWN `let` declaration, so every later `IDENT.await` in that same file is bogus and
must become bare `IDENT`. Confirmed on `semio-framework-plugin-host` (`🧵️shard/executor.rs`,
`⚡️effects/component.rs`) after the narrow tool's fixpoint still left dozens of these.

WHAT IT DOES
------------
1. Finds every `let (mut)? IDENT = EXPR.await;` declaration in a file (EXPR itself may contain
   nested `.await`s — only the outermost, trailing `.await;` before the statement's own `;` matters).
2. For each such IDENT, replaces every subsequent bare `IDENT.await` (not followed by `.` — that
   shape is already handled by the narrow tool and is left to it) with plain `IDENT`.

SAFETY
------
* Scoped per file: an IDENT is only ever touched in files where recorded as declared-and-resolved.
* `--scope` matches PATH SEGMENTS (same discipline as the other tools in this ticket).
* Still name-keyed WITHIN one file after a structural declaration is found — not a blind repo grep
  (R10's actual prohibition is a *global* name/regex sweep with no per-site evidence; this tool's
  evidence is the local's own resolved declaration in the exact same file).
* `--dry-run` prints planned edits; `--apply` writes.
"""
import argparse, os, re, sys

REPO = "/Users/ueli/Documents/semio"
DECL_RE = re.compile(r'\blet\s+(?:mut\s+)?([a-zA-Z_][a-zA-Z0-9_]*)\s*=\s*[^;]*\.await\s*;')


def in_scope(path, scope):
    rel = os.path.relpath(path, REPO)
    parts = rel.split(os.sep)
    want = [p for p in scope.strip('/').split('/') if p]
    if not want:
        return True
    return any(parts[i:i + len(want)] == want for i in range(len(parts) - len(want) + 1))


def iter_rs(root):
    for dp, dn, fn in os.walk(root):
        if '🎯️target' in dp or '🤖️generated' in dp:
            continue
        for f in fn:
            if f.endswith('.rs'):
                yield os.path.join(dp, f)


def process_file(path, apply_):
    text = open(path, encoding='utf-8').read()
    idents = sorted({m.group(1) for m in DECL_RE.finditer(text) if m.group(1) not in ('_', 'Self')})
    if not idents:
        return 0
    total = 0
    for ident in idents:
        # 🩹 terra-plugin-crate (2026-08-20): dropped the `(?!\.)` exclusion — it silently
        # contradicted this file's own docstring, which claims shape 4 (`IDENT.await.FIELD`
        # bare field access) is handled. Confirmed on `semio-framework-plugin`'s `component.rs`:
        # dozens of `definition.await.window_kinds` / `history.await.commands` sites were left
        # untouched by the old regex. Matching `IDENT.await` regardless of what follows is safe
        # here — the evidence (this IDENT's own unawaited `let` declaration in the SAME file) is
        # unchanged; a dot-followed match is simply one more confirmed-bogus `.await` to strip.
        pat = re.compile(r'\b' + re.escape(ident) + r'\.await\b')
        new_text, n = pat.subn(ident, text)
        if n:
            print(f"  {path}: {ident}.await -> {ident} x{n}")
            text = new_text
            total += n
    if total and apply_:
        with open(path, 'w', encoding='utf-8') as f:
            f.write(text)
    return total


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument('--scope', required=True)
    ap.add_argument('--apply', action='store_true')
    ap.add_argument('--dry-run', action='store_true')
    args = ap.parse_args()
    if not args.apply and not args.dry_run:
        ap.error('choose --apply or --dry-run')
    root = os.path.join(REPO, args.scope)
    total = 0
    files = 0
    for path in iter_rs(root):
        if not in_scope(path, args.scope):
            continue
        n = process_file(path, args.apply)
        if n:
            total += n
            files += 1
    print(f"total: {total} edits across {files} files")


if __name__ == '__main__':
    main()
