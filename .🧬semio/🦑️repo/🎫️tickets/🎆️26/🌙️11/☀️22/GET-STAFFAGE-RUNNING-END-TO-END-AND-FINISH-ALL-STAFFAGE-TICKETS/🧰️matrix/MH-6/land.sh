#!/bin/zsh
# 🛬️ Writes the repair of an asset into its catalog file: applies `ops/<id>.json` with the batch's fit.ts until a run changes nothing (at most five runs; the tool refuses when the file changed on disk meanwhile), then, where `ops/<id>.bodies.json` exists, redraws the parts it names from the volume solids with bodies.ts (the last step of the chain; a second run changes nothing), and where `ops/<id>.lofts.json` exists (here, or in the folder of MX-2 for an asset MX-2 drew: `tour-guide`) draws the parts it names as `loft` shapes with the tool of MX-2 (the last step): land.sh <id>…   (OUT=<file> applies the chain to a copy instead: the catalog file is copied to OUT first)
export BATCH=MH-6; source "${0:A:h}/../MR/env.sh"
for a in "$@"; do
  target="$a"
  if [[ -n "$OUT" ]]; then cp "$(ls "$S"/🗂️catalog/*/"$a"/🔣️.json)" "$OUT"; target="$OUT"; fi
  for i in 1 2 3 4 5; do
    out="$("$B" "$W/fit.ts" "$target" --ops "$W/ops/$a.json" --write | tail -2)"; echo "$a run $i: ${out//$'\n'/ | }" | cut -c1-200
    [[ "$out" == *" 0 changed something"* ]] && break
  done
  if [[ -f "$W/ops/$a.bodies.json" ]]; then
    out="$("$B" "$W/bodies.ts" "$target" --ops "$W/ops/$a.bodies.json" --write | tail -2)"; echo "$a bodies: ${out//$'\n'/ | }" | cut -c1-200
  fi
  lofts="$W/ops/$a.lofts.json"; [[ -f "$lofts" ]] || lofts="$U/🧰️matrix/MX-2/ops/$a.lofts.json"
  if [[ -f "$lofts" ]]; then
    out="$("$B" "$U/🧰️matrix/MX-2/loft.ts" "$target" --ops "$lofts" --write | tail -2)"; echo "$a lofts: ${out//$'\n'/ | }" | cut -c1-200
  fi
done
