#!/bin/sh
# verify.sh [root]: the laws of the nxfix-g batch, single-file runs, graph-free (run after `python3 ../stage.py apply`).
cd "${1:-/Users/ueli/Documents/semio}" || exit 1
F="🧰️framework"; L="$F/🛍️products/🦑️repo/🔨️modules/📚️library"; X="$F/🔨️modules"
set -e
for t in \
 "$L/🏗️build/🔧️tools/🧶️nx/🪵️failure-tail/🧪️tests/🟦️.ts" \
 "$X/📁️filesystem/🧾️observation/🔁️churn/🧪️tests/🧾️refusals/🟦️.ts" \
 "$X/🏃️process/🔒️leases/🧭️order/🧪️tests/🟦️.ts" \
 "$L/⚡️caching/💾️storage/🔒️enforcement/🔌️nx/🎟️put-slots/🧪️tests/🟦️.ts" \
 "$L/🏗️build/🧷️format-stamps/🧪️tests/🟦️.ts"; do
  echo "== $t"; nice -n 10 bun test --timeout 120000 "./$t" 2>&1 | tail -6
done
