#!/bin/zsh
# 🥢️ The five assets whose silhouette guard breaks when `lateralScale` is read, with the member removed on the shapes lateral-drop.ts found: trial copies in 🗑️generated/matrix/MX-2/lateral/<id>.dropped.json, guard at every scale, gate over every subject of the scale; runs inside a grant that is already held: lateral-dropped.sh
export BATCH=MX-2; source "${0:A:h}/../MR/env.sh"; P="$S/📦️packages/🟦️typescript"
for row in "horse 50 tail-1" "hiker-with-trekking-poles 50 upper-arm-l,upper-arm-r" "person-with-white-cane 50 forearm-l,forearm-r" "worker-carrying-box 50 upper-arm-l,upper-arm-r" "worker-with-pallet-jack 100 upper-arm-l,upper-arm-r"; do
  set -- ${=row}; f="$GEN/lateral/$1.dropped.json"
  (cd "$P" && timeout 600 "$B" "$W/lateral-drop.ts" "$1" "$2" --drop "$3" --out "$f"; timeout 1200 "$B" "$W/guard.ts" "$f" --tag "dropped ") 2>&1 | cut -c1-240
  (cd "$T" && timeout 3600 "$B" "$M/gate.ts" "$f" --scales "$2") > "$GEN/lateral/gate-$1-dropped.txt" 2>&1; echo "$1 1:$2 dropped gate: $(tail -1 "$GEN/lateral/gate-$1-dropped.txt")"
done
