#!/usr/bin/env python3
"""📇️ Moves an artifact schema descriptor function (`pub fn <x>_artifact_schema_descriptor()` with its `include_str!`
facet leaves) out of its schema root into `<schema dir>/📇️descriptor/🦀️.rs`, mounted as `<schema module>::descriptor`
in the crate root's schema tree, and retargets every `<schema module>::<fn>` path of the crate to `::descriptor::<fn>`.

usage: 📇️descriptor-extract.py <crate root.rs> <schema root.rs> <schema module path, e.g. schema> [apply]
"""
import os, re, sys
root, schema_file, module = sys.argv[1], sys.argv[2], sys.argv[3]
apply = len(sys.argv) > 4 and sys.argv[4] == "apply"
src = open(schema_file, encoding="utf-8").read()
m = re.search(r"(?ms)^((?:// [^\n]*\n|///[^\n]*\n)*)pub fn (\w+_artifact_schema_descriptor)\(\)[^{]*\{\n.*?^\}\n", src)
if not m:
    raise SystemExit("no descriptor in " + schema_file)
name = m.group(2)
block = m.group(0)
directory = os.path.join(os.path.dirname(schema_file), "📇️descriptor")
target = os.path.join(directory, "🦀️.rs")
moved = re.sub(r'include_str!\("([^"]+)"\)', lambda i: f'include_str!("../{i.group(1)}")', block)
text = f"//! 📇️ The schema descriptor: every schema facet leaf of this artifact (Rust, TypeScript, GraphQL, JSON Schema and\n//! protocol buffers), published to the schema registry by the artifact's registration.\n\n{moved}"
new_src = src.replace(block, "", 1)
root_src = open(root, encoding="utf-8").read()
start, end, indent = 0, len(root_src), ""
for seg in module.split("::"):
    found = None
    for m in re.finditer(r'(?m)^(?P<indent>[ \t]*)pub mod ' + re.escape(seg) + r' \{\n', root_src[start:end]):
        if len(m.group("indent")) == len(indent) + (4 if start else 0) or (not start and m.group("indent") == ""):
            found = m
            break
    if not found:
        raise SystemExit(f"no tree {seg} of {module} in {root}")
    open_at = start + found.end() - 2
    depth, j = 0, open_at
    while True:
        if root_src[j] == "{": depth += 1
        elif root_src[j] == "}":
            depth -= 1
            if depth == 0: break
        j += 1
    indent = found.group("indent")
    start, end = start + found.end(), j
rel = os.path.relpath(target, os.path.dirname(root))
mount = f'{indent}    #[path = "{rel}"]\n{indent}    pub mod descriptor;\n'
root_new = root_src[:start] + mount + root_src[start:]
print(f"{name} -> {os.path.relpath(target)}")
writes = {schema_file: new_src, target: text, root: root_new}
crate_dir = os.path.dirname(root)
for dirpath, dirnames, filenames in os.walk(crate_dir):
    dirnames[:] = [d for d in dirnames if d not in ("📦️packages", "target")]
    for f in filenames:
        if f == "🦀️.rs":
            p = os.path.join(dirpath, f)
            t = writes.get(p) or open(p, encoding="utf-8").read()
            n = re.sub(r"\b(crate::" + re.escape(module) + r")::" + name + r"\b", r"\1::descriptor::" + name, t)
            if p == root:
                n = re.sub(r"(?<![\w:])(" + re.escape(module) + r")::" + name + r"\b", r"\1::descriptor::" + name, n)
            glob = re.search(r"(?m)^use crate::" + re.escape(module) + r"::\*;\n", n)
            if glob and re.search(r"(?<![\w:])" + name + r"\s*\(", n) and "descriptor::" + name not in n:
                n = n[:glob.end()] + f"use crate::{module}::descriptor::{name};\n" + n[glob.end():]
            if n != t:
                writes[p] = n
                print("retarget", os.path.relpath(p, crate_dir))
if apply:
    os.makedirs(directory, exist_ok=True)
    for p, t in writes.items():
        open(p, "w", encoding="utf-8").write(t)
