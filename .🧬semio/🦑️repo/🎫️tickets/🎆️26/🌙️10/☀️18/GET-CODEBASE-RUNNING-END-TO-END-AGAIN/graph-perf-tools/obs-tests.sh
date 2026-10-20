#!/bin/sh
# usage: obs-tests.sh OUT ; runs the observation suites against the control and the patched scratch roots, one process at a time.
HERE="$(cd "$(dirname "$0")" && pwd)"; OUT="$1"; export SEMIO_TEST_ARTIFACT_DIR=/private/tmp/gp/art
: > "$OUT"
for t in "👁️observation/🧪️tests/🟦️.ts" "👁️observation/🧪️tests/⚡️incremental/🟦️.ts" "👁️observation/🧪️tests/📥️reuse/🟦️.ts" "👁️observation/🧪️tests/📦️projects/🟦️.ts"; do
 for m in mirror-control mirror; do
  echo "### $m $t" >> "$OUT"
  LINES_MAX=16 "$HERE/run-tests.sh" /private/tmp/gp/$m "🏗️build/$t" >> "$OUT" 2>&1
 done
done
echo DONE >> "$OUT"
