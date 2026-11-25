#!/bin/zsh
# 🌬️ The wind poses of the plants that carry depth plates, meant for one grant (`grant.sh 1200 <timeout> wind.sh <id>…`): `visible.ts` per plant (what the plates show beside the drawn outlines in every pose but the default), then the sheet of its worst scale and pose (`look.sh`). Lines go to `$GEN/log/wind.txt`.
export BATCH=MV-3 MV3_GRANT=1; source "${0:A:h}/../MR/env.sh"
o="$GEN/log/wind.txt"; echo "wind $(date +%F\ %H:%M)" >> "$o"
for a in "$@"; do
  (cd "$T" && timeout 1500 "$B" "$W/visible.ts" "$a") > "$GEN/log/visible-$a.log" 2>&1; echo "$a visible exit $? $(tail -1 "$GEN/log/visible-$a.log" | cut -c1-260)" >> "$o"
  for item in $(grep '^WORST' "$GEN/log/visible-$a.log" | sed -nE 's/^WORST [^ ]+ 1:([0-9]+) [a-z]+ ([^:]+):.*/\1:\2/p'); do POSE="${item#*:}" "$W/look.sh" "$a" "${item%%:*}" >> "$o" 2>&1; done
done
echo "wind done $(date +%H:%M)" >> "$o"
zsh "$W/worker.sh" 800
