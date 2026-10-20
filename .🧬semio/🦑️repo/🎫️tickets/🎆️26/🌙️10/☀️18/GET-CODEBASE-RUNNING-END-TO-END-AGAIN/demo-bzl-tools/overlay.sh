#!/usr/bin/env bash
# usage: overlay.sh   builds /private/tmp/dbz/ov: COPIES of the six edited files with the staged set applied (never links), ready for verify.sh.
cd /Users/ueli/Documents/semio || exit 2
here="$(pwd)/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/demo-bzl-tools"
ov=/private/tmp/dbz/ov; rm -rf "$ov"; mkdir -p "$ov"
python3 - "$here" "$ov" <<'PY'
import sys, os, shutil
sys.path.insert(0, sys.argv[1]); import apply
for _, path, _ in apply.EDITS:
    os.makedirs(os.path.dirname(os.path.join(sys.argv[2], path)), exist_ok=True); shutil.copy2(os.path.join(apply.LIVE, path), os.path.join(sys.argv[2], path))
PY
python3 "$here/apply.py" --root "$ov" --write --accept-drift | tail -1
