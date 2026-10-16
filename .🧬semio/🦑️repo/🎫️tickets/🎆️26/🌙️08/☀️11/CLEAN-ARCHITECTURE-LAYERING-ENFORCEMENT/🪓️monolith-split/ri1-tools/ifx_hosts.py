#!/usr/bin/env python3
# 🧷️ Moves the two Interpreter unit-test files that name the monolith (command wiring, introspection) out of the ui-engine crates into test hosts of the WP3 hub.
import sys, os, re
sys.argv = [sys.argv[0], "--dry"]
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)) + "/w1")
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import ifx, items, needs
from ed import *
from rs import uses, blank
ifx.RANK["monolith"] = 3
PRE = SP + "/ri1/pre-ifx/Interpreter"

def old(suffix):
    for d in os.listdir(PRE):
        if d.endswith(suffix):
            return PRE + "/" + d + "/🦀️.rs"

HUB = R + "/📥️mounts/🧵️wp3-frame-core"
TESTS = EL + "/🗣️Interpreter/🧪️tests"
JOBS = [
    dict(slug="🧷️interpreter-command-wiring", module="interpreter_command_wiring_host", cfg='all(test, not(target_arch = "wasm32"))', parent_old=old("retained-ui-engine"), crate="ui_engine", mod="retained_ui_engine", test=TESTS + "/🔬️wgpu-ui-command-wiring/🦀️.rs",
         src=ifx.MODS["ui_engine"]["retained_ui_engine"], mount_text='#[cfg(all(test, not(target_arch = "wasm32")))]\n#[path = "../../../🧪️tests/🔬️wgpu-ui-command-wiring/🦀️.rs"]\nmod ui_command_wiring_tests;\n\n'),
    dict(slug="🔎️interpreter-introspection", module="interpreter_introspection_host", cfg="test", parent_old=old("introspection"), crate="ui_engine_dump", mod="introspection", test=TESTS + "/🔬️wgpu-introspection/🦀️.rs",
         src=ifx.MODS["ui_engine_dump"]["introspection"], mount_text='#[cfg(test)]\n#[path = "../../../🧪️tests/🔬️wgpu-introspection/🦀️.rs"]\nmod introspection_tests;\n'),
]

def widen_private(path, names):
    t = read(path)
    for n in names:
        pat = re.compile(r"(?m)^(\s*)((?:async\s+|unsafe\s+)*(?:fn|static|const|struct|enum|type)\s+)" + re.escape(n) + r"\b")
        ms = list(pat.finditer(t))
        if not ms:
            assert re.search(r"(?m)^\s*pub\s+(?:async\s+|unsafe\s+)*(?:fn|static|const|struct|enum|type)\s+" + re.escape(n) + r"\b", t), (path, n)
            continue
        t = pat.sub(lambda m: m.group(1) + "pub " + m.group(2) + n, t)
    write(path, t)

hub_text = read(HUB + "/🦀️.rs")
for j in JOBS:
    test = read(j["test"])
    parent_old = read(j["parent_old"])
    toks = needs.tokens(test)
    local = {it["name"] for it in items.table(test) if not it["name"].startswith("impl")}
    imports = {}
    for cfg, vis, path, loc, glob in uses(parent_old, top_only=True):
        if loc and not vis.startswith("pub"):
            imports.setdefault(loc, []).append((cfg, path))
    pitems = {it["name"]: it for it in items.table(parent_old) if it["kind"] not in ("impl", "use")}
    priv = sorted(n for n in toks & set(pitems) if n not in local and not pitems[n]["vis"].startswith("pub") and not pitems[n].get("thread_local") and pitems[n]["kind"] in ("fn", "static", "const", "struct", "enum", "type"))
    tl = sorted(n for n in toks & set(pitems) if n not in local and pitems[n].get("thread_local") and not pitems[n]["vis"].startswith("pub"))
    widen_private(j["src"], priv + tl)
    lines = []
    seen = set()
    for n in sorted(toks & set(imports)):
        if n in local:
            continue
        for cfg, path in imports[n]:
            mapped = ifx.map_path(path, "monolith") if len(path) >= 2 else None
            p = mapped if mapped is not None else path
            leaf = p[-1]
            line = (cfg + "\n" if cfg else "") + "use " + "::".join(p) + ("" if leaf == n else f" as {n}") + ";"
            if line not in seen:
                seen.add(line)
                lines.append(line)
    rel = os.path.relpath(j["test"], HUB + "/🧪️test-hosts/" + j["slug"])
    body = f'''//! {j["slug"][0]} Test host of the laws in `{os.path.basename(os.path.dirname(j["test"]))}`: they name the monolith (scenes, engine canvas, the frame fixtures), so they run in the renderer test unit with the namespace of the `{j["mod"]}` module of the `ui-engine` partition around them.

pub(crate) use semio_framework_os_renderer_wgpu_{j["crate"]}::{j["mod"]}::*;
''' + "\n".join(lines) + f'''

#[path = "{rel}"]
mod tests;
'''
    write(HUB + "/🧪️test-hosts/" + j["slug"] + "/🦀️.rs", body)
    hub_text += f'\n#[cfg({j["cfg"]})]\n#[path = "🧪️test-hosts/{j["slug"]}/🦀️.rs"]\nmod {j["module"]};\n'
    s = read(j["src"])
    assert s.count(j["mount_text"].strip("\n")) == 1, (j["mod"], s.count(j["mount_text"]))
    s = s.replace(j["mount_text"].strip("\n") + "\n", "")
    write(j["src"], s)
    print(j["mod"], "imports", len(lines), "widened", priv + tl)
write(HUB + "/🦀️.rs", hub_text)
