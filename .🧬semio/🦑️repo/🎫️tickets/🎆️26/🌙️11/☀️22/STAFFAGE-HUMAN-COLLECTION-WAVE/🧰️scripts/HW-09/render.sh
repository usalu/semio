#!/bin/sh
# 🖼️ HW-09 preview: render.sh <asset> <scale> <pose|clip:id@t> [view] [extra preview flags...] -> PNG sheet in 🗑️generated/HW-09/<asset>/
S="/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage"
T="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/STAFFAGE-HUMAN-COLLECTION-WAVE"
B=$(ls /Users/ueli/Documents/semio/.cache/b/t/m/bun/*/bun)
A=$1; SC=$2; Q=$3; V=${4:-light-solid}; shift 4 2>/dev/null
case "$Q" in clip:*) C=${Q#clip:}; ID=${C%@*}; TM=${C#*@}; QF="--clip $ID --time $TM";; *) QF="--pose $Q";; esac
OUT="$T/🗑️generated/HW-09/$A"
mkdir -p "$OUT"
cd "$S" && repo tools exec bun -- "🔨️modules/🎨️depiction/🖨️preview/🟦️.ts" --asset "$A" --scale "$SC" $QF --view "$V" --views elevation,front,plan --projection third --flow none --profile ${PROFILE:-office-laser} --no-png --out "$OUT" "$@" 2>&1 | grep -v "^\[" | tail -3
cd "$T/🧰️scripts/HW-09" && $B png.ts --density ${DENSITY:-200} "$OUT" >/dev/null
ls -t "$OUT"/*.png | head -1
