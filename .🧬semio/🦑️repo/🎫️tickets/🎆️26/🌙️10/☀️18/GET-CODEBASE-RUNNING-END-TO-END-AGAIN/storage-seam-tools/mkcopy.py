#!/usr/bin/env python3
"""🧫 mkcopy.py OUT: a REAL snapshot (no symlinks) of the process module, the filesystem and schema framework modules and a node_modules subset, for verification outside the checkout. OUT must live under /private/tmp/storage-seam/. Nothing under the checkout is written."""
import os, shutil, sys
ROOT = "/Users/ueli/Documents/semio"
out = sys.argv[1]
if not out.startswith("/private/tmp/storage-seam/"):
    raise SystemExit("output must live under /private/tmp/storage-seam/")
F = "🧰️framework"
P = F + "/🔨️modules/🏃️process"
DIRS = [P, F + "/🔨️modules/📁️filesystem", F + "/🔨️modules/🧬️schema", F + "/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧹️cache-gc"]
FILES = [".devcontainer/devcontainer.json", ".devcontainer/docker-compose.yml", "package.json", "bunfig.toml"]
NODE = ["ajv", "fast-deep-equal", "fast-uri", "json-schema-traverse", "require-from-string", "typescript"]
if os.path.exists(out):
    shutil.rmtree(out)
for rel in DIRS:
    src = os.path.join(ROOT, rel)
    if os.path.isdir(src):
        shutil.copytree(src, os.path.join(out, rel), symlinks=False, ignore=shutil.ignore_patterns("node_modules", ".DS_Store", "target"))
for rel in FILES:
    if os.path.exists(os.path.join(ROOT, rel)):
        os.makedirs(os.path.dirname(os.path.join(out, rel)), exist_ok=True)
        shutil.copyfile(os.path.join(ROOT, rel), os.path.join(out, rel))
for name in NODE:
    cand = os.path.realpath(os.path.join(ROOT, "node_modules", name))
    if not os.path.isdir(cand):
        continue
    siblings = os.path.dirname(cand)
    for entry in sorted(os.listdir(siblings)) if os.path.basename(siblings) == "node_modules" else [os.path.basename(cand)]:
        source, target = os.path.join(siblings, entry), os.path.join(out, "node_modules", entry)
        if os.path.isdir(source) and not os.path.exists(target) and not entry.startswith("."):
            shutil.copytree(os.path.realpath(source), target, symlinks=False)
total = sum(os.path.getsize(os.path.join(d, f)) for d, _, fs in os.walk(out) for f in fs)
print("real copy", out, "%.1f MB" % (total / 1048576))
