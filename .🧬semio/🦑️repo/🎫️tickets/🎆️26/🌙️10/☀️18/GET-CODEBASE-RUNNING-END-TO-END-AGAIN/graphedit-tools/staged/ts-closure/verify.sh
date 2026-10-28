#!/bin/sh
# 🧪 Laws of batch ts-closure, run against ROOT (a mirror with the batch applied): sh verify.sh <root>
# Exit 0 when the new law passes 4/4 and the neighbour suites keep their recorded pass/fail sets.
set -u
ROOT=${1:?root}; HERE=$(cd "$(dirname "$0")" && pwd); L="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build"; ART="$ROOT/.tmp/ts-closure-art"; OUT=$(mktemp -d); mkdir -p "$ART"; cd "$ROOT" || exit 2
python3 -I "$HERE/stage.py" check "$ROOT" | grep -v '^applied' && { echo "batch not applied at $ROOT"; exit 2; }
run() { SEMIO_TEST_ARTIFACT_DIR="$ART" bun test "./$1" > "$OUT/$2.txt" 2>&1; grep -E '^\((pass|fail)\)' "$OUT/$2.txt" | sed -E 's/ \[[0-9.]+m?s\]$//' | sort > "$OUT/$2.set"; printf '%s: %s\n' "$2" "$(grep -E ' (pass|fail)$' "$OUT/$2.txt" | tr '\n' ' ')"; }
run "$L/🕸️graph/📥️inference/🧭️script-inputs/🧪️tests/🪁️program-reads/🟦️.ts" law
run "$L/🕸️graph/📥️inference/🧭️script-inputs/🧪️tests/🟦️.ts" inputs
run "$L/🕸️graph/📥️inference/🧪️tests/✂️derived-walks/🟦️.ts" walks
run "$L/🧪️tests/🕸️graph/🎯️source-demand/🟦️.ts" demand
ok=0
grep -q '^ 4 pass' "$OUT/law.txt" && grep -q '^ 0 fail' "$OUT/law.txt" || { echo "LAW RED"; ok=1; }
grep -q '^ 0 fail' "$OUT/inputs.txt" || { echo "script-inputs suite red"; ok=1; }
[ "$(grep -c '^(fail)' "$OUT/walks.set")" -le 1 ] && grep -q 'reused by a project is named by one derived fact' "$OUT/walks.set" || { echo "derived-walks changed (base: 3 pass, 1 pre-existing fail)"; ok=1; }
diff "$HERE/baselines/source-demand.set" "$OUT/demand.set" > /dev/null || { echo "source-demand pass/fail set differs from base"; diff "$HERE/baselines/source-demand.set" "$OUT/demand.set"; ok=1; }
rm -rf "$OUT"; [ $ok = 0 ] && echo "ts-closure VERIFIED"; exit $ok
