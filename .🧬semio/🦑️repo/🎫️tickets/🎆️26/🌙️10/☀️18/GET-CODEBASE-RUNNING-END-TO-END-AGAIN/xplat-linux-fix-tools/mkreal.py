#!/usr/bin/env python3
"""mkreal.py OUT: a REAL copy (no symlinks, no hard links) of exactly what the Linux suites need, for streaming into a container volume: the whole process module (the native-build module imports most of it), the library parts the suites touch, the host-capacity module and a node_modules subset (ajv and its dependencies, typescript). Refuses to exceed 4 GB. The groups of this set are applied afterwards with apply.py --root OUT."""
import os, shutil, sys
ROOT = "/Users/ueli/Documents/semio"
out = sys.argv[1]
if not out.startswith("/private/tmp/xplat-linux-fix/"):
    raise SystemExit("output must live under /private/tmp/xplat-linux-fix/")
F = "🧰️framework"
P = F + "/🔨️modules/🏃️process"
DIRS = [P, F + "/🔨️modules/📁️filesystem", F + "/🔨️modules/🧬️schema", F + "/🛍️products/🦑️repo/🔨️modules/📚️library", F + "/🛍️products/🦑️repo/🔨️modules/🎛️dashboard"]
FILES = [F + "/🛍️products/🦑️repo/README.md", ".devcontainer/devcontainer.json", ".devcontainer/docker-compose.yml", "package.json", "bunfig.toml"]
NODE = ["ajv", "fast-deep-equal", "fast-uri", "json-schema-traverse", "require-from-string", "typescript"]
if os.path.exists(out):
    shutil.rmtree(out)
total = 0
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
for directory, _, files in os.walk(out):
    for name in files:
        total += os.path.getsize(os.path.join(directory, name))
if total > 4 * 1024 ** 3:
    raise SystemExit("copy exceeds 4 GB: %d" % total)
print("real copy", out, "%.1f MB" % (total / 1048576))
