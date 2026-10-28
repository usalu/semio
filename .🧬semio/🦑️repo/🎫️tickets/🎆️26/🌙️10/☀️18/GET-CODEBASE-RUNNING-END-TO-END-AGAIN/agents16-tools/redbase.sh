#!/bin/sh
# 🟥 agents16: runs the new laws of a staged batch with the SOURCE files at base (tests and fixtures of the batch kept) to show they are red on base, then puts the batch sources back.
# Usage: sh redbase.sh <batch> <mirror-root> <law file>...
B=$1; ROOT=$2; shift 2; HERE=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd); D="$HERE/staged/$B"
swap() { python3 -I - "$D" "$ROOT" "$1" <<'PY'
import json, os, shutil, sys
d, root, side = sys.argv[1:]
for rel, row in json.load(open(os.path.join(d, "manifest.json"), encoding="utf8")).items():
    if "🧪️tests" in rel or row["base"] is None: continue
    shutil.copyfile(os.path.join(d, side, row["file"]), os.path.join(root, rel))
PY
}
swap base
cd "$ROOT" && for f in "$@"; do bun test "./$f" 2>&1 | grep -E '^\(fail\)|^ *[0-9]+ (pass|fail)$' | sed "s|^|[DEBUG] base $(basename "$(dirname "$f")"): |"; done
swap files
echo "[DEBUG] sources of $B restored"
