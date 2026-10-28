#!/bin/sh
# [walk-g] revert -> apply round trip of a staged batch on a scratch copy of exactly the files it names: sh roundtrip.sh <batch dir>. Prints status before/after apply and whether apply -> revert restores every byte.
set -e
cd /Users/ueli/Documents/semio
BATCH=$1; HERE="$(cd "$(dirname "$0")" && pwd)"
SCRATCH=$(mktemp -d /private/tmp/walk-g-roundtrip.XXXXXX)
python3 - "$BATCH" "$SCRATCH" <<'PY'
import json, os, shutil, sys
batch, scratch = sys.argv[1], sys.argv[2]
spec = json.load(open(os.path.join(batch, "edits.json"), encoding="utf8"))
for file in sorted({e["file"] for e in spec["edits"]}):
    os.makedirs(os.path.dirname(os.path.join(scratch, file)), exist_ok=True)
    shutil.copyfile(file, os.path.join(scratch, file))
PY
echo "status before:"; BATCH_ROOT=$SCRATCH python3 "$HERE/batch_generic.py" "$BATCH" status
BATCH_ROOT=$SCRATCH python3 "$HERE/batch_generic.py" "$BATCH" apply
echo "status after apply:"; BATCH_ROOT=$SCRATCH python3 "$HERE/batch_generic.py" "$BATCH" status
BATCH_ROOT=$SCRATCH python3 "$HERE/batch_generic.py" "$BATCH" apply
BATCH_ROOT=$SCRATCH python3 "$HERE/batch_generic.py" "$BATCH" revert
python3 - "$BATCH" "$SCRATCH" <<'PY'
import json, os, sys, filecmp
batch, scratch = sys.argv[1], sys.argv[2]
spec = json.load(open(os.path.join(batch, "edits.json"), encoding="utf8"))
same = all(filecmp.cmp(file, os.path.join(scratch, file), shallow=False) for file in sorted({e["file"] for e in spec["edits"]}))
print("revert restores every byte:", same)
PY
rm -rf "$SCRATCH"
