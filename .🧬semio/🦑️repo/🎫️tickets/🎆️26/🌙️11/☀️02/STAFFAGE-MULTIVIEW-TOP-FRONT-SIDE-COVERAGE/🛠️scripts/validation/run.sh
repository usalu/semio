#!/usr/bin/env sh
# Regenerates the coverage matrix, its summary and the proof sheets.
# usage: run.sh <ticket-generated-dir>   (default: ../../🗑️generated/validation relative to this script)
set -eu
here="$(cd "$(dirname "$0")" && pwd)"
out="${1:-$here/../../🗑️generated/validation}"
tests="$(cd "$here/../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🔭️multiview-coverage" && pwd)"
mkdir -p "$out/proofs"
bun "$tests/📊️matrix.ts" --out "$out" --jobs "${JOBS:-14}"
bun "$here/summary.ts" "$out" "$out/summary.md"
for c in humans animals vegetation furniture appliances objects vehicles machinery components; do
  bun "$tests/🖨️proofs.ts" --out "$out/proofs" --categories "$c" &
done
wait
