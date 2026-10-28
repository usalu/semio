#!/bin/sh
# Sandbox simulation of every flat batch of boot-g: apply, idempotent re-apply, revert (live bytes restored), drift refusal (exit 2); the live tree is only read.
HERE=$(cd "$(dirname "$0")" && pwd)
SB=$(mktemp -d "${TMPDIR:-/tmp}/bootg-selftest.XXXXXX"); status=0
for batch in "$HERE"/staged/*/; do
  name=$(basename "$batch")
  python3 - "$batch" "$SB" <<'PY'
import json, os, shutil, sys
batch, sb = sys.argv[1], sys.argv[2]
for rel, row in json.load(open(os.path.join(batch, "manifest.json"), encoding="utf8")).items():
    if row["base"] is not None:
        t = os.path.join(sb, rel); os.makedirs(os.path.dirname(t), exist_ok=True); shutil.copyfile(os.path.join(batch, "base", row["file"]), t)
PY
  S="env STAGE_ROOT=$SB python3 -I $HERE/stage-flat.py $batch"
  $S check >/dev/null || { echo "$name: base check failed"; status=1; }
  $S apply >/dev/null || { echo "$name: apply failed"; status=1; }
  $S apply >/dev/null || { echo "$name: re-apply failed"; status=1; }
  $S check >/dev/null || { echo "$name: applied check failed"; status=1; }
  $S revert >/dev/null || { echo "$name: revert failed"; status=1; }
  python3 - "$batch" "$SB" <<'PY' || { echo "$name: revert left bytes behind"; status=1; }
import hashlib, json, os, sys
batch, sb = sys.argv[1], sys.argv[2]
for rel, row in json.load(open(os.path.join(batch, "manifest.json"), encoding="utf8")).items():
    p = os.path.join(sb, rel); h = hashlib.sha256(open(p, "rb").read()).hexdigest() if os.path.exists(p) else None
    if h != row["base"]: sys.exit(1)
PY
  first=$(python3 -c "import json;d=json.load(open('$batch/manifest.json',encoding='utf8'));print(next(k for k,v in d.items() if v['base'] is not None))")
  echo drift >> "$SB/$first"
  $S apply >/dev/null 2>&1; code=$?
  [ $code -eq 2 ] || { echo "$name: drift not refused ($code)"; status=1; }
  echo "$name: selftest done"
done
rm -rf "$SB"; exit $status
