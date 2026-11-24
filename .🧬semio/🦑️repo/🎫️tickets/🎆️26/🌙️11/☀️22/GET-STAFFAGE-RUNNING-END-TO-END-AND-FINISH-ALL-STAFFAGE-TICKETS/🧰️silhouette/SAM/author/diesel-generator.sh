#!/bin/zsh
# ⚡️ Repair ops of `diesel-generator`, in order: dimensions, corner castings and base frame (diesel-generator.py), then three parts drawn as their solids project at 1:50: the silencer inlet in the side elevation (15.0 mm, at the limit), the rain cap with its skirt in the front elevation (20.6 mm) and, in plan and front, the hinge knuckles of the outlet flap, which were not drawn (16.9 mm).
source "${0:A:h}/../env.sh"; cd "$W/author" && python3 diesel-generator.py && cd "$W"
"$B" trace.ts diesel-generator 50 elevation silencer-inlet,silencer-inlet-collar --id silencer-inlet --bone silencer --like silencer-inlet
"$B" trace.ts diesel-generator 50 front rain-cap,rain-cap-skirt --id rain-cap --bone cap-flap --like rain-cap
for v in plan front; do "$B" trace.ts diesel-generator 50 $v outlet-flap-knuckle --id outlet-flap-knuckle --bone outlet-flap --like outlet-flap; done
# 1:200 side elevation (67.5 mm): the drawn discharge hood ended 70 mm short of the container end under the flap; it is drawn as the hood solids project.
"$B" trace.ts diesel-generator 200 elevation "hood-*" --id discharge-hood --bone outlet-hood --like discharge-hood
