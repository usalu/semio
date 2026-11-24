#!/bin/zsh
# 🧹️ Repair ops of `vacuum-cleaner` (1:50 front 43.1 mm, plan 20.2 mm): the two axle bosses that carry the rear wheels (solids `wheel-boss-l`, `wheel-boss-r`, 40 mm across, 33 mm long) were drawn in no view, so front and plan showed air between shell and wheel. They are drawn in front and plan as they project, under the shell. The front outline of the shell was a coarse polygon that stood 1 mm inside the body beside the wheels (a slit of one raster cell between shell and wheel, 26.3 mm); it is drawn as the solid `shell` projects.
source "${0:A:h}/../env.sh"; cd "$W"
echo '[]' > ops/vacuum-cleaner.json
for v in front plan; do
  "$B" trace.ts vacuum-cleaner 50 $v wheel-boss-l --id wheel-boss-left --bone body --like shell --material abs-grey --tone 0.3 --z 0.9
  "$B" trace.ts vacuum-cleaner 50 $v wheel-boss-r --id wheel-boss-right --bone body --like shell --after wheel-boss-left --material abs-grey --tone 0.3 --z 0.9
done
"$B" trace.ts vacuum-cleaner 50 front shell --id shell --bone body --like shell
