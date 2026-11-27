#!/bin/zsh
# ⚖️ The two budgets of trial copies in `$GEN/try`, one asset after the other inside the grant of its caller: the triangles per scene as `🧊️model-catalog` counts them (`groups.ts`) and the GLB of the default pose (`glb.ts`); the summary lines go to `$GEN/size/<id>.txt`. size.sh <id>…
source "${0:A:h}/env.sh"
mkdir -p "$GEN/size"
for a in "$@"; do f="$GEN/try/$a.json"; o="$GEN/size/$a.txt"
(zsh "$W/run.sh" groups.ts "$f" --top 8) > "$GEN/size/$a-budget.log" 2>&1; grep -E "^$a (body|thermal|structure):|^  !|^BUDGET" "$GEN/size/$a-budget.log" | cut -c1-200 > "$o"
(zsh "$W/run.sh" glb.ts "$f") > "$GEN/size/$a-glb.log" 2>&1; head -3 "$GEN/size/$a-glb.log" | cut -c1-220 >> "$o"
echo "$a $(grep -m1 BUDGET "$o") $(grep -m1 -o '^glb [0-9]* bytes ([0-9.]* MiB' "$o"))"
done
