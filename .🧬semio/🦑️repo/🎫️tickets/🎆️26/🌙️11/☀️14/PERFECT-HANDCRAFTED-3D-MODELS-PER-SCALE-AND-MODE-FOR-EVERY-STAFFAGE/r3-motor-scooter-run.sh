#!/bin/sh
# 🛵 Builds the scooter's volumes source, projects the outlines of the drawn parts, builds the drawings from them, writes the candidate document, validates it, prints the print suitability and renders the sheets named as arguments ("50 body", "100 structure --cut", ...).
cd "$(dirname "$0")" || exit 1
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
G="🗑️generated/r3-motor-scooter"
C="$G/🔣️.json"
python3 r3-motor-scooter-build.py >/dev/null && python3 r3-motor-scooter-roots.py --no-2d >/dev/null || exit 1
"$BUN" r3-motor-scooter-project2d.ts "$C" "$G/jobs.json" "$G/outlines.json" || exit 1
python3 r3-motor-scooter-build.py && python3 r3-motor-scooter-roots.py >/dev/null && "$BUN" r3-validate.ts --file "$C" 2>&1 | cut -c1-360 | tail -30
"$BUN" r3-motor-scooter-suit.ts --file "$C"
for spec in "$@"; do
  set -f
  # shellcheck disable=SC2086
  set -- $spec
  scale=$1; list=$2; shift 2
  "$BUN" r3-motor-scooter-sheet.ts vehicles/motor-scooter --scale "$scale" --list "$list" "$@" 2>&1 | tail -1
done
