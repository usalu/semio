#!/bin/sh
# ✅ Verifies batch selnear on a root (default: the live checkout): the new law (5 tests: schema, declared = inventory = Cargo metadata with zero listings, exhaustive corpus oracle, stale member, wiring) and the 29 workspace/admission suites, whose failure set must equal the recorded pre-existing set (envelope-only failures under bare `bun test`).
# usage: sh verify.sh [root]   (SEMIO_TEST_ARTIFACT_DIR defaults to <ticket>/🗑️generated/bootreg/verify)
HERE=$(cd "$(dirname "$0")" && pwd); C=$(cd "$HERE/../../.." && pwd); R=${1:-$(cd "$C/../../../../../../.." && pwd)}
A=${SEMIO_TEST_ARTIFACT_DIR:-$C/🗑️generated/bootreg/verify}; mkdir -p "$A"; X="$A/junit.xml"
cd "$R" || exit 2
SEMIO_TEST_ARTIFACT_DIR="$A" bun test --timeout 120000 "./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🧪️tests/🪧️declared-selection/🟦️.ts" 2>&1 | tail -4
SEMIO_TEST_ARTIFACT_DIR="$A" bun test --timeout 120000 --reporter=junit --reporter-outfile="$X" $(tr '\n' ' ' < "$HERE/suites.txt") > "$A/suites.out" 2>&1
python3 -I "$C/cargo2-tools/junit-set.py" "$X" > "$A/suites.set"; head -1 "$A/suites.set"
if sed 1d "$A/suites.set" | diff - "$HERE/expected-failures.txt" > "$A/suites.diff"; then echo "VERIFY OK: failure set equals the pre-existing set"; else echo "VERIFY FAIL: failure set differs (see $A/suites.diff)"; exit 1; fi
