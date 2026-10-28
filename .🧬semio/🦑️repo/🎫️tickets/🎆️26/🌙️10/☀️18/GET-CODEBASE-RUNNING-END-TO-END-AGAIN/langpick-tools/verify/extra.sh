#!/bin/sh
# 🔁️ extra.sh <root>: suites that embed or consume the language-touched code but are in no batch verify (the captured-command loader family incl. the argv law, the authorizer closure law, the dashboard TypeScript laws). LP_BASE=1 on an unpatched root writes the base failure sets; compare with `compare.sh`.
set -u
ROOT=${1:?usage: extra.sh <root>}
HERE=$(cd "$(dirname "$0")" && pwd)
C=$(cd "$HERE/../.." && pwd)
OUT="$C/🗑️generated/${LP_OUT:-langpick}"; mkdir -p "$OUT" "$OUT/art"
cd "$ROOT" || exit 2
L=./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library; K="$L/🏗️build/📥️source/🔏️closure"; D=./🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🧪️tests
fail=0
for t in \
  "$K/🚪️command/🧪️tests/🏷️loader-identity/🟦️.ts" "$K/🚪️command/🧪️tests/🟦️.ts" "$K/🧪️tests/🧭️sites/🟦️.ts" "$K/🗓️scheduling/🧪️tests/🟦️.ts" \
  "$K/🧪️tests/📥️stream/🟦️.ts" "$K/🧪️tests/🔭️trace/🟦️.ts" "$K/🧪️tests/🕸️resolution/🟦️.ts" "$K/🧪️tests/🚫️cache-root-refusal/🟦️.ts" \
  "$K/🧾️diagnostics/🧪️tests/🟦️.ts" "$K/🟢️node/🧪️tests/🟦️.ts" "$K/🏗️compiler/🧪️tests/🟦️.ts" \
  "$L/⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/🧪️tests/🧭️closure-law/🟦️.ts" \
  "$D/🌐️language/🟦️.ts" "$D/🎮️registry/🟦️.ts" "$D/🧭️cli/🟦️.ts"; do
  [ -f "$t" ] || { echo "[DEBUG] missing $t"; continue; }
  log="$OUT/extra${LP_BASE:+-base}-$(echo "$t" | cksum | cut -d' ' -f1).log"
  SEMIO_TEST_ARTIFACT_DIR="$OUT/art" bun test "$t" > "$log" 2>&1 || { fail=1; }
  printf '%s | %s\n' "$(grep -E '^ *[0-9]+ (pass|fail)' "$log" | tr -s ' \n' ' ')" "$t"
  grep -E '^\(fail\)' "$log" | sed -E 's/ \[[0-9.]+m?s\]$//' | sort > "$log.fails"
done
exit $fail
