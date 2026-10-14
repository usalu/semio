#!/usr/bin/env python3
"""🧰️ Lifts an editor's shared app test harness (`mod context { … }` inside its unit-test file) into its own file that
every test target mounts through `#[path]` (split lane P7).

After the family cut the panels, modes and tools tests are `[[test]]` targets of lower crates and named the harness as
`<editor>::unit_tests::context`, a `#[cfg(test)]` module that never exists in a dependency build. The harness file
spells the editor by its crate name, so it compiles inside any test crate; each consumer root mounts it as `context`.
With `--unit-target <name>` the editor's own `#[cfg(test)] unit_tests` lib module becomes that `[[test]]` target first.

usage: 🧪️shared-harness.py <cut config> <unit file> <support file> <editor package> [--unit-target <name>] plan|apply
"""
import json, os, re, sys
HERE = os.path.dirname(os.path.abspath(__file__))
REPO = os.path.abspath(os.path.join(HERE, *[".."] * 8))
args = sys.argv[1:]
mode = args.pop()
unit_target = None
if "--unit-target" in args:
    i = args.index("--unit-target")
    unit_target = args[i + 1]
    del args[i:i + 2]
config_path, unit_rel, support_rel, editor = args
config = json.load(open(config_path, encoding="utf-8"))
artifact = os.path.join(REPO, config["artifact"])
ident = editor.replace("-", "_")
unit = os.path.join(artifact, unit_rel)
support = os.path.join(artifact, support_rel)
writes = {}


def block(text, start):
    i = text.index("{", start)
    depth, j = 0, i
    while True:
        depth += text[j] == "{"
        depth -= text[j] == "}"
        if depth == 0:
            return i, j + 1
        j += 1


text = open(unit, encoding="utf-8").read()
m = re.search(r"(?m)^pub\(crate\) mod context \{", text)
a, b = block(text, m.end() - 1)
body = text[a + 1:b - 1].strip("\n")
body = "\n".join(line[4:] if line.startswith("    ") else line for line in body.split("\n"))
body = re.sub(r"(?m)^use super::(?:super::)?\*;", f"use {ident}::*;", body)
docs = [l for l in body.split("\n") if l.startswith("//!")]
rest = "\n".join(l for l in body.split("\n") if not l.startswith("//!")).strip("\n")
docs = docs or ["//! 🧰️ The one shared app test harness — every test target of the family mounts it as `context`."]
aliases = config.get("aliases", {})
used = [f"extern crate {t} as {al};" for al, t in aliases.items() if re.search(r"(?<![\w.$])" + re.escape(al) + r"\s*::", rest)]
writes[support] = "\n".join(docs) + "\n\n" + rest + "\n"
mount = f'#[path = "{os.path.relpath(support, os.path.dirname(unit))}"]\nmod context;'
text = text[:m.start()] + mount + text[b:]
text = text.replace("crate::unit_tests::context::", "crate::context::").replace("unit_tests::context::", "context::")
if unit_target:
    text = re.sub(r"(?m)^use super::\*;", f"use {ident}::*;", text)
    text = re.sub(r"(?<![\w:$])crate::(?!context::|\{context::)", ident + "::", text)
    text = re.sub(r"(?<![\w:$])super::", ident + "::", text)
writes[unit] = text
consumers = []
for t in config.get("tests", []) + config.get("existing_tests", []):
    path = os.path.join(artifact, t["file"])
    if path == unit:
        continue
    src = writes.get(path) or open(path, encoding="utf-8").read()
    src = re.sub(r"use " + re.escape(ident) + r"::\{((?:\s*unit_tests::context::[^,}]+,?)+)\s*\};", lambda x: "use crate::context::{" + ", ".join(p.strip().replace("unit_tests::context::", "") for p in x.group(1).split(",") if p.strip()) + "};", src)
    if ident + "::unit_tests::context" not in src and "use crate::context::{" not in src:
        continue
    src = src.replace(ident + "::unit_tests::context", "crate::context")
    lines = src.split("\n")
    k = 0
    while k < len(lines) and (lines[k].startswith("//!") or lines[k].startswith("#![") or lines[k].startswith("extern crate") or not lines[k].strip()):
        k += 1
    lines[k:k] = [f'#[path = "{os.path.relpath(support, os.path.dirname(path))}"]', "mod context;"]
    writes[path] = "\n".join(lines)
    consumers.append(t["name"])
if unit_target:
    owner_root = None
    for dirpath, _, files in os.walk(artifact):
        if "🦀️.rs" in files:
            candidate = os.path.join(dirpath, "🦀️.rs")
            src = writes.get(candidate) or open(candidate, encoding="utf-8").read()
            pattern = re.compile(r'(?m)^[ \t]*#\[cfg\(test\)\]\n[ \t]*#\[path = "' + re.escape(os.path.relpath(unit, dirpath)) + r'"\]\n[ \t]*(?:pub(?:\(crate\))? )?mod unit_tests;\n')
            if pattern.search(src):
                writes[candidate] = pattern.sub("", src)
                owner_root = candidate
    assert owner_root, "unit_tests mount not found"
    if not any(t["name"] == unit_target for t in config["tests"]):
        config["tests"].append({"package": editor, "file": unit_rel, "name": unit_target})
for path in writes:
    print(f"{'write' if os.path.exists(path) else 'create'} {os.path.relpath(path, artifact)}")
print(f"consumers={consumers} extern-aliases-needed={used}")
if mode == "apply":
    for path, content in writes.items():
        os.makedirs(os.path.dirname(path), exist_ok=True)
        open(path, "w", encoding="utf-8").write(content)
    json.dump(config, open(config_path, "w", encoding="utf-8"), indent=1, ensure_ascii=False)
