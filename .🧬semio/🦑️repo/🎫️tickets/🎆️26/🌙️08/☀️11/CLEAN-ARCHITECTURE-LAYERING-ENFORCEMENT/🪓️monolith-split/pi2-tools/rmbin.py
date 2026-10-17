#!/usr/bin/env python3
"""🧹️ Retires companion test bins `tests-NN` that the final pack leaves empty: workspace member, build owner, the six launch routes in both launch files, the registry row and the stub folder.
usage: rmbin.py <NN>... [--apply]"""
import json, os, re, shutil, sys
from lib import *
import ed

apply = "--apply" in sys.argv
nums = [int(a) for a in sys.argv[1:] if a.isdigit()]
ROOT_CARGO, OWNERS = REPO + "/Cargo.toml", REPO + "/📋️build-owners.json"
LAUNCH, SEED = REPO + "/.vscode/launch.json", REPO + "/.vscode/🧩️launch.seed.jsonc"
crates = json.load(open(TOOLS + "/crates.json"))


def drop_routes(text, cname):
    out, n = text, 0
    while True:
        m = re.search(r'    \{\n      "name": "🦀️ ' + re.escape(cname) + r' [^"]*",\n.*?\n    \},\n', out, re.S)
        if not m:
            return out, n
        out = out[:m.start()] + out[m.end():]
        n += 1


for N in nums:
    key = f"tests-{N:02d}"
    row = crates.get(key)
    assert row, key
    cname, pid, d = row["name"], row["project"], REPO + "/" + row["dir"]
    cargo, own, la, se = read(ROOT_CARGO), read(OWNERS), read(LAUNCH), read(SEED)
    rel_member = row["pkg"]
    cargo2 = re.sub(r'\n    "' + re.escape(rel_member) + r'",', "", cargo)
    own2 = re.sub(r'    \{\n      "name": "' + re.escape(pid) + r'",\n.*?\n    \},\n', "", own, flags=re.S)
    la2, nl = drop_routes(la, cname)
    se2, ns = drop_routes(se, cname)
    stub = [l for l in (read(d + "/🦀️.rs").split("\n") if os.path.exists(d + "/🦀️.rs") else []) if l.strip() and not l.startswith("//!")]
    print(key, "member", cargo2 != cargo, "owner", own2 != own, "routes", nl, ns, "stub-only", not stub)
    assert not stub, ("bin has content", key)
    if apply:
        for path, old, new in ((ROOT_CARGO, cargo, cargo2), (OWNERS, own, own2), (LAUNCH, la, la2), (SEED, se, se2)):
            if new != old:
                assert read(path) == old
                ed.put(path, new)
        if os.path.isdir(d):
            for root_, _, fs in os.walk(d):
                for f in fs:
                    ed.delete(os.path.join(root_, f)) if hasattr(ed, "delete") else os.remove(os.path.join(root_, f))
            shutil.rmtree(d, ignore_errors=True)
        del crates[key]
if apply:
    json.dump(crates, open(TOOLS + "/crates.json", "w"), ensure_ascii=False, indent=1)
