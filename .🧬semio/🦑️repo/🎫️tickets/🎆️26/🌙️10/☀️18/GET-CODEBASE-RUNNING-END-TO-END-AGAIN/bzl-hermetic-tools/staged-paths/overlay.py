#!/usr/bin/env python3
"""Builds /private/tmp/bzl-hermetic/ovp: live closure copies, then the short-path set applied on top. NOEDIT=1 builds the untouched live tree (baseline). Never writes the repo."""
import os, shutil, subprocess, sys
sys.dont_write_bytecode = True
HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
import edits
ROOT = edits.ROOT
OV = os.environ.get("OV", "/private/tmp/bzl-hermetic/ovp")
NOEDIT = os.environ.get("NOEDIT") == "1"
L, B, BZ = edits.L, edits.B, edits.BZ
if os.path.isdir(OV):
    shutil.rmtree(OV)
os.makedirs(OV)
ignore = shutil.ignore_patterns("node_modules", "target", "dist", ".cache", "🗑️generated")
for sub in (L + "/🏗️build", L + "/⚡️caching", L + "/🏃️process", "🧰️framework/🔨️modules/🏃️process", "🧰️framework/🔨️modules/🧬️schema/✅️validator", "🧰️framework/📦️packages"):
    shutil.copytree(os.path.join(ROOT, sub), os.path.join(OV, sub), ignore=ignore, symlinks=False, dirs_exist_ok=True)
entries = [edits.ADAPTER, BZ + "/🏃️execution/🧪️tests/🟦️.ts", BZ + "/🏃️execution/🧪️tests/🔒️hermetic/🟦️.ts", BZ + "/🏃️execution/🧪️tests/🔒️classes/🟦️.ts", BZ + "/🌐️remote-cache/🟦️.ts", edits.PATHS_TEST, B + "/📥️inputs/📋️data/🟨️.mjs", B + "/📥️inputs/📋️data/📥️load/🟨️.mjs"]
args = [sys.executable, os.path.join(HERE, "..", "closure.py"), OV]
for e in entries:
    args += ["--entry", e]
for extra in [".bazelrc", "package.json", "bunfig.toml", B + "/🔣️policy.json", L + "/🔣️taxonomy.json"]:
    args += ["--extra", extra]
subprocess.run(args, check=True)
open(os.path.join(OV, "📋️project.json"), "w").write("{}")
if not NOEDIT:
    for staged, live in [(k, v) for k, v in edits.NEW.items() if os.path.exists(os.path.join(HERE, k))]:
        target = os.path.join(OV, live)
        os.makedirs(os.path.dirname(target), exist_ok=True)
        shutil.copyfile(os.path.join(HERE, staged), target)
    for live, text in edits.build().items():
        target = os.path.join(OV, live)
        os.makedirs(os.path.dirname(target), exist_ok=True)
        open(target, "w", encoding="utf8").write(text)
nm = os.path.realpath(os.path.join(ROOT, "node_modules"))
dest = os.path.join(OV, "node_modules")
os.makedirs(dest, exist_ok=True)
subprocess.run(f'for p in ajv@8.20.0 typescript@5.9.3 minimatch@9.0.3 brace-expansion@2.1.7 iconv-lite@0.7.3 safer-buffer@2.1.2; do cp -RL "{nm}"/.bun/$p/node_modules/* "{dest}"/ 2>/dev/null; done; true', shell=True, check=True)
os.makedirs(os.path.join(dest, "@semio-tech"), exist_ok=True)
os.symlink(os.path.join(OV, "🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript"), os.path.join(dest, "@semio-tech/process"))
print("overlay ready", OV)
