"""🎨 c4p19 rustfmt: hunk counts (`rustfmt --check`, repo config, skip_children) of each snapshot versus the staged or live root and children, plus a parse check (--emit stdout) of every file.
usage: fmt.py stage|live"""
import os, re, subprocess, sys, importlib, json
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import c4p19 as P
from mkspec import TABLE

CFG = ["+nightly-2026-07-20", "--edition", "2021", "--config-path", P.REPO, "--config", "skip_children=true"]
mode = sys.argv[1]


def hunks(path):
    p = subprocess.run(["rustfmt"] + CFG + ["--check", path], capture_output=True, text=True)
    return len(re.findall(r"^Diff in ", p.stdout, flags=re.M)), p.returncode


def parses(path):
    p = subprocess.run(["rustfmt"] + CFG + ["--emit", "stdout", path], capture_output=True, text=True)
    return p.returncode == 0, p.stderr.strip()[:300]


tb = tn = 0
bad = []
nfiles = 0
for tag, sp in TABLE.items():
    snap = os.path.join(P.SCR, 'fmt-' + tag + '.rs')
    open(snap, 'w', encoding='utf-8').write(open(os.path.join(P.T, 'orig', sp['orig']), encoding='utf-8').read())
    hb, _ = hunks(snap)
    if mode == 'stage':
        root = os.path.join(P.SCR, 'stage', tag)
        files = [os.path.join(root, '🦀️.rs')] + [os.path.join(root, c['folder'], '🦀️.rs') for c in sp['children']]
    else:
        base = os.path.dirname(os.path.join(P.REPO, sp['file']))
        files = [os.path.join(P.REPO, sp['file'])] + [os.path.join(base, c['folder'], '🦀️.rs') for c in sp['children']]
    hn = 0
    for f in files:
        h, _ = hunks(f)
        ok, err = parses(f)
        nfiles += 1
        if not ok:
            bad.append((f[-70:], err))
        hn += h
    tb += hb
    tn += hn
    print('[%s] hunks snapshot=%d %s=%d' % (tag, hb, mode, hn))
print('TOTAL hunks snapshot', tb, mode, tn, 'files parsed', nfiles, 'parse failures', len(bad))
for b in bad:
    print('  FAIL', b)
