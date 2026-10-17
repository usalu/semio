#!/usr/bin/env python3
"""✅️ Static gates after a group was applied: path literals, regions, rustfmt parse, TOML, manifest dependencies, single owner, orphan rule, census rows.
usage: postapply.py <crate>... [--census]"""
import json, os, subprocess, sys
from lib import *
import verify
import orphans
import world as Wd
import layout as L
import uo


def main():
    crates = [a for a in sys.argv[1:] if not a.startswith("--")]
    res = {}
    for K in crates:
        out, files = verify.run(K)
        print(json.dumps(out, ensure_ascii=False))
    print("shared files:", json.dumps(verify.single_owner(list(uo.real_crates())), ensure_ascii=False))
    W = Wd.World()
    LY = L.Layout(W)
    f = orphans.scan(W, LY, set(crates))
    print("orphan findings:", len(f))
    for x in f:
        print("  ", x)
    if "--census" in sys.argv:
        only = ",".join("semio-framework-plugin-" + k for k in crates)
        p = subprocess.run(["bun", "census.ts"], cwd=GEN, env=dict(os.environ, ONLY=only), capture_output=True, text=True)
        write(GEN + "/pi2-census-" + "-".join(crates)[:60] + ".log", p.stdout + p.stderr)
        print("census ->", GEN + "/pi2-census-" + "-".join(crates)[:60] + ".log")

main()
