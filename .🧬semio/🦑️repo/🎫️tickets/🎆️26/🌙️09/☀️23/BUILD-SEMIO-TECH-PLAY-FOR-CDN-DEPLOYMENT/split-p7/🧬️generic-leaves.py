#!/usr/bin/env python3
"""🧬️ Makes every mutation leaf of one artifact generic over its dispatch enum (split lane P7, wave 0).

A leaf implements `MutationKind<Snapshot, Op>` for every `Op: protocol::Mutation<Snapshot, Diff = <diff>>` that converts
`From` exactly the leaf payloads its own code and its `↩️inverse` emit; the inverse function is generic over the same
`Op`. That removes the leaf -> dispatch-enum edge, so leaves can live in group crates below the dispatch crate.

usage: 🧬️generic-leaves.py plan|apply <config.json>
config: {"artifact": dir, "mutations": artifact-relative 🧬️mutations dir, "union": "CadMutation", "snapshot": "CadSnapshot",
         "diff": "crate::…::CadDiff", "union_module": "crate::…::mutations"}
"""
import json, os, re, sys

HERE = os.path.dirname(os.path.abspath(__file__))
REPO = os.path.abspath(os.path.join(HERE, *[".."] * 8))
IDENT = r"[A-Za-z_][A-Za-z0-9_]*"
SEP = r"\s*::\s*"


def leaves(prefix, body):
    m = re.match(r"^((?:" + IDENT + SEP + r")*)\{(.*)\}$", body.strip(), re.S)
    if m:
        pre = prefix + [x for x in re.split(SEP, m.group(1)) if x]
        out, depth, cur = [], 0, ""
        for ch in m.group(2):
            depth += ch == "{"
            depth -= ch == "}"
            if ch == "," and depth == 0:
                out += leaves(pre, cur)
                cur = ""
            else:
                cur += ch
        if cur.strip():
            out += leaves(pre, cur)
        return out
    alias = re.search(r"\s+as\s+(" + IDENT + r")$", body.strip())
    core = re.sub(r"\s+as\s+" + IDENT + r"$", "", body.strip())
    return [(prefix + [x for x in re.split(SEP, core) if x], alias.group(1) if alias else None)]


def drop_union(text, union):
    def fix(m):
        rows = [(segs, alias) for segs, alias in leaves([], m.group("body")) if segs[-1] != union]
        if not rows:
            return ""
        groups = {}
        for segs, alias in rows:
            groups.setdefault(segs[0], []).append("::".join(segs[1:]) + (f" as {alias}" if alias else ""))
        lines = []
        for head, parts in groups.items():
            tree = parts[0] if len(parts) == 1 else "{" + ", ".join(parts) + "}"
            lines.append(f"{m.group('indent')}{m.group('vis') or ''}use {head}::{tree};\n")
        return "".join(lines)
    return re.sub(r"(?m)^(?P<indent>[ \t]*)(?P<vis>pub(?:\s*\([^)]*\))?\s+)?use\s+(?P<body>[^;]*\b" + union + r"\b[^;]*);[ \t]*\n", fix, text)


def main():
    mode, config_path = sys.argv[1], sys.argv[2]
    config = json.load(open(config_path, encoding="utf-8"))
    root = os.path.join(REPO, config["artifact"], config["mutations"])
    union, snapshot, diff, umod = config["union"], config["snapshot"], config["diff"], config["union_module"]
    enum = open(os.path.join(root, "🦀️.rs"), encoding="utf-8").read()
    body = re.search(r"pub enum " + union + r"\s*\{(.*?)\n\}", enum, re.S).group(1)
    variants = {v: umod + "::" + re.sub(r"^(?:super|self)\s*::\s*", "", t.strip()) for v, t in re.findall(r"(?m)^\s*(" + IDENT + r")\s*\(\s*([^)]*?)\s*\)\s*,", body)}
    changed = {}
    for leaf in sorted(os.listdir(root)):
        leaf_rs = os.path.join(root, leaf, "🦀️.rs")
        if not os.path.isfile(leaf_rs):
            continue
        inv_rs = os.path.join(root, leaf, "↩️inverse", "🦀️.rs")
        text = open(leaf_rs, encoding="utf-8").read()
        inv = open(inv_rs, encoding="utf-8").read() if os.path.isfile(inv_rs) else None
        head = re.search(r"impl\s+((?:protocol\s*::\s*)?MutationKind)\s*<\s*" + snapshot + r"\s*,\s*" + union + r"\s*>\s+for\s+(" + IDENT + r")\s*\{", text)
        if not head:
            print(f"SKIP {leaf}: no MutationKind<{snapshot}, {union}> impl")
            continue
        own = head.group(2)
        emitted = sorted(set(re.findall(union + r"\s*::\s*(" + IDENT + r")\s*\(", text + (inv or ""))))
        unknown = [v for v in emitted if v not in variants]
        if unknown:
            raise SystemExit(f"{leaf}: unknown variants {unknown}")
        types = [variants[v] for v in emitted]
        bounds = "".join(f" + From<{t}>" for t in types)
        text = drop_union(text, union)
        text = text.replace(head.group(0), f"impl<Op> {head.group(1)}<{snapshot}, Op> for {own}\nwhere\n    Op: protocol::Mutation<{snapshot}, Diff = {diff}>{bounds},\n{{", 1)
        text = re.sub(r"Result<\s*Vec<\s*" + union + r"\s*>", "Result<Vec<Op>", text)
        text = re.sub(r"<\s*" + union + r"\s+as\s+", "<Op as ", text)
        text = re.sub(union + r"\s*::\s*" + IDENT + r"\s*\(", "Op::from(", text)
        leftover = [line for line in text.split("\n") if re.search(r"\b" + union + r"\b", line) and not line.lstrip().startswith("//")]
        if leftover:
            raise SystemExit(f"{leaf}: {union} left in leaf: {leftover}")
        changed[leaf_rs] = text
        if inv is not None:
            inv = drop_union(inv, union)
            generic = "<Op" + (": " + " + ".join(f"From<{t}>" for t in types) if types else "") + ">"
            inv, n = re.subn(r"pub fn inverse\s*\(", f"pub fn inverse{generic}(", inv, count=1)
            if n != 1:
                raise SystemExit(f"{leaf}: inverse fn not found")
            inv = re.sub(r"Result<\s*Vec<\s*" + union + r"\s*>", "Result<Vec<Op>", inv)
            inv = re.sub(r"<\s*" + union + r"\s+as\s+", "<Op as ", inv)
            inv = re.sub(union + r"\s*::\s*" + IDENT + r"\s*\(", "Op::from(", inv)
            leftover = [line for line in inv.split("\n") if re.search(r"\b" + union + r"\b", line) and not line.lstrip().startswith("//")]
            if leftover:
                raise SystemExit(f"{leaf}: {union} left in inverse: {leftover}")
            changed[inv_rs] = inv
        print(f"{own}: emits {emitted}")
    if mode == "apply":
        for path, text in changed.items():
            open(path, "w", encoding="utf-8").write(text)
    print(f"files={len(changed)}")


main()
