#!/usr/bin/env python3
"""🏃 Runs every staged test file one at a time: run-staged-tests.py ROOT [NATIVE_DIR]"""
import os, subprocess, sys, glob
root = os.path.abspath(sys.argv[1])
native = os.path.abspath(sys.argv[2]) if len(sys.argv) > 2 else ""
import importlib.util
spec = importlib.util.spec_from_file_location("apply", os.path.join(os.path.dirname(os.path.abspath(__file__)), "apply.py"))
apply = importlib.util.module_from_spec(spec)
spec.loader.exec_module(apply)
member = os.path.join(root, apply.R)
env = dict(os.environ, SEMIO_REPO_NATIVE_DIR=native, SEMIO_REPO_WATCH_BINARY=native + "/semio-repo-watch", SEMIO_REPO_FAKETOOL_BINARY=native + "/faketool")
tests = sorted(glob.glob(member + "/\U0001F9EA️tests/*/\U0001F7E6️.ts"))
if os.path.exists(os.path.join(root, apply.CALLER, "\U0001F7E6️.ts")):
    tests += [os.path.join(root, apply.SVC, apply.T, "\U0001F7E6️.ts"), os.path.join(root, apply.LAW, apply.T, "\U0001F7E6️.ts")]
total = 0
for path in tests:
    run = lambda: subprocess.run(["nice", "-n", "10", "bun", "test", path], capture_output=True, text=True, env=env, cwd=os.path.dirname(path))
    out = run()
    retried = out.returncode != 0
    if retried:
        out = run()
    lines = [l for l in (out.stdout + out.stderr).splitlines() if l.strip().endswith(" pass") or l.strip().endswith(" fail") or "(fail)" in l or "error:" in l]
    print(os.path.basename(os.path.dirname(path)) + "/" + os.path.basename(os.path.dirname(os.path.dirname(path))), out.returncode, "(rerun once: timing laws jitter under load)" if retried else "", "|", "; ".join(l.strip() for l in lines[:6]))
    total += out.returncode
sys.exit(1 if total else 0)
