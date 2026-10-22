#!/usr/bin/env python3
# usage: undo_test.py   proves on the baseline overlay (live copies + the rcache-server set): dry run writes nothing, --write applies, a second --write finds nothing, --undo restores every touched file byte for byte and removes every created one.
import hashlib, os, subprocess, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from lib import *

BASE = "/private/tmp/rexec-inhouse/base"
targets, edits = load("targets"), load("edits").EDITS
files = sorted({e["file"] for e in edits} | set(targets.REPLACE.values()) | set(targets.REMOVE) | set(targets.NEW.values()))


def snapshot():
    rows = {}
    for relative in files:
        path = os.path.join(BASE, relative)
        rows[relative] = hashlib.sha256(open(path, "rb").read()).hexdigest() if os.path.isfile(path) else None
    return rows


def apply(*arguments):
    result = subprocess.run([sys.executable, os.path.join(TOOLS, "apply.py"), "--root", BASE, "--no-help", *arguments], capture_output=True, text=True)
    return result.returncode, result.stdout


before = snapshot()
code, out = apply()
assert code == 0 and "dry run ok" in out, out
assert snapshot() == before, "dry run wrote"
print(out.strip().splitlines()[-1])
code, out = apply("--write")
assert code == 0, out
after = snapshot()
assert after != before
backup = out.strip().splitlines()[-1].split("backup ")[1]
code, out = apply("--write")
assert code == 0 and "applied: 0 file(s) written, 0 removed" in out, out
assert snapshot() == after, "second apply changed files"
subprocess.run([sys.executable, os.path.join(TOOLS, "apply.py"), "--root", BASE, "--undo", backup], check=True, capture_output=True)
assert snapshot() == before, "undo did not restore"
print("apply, idempotent re-apply and undo verified on", len(files), "paths")
