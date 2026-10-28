#!/bin/sh
# 🗣️ verify.sh <root>: parse-check + laws of batch core (lane langpick). Run on a mirror root with the batch applied.
set -u
ROOT=${1:?usage: verify.sh <root>}
HERE=$(cd "$(dirname "$0")" && pwd)
C=$(cd "$HERE/../../.." && pwd)
OUT="$C/🗑️generated/${LP_OUT:-langpick}"; mkdir -p "$OUT" "$OUT/art"
cd "$ROOT" || exit 2
python3 -I - "$HERE/manifest.json" > "$OUT/core-files.txt" <<'PY'
import json,sys
for rel,row in json.load(open(sys.argv[1],encoding="utf8")).items():
    if row["new"] and rel.endswith((".ts",".mjs")): print(rel)
PY
[ -n "${LP_BASE:-}" ] || bun "$C/gate-g-tools/scan.ts" "$OUT/core-files.txt" "$OUT/scan-core" || exit 1
fail=0
for t in \
  "./🧰️framework/🔨️modules/⚙️runtime/🗣️language/🧪️tests/🟦️.ts" \
  "./🧰️framework/🔨️modules/🏃️process/🏗️build/🗣️language/📖️catalog/🧪️tests/🟦️.ts" \
  "./🧰️framework/🔨️modules/🏃️process/🏗️build/🗣️language/📣️progress/🧪️tests/🟦️.ts" \
  "./🧰️framework/🔨️modules/🏃️process/🏗️build/🗣️language/🚨️error/🧪️tests/🟦️.ts" \
  "./🧰️framework/🔨️modules/🏃️process/🏗️build/🧰️toolchains/🧪️tests/🟦️.ts" \
  "./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/⚙️operations/🧪️tests/🧭️help-offline/🟦️.ts" \
  "./🧰️framework/🛍️products/🦑️repo/🔨️modules/🚀️launcher/🧪️tests/📌️runtime-pin-contract/🟦️.ts"; do
  [ -f "$t" ] || { echo "[DEBUG] missing $t"; continue; }
  log="$OUT/core${LP_BASE:+-base}-$(echo "$t" | cksum | cut -d' ' -f1).log"
  SEMIO_TEST_ARTIFACT_DIR="$OUT/art" bun test "$t" > "$log" 2>&1 || { fail=1; echo "RED $t"; }
  printf '%s | %s\n' "$(grep -E '^ *[0-9]+ (pass|fail)' "$log" | tr -s ' \n' ' ')" "$t"
  grep -E '^\(fail\)' "$log" | sed -E 's/ \[[0-9.]+m?s\]$//' | sort > "$log.fails"
done
[ -n "${LP_BASE:-}" ] && exit $fail
mkdir -p "/private/tmp/m/lpgo"
bun "$C/langpick-tools/gotest.ts" "$ROOT" "🧰️framework/🛍️products/🦑️repo/🔨️modules/🚀️launcher/📦️packages/🐹️go" "/private/tmp/m/lpgo" -v -run "Language|Registry" | grep -E "^(--- FAIL|ok|FAIL)|TestLanguage" || fail=1
exit $fail
