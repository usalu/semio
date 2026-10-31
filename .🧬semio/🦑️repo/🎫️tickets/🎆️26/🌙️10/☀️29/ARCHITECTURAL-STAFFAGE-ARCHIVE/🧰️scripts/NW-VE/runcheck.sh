#!/bin/bash
# usage: runcheck.sh tag assets(comma)
cd /home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage
T="/home/deck/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️29/ARCHITECTURAL-STAFFAGE-ARCHIVE"
G="$T/🗑️generated/NW-VE"
repo tools exec bun -- "$T/🧰️scripts/NW-VE/probe.ts" --assets "$2" 2>&1 | grep -v capacity | grep -v "still working" > "$G/probe-$1.txt"
repo tools exec bun -- "$T/🧰️scripts/core/check-all.ts" --out "$G/check-$1.md" --json "$G/check-$1.json" --samples 3 --only "$2" > "$G/check-$1.log" 2>&1
echo done > "$G/done-$1"
