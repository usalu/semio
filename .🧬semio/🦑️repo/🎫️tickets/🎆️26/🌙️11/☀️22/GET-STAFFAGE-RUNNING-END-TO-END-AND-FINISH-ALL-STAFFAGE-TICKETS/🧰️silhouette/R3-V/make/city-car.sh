#!/bin/zsh
# 🚗️ Writes the repair ops of `city-car` (ops/city-car.json): the 1:50 side elevation at the rear edge of the bonnet. The drawn bonnet lay 11 mm ahead of its solid and the drawn body shell cut the notch between cowl and bonnet straight; the bonnet is drawn by the outline of its solid and the body shell follows scuttle and cowl into the notch. The outline of the chassis solids that batch SAV put below the side elevation (`chassis-sil`) had one point of its roof 0.4 mm above the 1.500 m of the body (a point of the simplified outline): it lies at 1.500 m, so the elevation cast of the car is 1.5999 m high again (`🧍️plan-projection`, `side-car` below 1.6 m).
source "${0:A:h}/../env.sh"
O="$W/ops/city-car.json"; rm -f "$O"
cd "$S/📦️packages/🟦️typescript" || exit 1
timeout 600 "$B" "$W/outline.ts" city-car 50 elevation "$O" "bonnet-slab=bonnet-skin" || exit 1
python3 - <<'P' | python3 "$W/hand.py" city-car
import json, os, sys
sys.path.insert(0, os.environ["W"])
from lib import asset, shape
d = shape(asset("city-car"), 50, "elevation", "body-shell")["d"]
old, new = "L 0.94,0.972 L 0.985,1.005", "L 0.94,0.972 L 0.934,0.988 L 0.965,0.99 L 0.976,0.995 L 0.985,1.005"
assert old in d or new in d
sil = shape(asset("city-car"), 50, "elevation", "chassis-sil")["d"]
assert "-0.70880 1.50040" in sil or "-0.70880 1.50000" in sil
print(json.dumps([{"op": "shape-set", "scale": "50", "view": "elevation", "id": "body-shell", "set": {"d": d.replace(old, new)}}, {"op": "shape-set", "scale": "50", "view": "elevation", "id": "chassis-sil", "set": {"d": sil.replace("-0.70880 1.50040", "-0.70880 1.50000")}}]))
P
