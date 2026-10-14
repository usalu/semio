#!/usr/bin/env python3
"""🧺️ Gathers a mutation group's leaf fixture tests into one `[[test]]` target of the group crate (split lane P7).

The leaf fixture tests are mounted below the dispatch crate (`-mutations`) root, whose test unit then exceeds the Source
ceiling. Each group (objects, geometry, paint, …) gets one test root in its own package-only directory
(`🧬️mutations/<group>/🧪️tests/🦀️.rs`) that mounts the group's leaf tests under their former module names plus the shared
`laws` helpers; the leaf tests and `laws` spell the dispatch crate by name, and the dispatch root loses those mounts.

usage: 🧪️group-tests.py <cut config> <mutations dir> <laws file> <group package> <group dir> <target name> <leaf dir>... plan|apply
"""
import json, os, re, sys
HERE = os.path.dirname(os.path.abspath(__file__))
REPO = os.path.abspath(os.path.join(HERE, *[".."] * 8))
args = sys.argv[1:]
mode = args.pop()
config_path, mutations_rel, laws_rel, package, group_rel, target = args[:6]
leaves = args[6:]
config = json.load(open(config_path, encoding="utf-8"))
artifact = os.path.join(REPO, config["artifact"])
mutations_dir = os.path.join(artifact, mutations_rel)
mutations_root = os.path.join(mutations_dir, "🦀️.rs")
ident = next(c["package"] for c in config["crates"] if os.path.normpath(os.path.join(artifact, c["owner"])) == os.path.normpath(mutations_dir)).replace("-", "_")
laws = os.path.join(artifact, laws_rel)
root_file = os.path.join(artifact, group_rel, "🧪️tests", "🦀️.rs")
text = open(mutations_root, encoding="utf-8").read()
writes, mounts = {}, []
pattern = re.compile(r'(?m)^[ \t]*#\[cfg\(test\)\]\n[ \t]*#\[path = "(?P<path>(?:' + "|".join(re.escape(l) for l in leaves) + r')/🧪️tests/[^"]+)"\]\n[ \t]*mod (?P<name>\w+);\n')
for m in pattern.finditer(text):
    mounts.append((m.group("name"), os.path.join(mutations_dir, m.group("path"))))
text = pattern.sub("", text)
laws_mount = re.compile(r'(?m)^[ \t]*#\[cfg\(test\)\]\n[ \t]*#\[path = "' + re.escape(os.path.relpath(laws, mutations_dir)) + r'"\]\n[ \t]*pub mod laws;\n')
text = laws_mount.sub("", text)
writes[mutations_root] = text
for _, path in mounts:
    src = open(path, encoding="utf-8").read()
    src = re.sub(r"(?<![\w:$])super::laws", "crate::laws", src)
    src = re.sub(r"(?<![\w:$])crate::(?!laws\b)", ident + "::", src)
    writes[path] = src
law = open(laws, encoding="utf-8").read()
writes[laws] = re.sub(r"(?<![\w:$])crate::", ident + "::", law)
lines = [f"//! 🧺️ The `{os.path.basename(group_rel)}` mutation group's leaf fixture tests, run as one target of its group crate.", "",
         f'#[path = "{os.path.relpath(laws, os.path.dirname(root_file))}"]', "pub mod laws;"]
for name, path in mounts:
    lines += [f'#[path = "{os.path.relpath(path, os.path.dirname(root_file))}"]', f"mod {name};"]
writes[root_file] = "\n".join(lines) + "\n"
rel_root = os.path.relpath(root_file, artifact)
if not any(t["name"] == target for t in config["tests"]):
    config["tests"].append({"package": package, "file": rel_root, "name": target})
print(f"{target}: {len(mounts)} leaf tests into {rel_root}")
if mode == "apply":
    for path, content in writes.items():
        os.makedirs(os.path.dirname(path), exist_ok=True)
        open(path, "w", encoding="utf-8").write(content)
    json.dump(config, open(config_path, "w", encoding="utf-8"), indent=1, ensure_ascii=False)
