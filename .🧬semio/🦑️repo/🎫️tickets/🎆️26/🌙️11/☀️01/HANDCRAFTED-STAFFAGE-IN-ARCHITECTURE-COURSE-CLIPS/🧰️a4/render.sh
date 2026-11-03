#!/bin/bash
# usage: render.sh asset scale cellpx pose:view[:env] ...   -> sheet png
cd /c/git/semio
S="🎓️teaching/🏛️architecture/🧍️staffage"
T=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/HANDCRAFTED-STAFFAGE-IN-ARCHITECTURE-COURSE-CLIPS"
A=$1; SC=$2; PX=$3; shift 3
OUT="$T/🗑️generated/a4/$A"; rm -rf "$OUT/cast"; mkdir -p "$OUT/cast"
M="$OUT/cast/m.json"; echo '{ "version": 1, "casts": {' > $M; IDS=""; FIRST=1
for X in "$@"; do IFS=: read P V E TR <<< "$X"; ID="$P-$V${TR:+-$TR}"; [ -n "$IDS" ] && echo ',' >> $M; IDS="$IDS,$ID"
  ENV=""; [ -n "$E" ] && ENV=", \"environment\": \"$E\""
  TRJ=""; [ -n "$TR" ] && TRJ=", \"transport\": [\"$TR\"]"; echo "\"$ID\": { \"asset\": \"$A\", \"pose\": \"$P\", \"view\": \"$V\", \"theme\": \"dark\", \"scale\": $SC$ENV$TRJ }" >> $M; done
echo '} }' >> $M
repo tools exec bun -- $S/🔨️modules/📤️export/🎭️cast/🟦️.ts --manifest "$M" --ids "${IDS:1}" --out "$OUT/cast" --lang en 2>&1 | grep -v '^\[' | tail -3
FILES=(); for d in "$OUT"/cast/*/; do FILES+=("$d"0000.svg); done
N=${#FILES[@]}; COLS=$N; [ $COLS -gt 4 ] && COLS=4
bun "$T/🧰️a4/sheet.mjs" "$OUT/sheet-$SC.png" $PX $COLS "${FILES[@]}"
echo "$OUT/sheet-$SC.png"
