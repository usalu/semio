#!/bin/sh
# 🗝️ Laws of batch `keyparts` after apply: sh verify.sh <root>
ROOT=${1:-${VERIFY_ROOT:?root required}}; HERE=$(cd "$(dirname "$0")" && pwd)
R='🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build'; BZ="$R/🔧️tools/🏗️bazel"
cd "$ROOT" || exit 2
python3 -I "$HERE/stage.py" check --root "$ROOT" || exit 2
FAILED=0
law() { name=$1; shift; out=$("$@" 2>&1); code=$?; summary=$(printf '%s\n' "$out" | grep -E "^ *[0-9]+ (pass|skip|fail)$" | tr '\n' ' '); if [ $code -eq 0 ]; then echo "GREEN $name: $summary"; else echo "RED   $name: $summary"; printf '%s\n' "$out" | grep -E "^\(fail\)|^error" | head -8; FAILED=1; fi; }
law "plan key parts (Python digest oracle, switch, static call-site law)" bun test "./$BZ/♻️plan-reuse/🧪️tests/🗝️parts/🟦️.ts"
law "plan reuse"                                       bun test "./$BZ/♻️plan-reuse/🧪️tests/🟦️.ts"
law "world witness"                                    bun test "./$R/🔭️witness/🧪️tests/🟦️.ts"
law "action keys (fingerprint harness)"                bun test "./$BZ/🧪️tests/📥️fingerprint/🟦️.ts"
law "every test file belongs to one suite"             bun test "./$R/🗂️suites/🧪️tests/🟦️.ts"
exit $FAILED
