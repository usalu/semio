#!/usr/bin/env python3
"""🎟️ Resolves the last shared file `🧪️testing/🎟️caller/🦀️.rs`: it becomes the lib root of the test crate `semio-framework-plugin-testing-caller` (dm2 §3.1 shape); the three monolith mounts become `use <crate> as original_caller;` and the testing root re-exports the fixture.
usage: mkcaller.py [--apply]"""
import json, os, re, sys
from lib import *
import ed

REPO_ = REPO
CNAME, PID = "semio-framework-plugin-testing-caller", "@semio-tech/plugin-testing-caller-rs"
DIR = PLUG + "/🧪️testing/🎟️caller"
PKG = DIR + "/" + PKGDIR
ROOT_CARGO, OWNERS = REPO + "/Cargo.toml", REPO + "/📋️build-owners.json"
LAUNCH, SEED = REPO + "/.vscode/launch.json", REPO + "/.vscode/🧩️launch.seed.jsonc"
SEED_ENV = '\n      "env": {"BUN_RUNTIME_TRANSPILER_CACHE_PATH": "0", "NODE_DISABLE_COMPILE_CACHE": "1"}'
ANCHOR = '"name": "🦀️ semio-framework-os-mcp-http-transport 🧪️test 🚀️release"'

MANIFEST = f'''[package]
workspace = "{os.path.relpath(REPO, PKG)}"
name = "{CNAME}"
version = "0.1.0"
edition = "2021"
rust-version.workspace = true
description = "Independently authored fixture caller authority of the plugin SDK tests: survives original settlement and terminal closure"
publish = false

[package.metadata.semio]
role = "test"
id = "plugin-testing-caller"

[package.metadata.semio.rust-source-boundary]
maxSourceBytes = 262144
maxSourceFiles = 32
maxActiveLines = 4000

[lints]
workspace = true

[lib]
path = "../../🦀️.rs"

[dependencies]
semio-framework-job = {{ path = "{os.path.relpath(REPO + '/🧰️framework/🔨️modules/🧵️job/📦️packages/🦀️rust', PKG)}", package = "semio-framework-job" }}
semio-framework-value = {{ path = "{os.path.relpath(PLUG + '/../../../../🔨️modules/🌱️value/📦️packages/🦀️rust', PKG)}" }}
serde = {{ version = "1.0.219", features = ["derive"] }}
serde_json = "1.0.140"

[dev-dependencies]
semio-framework-trace = {{ workspace = true }}
'''


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


def routes(texts):
    used = set()
    for t in texts:
        used |= set(re.findall(r'"order":\s*([0-9.]+)', t))
    def band(prefix):
        out, n = [], 1
        while len(out) < 2:
            v = f"{prefix}{n:03d}"
            n += 1
            if v not in used:
                out.append(v)
        return out
    orders = {"build": band("900.35"), "check": band("922.35"), "test": band("903.35")}
    out = []
    for target, label, group in (("build", "🏗️build", "4_build"), ("check", "🔎️check", "4_gate"), ("test", "🧪️test", "4_test")):
        for ci, (conf, clabel) in enumerate((("prototype", "🛠️prototype"), ("release", "🚀️release"))):
            out.append(f'''    {{
      "name": "🦀️ {CNAME} {label} {clabel}",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun repo run {PID}:{target} --configuration={conf} --parallel=1 -- --lib --offline",
      "cwd": "${{workspaceFolder}}",
      "presentation": {{
        "group": "{group}",
        "order": {orders[target][ci]}
      }}{"," + SEED_ENV}
    }},
''')
    return "".join(out)


def main():
    apply = "--apply" in sys.argv
    caller = read(DIR + "/🦀️.rs")
    cargo, own, la, se = read(ROOT_CARGO), read(OWNERS), read(LAUNCH), read(SEED)
    rel_member = os.path.relpath(PKG, REPO)
    cargo2 = members(cargo, rel_member)
    own2 = owner(own, PID, os.path.relpath(PKG, REPO), os.path.relpath(PKG + "/📋️project.json", REPO))
    blocks = routes([la, se])
    outs = {}
    for p, t in ((LAUNCH, la), (SEED, se)):
        if f'"name": "🦀️ {CNAME} 🏗️build 🛠️prototype"' in t:
            outs[p] = t
            continue
        i = t.index(ANCHOR)
        end = t.index("\n    },\n", i) + len("\n    },\n")
        outs[p] = t[:end] + blocks + t[end:]
    mounts = {
        PLUG + "/⚛️reactor/🩹️patches/🔁️retry/🧪️tests/🦀️.rs": ('#[doc(hidden)]\n#[path="../../../../🧪️testing/🎟️caller/🦀️.rs"]\npub mod original_caller;\n', "#[doc(hidden)]\npub use semio_framework_plugin_testing_caller as original_caller;\n"),
        PLUG + "/⚛️reactor/🩹️patches/🧪️tests/🔬️unit/🦀️.rs": ('#[doc(hidden)]\n#[path = "../../../../🧪️testing/🎟️caller/🦀️.rs"]\npub mod original_caller;\n', "#[doc(hidden)]\npub use semio_framework_plugin_testing_caller as original_caller;\n"),
        PLUG + "/⚛️reactor/🏃️runtime/🎞️command/📥️custody/🛂️manifest/🧪️tests/🦀️.rs": ('#[doc(hidden)]\n#[path="../../../../../../🧪️testing/🎟️caller/🦀️.rs"]\npub mod original_caller;\n', "#[doc(hidden)]\npub use semio_framework_plugin_testing_caller as original_caller;\n"),
        PLUG + "/🧪️testing/🦀️.rs": ('#[path="🎟️caller/🦀️.rs"]\nmod original_fixture_caller;\npub use original_fixture_caller::OriginalFixtureCaller;\n', "pub use semio_framework_plugin_testing_caller::OriginalFixtureCaller;\n"),
    }
    edits = {}
    for f, (old, new) in mounts.items():
        t = read(f)
        assert t.count(old) == 1, (rel(f), t.count(old))
        edits[f] = t.replace(old, new)
    print("members", cargo2 != cargo, "owners", own2 != own, "launch", len(outs[LAUNCH]) - len(la), len(outs[SEED]) - len(se), "mount edits", len(edits))
    if not apply:
        return
    ed.put(PKG + "/Cargo.toml", MANIFEST)
    ed.put(PKG + "/📋️project.json", json.dumps({"name": PID, "tags": ["scope:framework", "language:rust"]}, indent=2, ensure_ascii=False) + "\n")
    ed.put(ROOT_CARGO, cargo2)
    ed.put(OWNERS, own2)
    for p, t in outs.items():
        ed.put(p, t)
    for f, t in edits.items():
        ed.put(f, t)
    print("applied")


main()
