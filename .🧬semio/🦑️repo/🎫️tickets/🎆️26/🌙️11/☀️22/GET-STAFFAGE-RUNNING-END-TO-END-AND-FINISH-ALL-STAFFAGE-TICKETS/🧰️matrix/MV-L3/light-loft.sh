#!/bin/zsh
# 🪶️ The light proof of the loft copies inside one grant: `light-loft.sh <id>…` runs `light.sh <id> loft` per plant, prints its lines and, for every view the guard marks with `!`, where the guard sees its distance (`../MX-3/guard-where.ts <copy> <scale> <view>`).
source "${0:A:h}/env.sh"
for a in "$@"; do
  zsh "$W/light.sh" "$a" loft; cat "$GEN/light/$a-loft.txt"
  grep "bbox" "$GEN/light/$a-loft.txt" | while read -r line; do
    s="${${line#* 1:}%% *}"
    for v in elevation front plan; do print -r -- "$line" | grep -qE "$v [0-9.]+!?/[0-9.]+mm!" && { echo "-- where 1:$s $v"; (cd "$T" && timeout 1200 "$B" "$U/🧰️matrix/MX-3/guard-where.ts" "$GEN/loft/$a.json" "$s" "$v" 2>&1 | tail -25 | cut -c1-260); }; done
  done
done
