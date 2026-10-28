#!/bin/sh
# verify.sh [root]: laws of the nx-patch batch (graph-free, single files) after `python3 ../../stage.py apply --batch nx-patch`.
# NOTE: this batch changes the bytes of the Nx toolkit (patch digest): the next Nx run builds a new toolkit and re-keys every Nx task once.
cd "${1:-/Users/ueli/Documents/semio}" || exit 1
L="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
set -e
for t in \
 "$L/⚡️caching/🕸️graph/🏁️startup/🧬️lifetime/🧪️tests/🟦️.ts" \
 "$L/⚡️caching/🚀️bootstrap/🛠️tools/🩹️patch/🧪️tests/🟦️.ts" \
 "$L/⚡️caching/💾️storage/🔒️enforcement/🔌️nx/📦️outputs/🧪️tests/🟦️.ts" \
 "$L/⚡️caching/🕸️graph/🔏️seal/🎯️selection/📥️scope/🧪️tests/🟦️.ts" \
 "$L/⚡️caching/🧪️tests/🌍️caller-environment/🟦️.ts"; do
  [ -f "$t" ] || { echo "skip (missing) $t"; continue; }
  echo "== $t"; nice -n 10 bun test --timeout 180000 "./$t" 2>&1 | tail -6
done
