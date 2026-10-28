#!/bin/sh
# 🛤️ verify.sh <root>: parse-check + laws of batch routing (lane langpick). Run on a mirror root with every earlier batch and routing applied; compare failure sets with the base run (`LP_BASE=1`).
set -u
ROOT=${1:?usage: verify.sh <root>}
HERE=$(cd "$(dirname "$0")" && pwd)
C=$(cd "$HERE/../../.." && pwd)
OUT="$C/🗑️generated/${LP_OUT:-langpick}"; mkdir -p "$OUT" "$OUT/art"
cd "$ROOT" || exit 2
python3 -I - "$HERE/manifest.json" > "$OUT/routing-files.txt" <<'PY'
import json,sys
for rel,row in json.load(open(sys.argv[1],encoding="utf8")).items():
    if row["new"] and rel.endswith((".ts",".mjs")) and not rel.endswith(".d.mts"): print(rel)
PY
[ -n "${LP_BASE:-}" ] || bun "$C/gate-g-tools/scan.ts" "$OUT/routing-files.txt" "$OUT/scan-routing" || exit 1
F=./🧰️framework/🔨️modules; L=./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library
fail=0
P=./🧰️framework/🔨️modules/🏃️process
for t in \
  "$F/⚙️runtime/🗣️language/🧪️tests/🟦️.ts" \
  "$P/🧭️routing/📥️invocation/🏃️process/🧪️tests/📈️progress/🟦️.ts" \
  "$P/🧭️routing/📥️invocation/🏃️process/⏲️expiry/🧪️tests/⏳️progress/🟦️.ts" \
  "$P/🧭️routing/📥️invocation/🏃️process/🐕️watchdog/🧪️tests/🟦️.ts" \
  "$P/🎛️owned-execution/🧪️tests/🟦️.ts" \
  "$P/🎛️owned-execution/🧪️tests/📤️progress/🟦️.ts" \
  "$P/📦️artifacts/🏗️native-build/📣️diagnostics/🧪️tests/🟦️.ts"; do
  [ -f "$t" ] || { echo "[DEBUG] missing $t"; continue; }
  log="$OUT/routing${LP_BASE:+-base}-$(echo "$t" | cksum | cut -d' ' -f1).log"
  SEMIO_TEST_ARTIFACT_DIR="$OUT/art" bun test "$t" > "$log" 2>&1 || { fail=1; }
  printf '%s | %s\n' "$(grep -E '^ *[0-9]+ (pass|fail)' "$log" | tr -s ' \n' ' ')" "$t"
  grep -E '^\(fail\)' "$log" | sed -E 's/ \[[0-9.]+m?s\]$//' | sort > "$log.fails"
done
exit $fail
