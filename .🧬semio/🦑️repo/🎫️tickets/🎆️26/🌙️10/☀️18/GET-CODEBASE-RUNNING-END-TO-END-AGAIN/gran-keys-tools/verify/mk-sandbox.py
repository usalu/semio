#!/usr/bin/env python3
# 🧪 Builds the verification sandbox /private/tmp/gk/sb: a real copy of the library and of every project manifest at its repository path, every other entry a symlink to the real tree, so patched modules import each other while the process, schema and print packages stay the real ones. Nothing under the live repository is written.
import os, shutil, subprocess
ROOT = "/Users/ueli/Documents/semio"
SB = os.environ.get("SB", "/private/tmp/gk/sb")
LIB = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
shutil.rmtree(SB, ignore_errors=True)
os.makedirs(os.path.dirname(SB + "/" + LIB), exist_ok=True)
subprocess.run(["cp", "-Rc", ROOT + "/" + LIB, SB + "/" + LIB], check=True)
manifests = [m for m in open("/private/tmp/gk/manifests.txt", encoding="utf8").read().split("\n") if m]
for m in manifests:
    if m.startswith(LIB + "/"): continue
    dest = SB + "/" + m
    os.makedirs(os.path.dirname(dest), exist_ok=True)
    shutil.copyfile(ROOT + "/" + m, dest)
for f in ["nx.json", "bunfig.toml", "package.json"]:
    shutil.copyfile(ROOT + "/" + f, SB + "/" + f)
queue = [""]
while queue:
    rel = queue.pop()
    real, box = os.path.join(ROOT, rel), os.path.join(SB, rel)
    if rel == LIB: continue
    for name in os.listdir(real):
        if name == ".git": continue
        child = os.path.join(box, name)
        if os.path.lexists(child):
            if os.path.isdir(child) and not os.path.islink(child): queue.append(os.path.join(rel, name) if rel else name)
            continue
        os.symlink(os.path.join(real, name), child)
print("sandbox ready", len(manifests), "manifests")
