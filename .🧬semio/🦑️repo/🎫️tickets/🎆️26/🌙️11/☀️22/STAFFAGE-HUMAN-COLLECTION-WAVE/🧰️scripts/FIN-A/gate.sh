#!/bin/zsh
# 🌓️ Silhouette guard (bounding box, IoU, largest boundary distance per scale and view) and `checkVolumes` of people, one log per person in 🗑️generated/FIN-A/gate: gate.sh <slug>…
source "${0:A:h}/env.sh"
mkdir -p "$GEN/gate"
for a in "$@"; do
  (cd "$T/🧰️scripts/HW-03" && $B measure.ts "$a" > "$GEN/gate/$a.log" 2>&1)
  echo "$(tail -1 "$GEN/gate/$a.log")"
done
