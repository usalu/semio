#!/bin/sh
# Runs one rewrite batch: dry run, write with snapshot, verify. Usage: batch.sh <label> <area-prefix>...
L="$1"; shift
F="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
G="$F/🗑️generated"
cd "$F" || exit 2
ARGS=""
for a in "$@"; do ARGS="$ARGS --area $a"; done
bun psc-tools/rewrite.ts "$G/psc-plan.json" "$G/psc-scan-2.json" $ARGS --write --snapshot "$G/psc-snap/$L.json" --report "$G/psc-snap/$L.report.json.txt" > "$G/psc-batch-$L.log" 2>&1
tail -n 60 "$G/psc-batch-$L.log" | cut -c1-260
