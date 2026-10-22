#!/usr/bin/env python3
"""(Re)builds a test overlay of round 1 with the round 2 increment applied.
usage: overlay2.py [--base /private/tmp/graph-parallel/ws-farm3] [--target /private/tmp/graph-parallel/ws-r2]
First run: an APFS clone (cp -c) of the base (a tree with round 1 applied, see overlay.py), then ops2_gen.py and `apply.py apply --spec round2 --write`. Later runs restore every file the previous run touched from the base and apply again. Stdlib only."""
import json, os, pathlib, shutil, subprocess, sys

HERE = pathlib.Path(__file__).resolve().parent
args = sys.argv[1:]
base = pathlib.Path(args[args.index("--base") + 1] if "--base" in args else "/private/tmp/graph-parallel/ws-farm3")
target = pathlib.Path(args[args.index("--target") + 1] if "--target" in args else "/private/tmp/graph-parallel/ws-r2")
marker = target / ".gp-ops2.json"
if not target.exists():
    subprocess.run(["cp", "-cR", str(base), str(target)], check=True)
else:
    previous = json.loads(marker.read_text(encoding="utf8")) if marker.exists() else {"edits": []}
    first = json.loads((base / ".gp-ops.json").read_text(encoding="utf8"))
    touched = {op["file"] for op in previous["edits"] + first["edits"]} | {item["file"] for spec in (previous, first) for key in ("newFiles", "replaceFiles", "deleteFiles") for item in spec.get(key, [])}
    for file in touched:
        source, destination = base / file, target / file
        if source.exists():
            destination.parent.mkdir(parents=True, exist_ok=True)
            shutil.copyfile(source, destination)
            shutil.copystat(source, destination)
        elif destination.exists():
            destination.unlink()
subprocess.run([sys.executable, str(HERE / "ops2_gen.py"), "--root", str(base)], check=True)
subprocess.run([sys.executable, str(HERE / "apply.py"), "apply", "--spec", "round2", "--write", "--root", str(target), "--ignore-running"], check=True)
shutil.copyfile(HERE / "staged/round2.json", marker)
print("overlay ready:", target)
