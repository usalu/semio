#!/usr/bin/env python3
"""🧬️ Makes every mutation leaf of one union crate generic over the dispatch enum (P4/P5 D2, P9 N4).

For each kind module mounted by the union root (`pub mod <kind> { #[path = "<dir>/🦀️.rs"] mod component; … }`):
`impl MutationKind<S, Union> for Leaf` becomes `impl<Op> MutationKind<S, Op> for Leaf where Op: Mutation<S, Diff = D>
+ From<…>` with exactly the leaves its inverse emits, the inverse facet becomes `inverse<Op: From<…>>(…) -> Vec<Op>`,
union variant construction becomes `Op::from(…)`, and the leaf's builder returns any `Op: From<Leaf>`. Afterwards no
leaf names the union, so leaves can sit below it.

usage: 🧬️generic-leaves.py <mutations dir> <Union> <Snapshot> <Diff path> plan|apply
"""
import os, re, sys

IDENT = r"[A-Za-z_][A-Za-z0-9_]*"


def strip_comments(text):
    return re.sub(r"//[^\n]*", "", text)


def main():
    base, union, snap, diff, mode = sys.argv[1:6]
    root = open(os.path.join(base, "🦀️.rs"), encoding="utf-8").read()
    mods = re.findall(r'pub mod (' + IDENT + r') \{\n\s*#\[path = "([^/"]+)/🦀️.rs"\]', root)
    kinds = {}
    for name, d in mods:
        kind = open(os.path.join(base, d, "🦀️.rs"), encoding="utf-8").read()
        struct = re.search(r"#\[derive\([^\]]*MutationLeaf[^\]]*\)\](?:\s*#\[[^\]]*\])*\s*pub struct (" + IDENT + r")", kind).group(1)
        kinds[name] = (d, struct)
    out = {}
    for name, (d, struct) in kinds.items():
        kp = os.path.join(base, d, "🦀️.rs")
        ip = os.path.join(base, d, "↩️inverse", "🦀️.rs")
        kind = open(kp, encoding="utf-8").read()
        inv = open(ip, encoding="utf-8").read()
        emitted = []
        for other, (_, s) in kinds.items():
            if re.search(r"(?<!fn )\b" + other + r"\s*\(", strip_comments(inv)) or re.search(r"\b" + s + r"\s*\{", strip_comments(inv)):
                emitted.append((other, s))
        own_path = lambda other, s: s if other == name else f"crate::{other}::{s}"
        bounds = " + ".join(f"From<{own_path(o, s)}>" for o, s in emitted)
        header = re.search(r"impl\s+(?:protocol::)?MutationKind\s*<\s*" + re.escape(snap) + r"\s*,\s*" + re.escape(union) + r"\s*>\s+for\s+" + struct + r"\s*\{", kind)
        if not header:
            sys.exit(f"no MutationKind impl in {kp}")
        where = f"protocol::Mutation<{snap}, Diff = {diff}>" + (f" + {bounds}" if bounds else "")
        kind = kind[:header.start()] + f"impl<Op> protocol::MutationKind<{snap}, Op> for {struct}\nwhere\n    Op: {where},\n{{" + kind[header.end():]
        kind = re.sub(r"Result<\s*Vec<\s*" + re.escape(union) + r"\s*>\s*,", "Result<Vec<Op>,", kind)
        ctor = re.search(r"pub fn (" + IDENT + r")\(([^)]*)\)\s*->\s*" + re.escape(union) + r"\s*\{\s*" + re.escape(union) + r"::" + IDENT + r"\((.*?)\)\s*\}\n", kind, re.S)
        if ctor:
            kind = kind[:ctor.start()] + f"pub fn {ctor.group(1)}<Op: From<{struct}>>({ctor.group(2)}) -> Op {{\n    Op::from({ctor.group(3).strip()})\n}}\n" + kind[ctor.end():]
        kind = re.sub(r"(?m)^use crate::" + re.escape(union) + r";\n", "", kind)
        if re.search(r"\b" + re.escape(union) + r"\b", kind):
            print(f"LEFT {kp}: names {union}")
        generics = f"<Op: {bounds}>" if bounds else "<Op>"
        inv2 = re.sub(r"pub fn inverse\(", f"pub fn inverse{generics}(", inv, count=1)
        inv2 = re.sub(r"Result<\s*Vec<\s*" + re.escape(union) + r"\s*>\s*,", "Result<Vec<Op>,", inv2)
        inv2 = re.sub(re.escape(union) + r"::" + IDENT + r"\(", "Op::from(", inv2)
        inv2 = re.sub(r"(?m)^use crate::" + re.escape(union) + r";\n", "", inv2)
        if re.search(r"\b" + re.escape(union) + r"\b", inv2):
            print(f"LEFT {ip}: names {union}")
        print(f"{name}: emits {[s for _, s in emitted]}{' builder' if ctor else ''}")
        out[kp], out[ip] = kind, inv2
    if mode == "apply":
        for p, t in out.items():
            open(p, "w", encoding="utf-8").write(t)
        print(f"applied {len(out)}")


main()
