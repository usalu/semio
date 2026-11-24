#!/bin/zsh
# 🌓️ Every check of the silhouette guard of people, one process after the other, one log per person in 🗑️generated/silhouette/R2-HB/<dir>: gate.sh <dir> <slug>…   (VOL=1 adds every rule of `checkVolumes`, SCALES=50,100 limits the scales)
source "${0:A:h}/env.sh"
d="$GEN/$1"; shift; mkdir -p "$d"
for a in "$@"; do
  (cd "$S/📦️packages/🟦️typescript" && timeout 900 "$B" "$W/measure.ts" "$a" ${VOL:+--volumes} ${SCALES:+--scales=$SCALES} > "$d/${a:t:r}.log" 2>&1)
  grep -E "!|: (OK|[0-9]+ failing)|findings [1-9]|rror" "$d/${a:t:r}.log"
done
