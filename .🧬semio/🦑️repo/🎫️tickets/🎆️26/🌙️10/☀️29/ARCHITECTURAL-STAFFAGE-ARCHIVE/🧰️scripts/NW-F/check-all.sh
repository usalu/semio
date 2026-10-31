#!/bin/sh
S="/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage"
G="/home/deck/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️29/ARCHITECTURAL-STAFFAGE-ARCHIVE/🗑️generated/NW-F"
cd "$S"
for a in "$@"; do
  repo tools exec bun -- 🔨️modules/🗂️catalog/🩺️check/🟦️.ts --asset $a 2>&1 | grep -v '^\[capacity\]\|^\[repo\]\|^\[tools\]' > "$G/check-$a.txt"
done
