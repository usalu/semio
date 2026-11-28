#!/bin/zsh
# 📏️ `sizes.ts` inside the grant of its caller, appended to `$GEN/sizes.txt` with the time: sizes.sh <id | file.json>…
source "${0:A:h}/env.sh"
zsh "$W/run.sh" sizes.ts "$@" 2>&1 | grep SIZES | sed "s/^/$(date +%H:%M) /" | tee -a "$GEN/sizes.txt"
