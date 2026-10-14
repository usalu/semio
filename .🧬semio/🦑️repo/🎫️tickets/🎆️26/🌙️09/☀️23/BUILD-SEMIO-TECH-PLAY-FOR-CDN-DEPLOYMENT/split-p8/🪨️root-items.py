#!/usr/bin/env python3
"""🪨️ Moves `pub const` items (with their doc comments) out of a format root into the module file that owns them, and
retargets every crate-relative, root-relative and consumer path (including names inside `use crate::{..}` groups).

usage: 🪨️root-items.py <artifact dir (repo-rel)> <target file (artifact-rel)> <target module path> <NAME>... [--apply]
"""
import json, os, re, sys
HERE = os.path.dirname(os.path.abspath(__file__))
REPO = os.path.abspath(os.path.join(HERE, *[".."] * 8))
args = [a for a in sys.argv[1:] if a != "--apply"]
apply = "--apply" in sys.argv
artifact, target_rel, module, names = args[0], args[1], args[2], args[3:]
A = os.path.join(REPO, artifact)
root, target = os.path.join(A, "🦀️.rs"), os.path.join(A, target_rel)
ns = {}
exec(open(os.path.join(HERE, "🕸️module-graph.py"), encoding="utf-8").read().rsplit("\nmain()", 1)[0], ns)
files = set(ns["Crate"](root).files)
crate = re.search(r'(?m)^name\s*=\s*"([^"]+)"', open(os.path.join(A, "📦️packages", "🦀️rust", "Cargo.toml"), encoding="utf-8").read()).group(1).replace("-", "_")
writes = {}
rsrc = open(root, encoding="utf-8").read()
moved = []
for name in names:
    m = re.search(r"(?m)^((?:///[^\n]*\n)*)pub const " + name + r"\b[^;]*;\n", rsrc)
    if not m:
        raise SystemExit("no root const " + name)
    moved.append(m.group(0))
    rsrc = rsrc.replace(m.group(0), "", 1)
for name in names:
    rsrc = re.sub(r"(?<![\w:])" + name + r"\b", f"{module}::{name}", rsrc)
writes[root] = rsrc
def retarget(text, own):
    for name in names:
        text = re.sub(r"\bcrate::" + name + r"\b", f"crate::{module}::{name}", text)
        def group(m):
            body = m.group(1)
            if own:
                body = re.sub(r"\s*,\s*" + name + r"\b(?![\w:])|(?<![\w:])" + name + r"\b\s*,\s*|(?<![\w:])" + name + r"\b(?![\w:])", "", body, count=1)
            else:
                body = re.sub(r"(?<![\w:])" + name + r"\b(?![\w:])", f"{module}::{name}", body)
            return "use crate::{" + body + "};" if body.strip() else ""
        text = re.sub(r"use crate::\{((?:[^;])*?)\};", lambda m: group(m) if re.search(r"(?<![\w:])" + name + r"\b(?![\w:])", m.group(1)) else m.group(0), text)
    return text
for f in files:
    if f == root:
        continue
    raw = writes.get(f) or open(f, encoding="utf-8").read()
    new = retarget(raw, f == target)
    if new != raw:
        writes[f] = new
t = writes.get(target) or open(target, encoding="utf-8").read()
uses = list(re.finditer(r"(?m)^use [^\n]*\n", t))
at = uses[-1].end() if uses else 0
writes[target] = t[:at] + "\n" + "".join(moved) + t[at:]
index = json.load(open(os.path.join(HERE, "🗑️generated", "stdio-consumers.json"), encoding="utf-8"))
for rel in index.get(crate, []):
    f = os.path.join(REPO, rel)
    if f in files: continue
    raw = open(f, encoding="utf-8").read()
    new = raw
    for name in names:
        new = re.sub(r"\b" + crate + r"::" + name + r"\b", f"{crate}::{module}::{name}", new)
        new = re.sub(r"(use " + crate + r"::\{[^;]*?)(?<![\w:])" + name + r"\b(?![\w:])", r"\1" + f"{module}::{name}", new)
    if new != raw:
        writes[f] = new
for f in sorted(writes):
    print("write", os.path.relpath(f, REPO))
if apply:
    for f, t in writes.items():
        open(f, "w", encoding="utf-8").write(t)
