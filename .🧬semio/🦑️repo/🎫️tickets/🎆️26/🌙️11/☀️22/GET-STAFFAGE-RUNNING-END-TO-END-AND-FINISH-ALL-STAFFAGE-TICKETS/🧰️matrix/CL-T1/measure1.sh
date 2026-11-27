#!/bin/zsh
# 🔬️ An experiment on trial copies in a folder of `$GEN`: per asset the triangle budgets (`groups.ts`), the GLB of the default pose (`glb.ts`) and the silhouette guard with `validateAsset` and the three-views sweep (`../MR/measure.ts`); the lines that matter go to `$GEN/<folder>/<id>.txt`. measure1.sh <folder> <id>…
source "${0:A:h}/env.sh"
k="$1"; shift
for a in "$@"; do f="$GEN/$k/$a.json"; o="$GEN/$k/$a.txt"; : > "$o"
(zsh "$W/run.sh" groups.ts "$f" --top 10) > "$GEN/$k/$a-budget.log" 2>&1; grep -E "^$a (body|thermal|structure):|^  !|^BUDGET" "$GEN/$k/$a-budget.log" | cut -c1-200 >> "$o"
(zsh "$W/run.sh" glb.ts "$f") > "$GEN/$k/$a-glb.log" 2>&1; grep -A12 '^glb ' "$GEN/$k/$a-glb.log" | cut -c1-200 >> "$o"
(cd "$S/📦️packages/🟦️typescript" && timeout 2400 "$B" "$M/measure.ts" "$f") > "$GEN/$k/$a-measure.log" 2>&1; grep -E "silhouette:|validate:|sweep:|bbox" "$GEN/$k/$a-measure.log" | cut -c1-300 >> "$o"
done
