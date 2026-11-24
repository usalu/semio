#!/bin/zsh
# 🧵️ Repair ops of `used-sewing-machine`: the carrying handle of the hood is a U-shaped grip in the volume (a 124 × 20 mm hand opening); the front elevation of 1:50 drew it as a full bar (31.9 mm). It is drawn as the solid `hood-handle` projects, with the opening.
source "${0:A:h}/../env.sh"; cd "$W"
echo '[]' > ops/used-sewing-machine.json
"$B" trace.ts used-sewing-machine 50 front hood-handle --id hood-handle --bone hood --like hood-handle --holes --tol 0.0003
