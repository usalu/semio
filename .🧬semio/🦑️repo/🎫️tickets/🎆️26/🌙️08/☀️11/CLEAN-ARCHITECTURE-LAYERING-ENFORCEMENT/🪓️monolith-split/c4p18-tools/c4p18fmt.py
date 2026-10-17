"""🎨 c4p18 rustfmt: hunk counts (`rustfmt --check`, repo config, skip_children) of each snapshot versus the staged (or live, with LIVE=1) files, plus a parse check of every file.
usage: c4p18fmt.py <tag>..."""
import os, re, subprocess, sys, importlib
T = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, T)
import c4p18run as R

ROOT = R.REPO
CFG = ["+nightly-2026-07-20", "--edition", "2021", "--config-path", ROOT, "--config", "skip_children=true"]


def hunks(path):
    p = subprocess.run(["rustfmt"] + CFG + ["--check", path], capture_output=True, text=True)
    return len(re.findall(r"^Diff in ", p.stdout, flags=re.M)), p.returncode, p.stderr.strip()[:200]


def parses(path):
    p = subprocess.run(["rustfmt"] + CFG + ["--emit", "stdout", path], capture_output=True, text=True)
    return p.returncode == 0, p.stderr.strip()[:200]


tb = tn = 0
bad = []
scratch = os.path.join(R.SCR, 'fmt')
os.makedirs(scratch, exist_ok=True)
for tag in sys.argv[1:]:
    sp = importlib.import_module('specs.' + tag).SPEC
    snap = os.path.join(scratch, tag + '.rs')
    open(snap, 'w', encoding='utf-8').write(open(os.path.join(T, 'orig', sp['orig']), encoding='utf-8').read())
    hb, rc, err = hunks(snap)
    files, _ = R.read_stage(tag)
    live = os.environ.get('LIVE')
    base = os.path.dirname(os.path.join(ROOT, sp['file']))
    hn = 0
    for rel in files:
        p = os.path.join(base, rel) if live else os.path.join(R.stage_dir(tag), rel)
        h, rc2, err2 = hunks(p)
        ok, perr = parses(p)
        if not ok:
            bad.append((tag, rel, perr))
        hn += h
    tb += hb
    tn += hn
    print(f"[{tag}] rustfmt hunks snapshot={hb} {'live' if live else 'staged'}-total={hn} files={len(files)}")
print("TOTAL hunks snapshot", tb, "new", tn, "parse failures", bad)
