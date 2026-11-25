#!/bin/zsh
# 🔥️ Phase times of the matrix for `weeping-willow` with the tool of lane MM (`../MM/hot.ts`, read only) on the live tree, one process per phase, meant for one grant (`grant.sh 1400 3000 hot.sh`), and the scan of `saguaro-cactus` as it stands. Lines go to `$GEN/log/hot.txt`.
export BATCH=MV-3; source "${0:A:h}/../MR/env.sh"
o="$GEN/log/hot.txt"; echo "hot $(date +%F\ %H:%M)" >> "$o"
(cd "$T" && timeout 1500 "$B" 🔎️scan.ts --language en saguaro-cactus) > "$GEN/log/scan-now-saguaro-cactus.log" 2>&1; echo "saguaro scan exit $? $(tail -1 "$GEN/log/scan-now-saguaro-cactus.log" | cut -c1-200)" >> "$o"
for p in suitability evaluate derived render; do (cd "$T" && timeout 900 "$B" "$U/🧰️matrix/MM/hot.ts" --tree "$R" --only $p --count 8 weeping-willow:50) 2>&1 | grep DEBUG >> "$o"; echo "$p exit $pipestatus[1] $(date +%H:%M)" >> "$o"; done
