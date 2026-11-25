#!/bin/zsh
# 🔬️ The search run of `rowan` on copies, for a job inside a grant (`steps.sh`): `base-rowan-200.ts` writes `$GEN/try/rowan-base.json`; a copy without the fruit dots of the 1:200 front and plan (`$GEN/try/rowan-nofruit.json`: `../MV-L1/loft.ts` converts a bone only when it draws exactly one closed body outline in the view, and four masses carry fruit dots there) goes through `../MV-L1/loft.ts --save ops/rowan.json`; the saved lofts (absolute values) are then applied to the full base with `--ops`, giving `$GEN/try/rowan.json`, twice for the byte comparison; then the gate over every pose, clip instant and transition. Nothing in the catalog is written. search-rowan.sh
source "${0:A:h}/steps.sh"
L="$U/🧰️matrix/MV-L1"; mkdir -p "$GEN/try" "$W/ops"
tool rowan-base base-rowan-200.ts --out "$GEN/try/rowan-base.json"
python3 - "$GEN/try/rowan-base.json" "$GEN/try/rowan-nofruit.json" <<'P'
import json, re, sys
doc = json.load(open(sys.argv[1]))
geometry = doc["asset"]["representations"]["200"]["geometry"]
for view in ("front", "plan"):
    geometry[view] = [shape for shape in geometry[view] if not re.search(r"-fruit\d+$", shape["id"])]
json.dump(doc, open(sys.argv[2], "w"), ensure_ascii=False, indent=2)
P
TOOL_TIMEOUT=5400 tool rowan-loft "$L/loft.ts" "$GEN/try/rowan-nofruit.json" --save "$W/ops/rowan.json" --out "$GEN/try/rowan-nofruit-loft.json"
grep -E "one evaluation|HOLES|ISSUE|tolerance" "$GEN/log/rowan-loft.log" | cut -c1-300
tool rowan-apply "$L/loft.ts" "$GEN/try/rowan-base.json" --ops "$W/ops/rowan.json" --out "$GEN/try/rowan.json"
tool rowan-apply-again "$L/loft.ts" "$GEN/try/rowan.json" --ops "$W/ops/rowan.json" --out "$GEN/try/rowan-2.json"
[[ -f "$GEN/try/rowan-2.json" ]] && { cmp "$GEN/try/rowan.json" "$GEN/try/rowan-2.json" && echo "idempotent"; rm -f "$GEN/try/rowan-2.json"; } || echo "second run wrote nothing (unchanged)"
gate "$GEN/try/rowan.json"
tail -14 "$GEN/log/rowan-gate.log" | cut -c1-220
