#!/bin/sh
# 🔏️ Laws of batch `rowid` after apply: sh verify.sh <root>   (repository or mirror root; SEMIO_TEST_ARTIFACT_DIR defaults below the ticket's generated folder)
ROOT=${1:-${VERIFY_ROOT:?root required}}; HERE=$(cd "$(dirname "$0")" && pwd)
R='🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build'; BZ="$R/🔧️tools/🏗️bazel"
cd "$ROOT" || exit 2

FAILED=0
law() { name=$1; shift; out=$("$@" 2>&1); code=$?; summary=$(printf '%s\n' "$out" | grep -E "^ *[0-9]+ (pass|fail)$" | tr '\n' ' '); if [ $code -eq 0 ]; then echo "GREEN $name: $summary"; else echo "RED   $name: $summary"; printf '%s\n' "$out" | grep -E "^\(fail\)|^error" | head -8; FAILED=1; fi; }
law "runtime row identity (module law, Python oracle, real registry)" bun test "./$BZ/🌿️environment/🔏️identity/🧪️tests/🟦️.ts"
law "keyed command environment"                                       bun test "./$BZ/🌿️environment/🧪️tests/🟦️.ts"
law "action keys (fingerprint harness)"                               bun test "./$BZ/🧪️tests/📥️fingerprint/🟦️.ts"
law "world witness"                                                   bun test "./$R/🔭️witness/🧪️tests/🟦️.ts"
law "plan reuse"                                                      bun test "./$BZ/♻️plan-reuse/🧪️tests/🟦️.ts"
law "plan reuse ignore sources"                                       bun test "./$BZ/♻️plan-reuse/🧪️tests/🧭️ignore-sources/🟦️.ts"
law "every test file belongs to one suite"                            bun test "./$R/🗂️suites/🧪️tests/🟦️.ts"

exit $FAILED
