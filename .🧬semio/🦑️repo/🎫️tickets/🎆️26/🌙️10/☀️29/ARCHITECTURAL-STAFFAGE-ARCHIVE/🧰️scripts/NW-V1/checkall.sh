#!/bin/bash
# usage: checkall.sh asset-id... ; one check per asset, sequential (shared machine)
S=/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage
G="/home/deck/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️29/ARCHITECTURAL-STAFFAGE-ARCHIVE/🗑️generated/NW-V1"
cd "$S"
rm -f "$G/check-done.txt"
for a in "$@"; do
  repo tools exec bun -- 🔨️modules/🗂️catalog/🩺️check/🟦️.ts --asset "$a" 2>&1 | grep -v "capacity\|still working" > "$G/check-$a.txt"
done
echo done > "$G/check-done.txt"
