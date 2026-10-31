#!/bin/bash
# usage: prev.sh asset scale pose dpi
cd /home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage
T="/home/deck/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️29/ARCHITECTURAL-STAFFAGE-ARCHIVE"
repo tools exec bun -- 🔨️modules/🎨️depiction/🖨️preview/🟦️.ts --asset "$1" --scale "$2" --pose "$3" --print-size --dpi "${4:-1200}" --out "$T/🗑️generated/NW-VE/$1-$2-$3" 2>&1 | grep -v capacity | grep -v "still working" | tail -3
