# ✅️ Static verification of an S0 cut output dir against the baseline snapshot: losslessness, regions, path literals, rustfmt parse.
import sys, os, re, collections, subprocess, json
from common import *
OUT = sys.argv[1]
BASE = SCR + '/base/' + P
bad = 0
def norm(l):
    l = l.strip()
    l = re.sub(r'(?<![\w])pub\(super\) ', '', l, count=1) if l.startswith('pub(super) ') else l
    l = l.replace('pub(in super::super) ', 'pub(super) ', 1) if l.startswith('pub(in super::super) ') else l
    return l
def lines(t): return [norm(l) for l in t.split('\n') if l.strip()]
changed = []
for dp, dn, fn in os.walk(OUT):
    for f in fn:
        if f.endswith('.rs'): changed.append(os.path.relpath(os.path.join(dp, f), OUT))
changed.sort()
print(len(changed), 'files in', OUT)
# group by source: lossless is checked over the union of all changed files vs the baseline of the sources
src_files = [c for c in changed if os.path.exists(BASE + c)]
new_files = [c for c in changed if c not in src_files]
before = collections.Counter(); after = collections.Counter()
for c in src_files: before.update(lines(open(BASE + c, encoding='utf-8').read()))
for c in changed:
    t = open(os.path.join(OUT, c), encoding='utf-8').read()
    after.update(lines(t))
lost = before - after; gained = after - before
print('baseline non-blank lines', sum(before.values()), 'after', sum(after.values()), 'lost', sum(lost.values()), 'gained', sum(gained.values()))
open(GEN + '/pwp4-lossless-lost.log', 'w').write('\n'.join(f'{v}\t{k}' for k, v in sorted(lost.items())))
open(GEN + '/pwp4-lossless-gained.log', 'w').write('\n'.join(f'{v}\t{k}' for k, v in sorted(gained.items())))
# regions balanced
for c in changed:
    t = open(os.path.join(OUT, c), encoding='utf-8').read()
    st = []
    for l in t.split('\n'):
        m = re.match(r'^\s*//#(region|endregion)\b(.*)$', l)
        if m:
            if m.group(1) == 'region': st.append(m.group(2).strip())
            else:
                if not st: print('UNBALANCED endregion', c, l); bad += 1
                else: st.pop()
    if st: print('UNBALANCED open regions', c, st); bad += 1
# path literals resolve (against the out dir first, then the repo)
RX = re.compile(r'(?:#\[\s*path\s*=\s*"([^"]*)"|\binclude(?:_str|_bytes)?!\s*\(\s*"([^"]*)")')
for c in changed:
    t = open(os.path.join(OUT, c), encoding='utf-8').read()
    d = os.path.dirname(PLUG + '/' + c)
    for m in RX.finditer(t):
        lit = m.group(1) or m.group(2)
        tgt = os.path.normpath(os.path.join(d, lit))
        rel = os.path.relpath(tgt, PLUG)
        if not (os.path.exists(tgt) or os.path.exists(os.path.join(OUT, rel))):
            print('PATH MISSING', c, lit); bad += 1
# rustfmt parse
fails = 0
for c in changed:
    p = os.path.join(OUT, c)
    r = subprocess.run(['rustfmt', '+nightly-2026-07-20', '--edition', '2021', '--emit', 'stdout', '--config', 'skip_children=true', p], capture_output=True, text=True)
    if r.returncode != 0 and 'error' in r.stderr.lower():
        fails += 1; print('RUSTFMT FAIL', c, r.stderr.strip().split('\n')[0][:200])
print('rustfmt parse failures', fails, 'problems', bad)
