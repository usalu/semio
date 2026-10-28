#!/bin/sh
# 🔗️ verify.sh <root>: parse-check + suites of batch imports (lane langpick) on a mirror root with core + copies + imports applied.
set -u
ROOT=${1:?usage: verify.sh <root>}
HERE=$(cd "$(dirname "$0")" && pwd)
C=$(cd "$HERE/../../.." && pwd)
OUT="$C/🗑️generated/${LP_OUT:-langpick}"; mkdir -p "$OUT" "$OUT/art"
cd "$ROOT" || exit 2
python3 -I - "$HERE/manifest.json" > "$OUT/imports-files.txt" <<'PY'
import json,sys
for rel,row in json.load(open(sys.argv[1],encoding="utf8")).items():
    if row["new"] and rel.endswith((".ts",".mjs")): print(rel)
PY
if [ -z "${LP_BASE:-}" ]; then
bun "$C/gate-g-tools/scan.ts" "$OUT/imports-files.txt" "$OUT/scan-imports" || exit 1
if grep -rl --include=*.ts --include=*.mjs -F "🏃️process/📦️packages/🟦️typescript/🏗️build/🗣️language/🟦️.ts" 🧰️framework 2>/dev/null | grep -v node_modules; then echo "RED: an importer of the removed entry is left"; exit 1; fi
fi
fail=0
L=./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library
for t in \
  "$L/🏗️build/⚙️operations/🧪️tests/🧭️help-offline/🟦️.ts" \
  "$L/⚡️caching/🦀️cargo/🗃️unit-cache/🧪️tests/🟦️.ts" \
  "$L/🏗️build/🧾️ci-report/🧪️tests/🟦️.ts" \
  "$L/⚡️caching/💾️storage/🧾️ledger/🧪️tests/🟦️.ts" \
  "$L/🏗️build/🔧️tools/🏗️bazel/🧵️sessions/🧪️tests/🟦️.ts" \
  "$L/🏗️build/🚪️door/🧪️tests/🚦️startup/🟦️.ts" \
  "./🧰️framework/🛍️products/🦑️repo/🔨️modules/🚀️launcher/🧪️tests/📌️runtime-pin-contract/🟦️.ts"; do
  [ -f "$t" ] || { echo "[DEBUG] missing $t"; continue; }
  log="$OUT/imports${LP_BASE:+-base}-$(echo "$t" | cksum | cut -d' ' -f1).log"
  SEMIO_TEST_ARTIFACT_DIR="$OUT/art" bun test "$t" > "$log" 2>&1 || fail=1
  printf '%s | %s\n' "$(grep -E '^ *[0-9]+ (pass|fail)' "$log" | tr -s ' \n' ' ')" "$t"
  grep -E '^\(fail\)' "$log" | sed -E 's/ \[[0-9.]+m?s\]$//' | sort > "$log.fails"
done
for lang in en de; do SEMIO_LANGUAGE=$lang bun repo --help 2>&1 | head -2 | cut -c1-100; done
env -u SEMIO_LANGUAGE -u LANG -u LC_ALL -u LC_MESSAGES -u LANGUAGE bun repo --help 2>&1 | head -2 | cut -c1-160
exit $fail
