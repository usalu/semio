#!/usr/bin/env python3
"""🗂️ Copies the sources apply2.py reads (module trees, bin/test/example/build roots, manifests, workspace roots) of the hygiene and facade-tail crates into an overlay that already holds the value tree.

usage: mkoverlay2.py <overlay-root>   (reads the checkout read-only)
"""
import json, os, re, subprocess, sys, tempfile
HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE + "/../analysis")
ROOT = "/Users/ueli/Documents/semio"
import tree
dest = os.path.abspath(sys.argv[1])
dirs = {e["dir"] for e in json.load(open(HERE + "/../data/hygiene-edges.json", encoding="utf-8"))}
gen = os.path.abspath(HERE + "/../../🗑️generated/gran-value/consumers-g5.json")
dirs |= {os.path.relpath(c["dir"], ROOT) for c in json.load(open(gen, encoding="utf-8"))}
files = set()
def add_manifest_chain(d):
    p = d
    while True:
        m = os.path.join(p, "Cargo.toml")
        if os.path.isfile(os.path.join(ROOT, m)):
            files.add(m)
        if p in ("", "."):
            break
        p = os.path.dirname(p) or "."
for d in sorted(dirs):
    add_manifest_chain(d)
    text = open(os.path.join(ROOT, d, "Cargo.toml"), encoding="utf-8").read()
    lib = re.search(r"^\[lib\][^\[]*?^\s*path\s*=\s*\"([^\"]+)\"", text, re.S | re.M)
    roots = [os.path.normpath(os.path.join(ROOT, d, lib.group(1) if lib else "src/lib.rs"))]
    for m in re.finditer(r"^\[\[(bin|test|example|bench)\]\](.*?)(?=^\[|\Z)", text, re.S | re.M):
        pm = re.search(r'^\s*path\s*=\s*"([^"]+)"', m.group(2), re.M)
        if pm:
            roots.append(os.path.normpath(os.path.join(ROOT, d, pm.group(1))))
    bm = re.search(r'^\s*build\s*=\s*"([^"]+)"', text, re.M)
    if bm:
        roots.append(os.path.normpath(os.path.join(ROOT, d, bm.group(1))))
    for r in roots:
        for f in tree.module_tree(r):
            files.add(os.path.relpath(f, ROOT))
files.add("🧰️framework/🛍️products/💻️os/🧪️tests/🧫️fixtures/📦️crate-boundaries/🔣️.json")
files = sorted(f for f in files if os.path.isfile(os.path.join(ROOT, f)) and not f.startswith(".."))
os.makedirs(dest, exist_ok=True)
with tempfile.NamedTemporaryFile("w", delete=False, encoding="utf-8") as lst:
    lst.write("\n".join(files) + "\n")
subprocess.run(["rsync", "-a", "--files-from=" + lst.name, ROOT + "/", dest + "/"], check=True)
os.remove(lst.name)
print(len(dirs), "crates,", len(files), "files copied to", dest)
