"""🎨 c4p16 rustfmt: hunk counts (`rustfmt --check`, repo config, skip_children) of each snapshot versus the staged or live files, plus a parse check of every file.
usage: c4p16fmt.py stage|live"""
import importlib, os, re, subprocess, sys
T = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, T)
import c4p16 as C
ROOT = C.REPO
CFG = ["+nightly-2026-07-20", "--edition", "2021", "--config-path", ROOT, "--config", "skip_children=true"]


def hunks(path):
    p = subprocess.run(["rustfmt"] + CFG + ["--check", path], capture_output=True, text=True)
    return len(re.findall(r"^Diff in ", p.stdout, flags=re.M)), p.returncode


def parses(path):
    p = subprocess.run(["rustfmt"] + CFG + ["--emit", "stdout", path], capture_output=True, text=True)
    return p.returncode == 0, p.stderr.strip()[:200]


mode = sys.argv[1]
KEYS = ['part21', 'document', 'header_diff', 'zip_io', 'mp4_io', 'bcf_oracles', 'step_geometry', 'xlsx_oracles']
tb = tn = 0
bad = []
for key in KEYS:
    sp = importlib.import_module('specs.' + key).SPEC
    snap = os.path.join(C.SCR, 'fmt-' + key + '.rs')
    os.makedirs(C.SCR, exist_ok=True)
    open(snap, 'w', encoding='utf-8').write(C.load_orig(sp))
    hb, rc = hunks(snap)
    base = os.path.dirname(sp['file'])
    hn = 0
    for rel in C.unit_files(sp):
        path = os.path.join(ROOT, base, rel) if mode == 'live' else os.path.join(C.SCR, 'stage', key, rel)
        h, rc2 = hunks(path)
        ok, perr = parses(path)
        if not ok:
            bad.append((rel, perr))
        hn += h
    tb += hb
    tn += hn
    print(f"[{key}] rustfmt hunks snapshot={hb} {mode}-total={hn} (rc snapshot {rc})")
print("TOTAL hunks snapshot", tb, mode, tn, "parse failures", bad)
