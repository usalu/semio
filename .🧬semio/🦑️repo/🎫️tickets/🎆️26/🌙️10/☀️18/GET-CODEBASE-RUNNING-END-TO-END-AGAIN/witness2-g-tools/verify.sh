#!/bin/sh
# 🧪️ usage: W2_ROOT=<root> sh verify.sh -> every law of the witness2-g batch plus the suites of the touched areas (run from the root; every bun test operand starts with ./).
ROOT="${W2_ROOT:-/Users/ueli/Documents/semio}"
cd "$ROOT" || exit 2
L="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"; B="$L/🏗️build"; N="🧰️framework/🔨️modules/📁️filesystem/🧾️observation/📜️changes"
fail=0
for f in "$N/🗻️mount/🧪️tests/🟦️.ts" "$N/🧪️tests/🟦️.ts" "$N/🪪️vouch/🧪️tests/🟦️.ts" "$N/🪪️vouch/🧪️tests/⚓️anchor/🟦️.ts" "$B/🏡️resident/📜️journal/🪪️vouch/🧪️tests/🟦️.ts" "$B/🏡️resident/📜️journal/🧪️tests/🟦️.ts" "$B/🔭️witness/🧪️tests/🟦️.ts" "$B/🔧️tools/🏗️bazel/♻️plan-reuse/🧪️tests/🟦️.ts" "$B/🔧️tools/🏗️bazel/♻️plan-reuse/🧪️tests/🧭️ignore-sources/🟦️.ts"; do
  printf '%s ... ' "$(basename "$(dirname "$f")")"
  out=$(bun test "./$f" --timeout 300000 2>&1); code=$?
  echo "$out" | grep -E '^ *[0-9]+ (pass|fail)' | tr '\n' ' '; echo
  [ $code -ne 0 ] && { echo "$out" | grep -E 'error:|\(fail\)' | head -5; fail=1; }
done
exit $fail
