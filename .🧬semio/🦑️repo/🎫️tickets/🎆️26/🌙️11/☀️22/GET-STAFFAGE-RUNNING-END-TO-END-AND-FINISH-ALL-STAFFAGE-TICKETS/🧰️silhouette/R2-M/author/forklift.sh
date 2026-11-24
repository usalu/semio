#!/bin/zsh
# 🚜️ Repair ops of `forklift` of the second round (after the ops of SAM): the mirrored copies of seven pins, eyes and rollers of the volume (forklift-volume.py), then the mast pivot pin and the inner mast roller in the plan of 1:50 and 1:100 as the repaired solids project (both sides).
source "${0:A:h}/../env.sh"; cd "$W/author" && python3 forklift-volume.py && cd "$W"
for s in 50 100; do
  "$B" trace.ts forklift $s plan pivot-pin --id pivot-pin --bone mast-outer --like pivot-pin --replace pivot-pin | tail -1
  "$B" trace.ts forklift $s plan inner-roller --id inner-roller --bone mast-inner --like inner-roller --replace inner-roller | tail -1
done
"$B" drawable.ts forklift --ops ops/forklift.json
