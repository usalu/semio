#!/bin/zsh
# 🦾️ Repair ops of `excavator` of the third round (after the ops of R2-M), authored against the catalog file as it stood before this round (`🗑️generated/silhouette/R3-M/before`): the hand repairs of excavator.py. `ops/excavator.json` is the result and the reproducible repair.
set -e
source "${0:A:h}/../env.sh"; cd "$W/author"
SAM_SOURCE="$GEN/before" python3 excavator.py
cd "$W" && "$B" drawable.ts "$GEN/before/excavator.json" --ops ops/excavator.json
