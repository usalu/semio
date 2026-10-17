#!/bin/zsh
# runs the permanent route three times: sdk2 roots, psc roots, whole workspace; logs next to the generated projects
cd /Users/ueli/Documents/semio
G="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/🗑️generated"
: > "$G/tsl-tsc-status.txt"
run() { local label="$1"; shift
  local t0=$(date +%s)
  bun ./📜️script.ts verify typescript check --out "$G/tsl-$label.tsconfig.json" --max-diagnostics 1000000 "$@" > "$G/tsl-$label.route.txt" 2>&1
  echo "$label exit=$? seconds=$(( $(date +%s) - t0 )) $(date +%T)" >> "$G/tsl-tsc-status.txt"; }
run sdk2 --files-from "$G/tsl-roots-sdk2.txt"
run psc --files-from "$G/tsl-roots-psc.txt"
run whole
echo DONE >> "$G/tsl-tsc-status.txt"
