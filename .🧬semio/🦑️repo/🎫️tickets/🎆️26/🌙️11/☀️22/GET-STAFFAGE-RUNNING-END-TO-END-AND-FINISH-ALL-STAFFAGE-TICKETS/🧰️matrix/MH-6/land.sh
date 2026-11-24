#!/bin/zsh
# 🛬️ Writes the repair of an asset into its catalog file: applies `ops/<id>.json` with the batch's fit.ts until a run changes nothing (at most five runs; the tool refuses when the file changed on disk meanwhile): land.sh <id>…
export BATCH=MH-6; source "${0:A:h}/../MR/env.sh"
for a in "$@"; do
  for i in 1 2 3 4 5; do
    out="$("$B" "$W/fit.ts" "$a" --ops "$W/ops/$a.json" --write | tail -2)"; echo "$a run $i: ${out//$'\n'/ | }" | cut -c1-200
    [[ "$out" == *" 0 changed something"* ]] && break
  done
done
