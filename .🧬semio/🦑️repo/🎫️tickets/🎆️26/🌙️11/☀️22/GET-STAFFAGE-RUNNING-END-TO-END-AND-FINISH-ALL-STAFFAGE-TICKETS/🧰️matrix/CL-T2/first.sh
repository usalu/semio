#!/bin/zsh
# 🥇️ The first chain of lane CL-T2 inside one grant: the solids of the structure scenes of the gas boiler's trial copy, the reduced cases (`reduce.ts`), then light proof, write and proof of the assets given. first.sh <id>…
source "${0:A:h}/env.sh"
(cd "$S" && "$B" "$W/geo.ts" --solids --scene structure "$GEN/try/gas-boiler.json") 2>&1 | grep -v "s50-\|s100-" | cut -c1-200
zsh "$W/reduce.sh" 2>&1 | cut -c1-600
zsh "$W/chain.sh" "$@"
