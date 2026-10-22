#!/usr/bin/env python3
"""🏊️ (Re)builds a test overlay with the state cache-gc apply3 and apply4 leave (see cachegc-base.py), round 1 and round 2 of the graph farm set applied in that order.
usage: overlay-all.py [--base /private/tmp/graph-parallel/ws2c] [--target /private/tmp/graph-parallel/ws2r]
First run: an APFS clone (cp -c) of the base; later runs restore every file either round touched from the base and apply both rounds again. Stdlib only."""
import json, pathlib, shutil, subprocess, sys

HERE = pathlib.Path(__file__).resolve().parent
args = sys.argv[1:]
base = pathlib.Path(args[args.index("--base") + 1] if "--base" in args else "/private/tmp/graph-parallel/ws2c")
target = pathlib.Path(args[args.index("--target") + 1] if "--target" in args else "/private/tmp/graph-parallel/ws2r")
marker = target / ".gp-all.json"
touched = set()
if not target.exists():
    subprocess.run(["cp", "-cR", str(base), str(target)], check=True)
elif marker.exists():
    previous = json.loads(marker.read_text(encoding="utf8"))
    touched = set(previous)
py = sys.executable
subprocess.run([py, str(HERE / "ops_gen.py"), "--root", str(base)], check=True)
subprocess.run([py, str(HERE / "ops2_gen.py"), "--root", str(base), "--virtual-round1"], check=True)
first = json.loads((HERE / "staged/ops.json").read_text(encoding="utf8")); second = json.loads((HERE / "staged/round2.json").read_text(encoding="utf8"))
touched |= {op["file"] for op in first["edits"] + second["edits"]} | {item["file"] for spec in (first, second) for key in ("newFiles", "replaceFiles", "deleteFiles") for item in spec.get(key, [])}
for file in touched:
    source, destination = base / file, target / file
    if source.exists():
        destination.parent.mkdir(parents=True, exist_ok=True)
        shutil.copyfile(source, destination)
        shutil.copystat(source, destination)
    elif destination.exists():
        destination.unlink()
subprocess.run([py, str(HERE / "apply.py"), "apply", "--spec", "ops", "--write", "--root", str(target), "--ignore-running"], check=True)
subprocess.run([py, str(HERE / "ops2_gen.py"), "--root", str(target)], check=True)
subprocess.run([py, str(HERE / "apply.py"), "apply", "--spec", "round2", "--write", "--root", str(target), "--ignore-running"], check=True)
marker.write_text(json.dumps(sorted(touched)), encoding="utf8")
print("overlay ready:", target)
