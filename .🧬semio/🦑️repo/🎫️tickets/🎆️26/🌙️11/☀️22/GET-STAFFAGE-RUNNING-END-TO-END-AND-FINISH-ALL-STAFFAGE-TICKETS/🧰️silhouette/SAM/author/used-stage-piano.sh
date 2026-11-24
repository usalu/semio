#!/bin/zsh
# 🎹️ Repair ops of `used-stage-piano` (plan of 1:50 and 1:100, 45.0 and 37.5 mm): the open gig bag has corners rounded at 40 mm and a U-shaped carrying grip with a 16 × 140 mm hand opening in the volume; the plan drew both halves with square corners (the swept end profile) and the grip as a full rectangle. Case half, lid half and grip are drawn as their solids project from above.
source "${0:A:h}/../env.sh"; cd "$W"
echo '[]' > ops/used-stage-piano.json
for s in 50 100; do
  "$B" trace.ts used-stage-piano $s plan case-walls,case-panel --id case-end --bone case --like case-end
  "$B" trace.ts used-stage-piano $s plan lid-walls,lid-panel --id lid-end --bone case-lid --like lid-end
  "$B" trace.ts used-stage-piano $s plan grip,grip-tab-1,grip-tab-2 --id grip --bone case --like grip
done
