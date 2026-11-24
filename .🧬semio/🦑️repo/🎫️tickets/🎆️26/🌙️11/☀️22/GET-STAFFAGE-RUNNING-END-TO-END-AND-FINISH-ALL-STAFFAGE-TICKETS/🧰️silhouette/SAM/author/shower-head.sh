#!/bin/zsh
# 🚿️ Repair ops of `shower-head`: the hose was one stroke on the wall elbow that ran 2 to 4 mm beside the hose of the volume (a 13 to 15 mm hose: side elevation IoU 0.901 at 1:50, 0.912 at 1:100). It is drawn as the volume has it, in two parts that follow their bones: the hanging loop on the elbow and the rise to the hand shower on the `hose` bone, each the traced outline of its solid.
source "${0:A:h}/../env.sh"; cd "$W"
echo '[]' > ops/shower-head.json
for s in 50 100; do
  "$B" trace.ts shower-head $s elevation hose --id hose --bone elbow --like hose
  "$B" trace.ts shower-head $s elevation hose-rise --id hose-rise --bone hose --like hose --after hose
done
