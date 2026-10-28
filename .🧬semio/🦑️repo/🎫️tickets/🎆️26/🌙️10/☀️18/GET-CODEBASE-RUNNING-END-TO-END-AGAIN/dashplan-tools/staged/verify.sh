#!/bin/sh
# 🧪️ usage: DP_ROOT=<root> sh verify.sh [tests] -> the slot-order laws (session suite) and the plan-reuse and witness suites (run from the root; every bun test operand starts with ./).
ROOT="${DP_ROOT:-/Users/ueli/Documents/semio}"
cd "$ROOT" || exit 2
B="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build"; Z="$B/🔧️tools/🏗️bazel"
fail=0
for f in "$Z/🧵️sessions/🧪️tests/🟦️.ts" "$Z/♻️plan-reuse/🧪️tests/🟦️.ts" "$Z/♻️plan-reuse/🧪️tests/🧭️ignore-sources/🟦️.ts" "$B/🔭️witness/🧪️tests/🟦️.ts"; do
  printf '%s ... ' "$(basename "$(dirname "$(dirname "$f")")")"
  out=$(bun test "./$f" --timeout 300000 2>&1); code=$?
  echo "$out" | grep -E '^ *[0-9]+ (pass|fail)' | tr '\n' ' '; echo
  [ $code -ne 0 ] && { echo "$out" | grep -E 'error:|\(fail\)' | head -8; fail=1; }
done
exit $fail
