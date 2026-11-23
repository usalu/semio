#!/bin/zsh
# 🧊️ The volume repairs in their order for people: thermal areas, connections, declared dimensions, structure solids into the body; then applies the ops to the catalog. fixvol.sh <slug>…   (STEPS="areas bridge dims nudge" limits the steps; ARGS_<step> adds arguments)
source "${0:A:h}/env.sh"
mkdir -p "$GEN/fixvol"
cd "$S/📦️packages/🟦️typescript"
for a in "$@"; do
  for step in ${=STEPS:-areas bridge dims nudge}; do
    extra="ARGS_$step"
    "$B" "$W/$step.ts" "$a" ${=${(P)extra}} >> "$GEN/fixvol/$a.log" 2>&1
    echo "$(tail -1 "$GEN/fixvol/$a.log")"
  done
  "$B" "$W/apply.ts" "$a" | tail -1
done
