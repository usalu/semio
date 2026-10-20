#!/usr/bin/env python3
"""Runs every staged test file one at a time: run-staged-tests.py ROOT [NATIVE_DIR]"""
import os, subprocess, sys, glob
root = os.path.abspath(sys.argv[1])
native = os.path.abspath(sys.argv[2]) if len(sys.argv) > 2 else ""
member = [p for p in glob.glob(root + "/**/*resident", recursive=True) if p.endswith("\U0001F3E1️resident")][0]
env = dict(os.environ, SEMIO_REPO_NATIVE_DIR=native, SEMIO_REPO_WATCH_BINARY=native + "/semio-repo-watch", SEMIO_REPO_FORWARD_BINARY=native + "/semio-repo-forward", SEMIO_REPO_FAKETOOL_BINARY=native + "/faketool")
tests = sorted(glob.glob(member + "/\U0001F9EA️tests/*/\U0001F7E6️.ts"))
total = 0
for path in tests:
    out = subprocess.run(["nice", "-n", "10", "bun", "test", path], capture_output=True, text=True, env=env, cwd=os.path.dirname(path))
    lines = [l for l in (out.stdout + out.stderr).splitlines() if l.strip().endswith(" pass") or l.strip().endswith(" fail") or "(fail)" in l or "error:" in l]
    print(os.path.basename(os.path.dirname(path)), out.returncode, "|", "; ".join(l.strip() for l in lines[:6]))
    total += out.returncode
sys.exit(1 if total else 0)
