#!/bin/zsh
# 📏️ All checks of the guard for plants, one process after the other, logs in 🗑️generated/silhouette/SV-2/<dir>: m.sh <dir> <slug>…  (SCALES=500,200 limits the scales)
source "${0:A:h}/env.sh"
d="$GEN/$1"; shift; mkdir -p "$d"
cd "$S/📦️packages/🟦️typescript"
for a in "$@"; do
  timeout ${TMO:-1500} "$B" "$W/measure.ts" "$a" ${SCALES:+--scales=$SCALES} > "$d/$a.log" 2>&1
  grep -E "FAIL|failing checks" "$d/$a.log" || tail -3 "$d/$a.log"
done
