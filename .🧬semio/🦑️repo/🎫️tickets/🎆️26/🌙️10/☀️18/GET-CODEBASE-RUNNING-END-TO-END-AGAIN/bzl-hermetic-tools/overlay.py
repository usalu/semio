#!/usr/bin/env python3
"""Builds /private/tmp/bzl-hermetic/ov: live import closure of the adapter and its tests (copies), then the staged set applied on top (new files and edits). Never symlinks, never writes the repo. usage: overlay.py"""
import os, shutil, subprocess, sys
sys.dont_write_bytecode = True
HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
import edits
from targets import NEW, OPTIONAL, B, BZ, H
ROOT = edits.ROOT
OV = os.environ.get("OV", "/private/tmp/bzl-hermetic/ov")
NOEDIT = os.environ.get("NOEDIT") == "1"
if os.path.isdir(OV):
    shutil.rmtree(OV)
os.makedirs(OV)
entries = [edits.ADAPTER, BZ + "/🏃️execution/🧪️tests/🟦️.ts", BZ + "/🏃️execution/🧪️tests/🔒️hermetic/🟦️.ts", BZ + "/🌐️remote-cache/🟦️.ts", B + "/📥️inputs/📋️data/🟨️.mjs", B + "/📥️inputs/📋️data/📥️load/🟨️.mjs"]
from targets import L
for sub in ("🏗️build", "⚡️caching", "🏃️process"):
    shutil.copytree(os.path.join(ROOT, L, sub), os.path.join(OV, L, sub), ignore=shutil.ignore_patterns("node_modules", "target", "dist", ".cache", "🗑️generated", "🧪️tests-output"), symlinks=False, dirs_exist_ok=True)
for sub in ("🧰️framework/🔨️modules/🏃️process", "🧰️framework/🔨️modules/🧬️schema/✅️validator", "🧰️framework/📦️packages"):
    shutil.copytree(os.path.join(ROOT, sub), os.path.join(OV, sub), ignore=shutil.ignore_patterns("node_modules", "target", "dist", ".cache", "🗑️generated"), symlinks=False, dirs_exist_ok=True)
args = [sys.executable, os.path.join(HERE, "closure.py"), OV]
for e in entries:
    args += ["--entry", e]
for extra in [".bazelrc", "package.json", "bunfig.toml", B + "/🔣️policy.json"]:
    args += ["--extra", extra]
subprocess.run(args, check=True)
open(os.path.join(OV, "📋️project.json"), "w").write("{}")
for staged, live in {} if NOEDIT else {**NEW, **{k: v for k, v in OPTIONAL.items() if os.path.exists(os.path.join(HERE, "staged", k))}}.items():
    target = os.path.join(OV, live)
    os.makedirs(os.path.dirname(target), exist_ok=True)
    shutil.copyfile(os.path.join(HERE, "staged", staged), target)
for live, text in ({} if NOEDIT else edits.build()).items():
    target = os.path.join(OV, live)
    os.makedirs(os.path.dirname(target), exist_ok=True)
    open(target, "w", encoding="utf8").write(text)
nm = os.path.realpath(os.path.join(ROOT, "node_modules"))
dest = os.path.join(OV, "node_modules")
os.makedirs(dest, exist_ok=True)
subprocess.run(f'cp -RL "{nm}"/.bun/ajv@8.20.0/node_modules/* "{dest}"/ && cp -RL "{nm}"/.bun/typescript@5.9.3/node_modules/typescript "{dest}"/ && cp -RL "{nm}"/.bun/minimatch@9.0.3/node_modules/* "{dest}"/ && cp -RL "{nm}"/.bun/brace-expansion@2.1.7/node_modules/* "{dest}"/ 2>/dev/null; true', shell=True, check=True)
os.makedirs(os.path.join(dest, "@semio-tech"), exist_ok=True)
os.symlink(os.path.join(OV, "🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript"), os.path.join(dest, "@semio-tech/process"))
print("overlay ready", OV)
