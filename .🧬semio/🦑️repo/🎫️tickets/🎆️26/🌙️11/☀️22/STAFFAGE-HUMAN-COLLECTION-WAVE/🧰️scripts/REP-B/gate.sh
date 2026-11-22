#!/bin/zsh
# 🌓️ Silhouette gate and `checkVolumes` of people, one log per person in 🗑️generated/REP-B/<dir>: gate.sh <dir> <slug>…
source "${0:A:h}/env.sh"
d="$GEN/$1"; shift; mkdir -p "$d"
for a in "$@"; do
  (cd "$K/🧰️scripts/HW-03" && $B measure.ts "$a" > "$d/$a.log" 2>&1)
  echo "$(tail -1 "$d/$a.log")"
done
