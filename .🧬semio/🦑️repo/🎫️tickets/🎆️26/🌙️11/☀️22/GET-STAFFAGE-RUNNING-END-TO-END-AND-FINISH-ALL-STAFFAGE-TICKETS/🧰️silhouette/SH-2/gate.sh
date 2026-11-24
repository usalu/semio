#!/bin/zsh
# 🌓️ Every check of the silhouette guard of people, one process after the other, one log per person in 🗑️generated/silhouette/SH-2/<dir>: gate.sh <dir> <slug>…   (VOL=1 adds every rule of `checkVolumes`)
source "${0:A:h}/env.sh"
d="$GEN/$1"; shift; mkdir -p "$d"
for a in "$@"; do
  (cd "$S/📦️packages/🟦️typescript" && timeout 600 "$B" "$W/measure.ts" "$a" ${VOL:+--volumes} > "$d/$a.log" 2>&1)
  grep -E "!|: (OK|[0-9]+ failing)|findings [1-9]" "$d/$a.log"
done
