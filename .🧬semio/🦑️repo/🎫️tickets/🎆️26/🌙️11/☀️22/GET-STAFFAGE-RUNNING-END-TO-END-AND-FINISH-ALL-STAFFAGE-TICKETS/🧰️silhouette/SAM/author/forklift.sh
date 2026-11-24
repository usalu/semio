#!/bin/zsh
# 🚜️ Repair ops of `forklift`: the declared length (declared-dimensions.py), then the parts of the volume that no view drew (mast rollers and pins, pivot brackets, cylinder seats, axle gear, the front of the frame at 1:500 and more; patch-views.sh), at 1:50 widened by 1.2 mm against seams of one raster cell, and the mast roller in the plan of 1:50 as its solid projects. One check stays red, see the report.
source "${0:A:h}/../env.sh"; cd "$W/author" && python3 declared-dimensions.py >/dev/null && cd "$W"
author/patch-views.sh forklift "100 200 500"
"$B" patch.ts forklift 500 elevation --thin 0.02 --min 0.2
PATCH_FLAGS="--grow 0.0012" author/patch-views.sh forklift 50
"$B" trace.ts forklift 50 plan inner-roller --id inner-roller --bone mast-inner --like inner-roller
"$B" printable.ts forklift
