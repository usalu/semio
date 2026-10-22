#!/usr/bin/env python3
"""(Re)builds a test overlay of the frozen workspace snapshot with the staged set applied.
usage: overlay.py [--base /private/tmp/graph-parallel/ws] [--target /private/tmp/graph-parallel/ws-farm]
First run: an APFS clone (cp -c) of the base, then `apply.py apply --write`. Later runs restore every file the previous run touched from the base and apply again,
so only a handful of files are copied. Stdlib only."""
import json, os, pathlib, shutil, subprocess, sys

HERE = pathlib.Path(__file__).resolve().parent
args = sys.argv[1:]
base = pathlib.Path(args[args.index("--base") + 1] if "--base" in args else "/private/tmp/graph-parallel/ws")
target = pathlib.Path(args[args.index("--target") + 1] if "--target" in args else "/private/tmp/graph-parallel/ws-farm")
marker = target / ".gp-ops.json"
if not target.exists():
    subprocess.run(["cp", "-cR", str(base), str(target)], check=True)
elif marker.exists():
    previous = json.loads(marker.read_text(encoding="utf8"))
    touched = {op["file"] for op in previous["edits"]} | {item["file"] for item in previous["newFiles"]}
    for file in touched:
        source, destination = base / file, target / file
        if source.exists():
            destination.parent.mkdir(parents=True, exist_ok=True)
            shutil.copyfile(source, destination)
            shutil.copystat(source, destination)
        elif destination.exists():
            destination.unlink()
subprocess.run([sys.executable, str(HERE / "ops_gen.py"), "--root", str(base)], check=True)
subprocess.run([sys.executable, str(HERE / "apply.py"), "apply", "--write", "--root", str(target), "--ignore-running"], check=True)
shutil.copyfile(HERE / "staged/ops.json", marker)
print("overlay ready:", target)
