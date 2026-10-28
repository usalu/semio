#!/bin/sh
# ✅ Verifies batch planmemo: the memo law (13 edits with real Cargo: served exactly while inputs hold, every served plan equals Cargo's own plan now; Ajv) and 27 capture/units/testing/launcher/unit-cache suites (no failure outside the pre-existing set).
# usage: sh verify.sh [root]   (default root: the checkout holding this ticket; SEMIO_TEST_ARTIFACT_DIR defaults to <ticket>/🗑️generated/bootreg/verify-<batch>)
HERE=$(cd "$(dirname "$0")" && pwd); B=$(basename "$HERE"); C=$(cd "$HERE/../../.." && pwd); R=${1:-$(cd "$C/../../../../../../.." && pwd)}
A=${SEMIO_TEST_ARTIFACT_DIR:-$C/🗑️generated/bootreg/verify-$B}; mkdir -p "$A"
cd "$R" || exit 2
python3 -I "$HERE/stage.py" check --root "$R" | grep '^base' && { echo "VERIFY FAIL: batch not applied at $R"; exit 1; }
status=0; for law in $(cat "$HERE/laws.txt"); do SEMIO_TEST_ARTIFACT_DIR="$A" bun test --timeout 300000 "$law" > "$A/law.out" 2>&1 || status=1; tail -4 "$A/law.out" | grep -E 'pass|fail' | tr '\n' ' '; echo "$law" | sed 's|.*/📚️library/||;s|.*/🔨️modules/||'; done
[ $status = 0 ] || { echo "VERIFY FAIL: a law of the batch is red (see $A/law.out)"; exit 1; }
SEMIO_TEST_ARTIFACT_DIR="$A" bun test --timeout 300000 --reporter=junit --reporter-outfile="$A/suites.xml" $(tr '\n' ' ' < "$HERE/suites.txt") > "$A/suites.out" 2>&1
python3 -I "$C/cargo2-tools/junit-set.py" "$A/suites.xml" > "$A/suites.set"; head -1 "$A/suites.set"
new=$(sed 1d "$A/suites.set" | sort | comm -13 "$HERE/known-failures.txt" -)
if [ -z "$new" ]; then echo "VERIFY OK: no suite failure outside the pre-existing set"; else echo "VERIFY FAIL: new failures:"; echo "$new"; exit 1; fi
