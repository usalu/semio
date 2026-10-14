#!/usr/bin/env python3
"""🧬️ Makes mutation leaves (single-file or triad) generic over their dispatch enum, so leaf crates sit below the enum.

Per leaf file: `impl [protocol::]MutationKind<S, E> for X` becomes `impl<Op> … MutationKind<S, Op> for X where Op:
protocol::Mutation<S, Diff = D> + From<P>…`; every free fn returning `E` or `…Vec<E>…` becomes generic `<Op: From<P>…>`
returning `Op`/`Vec<Op>`; every `E::Variant(` constructor becomes `Op::from(`. The `From` bounds of a file are the
payload types of the variants it constructs, closed over the generic fns it calls in other leaf files and, for a leaf
root that holds the kind impl, over the files in its own leaf directory (`↩️inverse`, `🔺️diff`). Files that still name
the enum afterwards are reported.

usage: 🧬️generic-leaves-flat.py <enum file> <leaf dir>... --enum E --snapshot S --diff D --payload-prefix crate::mutations [--write]
"""
import collections, os, re, sys

REPO = os.path.abspath(os.path.join(os.path.dirname(os.path.abspath(__file__)), *[".."] * 8))
args = sys.argv[1:]
opt = lambda name: args[args.index(name) + 1]
enum, snapshot, diff_path, prefix = opt("--enum"), opt("--snapshot"), opt("--diff"), opt("--payload-prefix")
write = "--write" in args
positional, skip = [], False
for a in args:
    if skip:
        skip = False
        continue
    if a.startswith("--"):
        skip = a != "--write"
        continue
    positional.append(a)
enum_file, leaf_dirs = os.path.join(REPO, positional[0]), positional[1:]
E = re.escape(enum)
enum_text = open(enum_file, encoding="utf-8").read()
body = enum_text[enum_text.index(f"enum {enum}"):]
body = body[:body.index("\n}")]
variants = {m.group(1): m.group(2) for m in re.finditer(r"(\w+)\s*\(\s*([\w:]+)\s*\)", body)}
files = []
for d in leaf_dirs:
    for dirpath, dirnames, filenames in os.walk(os.path.join(REPO, d)):
        dirnames[:] = [x for x in dirnames if x != "🧪️tests"]
        if "🦀️.rs" in filenames and os.path.join(dirpath, "🦀️.rs") != enum_file:
            files.append(os.path.join(dirpath, "🦀️.rs"))
RET = re.compile(r"(?:->\s*(?:Result<\s*)?(?:Vec<\s*)?(?:crate::(?:mutations::)?)?" + E + r"\b)")
FN = re.compile(r"((?:pub(?:\s*\([^)]*\))?\s+)?fn\s+)(\w+)\s*(<[^>(]*>)?\s*\(")
texts = {p: open(p, encoding="utf-8").read() for p in files}
generic_fns = collections.defaultdict(set)
direct = {}
for path, text in texts.items():
    direct[path] = set(re.findall(E + r"::(\w+)\s*\(", text)) & set(variants)
    for m in FN.finditer(text):
        sig = text[m.end():text.find("{", m.end())]
        if RET.search(sig) and "&self" not in sig and "self:" not in sig:
            generic_fns[m.group(2)].add(path)
bounds = {p: set(v) for p, v in direct.items()}
changed = True
while changed:
    changed = False
    for path, text in texts.items():
        for name, owners in generic_fns.items():
            if len(owners) > 1:
                continue
            for owner in owners:
                if owner != path and re.search(r"(?<![\w.])(?<!fn )" + name + r"\s*\(", text) and not bounds[owner] <= bounds[path]:
                    bounds[path] |= bounds[owner]
                    changed = True
        if re.search(r"impl\s+(?:protocol::)?MutationKind<", text):
            leaf = os.path.dirname(path)
            for other in texts:
                if other != path and other.startswith(leaf + os.sep) and not bounds[other] <= bounds[path]:
                    bounds[path] |= bounds[other]
                    changed = True


def bound_text(vs):
    return "".join(f" + From<{prefix}::{variants[v]}>" for v in sorted(vs))


out_texts, left = {}, []
for path in sorted(files):
    text = texts[path]
    b = bound_text(bounds[path])
    new = re.sub(r"impl\s+(protocol::)?MutationKind<\s*" + re.escape(snapshot) + r"\s*,\s*" + E + r"\s*>\s+for\s+(\w+)\s*\{",
                 lambda m: f"impl<Op> {m.group(1) or ''}MutationKind<{snapshot}, Op> for {m.group(2)} where Op: protocol::Mutation<{snapshot}, Diff = {diff_path}>{b} {{", text)
    pieces, pos = [], 0
    for m in FN.finditer(new):
        sig_end = new.find("{", m.end())
        sig = new[m.end():sig_end]
        if RET.search(sig) and "&self" not in sig and "self:" not in sig and not m.group(3):
            pieces.append(new[pos:m.start()] + f"{m.group(1)}{m.group(2)}<Op{': ' + b.lstrip(' +') if b else ''}>(")
            pos = m.end()
    new = "".join(pieces) + new[pos:]
    new = re.sub(r"Vec<\s*(?:crate::(?:mutations::)?)?" + E + r"\s*>", "Vec<Op>", new)
    new = re.sub(r"->\s*(?:crate::(?:mutations::)?)?" + E + r"\s*\{", "-> Op {", new)
    new = re.sub(r"(?<![\w:])" + E + r"::(\w+)\s*\(", "Op::from(", new)
    if new != text:
        out_texts[path] = new
    rest = re.sub(r"(?m)^\s*use [^;]*;", "", new)
    if re.search(r"(?<![\w])" + E + r"(?![\w])", rest):
        left.append(os.path.relpath(path, REPO))
    print(f"{os.path.relpath(path, REPO)[-70:]}: {sorted(bounds[path])}")
for path in left:
    print(f"STILL NAMES {enum}: {path}")
print(f"files changed: {len(out_texts)}")
if write:
    for path, text in out_texts.items():
        open(path, "w", encoding="utf-8").write(text)
