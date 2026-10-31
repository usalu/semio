#!/usr/bin/env bash
S="/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage"
T="/home/deck/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️29/ARCHITECTURAL-STAFFAGE-ARCHIVE"
O="$T/🗑️generated/NW-MA"
cd "$S"
for a in "$@"; do
  repo tools exec bun -- 🔨️modules/🗂️catalog/🩺️check/🟦️.ts --asset "$a" 2>&1 | grep -v "^\[" > "$O/check-$a.txt"
  repo tools exec bun -- "$T/🧰️scripts/core/suit-probe.ts" --asset "$a" --limit 14 2>&1 | grep -v "^\[" > "$O/probe-$a.txt"
done
echo done > "$O/runcheck-$$.done"
