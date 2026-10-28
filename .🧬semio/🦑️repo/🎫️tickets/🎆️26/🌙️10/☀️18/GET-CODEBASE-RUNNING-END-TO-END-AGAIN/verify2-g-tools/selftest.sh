#!/bin/sh
# Sandbox simulation of every staged batch: apply, idempotent re-apply, revert, drift refusal (exit 2), in a throw-away root; the live tree is only read.
HERE=$(cd "$(dirname "$0")" && pwd)
SB=$(mktemp -d "${TMPDIR:-/tmp}/v2g-selftest.XXXXXX")
mkdir -p "$SB/🧰️framework" "$SB/tools"
cp "$HERE/stage.py" "$SB/tools/stage.py"
status=0
for batch in "$HERE"/staged/*/; do
  name=$(basename "$batch")
  python3 - "$batch" "$SB" <<'PY'
import json, os, shutil, sys
batch, sb = sys.argv[1], sys.argv[2]
for rel, row in json.load(open(os.path.join(batch, "bases.json"), encoding="utf8")).items():
    if row["base"] is not None:
        t = os.path.join(sb, rel); os.makedirs(os.path.dirname(t), exist_ok=True); shutil.copyfile(os.path.join(batch, "base", rel), t)
PY
  rm -rf "$SB/b"; cp -R "$batch" "$SB/b"
  S="python3 -I $SB/tools/stage.py $SB/b"
  $S check >/dev/null || { echo "$name: base check failed"; status=1; }
  $S apply >/dev/null || { echo "$name: apply failed"; status=1; }
  $S apply >/dev/null || { echo "$name: re-apply failed"; status=1; }
  $S check >/dev/null || { echo "$name: applied check failed"; status=1; }
  $S revert >/dev/null || { echo "$name: revert failed"; status=1; }
  first=$(python3 -c "import json,sys;d=json.load(open('$SB/b/bases.json',encoding='utf8'));print(next(k for k,v in d.items() if v['base'] is not None))")
  [ -e "$SB/$first" ] && echo drift >> "$SB/$first"
  $S apply >/dev/null 2>&1; code=$?
  [ -e "$SB/$first" ] && { [ $code -eq 2 ] || { echo "$name: drift not refused ($code)"; status=1; }; }
  echo "$name: selftest done"
done
rm -rf "$SB"
exit $status
