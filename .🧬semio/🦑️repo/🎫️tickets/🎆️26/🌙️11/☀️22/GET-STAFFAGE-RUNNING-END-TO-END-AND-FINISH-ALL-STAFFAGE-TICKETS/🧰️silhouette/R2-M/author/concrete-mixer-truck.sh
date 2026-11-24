#!/bin/zsh
# 🚛️ Repair ops of `concrete-mixer-truck` of the second round (after the ops of SAM), authored against the catalog file as it was before this round: in every view of 1:50, 1:100 and 1:200 the shapes on the rigid bones (chassis, drum, axles) that were fuller than their solids are redrawn as the solids project and missing solids are drawn (retrace.py, three passes, because a redrawn shape can uncover a part it used to hide), then pieces thinner than half a feature are added (patch.ts), and at 1:200 the air filter overlaps its neighbour by 4 mm in plan (a seam of one cell). Ops of `ops/concrete-mixer-truck.json` are the result; a second run on the repaired file finds nothing to redraw.
source "${0:A:h}/../env.sh"; cd "$W"
FB="chassis,drum,axle-1,axle-2,axle-3,axle-4"
for s in ${=1:-200 100 50}; do
  f=$((s * 0.00015))
  for v in elevation front plan; do
    for pass in 1 2 3; do python3 retrace.py concrete-mixer-truck $s $v $FB | tail -1; done
    "$B" patch.ts concrete-mixer-truck $s $v --thin $((f / 5 * 4)) --min 0.1 --grow $((f / 10)) | tail -1
  done
done
"$B" trace.ts concrete-mixer-truck 200 plan "air-filter,air-intake-elbow" --id air-filter --bone chassis --like air-filter --replace air-filter --grow 0.004 | tail -1
"$B" drawable.ts concrete-mixer-truck --ops ops/concrete-mixer-truck.json
# By hand after the recipe: mirrors with their heads in the front elevation (the drawn arms stood for arm and head), the pedestals, the spare chute and the hopper with 2 to 4 mm of overlap where seams of one cell stayed.
t() { "$B" trace.ts concrete-mixer-truck "$@" | tail -1 }
t 100 front "mirror-l,mirror-arm-l" --id mirror-arm-left --bone chassis --like mirror-arm-left --replace mirror-arm-left
t 100 front "mirror-r,mirror-arm-r" --id mirror-arm-right --bone chassis --like mirror-arm-right --replace mirror-arm-right
t 50 front "mirror-head-l,mirror-arm-l" --id mirror-arm-left --bone chassis --like mirror-arm-left --replace mirror-arm-left
t 50 front "mirror-head-r,mirror-arm-r" --id mirror-arm-right --bone chassis --like mirror-arm-right --replace mirror-arm-right
t 100 elevation "front-pedestal-*,rear-pedestal-*" --id pedestal --bone chassis --like pedestal --replace pedestal --grow 0.003 --holes
t 100 elevation "spare-chute-*" --id spare-chute --bone chassis --like spare-chute --replace spare-chute --grow 0.003
t 50 plan "hopper-bowl,hopper-spout,hopper-bracket-*" --id hopper --bone drum --like hopper --replace hopper --grow 0.002
"$B" drawable.ts concrete-mixer-truck --ops ops/concrete-mixer-truck.json
