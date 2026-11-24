#!/bin/zsh
# 🛵️ Writes the repair ops of `motor-scooter` (ops/motor-scooter.json): outlines from the body solids with outline.ts, then the hand-made shapes of make/motor-scooter.py.
source "${0:A:h}/../env.sh"
O="$W/ops/motor-scooter.json"; rm -f "$O"
cd "$S/📦️packages/🟦️typescript" || exit 1
ol() { timeout 900 "$B" "$W/outline.ts" motor-scooter "$1" "$2" "$O" --base "$O" "${@:3}" || exit 1 }
ol 100 elevation "rim-r=rim-r+spokes-r@keep=1,tol=1" "rim-f=rim-f+spokes-f@keep=1,tol=1"
ol 50 plan "footrest-l=footrest-l@after=floor,z=9.2,region=body-panels" "footrest-r=footrest-r@after=floor,z=9.3,region=body-panels"
ol 100 elevation "rail-post=rail-post-r@after=grip-l,z=0.5,region=body-panels"
python3 "$W/make/motor-scooter.py" || exit 1
ol 500 front "bar=headset+headlamp+grip-l+grip-r+mirror-l+mirror-r@world=1,tol=6"
python3 "$W/plate.py" motor-scooter 500 bar 1 -65 0.0324 0.0344
python3 - <<'P' | python3 "$W/hand.py" motor-scooter
import json, os, sys
sys.path.insert(0, os.environ["W"])
from lib import asset, shape
old = shape(asset("motor-scooter"), 500, "front", "bar")
ops = json.load(open(os.environ["W"] + "/ops/motor-scooter.json"))
new = next(o for o in ops if o["op"] == "shape-add" and o["scale"] == "500" and o["view"] == "front" and o["shape"]["id"] == "bar")
new["shape"] = {**{k: v for k, v in old.items() if k not in ("points", "solid")}, "points": new["shape"]["points"], "solid": new["shape"]["solid"]}
print(json.dumps([new]))
P
