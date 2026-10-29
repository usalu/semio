#!/usr/bin/env bash
# [scratch] cow-png.sh <name> <preview args...>: previews the cow to OUT/dairy-cow/<name>.png
T=/home/deck/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️29/ARCHITECTURAL-STAFFAGE-ARCHIVE; OUT=$T/🗑️generated/animals/dairy-cow
n=$1; shift
bash $T/🧰️scripts/animals/cow-view.sh --out $OUT/$n.svg "$@"; rsvg-convert -z ${ZOOM:-3} $OUT/$n.svg -o $OUT/$n.png
