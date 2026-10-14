#!/usr/bin/env python3
"""🧬️ Wave 0: makes every remodeling mutation leaf generic over its dispatch enum. A leaf implements
`MutationKind<RemodelingSnapshot, Op>` for every `Op: Mutation<RemodelingSnapshot, Diff = RemodelingDiff>` that converts
`From` exactly the leaf payloads its inverse emits, and its builders return `Op`. This removes the leaf → enum edge so
leaves can live in group crates below the dispatch crate (lane P2)."""
import os, re
REPO = os.path.abspath(os.path.join(os.path.dirname(os.path.abspath(__file__)), *[".."] * 8))
M = os.path.join(REPO, "✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations")
root = open(os.path.join(M, "🦀️.rs"), encoding="utf-8").read()
PAYLOAD = dict(re.findall(r"use super::([a-z_]+)::([A-Za-z]+);", root))
IMPORT = re.compile(r"(?m)^use crate::standards::v1::subsets::any::schema::mutations::RemodelingMutation;\n")
def drop_import(text):
    text = IMPORT.sub("", text)
    def group(m):
        parts = [p.strip() for p in m.group(2).split(",") if p.strip() and p.strip() != "RemodelingMutation"]
        if not parts:
            return ""
        return m.group(1) + ("{" + ", ".join(parts) + "}" if len(parts) > 1 else parts[0]) + ";\n"
    return re.sub(r"(?m)^(use crate::standards::v1::subsets::any::schema::mutations::)\{([^}]*)\};\n", group, text)
def emits(text, own):
    found = []
    for module, payload in PAYLOAD.items():
        if re.search(r"(?<![\w])" + module + r"\s*\(", text):
            found.append(payload)
    if re.search(r"(?<![\w])rebind_asset\s*\(", text):
        found.append("CommitReconstruction")
    found += re.findall(r"RemodelingMutation::([A-Za-z]+)\(", text)
    return sorted(set(found))
def bounds(types):
    return " + ".join(f"From<{t}>" for t in types)
report = []
for leaf in sorted(os.listdir(M)):
    leaf_rs, inv_rs = os.path.join(M, leaf, "🦀️.rs"), os.path.join(M, leaf, "↩️inverse", "🦀️.rs")
    if not os.path.isfile(inv_rs):
        continue
    text, inv = open(leaf_rs, encoding="utf-8").read(), open(inv_rs, encoding="utf-8").read()
    own = re.search(r"impl protocol::MutationKind<RemodelingSnapshot, RemodelingMutation> for ([A-Za-z]+) \{", text).group(1)
    inverse_types = emits(inv, own)
    text = drop_import(text)
    def builder(m):
        name, args = m.group(1), m.group(2)
        body_types = emits(text[m.end():text.index("\n}\n", m.end())], own) or [own]
        return f"pub fn {name}<Op: {bounds(body_types)}>({args}) -> Op {{"
    text = re.sub(r"pub fn ([a-z_]+)\(([^)]*)\) -> RemodelingMutation \{", builder, text)
    where = "protocol::Mutation<RemodelingSnapshot, Diff = RemodelingDiff>" + (" + " + bounds(inverse_types) if inverse_types else "")
    text = text.replace(f"impl protocol::MutationKind<RemodelingSnapshot, RemodelingMutation> for {own} {{", f"impl<Op> protocol::MutationKind<RemodelingSnapshot, Op> for {own}\nwhere\n    Op: {where},\n{{")
    text = text.replace("Result<Vec<RemodelingMutation>,", "Result<Vec<Op>,")
    text = re.sub(r"RemodelingMutation::[A-Za-z]+\(", "Op::from(", text)
    inv = drop_import(inv)
    inv = re.sub(r"pub fn inverse\(", f"pub fn inverse<Op{(': ' + bounds(inverse_types)) if inverse_types else ''}>(", inv, count=1)
    inv = inv.replace("Result<Vec<RemodelingMutation>,", "Result<Vec<Op>,")
    inv = re.sub(r"RemodelingMutation::[A-Za-z]+\(", "Op::from(", inv)
    for path, new in ((leaf_rs, text), (inv_rs, inv)):
        assert "RemodelingMutation" not in new.replace("`RemodelingMutation`", ""), (path, [l for l in new.split("\n") if "RemodelingMutation" in l])
        open(path, "w", encoding="utf-8").write(new)
    report.append(f"{own}: inverse emits {inverse_types}")
print("\n".join(report))
