#!/usr/bin/env python3
"""🪓️ Turns one inline `mod name { … }` of a plugin module file into a top-level module of its own file.

The inline body is dedented into `<new file>`; a `use super::*;` glob is replaced by explicit imports of exactly the
names the body reads (the parent's own items, and the parent's import leaves). Every crate path through the old
nesting (`crate::<parent>::<name>::…`) is rewritten to the new top-level module. The root declaration of the new
module is added by hand. Usage: extract-inline.py <parent file> <mod name> <new file>
"""
import importlib.util, os, re, sys
HERE = os.path.dirname(os.path.abspath(__file__))
spec = importlib.util.spec_from_file_location("move", os.path.join(HERE, "🚚️move.py"))
mv = importlib.util.module_from_spec(spec); spec.loader.exec_module(mv)
ri, PLUGIN = mv.ri, mv.PLUGIN
parent_f, name, new_f = os.path.join(PLUGIN, sys.argv[1]), sys.argv[2], os.path.join(PLUGIN, sys.argv[3])
paths = mv.module_paths()
parent_path = paths[parent_f]
raw = open(parent_f, encoding="utf-8").read(); cl = ri.clean(raw)
m = re.search(r"(?m)^pub(?:\([^)]*\))?\s+mod\s+" + name + r"\s*\{|^mod\s+" + name + r"\s*\{", cl)
ob = cl.index("{", m.start()); cb = ri.skip_group(cl, ob)
body = raw[ob + 1:cb - 1]
lines = [l[4:] if l.startswith("    ") else l for l in body.strip("\n").split("\n")]
text = "\n".join(lines) + "\n"
its, _ = ri.items(raw, cl, 0, len(cl))
defs = {it["name"] for it in its if it["name"] and it["kind"] not in ("use", "impl", "macro_call") and it["name"] != name}
used = set(mv.IDENT.findall(ri.clean(text)))
imports = []
for st, a, e in mv.statements(raw, cl):
    if ob <= st.start() < cb or re.search(r"\bpub\b", st.group(3)): continue
    for segs, alias, _, _ in mv.parse_use(raw[a:e]):
        bound = alias or segs[-1]
        if bound in used and bound not in ("*", "self"):
            p = "::".join(segs)
            if segs[0] == "super" and len(parent_path.split("::")) == 1: p = "crate::" + "::".join(segs[1:])
            imports.append(f"{st.group(2).strip() + ' ' if st.group(2).strip() else ''}use {p}{' as ' + alias if alias else ''};")
for d in sorted(defs & used): imports.append(f"use crate::{parent_path}::{d};")
own = mv.bound_names(text.replace("use super::*;", ""))
imports = [i for i in dict.fromkeys(imports) if i.rstrip(";").split("::")[-1].split(" as ")[-1] not in own]
text = text.replace("use super::*;\n", "\n".join(imports) + "\n", 1)
os.makedirs(os.path.dirname(new_f), exist_ok=True)
open(new_f, "w", encoding="utf-8").write(text)
start = cl.rfind("\n", 0, m.start()) + 1
k = start
while True:
    prev = raw.rfind("\n", 0, k - 1) + 1
    l = raw[prev:k].strip()
    if k > 0 and (l.startswith("///") or l.startswith("#[")): k = prev; continue
    break
open(parent_f, "w", encoding="utf-8").write(raw[:k] + raw[cb:].lstrip("\n"))
new_path = name
pat = re.compile(r"\b(crate|super)(?:::component)?::(?:app|" + re.escape(parent_path) + r")::" + name + r"\b")
n = 0
for f in mv.crate_files():
    s = open(f, encoding="utf-8").read()
    t = pat.sub(lambda mm: f"crate::{new_path}", s)
    if t != s: open(f, "w", encoding="utf-8").write(t); n += 1
print(f"[extract] {parent_path}::{name} -> {new_path} ({len(imports)} imports); rewrote {n} files")
