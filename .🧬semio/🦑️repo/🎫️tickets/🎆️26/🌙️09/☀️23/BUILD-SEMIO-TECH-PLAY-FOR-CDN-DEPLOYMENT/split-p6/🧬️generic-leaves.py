#!/usr/bin/env python3
"""🧬️ Makes every subset mutation leaf generic over its dispatch enum, so the leaves no longer name the enum that wraps
them: builders become `fn x<Op: From<X>>(..) -> Op`, the `MutationKind` impl becomes a blanket over every `Op` that is
the artifact's `Mutation` with its diff and converts from each leaf the inverse emits, and every inverse returns
`Vec<Op>`. Run on the monolith before the crate cut; the enum crate then sits above the leaf crates.

usage: 🧬️generic-leaves.py <subsets dir> <enum> <snapshot> <diff path> <payload path prefix> [--write]
  e.g. 🧬️generic-leaves.py ✏️s/…/🪆️subsets DrawingMutation DrawingSnapshot crate::diff::DrawingDiff crate::mutations
"""
import os, re, sys

subsets, enum, snapshot, diff_path, payload_prefix = sys.argv[1:6]
write = "--write" in sys.argv
REPO = os.path.abspath(os.path.join(os.path.dirname(os.path.abspath(__file__)), *[".."] * 8))
root = os.path.join(REPO, subsets)
mutation_files, inverse_files = [], {}
for dirpath, dirnames, filenames in os.walk(root):
    dirnames[:] = [d for d in dirnames if d != "🧪️tests" and d != "🎮️prepare"]
    if "🦀️.rs" not in filenames:
        continue
    if dirpath.endswith("🦠️mutation"):
        mutation_files.append(os.path.join(dirpath, "🦀️.rs"))
    elif dirpath.endswith("↩️inverse"):
        inverse_files[os.path.dirname(dirpath)] = os.path.join(dirpath, "🦀️.rs")
E = re.escape(enum)
BUILDER = re.compile(r"pub fn (\w+)\s*\(((?:[^(){}]|\([^()]*\)|\{[^{}]*\})*)\)\s*->\s*" + E + r"\s*\{(\s*)" + E + r"::(\w+)\(", re.S)
builders = {}
for path in mutation_files:
    for m in BUILDER.finditer(open(path, encoding="utf-8").read()):
        builders[m.group(1)] = m.group(4)
VEC = re.compile(r"Vec<\s*(?:crate::(?:mutations::)?)?" + E + r"\s*>")
IMPL = re.compile(r"impl\s+protocol::MutationKind<\s*" + re.escape(snapshot) + r"\s*,\s*" + E + r"\s*>\s+for\s+(\w+)\s*\{")


def produced(text, own_builders):
    found = set()
    for name, payload in builders.items():
        for m in re.finditer(r"(?<![\w])" + name + r"\s*\(", text):
            before = text[max(0, m.start() - 7):m.start()]
            if before.endswith("fn "):
                continue
            found.add(payload)
    return sorted(found)


def bound_text(payloads, local):
    return "".join(f" + From<{p if p == local else payload_prefix + '::' + p}>" for p in payloads)


changed = {}
for path in sorted(mutation_files):
    text = open(path, encoding="utf-8").read()
    impl = IMPL.search(text)
    if not impl:
        continue
    payload = impl.group(1)
    leaf = os.path.dirname(os.path.dirname(path))
    inverse = inverse_files.get(leaf)
    inverse_text = open(inverse, encoding="utf-8").read() if inverse else ""
    builder_spans = [(m.start(), m.end()) for m in BUILDER.finditer(text)]
    body = "".join(ch if not any(a <= i < b for a, b in builder_spans) else " " for i, ch in enumerate(text))
    need = sorted(set(produced(body[impl.start():], ()) + produced(inverse_text, ())))
    new = BUILDER.sub(lambda m: f"pub fn {m.group(1)}<Op: From<{m.group(4)}>>({m.group(2)}) -> Op {{{m.group(3)}Op::from(", text)
    new = IMPL.sub(lambda m: f"impl<Op> protocol::MutationKind<{snapshot}, Op> for {m.group(1)} where Op: protocol::Mutation<{snapshot}, Diff = {diff_path}>{bound_text(need, payload)} {{", new)
    new = VEC.sub("Vec<Op>", new)
    if new != text:
        changed[path] = new
    if inverse:
        inv = VEC.sub("Vec<Op>", inverse_text)
        bounds = bound_text(need, None).lstrip(" +")
        inv = re.sub(r"pub fn inverse\s*\(", f"pub fn inverse<Op{': ' + bounds if bounds else ''}>(", inv, count=1)
        if inv != inverse_text:
            changed[inverse] = inv
    print(f"{payload}: emits {need}")
left = []
for path, text in changed.items():
    if re.search(r"(?<![\w])" + E + r"(?![\w])", re.sub(r"(?m)^\s*use [^;]*;", "", text)):
        left.append(os.path.relpath(path, root))
for path in left:
    print(f"STILL NAMES {enum}: {path}")
print(f"files changed: {len(changed)}")
if write:
    for path, text in changed.items():
        open(path, "w", encoding="utf-8").write(text)
