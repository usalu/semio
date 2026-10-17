#!/usr/bin/env python3
"""📦️ Scaffolds one new plugin test crate (class A, lib root `<dir>/🦀️.rs`): manifest, `📋️project.json`, workspace member, owner row, six launch routes in both launch files, and the `crates.json` row.
usage: mkcrate.py <key> <dir relative to PLUG> <rank> "<description>" [--apply]"""
import json, os, re, sys, copy
from lib import *
import ed

key, reldir, rank, desc = sys.argv[1], sys.argv[2], float(sys.argv[3]), sys.argv[4]
apply = "--apply" in sys.argv
CNAME = "semio-framework-plugin-" + key
PID = "@semio-tech/plugin-" + key + "-rs"
DIR = PLUG + "/" + reldir
PKG = DIR + "/" + PKGDIR
ROOT_CARGO, OWNERS = REPO + "/Cargo.toml", REPO + "/📋️build-owners.json"
LAUNCH, SEED = REPO + "/.vscode/launch.json", REPO + "/.vscode/🧩️launch.seed.jsonc"
SEED_ENV = '\n      "env": {"BUN_RUNTIME_TRANSPILER_CACHE_PATH": "0", "NODE_DISABLE_COMPILE_CACHE": "1"}'
ANCHOR = '"name": "🦀️ semio-framework-os-mcp-http-transport 🧪️test 🚀️release"'
BAND = {"testing-fixtures": "36"}.get(key, "37")

MANIFEST = f'''[package]
workspace = "{os.path.relpath(REPO, PKG)}"
name = "{CNAME}"
version = "0.1.0"
edition = "2021"
rust-version.workspace = true
description = "{desc}"
publish = false

[package.metadata.semio]
role = "test"
id = "plugin-{key}"

[package.metadata.semio.rust-source-boundary]
maxSourceBytes = 262144
maxSourceFiles = 32
maxActiveLines = 4000

[lints]
workspace = true

[lib]
path = "../../🦀️.rs"

[dependencies]

[dev-dependencies]
'''
exec(open(os.path.join(os.path.dirname(os.path.abspath(__file__)), "mkcaller.py")).read().split("def members(text, rel):")[1].split("def main():")[0].join(["def members(text, rel):", ""]).replace("orders = {\"build\": band(\"900.35\"), \"check\": band(\"922.35\"), \"test\": band(\"903.35\")}", "orders = {\"build\": band(\"900.%s\"), \"check\": band(\"922.%s\"), \"test\": band(\"903.%s\")}" % (BAND, BAND, BAND)))


def main():
    cargo, own, la, se = read(ROOT_CARGO), read(OWNERS), read(LAUNCH), read(SEED)
    rel_member = os.path.relpath(PKG, REPO)
    cargo2 = members(cargo, rel_member)
    own2 = owner(own, PID, rel_member, os.path.relpath(PKG + "/📋️project.json", REPO))
    blocks = routes([la, se])
    outs = {}
    for p, t in ((LAUNCH, la), (SEED, se)):
        if f'"name": "🦀️ {CNAME} 🏗️build 🛠️prototype"' in t:
            outs[p] = t
            continue
        i = t.index(ANCHOR)
        end = t.index("\n    },\n", i) + len("\n    },\n")
        outs[p] = t[:end] + blocks + t[end:]
    print("members", cargo2 != cargo, "owners", own2 != own, "launch", len(outs[LAUNCH]) - len(la), len(outs[SEED]) - len(se))
    if not apply:
        return
    ed.put(PKG + "/Cargo.toml", MANIFEST)
    ed.put(PKG + "/📋️project.json", json.dumps({"name": PID, "tags": ["scope:framework", "language:rust"]}, indent=2, ensure_ascii=False) + "\n")
    if not os.path.exists(DIR + "/🦀️.rs"):
        ed.put(DIR + "/🦀️.rs", f"//! 🧰️ {desc}.\n")
    ed.put(ROOT_CARGO, cargo2)
    ed.put(OWNERS, own2)
    for p, t in outs.items():
        ed.put(p, t)
    c = json.load(open(TOOLS + "/crates.json"))
    r = copy.deepcopy(c["testing-caller"])
    r.update(key=key, root=reldir, members=desc, name=CNAME, id="plugin-" + key, project=PID, rank=rank, desc=desc)
    r["dir"] = DIR.replace(REPO + "/", "")
    r["pkg"] = PKG.replace(REPO + "/", "")
    c[key] = r
    json.dump(c, open(TOOLS + "/crates.json", "w"), ensure_ascii=False, indent=1)
    f = json.load(open(TOOLS + "/flipped.json"))
    print("applied")


main()
