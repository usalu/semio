#!/bin/zsh
# 🚗️ Repair ops of `sectional-garage-door` of the third round, authored against the catalog file as it stood before this round (`🗑️generated/silhouette/R3-M/before`): sectional-garage-door.py. `ops/sectional-garage-door.json` is the result and the reproducible repair.
set -e
source "${0:A:h}/../env.sh"; cd "$W/author"
SAM_SOURCE="$GEN/before" python3 sectional-garage-door.py
cd "$W" && "$B" drawable.ts "$GEN/before/sectional-garage-door.json" --ops ops/sectional-garage-door.json
