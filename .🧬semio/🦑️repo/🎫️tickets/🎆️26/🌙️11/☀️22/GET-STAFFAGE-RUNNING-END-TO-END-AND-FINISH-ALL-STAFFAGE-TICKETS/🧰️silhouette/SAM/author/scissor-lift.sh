#!/bin/zsh
# 🪜️ Repair ops of `scissor-lift` (side and front elevation of 1:50, 1:100 and 1:200; 69 to 158 mm). The volume has parts that no view drew: the ladder bracket, the motor vent grille, the lift cylinder with its port, the mount of the control box, the tracks of the slide rollers, the lug bearers and base lugs of the chassis, the pin shafts of the scissor arms, the deck channels and the gate hinge and latch. Each is drawn where it shows beside what is already drawn, as the guard projects it (patch.ts), under the existing shapes. At 1:200 the eight scissor arms, drawn thicker than their solids, are drawn as their solids project.
source "${0:A:h}/../env.sh"; cd "$W"
echo '[]' > ops/scissor-lift.json
for s in 50 100 200; do for v in elevation front; do "$B" patch.ts scissor-lift $s $v; done; done
"$B" patch.ts scissor-lift 100 front --thin 0.0037 --min 0.2
for k in a1 a2 a3 a4 b1 b2 b3 b4; do "$B" trace.ts scissor-lift 200 elevation arm-$k --id arm-$k --bone $k --like arm-$k; done
"$B" patch.ts scissor-lift 200 elevation --thin 0.0075 --min 0.2
"$B" printable.ts scissor-lift
