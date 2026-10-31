#!/bin/bash
# usage: m.sh asset-id...  (prints violations; output to GEN)
cd /home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage
files=()
for a in "$@"; do files+=("/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/👤️humans/$a/🔣️.json"); done
repo tools exec bun -- "/home/deck/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️29/ARCHITECTURAL-STAFFAGE-ARCHIVE/🧰️scripts/NW-H2/measure.ts" "${files[@]}" 2>&1 | grep -v "^\[capacity\]\|^\[repo\]\|^\[tools\]"
