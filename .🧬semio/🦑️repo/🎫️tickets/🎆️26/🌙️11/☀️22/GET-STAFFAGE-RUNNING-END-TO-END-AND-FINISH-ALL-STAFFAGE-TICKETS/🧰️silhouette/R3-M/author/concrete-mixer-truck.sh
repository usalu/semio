#!/bin/zsh
# 🚛️ Repair ops of `concrete-mixer-truck` of the third round (after the ops of SAM and R2-M), authored against the catalog file as it stood before this round (`🗑️generated/silhouette/R3-M/before`): the hand repairs of concrete-mixer-truck.py, then the barrel in the front elevation of 1:50 as its shell, track ring, mouth rim and the two flights project (the flights reach 1 mm through the flats of the shell) (the kernel turns the barrel as a ring of 30 flats, up to 6 mm inside the round outline the drawing had, which ended the slit beside the right mirror head higher in the drawing than in the body). `ops/concrete-mixer-truck.json` is the result and the reproducible repair.
set -e
source "${0:A:h}/../env.sh"; cd "$W/author"
export SAM_SOURCE="$GEN/before"
python3 concrete-mixer-truck.py
cd "$W"
"$B" trace.ts concrete-mixer-truck 50 front "drum-shell,drum-track-ring,drum-mouth-rim,drum-blade-1,drum-blade-2" --id drum --bone drum --like drum --replace drum ${=TRACE_ARGS} | tail -1
"$B" drawable.ts concrete-mixer-truck --ops ops/concrete-mixer-truck.json
