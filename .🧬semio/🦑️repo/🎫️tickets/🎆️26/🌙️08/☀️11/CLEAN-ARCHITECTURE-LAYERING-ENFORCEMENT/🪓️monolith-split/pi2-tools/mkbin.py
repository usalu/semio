#!/usr/bin/env python3
"""🍓️ Scaffolds one more companion test bin `tests-NN` of the plugin SDK partition: folder `🧷️companions/<emoji>️tests-NN`, manifest in the skeleton shape of its siblings, `📋️project.json`, workspace member, owner row, six launch routes in both launch files (orders continue the +2 per rank sequence of the siblings), and the `crates.json` row.
usage: mkbin.py <NN> <emoji> "<description>" [--apply]"""
import copy, json, os, re, sys
from decimal import Decimal
from lib import *
import ed

N, EMOJI, DESC = int(sys.argv[1]), sys.argv[2], sys.argv[3]
apply = "--apply" in sys.argv
KEY = f"tests-{N:02d}"
CNAME = "semio-framework-plugin-" + KEY
PID = f"@semio-tech/plugin-{KEY}-rs"
DIR = PLUG + f"/🧷️companions/{EMOJI}️{KEY}"
PKG = DIR + "/" + PKGDIR
ROOT_CARGO, OWNERS = REPO + "/Cargo.toml", REPO + "/📋️build-owners.json"
LAUNCH, SEED = REPO + "/.vscode/launch.json", REPO + "/.vscode/🧩️launch.seed.jsonc"
SEED_ENV = ',\n      "env": {\n        "BUN_RUNTIME_TRANSPILER_CACHE_PATH": "0",\n        "NODE_DISABLE_COMPILE_CACHE": "1"\n      }'

MANIFEST = f'''[package]
workspace = "{os.path.relpath(REPO, PKG)}"
name = "{CNAME}"
version = "0.1.0"
edition = "2021"
rust-version.workspace = true
description = "{DESC}"
publish = false

[package.metadata.semio]
role = "framework"
id = "plugin-{KEY}"

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

ROOT_RS = f"//! {EMOJI}️ Companion test crate {KEY} of the plugin SDK partition: {DESC.split(': ', 1)[-1]}.\n"


def members(text, rel):
    m = re.search(r"members = \[\n(.*?)\n\]", text, re.S)
    lines = m.group(1).split("\n")
    existing = [l.strip().strip(",").strip('"') for l in lines]
    if rel in existing:
        return text
    pos = next((i for i, e in enumerate(existing) if e.encode() > rel.encode()), len(existing))
    lines.insert(pos, f'    "{rel}",')
    return text[: m.start(1)] + "\n".join(lines) + text[m.end(1):]


def owner(text, name, root, manifest):
    if f'"name": "{name}"' in text:
        return text
    block = f'    {{\n      "name": "{name}",\n      "root": "{root}",\n      "manifest": "{manifest}",\n      "packages": []\n    }},\n'
    for m in re.finditer(r'    \{\n      "name": "([^"]+)"', text):
        if m.group(1).encode() > name.encode():
            return text[:m.start()] + block + text[m.start():]
    raise SystemExit("owner position")


def fmt(d):
    s = format(d.normalize(), "f")
    return s


def routes():
    steps = Decimal(N - 7) * Decimal("0.00002")
    base = {"build": Decimal("900.3013"), "check": Decimal("922.0633"), "test": Decimal("903.6024")}
    out = []
    for target, label, group in (("build", "🏗️build", "4_build"), ("check", "🔎️check", "4_gate"), ("test", "🧪️test", "4_test")):
        for ci, (conf, clabel) in enumerate((("prototype", "🛠️prototype"), ("release", "🚀️release"))):
            order = fmt(base[target] + steps + Decimal(ci) * Decimal("0.00001"))
            out.append(f'''    {{
      "name": "🦀️ {CNAME} {label} {clabel}",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun repo run {PID}:{target} --configuration={conf} --parallel=1 -- --lib --offline",
      "cwd": "${{workspaceFolder}}",
      "presentation": {{
        "group": "{group}",
        "order": {order}
      }}{SEED_ENV}
    }},
''')
    return "".join(out)


def insert_routes(text, prev_key):
    if f'"name": "🦀️ {CNAME} 🏗️build 🛠️prototype"' in text:
        return text
    anchor = f'"name": "🦀️ semio-framework-plugin-{prev_key} 🧪️test 🚀️release"'
    i = text.index(anchor)
    end = text.index("\n    },\n", i) + len("\n    },\n")
    return text[:end] + routes() + text[end:]


def main():
    prev = f"tests-{N-1:02d}" if N != 10 else "tests-09"
    if N == 8:
        prev = "tests-07"
    if N == 9:
        prev = "tests-08"
    cargo, own, la, se = read(ROOT_CARGO), read(OWNERS), read(LAUNCH), read(SEED)
    rel_member = os.path.relpath(PKG, REPO)
    cargo2 = members(cargo, rel_member)
    own2 = owner(own, PID, rel_member, os.path.relpath(PKG + "/📋️project.json", REPO))
    la2, se2 = insert_routes(la, prev), insert_routes(se, prev)
    print("members", cargo2 != cargo, "owners", own2 != own, "launch", len(la2) - len(la), len(se2) - len(se))
    if not apply:
        print(routes()[:600])
        return
    ed.put(PKG + "/Cargo.toml", MANIFEST)
    ed.put(PKG + "/📋️project.json", json.dumps({"name": PID, "tags": ["scope:framework", "language:rust"]}, indent=2, ensure_ascii=False) + "\n")
    if not os.path.exists(DIR + "/🦀️.rs"):
        ed.put(DIR + "/🦀️.rs", ROOT_RS)
    for path, old, new in ((ROOT_CARGO, cargo, cargo2), (OWNERS, own, own2), (LAUNCH, la, la2), (SEED, se, se2)):
        cur = read(path)
        assert cur == old, ("a peer edited", path)
        if new != old:
            ed.put(path, new)
    c = json.load(open(TOOLS + "/crates.json"))
    r = copy.deepcopy(c["tests-07"])
    r.update(key=KEY, root=f"🧷️companions/{EMOJI}️{KEY}", members=DESC, hosts=["TOP"], deps=[], name=CNAME, id="plugin-" + KEY, project=PID, rank=c["tests-21"]["rank"] + (N - 21 if N > 21 else 0) + (0 if N > 21 else 0), emoji=EMOJI + "️", desc=DESC)
    r["dir"] = DIR.replace(REPO + "/", "")
    r["pkg"] = PKG.replace(REPO + "/", "")
    c[KEY] = r
    json.dump(c, open(TOOLS + "/crates.json", "w"), ensure_ascii=False, indent=1)
    print("applied")


main()
