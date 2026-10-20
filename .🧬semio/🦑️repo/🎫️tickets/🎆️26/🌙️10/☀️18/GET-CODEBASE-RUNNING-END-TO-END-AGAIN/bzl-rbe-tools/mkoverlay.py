#!/usr/bin/env python3
# usage: mkoverlay.py   builds /private/tmp/rb: COPIES of the authored new files at their live paths (edits applied when edits.py exists), the relative-import closure of the service tests and ajv. Never symlinks.
import os, re, shutil, subprocess, sys
sys.dont_write_bytecode = True
HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
from targets import *
from engine import EDITS, apply_one, read

ROOT = "/Users/ueli/Documents/semio"
OVERLAY = "/private/tmp/rb"
IMPORT = re.compile(r'''(?:from\s*|import\s*\(\s*|import\s+)"(\.{1,2}/[^"]+)"''')
SUPPORT = ["package.json", "bunfig.toml"]


def copy(relative):
    source = os.path.join(ROOT, relative)
    target = os.path.join(OVERLAY, relative)
    os.makedirs(os.path.dirname(target), exist_ok=True)
    if not os.path.exists(target):
        shutil.copy2(source, target)


if os.path.isdir(OVERLAY):
    shutil.rmtree(OVERLAY)
os.makedirs(OVERLAY)
for source, target in NEW.items():
    path = os.path.join(OVERLAY, target)
    os.makedirs(os.path.dirname(path), exist_ok=True)
    shutil.copy2(os.path.join(HERE, "src", source), path)
seen, pending = set(), [S + "/🧪️tests/🟦️.ts"]
while pending:
    relative = pending.pop()
    if relative in seen:
        continue
    seen.add(relative)
    path = os.path.join(OVERLAY, relative)
    if not os.path.isfile(path):
        path = os.path.join(ROOT, relative)
    if not os.path.isfile(path) or not relative.endswith((".ts", ".mjs", ".js")):
        continue
    for match in IMPORT.finditer(open(path, encoding="utf8").read()):
        target = os.path.normpath(os.path.join(os.path.dirname(relative), match.group(1)))
        if os.path.isfile(os.path.join(OVERLAY, target)) or os.path.isfile(os.path.join(ROOT, target)):
            pending.append(target)
        elif target.endswith(".json") and os.path.isfile(os.path.join(ROOT, target)):
            pending.append(target)
for relative in seen:
    if os.path.isfile(os.path.join(ROOT, relative)) and not os.path.exists(os.path.join(OVERLAY, relative)):
        copy(relative)
exec_schema = B + "/🔧️tools/🏗️bazel/🏃️execution/🧬️schema/🔣️.json"
copy(exec_schema)
text = read(os.path.join(OVERLAY, exec_schema))
for edit in [e for e in EDITS if e["file"] == exec_schema]:
    text = apply_one(text, edit)
open(os.path.join(OVERLAY, exec_schema), "w", encoding="utf8").write(text)
for relative in SUPPORT:
    copy(relative)
nm = os.path.realpath(os.path.join(ROOT, "node_modules"))
dest = os.path.join(OVERLAY, "node_modules")
os.makedirs(dest, exist_ok=True)
subprocess.run(f'cp -RL "{nm}"/.bun/ajv@8.20.0/node_modules/* "{dest}"/', shell=True, check=True)
print("overlay ready:", len(seen), "closure files")
