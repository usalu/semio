#!/usr/bin/env python3
"""🌊️ Universal async codemod for the MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME wave.

Adds `async` to every Rust fn the LANGUAGE permits, and reports the ones it does not.
Signature-only: call sites are fixed compiler-driven afterwards (`.await`), never guessed here.

Hard skips (the compiler rejects `async` on these; this is not a policy choice):
  - `const fn`                      : const and async are mutually exclusive
  - `extern "C" fn`                 : the ABI is fixed
  - impls of traits declared OUTSIDE this repo (Drop/Display/Iterator/Future/serde/...)
  - `fn main`                       : needs a runtime attribute instead, handled separately
  - already-`async fn`              : idempotent by design, safe to re-run

Usage:
  asyncify-universal.py --scan  <paths...>   # report only, changes nothing
  asyncify-universal.py --apply <paths...>   # rewrite in place
"""
import os, re, sys, json

SKIP_DIRS = ('/target', '/node_modules', '/.git', '/vendor', '/storybook-static')

FN_RE = re.compile(
    r'^(?P<indent>\s*)'
    r'(?P<vis>pub(?:\([^)]*\))?\s+)?'
    r'(?P<constkw>const\s+)?'
    r'(?P<unsafekw>unsafe\s+)?'
    r'(?P<externkw>extern\s+"[^"]*"\s+)?'
    r'(?P<asynckw>async\s+)?'
    r'fn\s+(?P<name>[a-zA-Z0-9_]+)'
)
TRAIT_DECL_RE = re.compile(r'^\s*(?:pub(?:\([^)]*\))?\s+)?(?:unsafe\s+)?trait\s+([A-Za-z0-9_]+)')
IMPL_RE       = re.compile(r'^(?P<indent>\s*)impl(?:<[^>]*>)?\s+(?P<head>.+?)\s*\{?\s*$')
IMPL_FOR_RE   = re.compile(r'^\s*impl(?:<[^>]*>)?\s+(?P<trait>[A-Za-z0-9_:]+)(?:<[^>]*>)?\s+for\s+')

def iter_rs(paths):
    for root in paths:
        if os.path.isfile(root) and root.endswith('.rs'):
            yield root; continue
        for dp, dn, fn in os.walk(root):
            if any(s in dp for s in SKIP_DIRS): continue
            for f in fn:
                if f.endswith('.rs'): yield os.path.join(dp, f)

def collect_local_traits(paths):
    """🏷️ Every trait DECLARED in first-party code. An `impl X for Y` whose X is absent here is an
    external trait with a signature we cannot change, so its methods must stay sync."""
    local = set()
    for p in iter_rs(paths):
        try: text = open(p, encoding='utf-8', errors='replace').read()
        except Exception: continue
        for line in text.split('\n'):
            m = TRAIT_DECL_RE.match(line)
            if m: local.add(m.group(1))
    return local

def process(path, local_traits, apply):
    try: lines = open(path, encoding='utf-8', errors='replace').read().split('\n')
    except Exception: return None
    out, stats = [], {'converted': 0, 'const': 0, 'extern': 0, 'external_trait': 0, 'main': 0, 'already': 0}
    impl_stack = []   # (indent, is_external_trait_impl)
    for line in lines:
        stripped = line.strip()
        mi = IMPL_RE.match(line)
        if mi and stripped.startswith('impl'):
            mf = IMPL_FOR_RE.match(line)
            external = bool(mf) and mf.group('trait').split('::')[-1] not in local_traits
            impl_stack.append((len(mi.group('indent')), external))
        if stripped.startswith('}') and impl_stack:
            closing = len(line) - len(line.lstrip())
            while impl_stack and impl_stack[-1][0] >= closing: impl_stack.pop()
        m = FN_RE.match(line)
        if m:
            in_external = any(ext for _, ext in impl_stack)
            if m.group('asynckw'):      stats['already'] += 1
            elif m.group('constkw'):    stats['const'] += 1
            elif m.group('externkw'):   stats['extern'] += 1
            elif m.group('name') == 'main': stats['main'] += 1
            elif in_external:           stats['external_trait'] += 1
            else:
                head = m.group('indent') + (m.group('vis') or '') + (m.group('unsafekw') or '')
                line = head + 'async fn ' + line[m.end('name') - len(m.group('name')):].lstrip()
                line = re.sub(r'^(\s*(?:pub(?:\([^)]*\))?\s+)?(?:unsafe\s+)?async fn )\s*', r'\1', line)
                stats['converted'] += 1
        out.append(line)
    if apply and stats['converted']:
        open(path, 'w', encoding='utf-8').write('\n'.join(out))
    return stats

def main():
    if len(sys.argv) < 3 or sys.argv[1] not in ('--scan', '--apply'):
        print(__doc__); sys.exit(2)
    apply, paths = sys.argv[1] == '--apply', sys.argv[2:]
    local = collect_local_traits(['/Users/ueli/Documents/semio/🧰️framework', '/Users/ueli/Documents/semio/✏️s'])
    total = {'converted': 0, 'const': 0, 'extern': 0, 'external_trait': 0, 'main': 0, 'already': 0}
    files = 0
    for p in iter_rs(paths):
        s = process(p, local, apply)
        if not s: continue
        files += 1
        for k in total: total[k] += s[k]
    print(f"local traits known: {len(local)} | files: {files}")
    print(json.dumps(total, indent=2))

if __name__ == '__main__':
    main()
