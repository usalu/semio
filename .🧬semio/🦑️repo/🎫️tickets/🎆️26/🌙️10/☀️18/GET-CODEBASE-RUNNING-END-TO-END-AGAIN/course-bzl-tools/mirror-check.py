#!/usr/bin/env python3
"""Byte-copy mirror proof of the course series: copies the touched files to <dir>/root, applies the series with --live there, then --check must report everything applied. usage: mirror-check.py <dir> [--with-graph]"""
import importlib.util, os, shutil, subprocess, sys
HERE = os.path.dirname(os.path.abspath(__file__))
directory, extra = os.path.abspath(sys.argv[1]), [flag for flag in sys.argv[2:] if flag == "--with-graph"]
script = os.path.join(HERE, "staged", "apply-course.py")
sys.argv = [sys.argv[0], *extra]
spec = importlib.util.spec_from_file_location("apply_course", script)
module = importlib.util.module_from_spec(spec)
spec.loader.exec_module(module)
live, root = module.series.LIVE, os.path.join(directory, "root")
shutil.rmtree(root, ignore_errors=True)
for rel in module.series.TOUCHED:
    source = os.path.join(live, rel)
    if os.path.isfile(source):
        target = os.path.join(root, rel)
        os.makedirs(os.path.dirname(target), exist_ok=True)
        with open(source, "rb") as reader, open(target, "wb") as writer:
            writer.write(reader.read())
        shutil.copymode(source, target)
status = 0
for mode in ("--live", "--check"):
    run = subprocess.run([sys.executable, "-I", script, mode, "--root", root, "--with-cleanup", *extra], capture_output=True, text=True)
    print(f"== {mode} rc={run.returncode}\n{run.stdout[-1800:]}{run.stderr[-800:]}")
    status = status or (run.returncode if mode == "--live" else (1 if any(line.startswith(("pending", "refused")) for line in run.stdout.splitlines()) else 0))
sys.exit(status)
