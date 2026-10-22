#!/usr/bin/env python3
# usage: apply.py [--write] [--root <checkout>] [--undo <backup dir>]
# Dry run by default. --write applies the staged set all-or-nothing (a replaced file that drifted since staging, or an anchor that does not match exactly once, refuses everything),
# keeps the previous content of every touched file in backups/<stamp>/ (flat, hashed names + manifest.json) and then regenerates the help snapshot fixture from the applied catalogue.
# Re-running reports "already applied". --undo restores a backup and removes the files the apply created.
import json, os, subprocess, sys, time
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from lib import *

root = sys.argv[sys.argv.index("--root") + 1] if "--root" in sys.argv else ROOT
if "--undo" in sys.argv:
    backup = sys.argv[sys.argv.index("--undo") + 1]
    manifest = json.load(open(os.path.join(backup, "manifest.json"), encoding="utf8"))
    for relative, name in manifest["restore"].items():
        with open(os.path.join(root, relative), "w", encoding="utf8", newline="") as handle:
            handle.write(read(os.path.join(backup, name)))
        print("restored:", relative)
    for relative in manifest["created"]:
        if os.path.exists(os.path.join(root, relative)):
            os.remove(os.path.join(root, relative))
            print("removed:", relative)
    sys.exit(0)

write = "--write" in sys.argv
writes, problems, notes = plan(root)
for line in notes:
    print("note:", line)
if problems:
    for line in problems:
        print("REFUSED:", line)
    print(f"refused: {len(problems)} problem(s), nothing written")
    sys.exit(1)
if not write:
    for relative, _ in writes:
        print("would write:", relative)
    print(f"dry run ok: {len(writes)} file(s)")
    sys.exit(0)
stamp = time.strftime("%Y%m%d-%H%M%S")
backup = os.path.join(TOOLS, "backups", stamp)
os.makedirs(backup)
manifest = {"restore": {}, "created": []}
for index, (relative, _) in enumerate(writes):
    path = os.path.join(root, relative)
    if os.path.exists(path):
        name = f"{index:03d}-{sha(relative)[:12]}"
        with open(os.path.join(backup, name), "w", encoding="utf8", newline="") as handle:
            handle.write(read(path))
        manifest["restore"][relative] = name
    else:
        manifest["created"].append(relative)
HELP = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/⚙️operations/🧪️tests/🧫️fixtures/📖️help/🔣️.json"
if "--no-help" not in sys.argv and HELP not in manifest["restore"]:
    if os.path.exists(os.path.join(root, HELP)):
        with open(os.path.join(backup, "help"), "w", encoding="utf8", newline="") as handle:
            handle.write(read(os.path.join(root, HELP)))
        manifest["restore"][HELP] = "help"
    else:
        manifest["created"].append(HELP)
json.dump(manifest, open(os.path.join(backup, "manifest.json"), "w", encoding="utf8"), ensure_ascii=False, indent=1)
for relative, text in writes:
    path = os.path.join(root, relative)
    os.makedirs(os.path.dirname(path), exist_ok=True)
    temporary = path + ".rcs.tmp"
    with open(temporary, "w", encoding="utf8", newline="") as handle:
        handle.write(text)
    os.replace(temporary, path)
    print("wrote:", relative)
if "--no-help" not in sys.argv:
    subprocess.run(["bun", os.path.join(TOOLS, "gen-help.ts")], cwd=root, check=True)
print(f"applied: {len(writes)} file(s); backup {backup}")
