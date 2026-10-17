"""📏 C4P47 formatting-debt proof: rustfmt --check hunk counts (skip_children) of every snapshot versus the live root and children of the same file. usage: fmt.py"""
import collections, glob, importlib, json, os, re, subprocess, sys
HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
REPO = "/Users/ueli/Documents/semio"
CMD = ["rustfmt", "+nightly-2026-07-20", "--edition", "2021", "--check", "--config", "skip_children=true"]


def hunks(path):
    r = subprocess.run(CMD + [path], capture_output=True, text=True)
    err = r.stderr
    bad = ("error" in err.lower() and "Diff in" not in r.stdout and r.returncode not in (0, 1))
    lines = collections.Counter()
    for l in r.stdout.split("\n"):
        if l.startswith(("-", "+")) and not l.startswith(("---", "+++")):
            k = re.sub(r"\s+", "", l[1:].replace("pub(super) ", "").replace("../../", "../"))
            lines[l[0] + k] += 1
    return len(re.findall(r"^Diff in ", r.stdout, flags=re.M)), bad, lines


rows = []
for tag in ["gr", "shs", "cab", "ph", "dsu", "neu", "hu"]:
    sp = importlib.import_module("specs." + tag).SPEC
    base = os.path.join(REPO, os.path.dirname(sp["file"]))
    o, ob, ol = hunks(os.path.join(HERE, "orig", sp["orig"]))
    files = [os.path.join(REPO, sp["file"])] + [os.path.join(base, c["folder"], "🦀️.rs") for c in sp["children"]]
    n, nb, nl = 0, False, collections.Counter()
    for f in files:
        h, b, l = hunks(f)
        n += h
        nb = nb or b
        nl += l
    rows.append((tag, o, n, ob, nb))
    same = ol == nl
    print(f"{tag}: hunks original={o} new={n} parse-failures original={ob} new={nb} files={len(files)} changed-line multiset original={sum(ol.values())} new={sum(nl.values())} identical={same}")
    for k, v in list((ol - nl).items())[:5]:
        print("   ORIG-ONLY", v, k[:140])
    for k, v in list((nl - ol).items())[:5]:
        print("   NEW-ONLY", v, k[:140])
