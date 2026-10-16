# ✅ Static verification of a built batch (before or after apply): rustfmt parse, region balance, path literals, corpus token law, longest path.
# usage: verify23.py <batch-name>   (works on the pending outputs of carve.solve, no write)
import os, re, subprocess, sys
HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE); sys.path.insert(0, HERE + '/../okwp0-tools')
import ns, model, carve
from okrs import tokenize

FORBID = {'semio_framework_os_kernel', 'semio_framework_plugin', 'os_store', 'os_spr', 'os_directory'}
MARK = re.compile(r'^\s*//#(end)?region\b(.*)$')
LIT = re.compile(r'(#\[\s*path\s*=\s*"|include(?:_str|_bytes)?!\(\s*")([^"]+)"')

def rustfmt_ok(text):
    p = subprocess.run(['rustfmt', '+nightly-2026-07-20', '--edition', '2021', '--emit', 'stdout'], input=text, capture_output=True, text=True)
    return p.returncode == 0 or ('error' not in p.stderr.lower()), p.stderr[:200]

def balanced(text):
    d = 0
    for l in text.split('\n'):
        m = MARK.match(l)
        if m:
            d += -1 if m.group(1) else 1
            if d < 0: return False
    return d == 0

def main():
    name = sys.argv[1]
    crates = open(HERE + f'/../🗑️generated/okwp23-{name}.crates').read().split()
    b = carve.solve(crates)
    pl = b.plan
    fails = 0
    longest = 0
    for p, t in sorted(b.out.items()):
        if not p.endswith('.rs'): continue
        ok, err = rustfmt_ok(t)
        if not ok: fails += 1; print('RUSTFMT', model.short(p)[-80:], err.replace('\n', ' ')[:160])
        old = open(p, encoding='utf8').read() if os.path.exists(p) else ''
        if not balanced(t) and balanced(old) is not False:
            fails += 1; print('REGIONS unbalanced', model.short(p)[-80:])
        for m in LIT.finditer(t):
            target = os.path.normpath(os.path.join(os.path.dirname(p), m.group(2)))
            if not os.path.exists(target) and target not in b.out:
                fails += 1; print('PATH missing', model.short(p)[-60:], m.group(2)[:80])
            longest = max(longest, len(os.path.relpath(target, ns.R).encode('utf8')))
        longest = max(longest, len(os.path.relpath(p, ns.R).encode('utf8')))
    print('longest relative path (bytes):', longest)
    for crate in crates:
        for f, c in pl.file_crate.items():
            if c != crate or b.is_test_file(f): continue
            for tk in tokenize(b.out[f]):
                if tk[0] == 'id' and tk[1] in FORBID:
                    fails += 1; print('TOKEN', crate, model.short(f)[-60:], tk[1]); break
        for (sf, c), st in pl.staging.items():
            if c != crate: continue
            for tk in tokenize(b.out[st.path]):
                if tk[0] == 'id' and tk[1] in FORBID:
                    fails += 1; print('TOKEN', crate, 'staging', model.short(st.path)[-60:], tk[1]); break
    print('failures', fails)

if __name__ == '__main__': main()
