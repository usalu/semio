#!/bin/zsh
# 🔬️ The search of `rowan` on copies, inside the grant of the worker. 1:100 and 1:200: `rowan-base.ts` writes `$GEN/base/rowan-flat.json`; a copy without the fruit dots (`../MV-L1/loft.ts` converts a bone only when it draws exactly one closed body outline in the view) goes through `../MV-L1/loft.ts --scales 100,200 --views front,plan --save loft/rowan.json` (the side elevation keeps its authored outline: as a loft it fails the silhouette guard at 1:200, 252 mm); `lofts.ts` applies the saved lofts to the flat base as `$GEN/base/rowan.json`, but not the plan of the top mass (SKIP): its four lobes leave slits of 0.3 mm on paper that the loft shows as specks in `gale`, so its plan keeps the authored outline as a sheet `-mvs` with a seed plate, for a body. `trial.sh rowan` then replaces the depth plates of 1:50 and the two seeds by rounded bodies (`../MV-3/body.ts`, `caps.ts`; the lofts fail the guard there, plan 67 mm) and writes `$GEN/try/rowan.json` with the gate. Then the reach of the lofts against their ellipsoids (`short.ts`) and the silhouette guard (`../MR/measure.ts`). Nothing in the catalog is written. rowan-search.sh
source "${0:A:h}/env.sh"
SKIP="100:plan:fol-top,200:plan:fol-top"
L="$U/🧰️matrix/MV-L1"; mkdir -p "$GEN/try" "$GEN/base" "$W/loft"; cd "$T"
timeout 600 "$B" "$W/rowan-base.ts" "$GEN/backup/rowan.json" --out "$GEN/base/rowan-flat.json"
python3 - "$GEN/base/rowan-flat.json" "$GEN/base/rowan-nofruit.json" <<'P'
import json, re, sys
doc = json.load(open(sys.argv[1]))
for key in ("100", "200"):
    geometry = doc["asset"]["representations"][key]["geometry"]
    for view in ("front", "plan"):
        geometry[view] = [shape for shape in geometry[view] if not re.search(r"-fruit\d+$", shape["id"])]
json.dump(doc, open(sys.argv[2], "w"), ensure_ascii=False, indent=2)
P
echo "loft tool sha-256 $(shasum -a 256 < "$L/loft.ts" | cut -c1-16)"
timeout 5400 "$B" "$L/loft.ts" "$GEN/base/rowan-nofruit.json" --scales 100,200 --views front,plan --save "$W/loft/rowan.json" --out "$GEN/try/rowan-nofruit-loft.json" "$@" > "$GEN/log/rowan-loft.log" 2>&1; echo "loft exit $?"
grep -E '^1:|ISSUE|LOST|HOLES|skip|left|holes' "$GEN/log/rowan-loft.log" | cut -c1-300
timeout 600 "$B" "$W/lofts.ts" "$GEN/base/rowan-flat.json" --ops "$W/loft/rowan.json" --skip "$SKIP" --out "$GEN/base/rowan.json"
python3 - "$GEN/base/rowan.json" "$SKIP" <<'P'
import json, sys
doc = json.load(open(sys.argv[1]))
for item in sys.argv[2].split(","):
    key, view, bone = item.split(":")
    shapes = doc["asset"]["representations"][key]["geometry"][view]
    at = next(i for i, shape in enumerate(shapes) if shape["bone"] == bone and shape.get("role") == "body" and shape.get("filled") and shape["kind"] != "loft")
    sheet = shapes[at]
    sheet["id"] += "-mvs"
    shapes.insert(at, {"id": f"mvc-{bone}-seed", "bone": bone, "role": sheet["role"], "z": sheet["z"], "filled": True, "kind": "poly", "points": [[0, 0], [0.001, 0], [0, 0.001]], "closed": True, "solid": {"kind": "extruded", "sweep": "u", "z0": 0, "z1": 0.000004}})
json.dump(doc, open(sys.argv[1], "w"), ensure_ascii=False, indent=2)
P
zsh "$W/trial.sh" rowan --paper 0.25
cd "$T"; timeout 1200 "$B" "$W/short.ts" "$GEN/try/rowan.json" --scales 100,200 | tail -8 | cut -c1-400
cd "$S/📦️packages/🟦️typescript"; timeout 3000 "$B" "$M/measure.ts" "$GEN/backup/rowan.json" "$GEN/try/rowan.json" | grep -E "bbox|silhouette:|validate:|sweep:" | cut -c1-260
cd "$T"; timeout 600 "$B" "$U/🧰️matrix/MC/prints.ts" "$GEN/try/rowan.json" | tail -3 | cut -c1-300
