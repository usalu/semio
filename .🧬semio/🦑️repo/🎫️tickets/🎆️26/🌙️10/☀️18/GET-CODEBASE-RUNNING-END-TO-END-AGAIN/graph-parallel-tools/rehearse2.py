#!/usr/bin/env python3
"""🎭️ Rehearses round 1 and round 2 of the graph farm set on a throwaway copy of the files they touch, taken from the live tree: round 1 is applied, round 2 is generated from the result and applied, and every drift is reported; the live tree is only read. Exit 0 when both rounds apply without drift.
usage: rehearse2.py [--root REPO_ROOT]"""
import json, pathlib, shutil, subprocess, sys, tempfile

HERE = pathlib.Path(__file__).resolve().parent
root = HERE
while not (root / ".git").exists() and root != root.parent:
    root = root.parent
if "--root" in sys.argv:
    root = pathlib.Path(sys.argv[sys.argv.index("--root") + 1]).resolve()
subprocess.run([sys.executable, str(HERE / "ops_gen.py"), "--root", str(root)], check=True)
first = json.loads((HERE / "staged/ops.json").read_text(encoding="utf8"))
# round 2 touches the files round 1 touched plus a few more; read both lists from a generation against the live tree with round 1 simulated
subprocess.run([sys.executable, str(HERE / "ops2_gen.py"), "--root", str(root), "--virtual-round1"], check=True)
second = json.loads((HERE / "staged/round2.json").read_text(encoding="utf8"))
touched = {op["file"] for op in first["edits"] + second["edits"]} | {item["file"] for spec in (first, second) for key in ("newFiles", "replaceFiles", "deleteFiles") for item in spec.get(key, [])}
scratch = pathlib.Path(tempfile.mkdtemp(prefix="graph-parallel-rehearse-"))
try:
    for file in sorted(touched):
        source = root / file
        if source.is_file():
            (scratch / file).parent.mkdir(parents=True, exist_ok=True)
            shutil.copyfile(source, scratch / file)
    for spec, label in (("ops", "round 1"), ("round2", "round 2")):
        if spec == "round2":
            subprocess.run([sys.executable, str(HERE / "ops2_gen.py"), "--root", str(scratch)], check=True)
        done = subprocess.run([sys.executable, str(HERE / "apply.py"), "apply", "--spec", spec, "--write", "--root", str(scratch), "--ignore-running"], capture_output=True, text=True)
        print(label + ":", (done.stdout.strip().splitlines() or [done.stderr.strip()[-200:]])[0], "| exit", done.returncode)
        if done.returncode != 0:
            print(done.stdout[-600:], done.stderr[-600:])
            raise SystemExit(1)
finally:
    shutil.rmtree(scratch, ignore_errors=True)
