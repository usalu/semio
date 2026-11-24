#!/bin/zsh
# 🔥️ Repair ops of `plate-heat-exchanger`: the two base rails are hollow sections in the volume (seen end-on in the side elevation: a 36 × 26 mm opening); the side elevation of 1:50 drew them as full blocks (18.8 mm). They are drawn as their solids project, with the opening.
source "${0:A:h}/../env.sh"; cd "$W"
echo '[]' > ops/plate-heat-exchanger.json
for r in rear front; do "$B" trace.ts plate-heat-exchanger 50 elevation base-rail-$r --id base-rail-$r --bone frame --like base-rail-$r --holes --tol 0.0003; done
