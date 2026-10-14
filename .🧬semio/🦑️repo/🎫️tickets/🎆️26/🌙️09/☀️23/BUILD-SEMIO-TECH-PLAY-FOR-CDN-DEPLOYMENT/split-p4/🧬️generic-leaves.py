#!/usr/bin/env python3
"""🧬️ Makes every mutation leaf of one `🧬️mutations` collection generic over its dispatch union (lane P4, decision D2).

Per leaf: `impl MutationKind<S, Union> for Leaf` becomes `impl<Op> MutationKind<S, Op> for Leaf where Op: Mutation<S, Diff = D>
+ From<Leaf> + From<…every payload its inverse emits>`; the builder `fn leaf(..) -> Union { Union::Leaf(Leaf { .. }) }` becomes
`fn leaf<Op: From<Leaf>>(..) -> Op { Op::from(Leaf { .. }) }`; the inverse `fn inverse(..) -> Result<Vec<Union>, _>` becomes
generic over the same bounds, and any direct `Union::X(..)` construction becomes `Op::from(..)`. Every import of the union
from a leaf, its diff or its inverse is dropped. The leaves then name no union at all.

usage: 🧬️generic-leaves.py <mutations dir> <Union> <Snapshot> <Diff> [--apply]
"""
import os, re, sys

root, union, snapshot, diff = sys.argv[1:5]
apply = "--apply" in sys.argv
leaves = sorted(d for d in os.listdir(root) if os.path.isfile(os.path.join(root, d, "🦀️.rs")) and os.path.isdir(os.path.join(root, d, "↩️inverse")))
builders = {}
for leaf in leaves:
    text = open(os.path.join(root, leaf, "🦀️.rs"), encoding="utf-8").read()
    for m in re.finditer(r"pub fn (\w+)\s*\(([^)]*)\)\s*->\s*" + union + r"\s*\{\s*" + union + r"::(\w+)\(", text):
        builders[m.group(1)] = m.group(3)


def drop_union_import(text):
    """✂️ Removes the union from every `use` line (whole line when it was the only leaf)."""
    def fix(m):
        body = m.group(2)
        if "{" in body:
            inner = body[body.index("{") + 1:body.rindex("}")]
            parts = [p.strip() for p in inner.split(",") if p.strip() and p.strip() != union]
            if not parts:
                return ""
            head = body[:body.index("{")]
            return f"{m.group(1)}use {head}{{{', '.join(parts)}}};" if len(parts) > 1 else f"{m.group(1)}use {head}{parts[0]};"
        if body.rstrip().endswith("::" + union):
            return ""
        return m.group(0)
    return re.sub(r"(?m)^([ \t]*)use\s+([^;]*" + union + r"[^;]*);[ \t]*\n?", lambda m: (fix(m) + "\n") if fix(m) else "", text)


def produced(text, own):
    kinds = set(re.findall(union + r"::(\w+)\(", text))
    for name in re.findall(r"\b(\w+)\s*\(", text):
        if name in builders:
            kinds.add(builders[name])
    kinds.discard(own)
    return sorted(kinds)


changed = 0
for leaf in leaves:
    path = os.path.join(root, leaf, "🦀️.rs")
    inv_path = os.path.join(root, leaf, "↩️inverse", "🦀️.rs")
    diff_path = os.path.join(root, leaf, "🔺️diff", "🦀️.rs")
    text = open(path, encoding="utf-8").read()
    inv = open(inv_path, encoding="utf-8").read()
    m = re.search(r"impl\s+protocol::MutationKind<\s*" + snapshot + r"\s*,\s*" + union + r"\s*>\s+for\s+(\w+)\s*\{", text)
    if not m:
        print(f"SKIP {leaf}: no MutationKind impl over {union}")
        continue
    own = m.group(1)
    extra = produced(inv, own)
    bounds = " + ".join([f"From<{own}>"] + [f"From<{k}>" for k in extra])
    text = text[:m.start()] + f"impl<Op> protocol::MutationKind<{snapshot}, Op> for {own}\nwhere\n    Op: protocol::Mutation<{snapshot}, Diff = {diff}> + {bounds},\n{{" + text[m.end():]
    text = re.sub(r"Result<Vec<" + union + r">,", "Result<Vec<Op>,", text)
    text = re.sub(r"pub fn (\w+)\s*\(([^)]*)\)\s*->\s*" + union + r"\s*\{\s*" + union + r"::(\w+)\((.*?)\)\s*\}",
                  lambda b: f"pub fn {b.group(1)}<Op: From<{b.group(3)}>>({b.group(2)}) -> Op {{\n    Op::from({b.group(4)})\n}}", text, flags=re.S)
    text = drop_union_import(text)
    if union in re.sub(r"//[^\n]*", "", text):
        print(f"WARN {leaf}: {union} still named in leaf")
    inv = re.sub(r"pub fn inverse\s*\(", f"pub fn inverse<Op: {bounds}>(", inv, count=1)
    inv = re.sub(r"Result<Vec<" + union + r">,", "Result<Vec<Op>,", inv)
    inv = re.sub(r"\b" + union + r"::(\w+)\(", "Op::from(", inv)
    inv = drop_union_import(inv)
    if union in re.sub(r"//[^\n]*", "", inv):
        print(f"WARN {leaf}: {union} still named in inverse")
    d = open(diff_path, encoding="utf-8").read()
    d2 = drop_union_import(d)
    if union in re.sub(r"//[^\n]*", "", d2):
        print(f"WARN {leaf}: {union} still named in diff")
    print(f"{leaf}: {own} bounds={bounds}")
    if apply:
        open(path, "w", encoding="utf-8").write(text)
        open(inv_path, "w", encoding="utf-8").write(inv)
        if d2 != d:
            open(diff_path, "w", encoding="utf-8").write(d2)
        changed += 1
print(f"leaves={len(leaves)} builders={len(builders)} changed={changed}")
