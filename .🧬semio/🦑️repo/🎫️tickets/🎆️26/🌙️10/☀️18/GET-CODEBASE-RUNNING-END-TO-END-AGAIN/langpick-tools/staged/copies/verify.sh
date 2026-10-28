#!/bin/sh
# 🧹️ verify.sh <root>: parse-check + laws of batch copies (lane langpick). Run on a mirror root with core + copies applied; compare failure sets with the base run (`LP_BASE=1`).
set -u
ROOT=${1:?usage: verify.sh <root>}
HERE=$(cd "$(dirname "$0")" && pwd)
C=$(cd "$HERE/../../.." && pwd)
OUT="$C/🗑️generated/${LP_OUT:-langpick}"; mkdir -p "$OUT" "$OUT/art"
cd "$ROOT" || exit 2
python3 -I - "$HERE/manifest.json" > "$OUT/copies-files.txt" <<'PY'
import json,sys
for rel,row in json.load(open(sys.argv[1],encoding="utf8")).items():
    if row["new"] and rel.endswith((".ts",".mjs")) and not rel.endswith(".d.mts"): print(rel)
PY
[ -n "${LP_BASE:-}" ] || bun "$C/gate-g-tools/scan.ts" "$OUT/copies-files.txt" "$OUT/scan-copies" || exit 1
F=./🧰️framework/🔨️modules; L=./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library
fail=0
for t in \
  "$F/⚙️runtime/🗣️language/🧪️tests/🟦️.ts" \
  "$F/⚙️runtime/🧪️tests/🟦️.ts" \
  "$F/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🧰️programs/🧪️tests/🟦️.ts" \
  "$F/🏃️process/🧪️testing/🧰️toolchain/🧪️tests/🟦️.ts" \
  "$F/🏃️process/🔒️leases/📈️standing/🧪️tests/🟦️.ts" \
  "$F/🏃️process/🔒️leases/🧭️order/🧪️tests/🟦️.ts" \
  "$F/📁️filesystem/🧾️observation/🔁️churn/🧪️tests/🟦️.ts" \
  "$L/🏗️build/🏛️sealed/🧪️tests/🗂️store/🟦️.ts" \
  "$L/🏗️build/🧷️format-stamps/🧪️tests/🟦️.ts" \
  "$L/🎮️playground/🖥️native-host/🧪️tests/🟦️.ts" \
  "$L/🏗️build/🕸️graph/📥️inference/📦️project-nodes/🏊️project-farm/🧪️tests/🟦️.ts" \
  "$L/🏗️build/🧠️memory/🧪️tests/🟦️.ts" \
  "$L/🏗️build/🌐️remote-cache/🧪️tests/🟦️.ts" \
  "$L/🏗️build/🩺️doctor/🧪️tests/🟦️.ts" \
  "$L/⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/🧭️sites/🧪️tests/🟦️.ts" \
  "$L/🏗️build/🚢️fleet-status/🧪️tests/🟦️.ts" \
  "$L/⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/♻️reset/🧪️tests/🟦️.ts" \
  "./🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🛠️developer-tools/🧪️tests/🟦️.ts" \
  "$L/🏗️build/🚪️door/🧪️tests/🚦️startup/🟦️.ts" \
  "$L/🏗️build/🏛️sealed/🧪️tests/🚦️trampoline/🟦️.ts"; do
  [ -f "$t" ] || { echo "[DEBUG] missing $t"; continue; }
  log="$OUT/copies${LP_BASE:+-base}-$(echo "$t" | cksum | cut -d' ' -f1).log"
  SEMIO_TEST_ARTIFACT_DIR="$OUT/art" bun test "$t" > "$log" 2>&1 || { fail=1; }
  printf '%s | %s\n' "$(grep -E '^ *[0-9]+ (pass|fail)' "$log" | tr -s ' \n' ' ')" "$t"
  grep -E '^\(fail\)' "$log" | sed -E 's/ \[[0-9.]+m?s\]$//' | sort > "$log.fails"
done
exit $fail
