#!/bin/zsh
# 🌬️ `wind.ts` for the plants of MV-2 inside ONE reservation of the memory queue: every line of the list is `<id> <pose,pose,…> <scale,scale,…>`; sheets land in `$GEN/sheets`, the numbers in `$GEN/wind.log`. wind.sh <megabytes> <list file>
BATCH=MV-2 source "${0:A:h}/../MR/env.sh"
export T14="$R/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE"
cd "$R" && repo capacity run --memory="$1" -- zsh -c '
cd "$T14"
while read -r a poses scales; do
  [[ -z "$a" ]] && continue
  /usr/bin/time -l timeout 3000 "$B" "$W/wind.ts" "$a" --poses "$poses" --scales "$scales" --sheets "$GEN/sheets" > "$GEN/wind-$a.log" 2>&1; echo "$a exit $? peak $(grep "maximum resident" "$GEN/wind-$a.log" | awk "{print int(\$1/1048576)}") MB" >> "$GEN/wind.log"
  grep -E "^$a 1:" "$GEN/wind-$a.log" >> "$GEN/wind.log"
done < "$0"' "${2:A}"
