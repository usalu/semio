#!/bin/sh
# 🛴 Builds the scooter's volumes source and drawings, writes the candidate document, validates it, prints the print suitability and renders the sheets named as arguments ("50 body", "100 structure --cut", ...).
cd "$(dirname "$0")" || exit 1
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
C="🗑️generated/r3-electric-kick-scooter/🔣️.json"
python3 r3-electric-kick-scooter-build.py && python3 r3-electric-kick-scooter-roots.py >/dev/null && "$BUN" r3-validate.ts --file "$C" 2>&1 | cut -c1-360 | tail -30
"$BUN" r3-electric-kick-scooter-suit.ts --file "$C"
for spec in "$@"; do
  set -f
  # shellcheck disable=SC2086
  set -- $spec
  scale=$1; list=$2; shift 2
  "$BUN" r3-electric-kick-scooter-sheet.ts vehicles/electric-kick-scooter --scale "$scale" --list "$list" "$@" 2>&1 | tail -1
done
