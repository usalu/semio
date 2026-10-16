import os, re, sys, json, subprocess, collections
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from cfg import *
from model import MARK
from cargo import pkg_dir
def check(R, census_json=None):
    out = []
    files = []
    for c, info in CLUSTERS.items():
        files.append(os.path.join(R, info['root']))
    # region balance for every rs file under cluster nodes
    allrs = set(files)
    if census_json:
        for r in json.load(open(census_json)):
            for p, k, b, l in r.get('list', []):
                if p.endswith('.rs'): allrs.add(os.path.join(R, p))
    for p in sorted(allrs):
        t = open(p, encoding='utf-8').read(); st = []; ok = True
        for ln in t.split('\n'):
            m = MARK.match(ln)
            if not m: continue
            if m.group(2):
                if not st: ok = False; break
                st.pop()
            else: st.append(m.group(3))
        if st: ok = False
        if not ok: out.append(('region-unbalanced', p))
    # new file docstring
    seen = {}
    for c, info in CLUSTERS.items():
        p = os.path.join(R, info['root'])
        first = open(p, encoding='utf-8').readline()
        if not first.startswith('//! ' + info['emoji']): out.append(('docstring', c, first[:20]))
        seen.setdefault(info['emoji'], []).append(c)
    for e, cs in seen.items():
        if len(cs) > 1: out.append(('emoji-dup', e, cs))
    # path length
    for c in CLUSTERS:
        for f in ('Cargo.toml', '📋️project.json'):
            p = os.path.join(pkg_dir(c), f)
            if len(p.encode()) > 240: out.append(('path-too-long', p, len(p.encode())))
    # rustfmt parse
    for p in sorted(allrs):
        r = subprocess.run(['rustfmt', '+nightly-2026-07-20', '--edition', '2021', '--emit', 'stdout', '--config', 'skip_children=true,unstable_features=true', p], capture_output=True, text=True)
        if r.returncode != 0: out.append(('rustfmt', p.replace(R, ''), r.stderr[:300]))
    return out
if __name__ == '__main__':
    R = sys.argv[1]
    for x in check(R, sys.argv[2] if len(sys.argv) > 2 else None): print(x)
    print('verify done')
