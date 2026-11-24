#!/bin/zsh
# ♨️ Repair ops of `heat-pump-outdoor-unit`.
# - Dimensions: the hinge knuckles of the service lid stand 9 mm behind the 0.470 m casing and the lid skirt overhangs the 1.100 m casing by 7.5 mm on each side (body of 1:50 and 1:100: 0.479 m wide, 1.115 m deep; 1:200 and 1:500: 0.470 and 1.115 m). Declared 0.479 m and 1.115 m instead of 0.47 m and 1.10 m (1:50 missed them by 9 and 15 mm against 7.5 mm).
# - Side elevation of 1:50 (22.5 mm): the two hinge leaves of the lid (solid `lid-hinge-leaves`, 8 × 32.5 mm) close the pocket between hinge bracket and lid in the volume and were not drawn; they are drawn as they project.
source "${0:A:h}/../env.sh"; cd "$W"
echo '[{"op":"dims","set":{"widthM":0.479,"depthM":1.115}}]' > ops/heat-pump-outdoor-unit.json
"$B" trace.ts heat-pump-outdoor-unit 50 elevation lid-hinge-leaves --id lid-hinge-leaves --bone lid --like lid --role detail --tone 0.3 --z 2.6 --tol 0.0003
