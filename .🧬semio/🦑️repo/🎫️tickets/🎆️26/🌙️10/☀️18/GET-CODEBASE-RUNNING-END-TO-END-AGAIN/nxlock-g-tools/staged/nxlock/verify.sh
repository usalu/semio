#!/bin/sh
# verify.sh [root]: the laws of the nxlock-g batch, single-file runs, graph-free (run after `python3 ../../stage.py apply`; the nxfix-g batch `core` must be applied first).
cd "${1:-/Users/ueli/Documents/semio}" || exit 1
F="🧰️framework"; L="$F/🛍️products/🦑️repo/🔨️modules/📚️library"; O="$F/🔨️modules/🏃️process/🔒️leases/🧭️order"; S="$L/⚡️caching/🕸️graph/🏁️startup"
set -e
for t in \
 "$O/🧪️tests/🟦️.ts" \
 "$O/🧪️tests/🧩️component-driver/🟦️.ts" \
 "$S/🧪️tests/🟦️.ts" \
 "$S/📥️phase/🧪️tests/🟦️.ts" \
 "$L/⚡️caching/💾️storage/🔒️enforcement/🔌️nx/🎟️put-slots/🧪️tests/🟦️.ts"; do
  echo "== $t"; nice -n 10 bun test --timeout 120000 "./$t" 2>&1 | tail -6
done
