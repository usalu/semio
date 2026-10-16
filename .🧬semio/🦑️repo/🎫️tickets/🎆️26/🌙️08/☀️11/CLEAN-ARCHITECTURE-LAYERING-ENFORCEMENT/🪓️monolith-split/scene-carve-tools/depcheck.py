"""🔗 Per-crate dependency completeness: compile every crate root alone and compare the unresolved external crate names with the manifest (analysis only)."""
import os, re, shutil, sys
import oracle
from lib import *
from model import *

def unresolved(msgs):
    names = set()
    for (code, msg, loc), n in msgs.items():
        m = re.search(r"unresolved import `(\w+)`", msg) or re.search(r"cannot find (?:module or )?crate `(\w+)`", msg) or re.search(r"use of unresolved module or unlinked crate `(\w+)`", msg) or re.search(r"cannot find module or crate `(\w+)`", msg)
        if m:
            names.add(m.group(1))
    return names

def ident_of_dep(alias_or_pkg, spec):
    pkg = spec.get("package", alias_or_pkg) if isinstance(spec, dict) else alias_or_pkg
    return alias_or_pkg.replace("-", "_")

def run(tree):
    sb = SP + "/dep-sandbox"
    shutil.rmtree(sb, ignore_errors=True)
    os.makedirs(sb)
    shutil.copytree(tree, sb + "/🎬️scene")
    oracle.patch_tree(sb)
    bad = 0
    for k in CRATES:
        sec, lib, dev = None, set(), set()
        for line in rd(tree + "/" + CRATES[k]["node"] + "/📦️packages/🦀️rust/Cargo.toml").split("\n"):
            m = re.match(r"\[(.*)\]$", line.strip())
            if m:
                sec = m.group(1)
                continue
            m = re.match(r"([A-Za-z0-9_\-]+)\s*=", line)
            if m and sec == "dependencies":
                lib.add(m.group(1).replace("-", "_"))
            elif m and sec == "dev-dependencies":
                dev.add(m.group(1).replace("-", "_"))
        root = sb + "/🎬️scene/" + CRATES[k]["node"] + "/🦀️.rs"
        feats = ["--cfg", 'feature="testkit"'] if k in ("mesh", "camera") else []
        o1, _ = oracle.run(sb, root, CRATES[k]["ident"], feats)
        o2, _ = oracle.run(sb, root, CRATES[k]["ident"], ["--test"] + feats)
        u1, u2 = unresolved(o1), unresolved(o2)
        std = {"std", "core", "alloc"}
        u1 -= std; u2 -= std
        status = "ok"
        if u1 != lib & u1 or not u1 <= lib or (lib - u1):
            status = "LIB-MISMATCH undeclared=%s unused=%s" % (sorted(u1 - lib), sorted(lib - u1))
        if not u2 <= (lib | dev) or ((lib | dev) - u2):
            status += " TEST-MISMATCH undeclared=%s unused=%s" % (sorted(u2 - lib - dev), sorted((lib | dev) - u2))
        if status != "ok":
            bad += 1
        print(k, status)
    return bad

if __name__ == "__main__":
    run(sys.argv[1] if len(sys.argv) > 1 else NEW)
