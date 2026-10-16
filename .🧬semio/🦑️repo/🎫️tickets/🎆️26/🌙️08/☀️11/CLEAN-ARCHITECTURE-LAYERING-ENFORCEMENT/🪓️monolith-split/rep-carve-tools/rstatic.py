"""🔎 Static checks of the staged carve: mount targets, path length, docstrings of new files, rustfmt parse, module ownership."""
import os, re, subprocess, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from rlib import *
import rcrates as C

def main():
    bad = 0
    owner = {}
    allfiles = []
    for k in C.KEYS + ["facade"]:
        root = FACADE_ROOT if k == "facade" else C.libroot(k)
        for f in crawl(root):
            if k != "facade" or True:
                owner.setdefault(f, []).append(k)
            for p in mount_targets(f):
                if not exists_stage_or_base(p): print("MISSING MOUNT", f.replace(R, ""), "->", p.replace(R, "")); bad += 1
    for f, ks in owner.items():
        if len(ks) > 1: print("OWNED BY SEVERAL", f.replace(R, ""), ks); bad += 1
    longest = 0
    for dp, dn, fn in os.walk(STAGE):
        for x in fn:
            rel = os.path.relpath(os.path.join(dp, x), STAGE)
            n = len(rel.encode()); longest = max(longest, n)
            if n > 240: print("PATH TOO LONG", n, rel); bad += 1
            if x == "🦀️.rs" and not rel.endswith(".deleted") and not os.path.exists(BASE + rel):
                first = open(STAGE + rel, encoding="utf8").readline()
                if not first.startswith("//!"): print("NO DOCSTRING", rel.replace(R, "")); bad += 1
    print("longest staged path bytes", longest)
    staged_rs = [os.path.join(dp, x) for dp, dn, fn in os.walk(STAGE) for x in fn if x.endswith(".rs")]
    fails = 0
    for i in range(0, len(staged_rs), 25):
        r = subprocess.run(["rustfmt", "+nightly-2026-07-20", "--edition", "2021", "--check", "--unstable-features", "--skip-children", "--emit", "stdout"] + staged_rs[i:i + 25], capture_output=True, text=True)
        for line in r.stderr.split("\n"):
            if line.startswith("error"):
                fails += 1; print("RUSTFMT", line[:160])
    print("rustfmt parse failures", fails, "of", len(staged_rs), "files")
    print("STATIC", "FAIL" if bad or fails else "OK")
main()
