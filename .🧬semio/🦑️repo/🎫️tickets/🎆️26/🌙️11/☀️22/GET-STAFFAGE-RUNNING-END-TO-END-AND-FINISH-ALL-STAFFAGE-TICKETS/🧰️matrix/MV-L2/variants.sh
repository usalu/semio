#!/bin/zsh
# 🎛️ Tries rounding variants of the bodies of one plant on copies, inside the grant of the worker: `variants.sh <id> <scales> "<flags of caps.ts>"…` rounds the raw bodies of the last `trial.sh` run (`$GEN/try/<id>-raw-ops.json`) with each set of flags, applies them to the base of that run and prints the hard findings and the reported group drift of the gate at the given scales, so the roundest body that keeps the extents can be chosen (`flags/<id>.caps`).
source "${0:A:h}/env.sh"
a="$1"; scales="$2"; shift 2; src="$GEN/backup/$a.json"; [[ -f "$GEN/base/$a.json" ]] && src="$GEN/base/$a.json"; cd "$T"
deep="$(python3 -c "import json,sys; d=json.load(open(sys.argv[1]))['asset']['dimensions']; print(round(max(0.015, 0.02*max(d['heightM'], d['widthM'], d['depthM']))*0.5, 4))" "$src")"
for flags in "$@"; do
  timeout 300 "$B" "$W/caps.ts" "$GEN/try/$a-raw-ops.json" "$GEN/try/$a-variant-ops.json" --deep "$deep" ${=flags} > /dev/null
  timeout 3000 "$B" "$U/🧰️matrix/MV-3/body.ts" "$src" --ops "$GEN/try/$a-variant-ops.json" --out "$GEN/try/$a-variant.json" > /dev/null 2>&1
  echo "== $a [$flags]: $(timeout 3000 "$B" "$M/gate.ts" "$GEN/try/$a-variant.json" --scales "$scales" --soft | grep -E "HARD|reported|hard findings" | cut -c1-150 | tr '\n' ';')"
done
