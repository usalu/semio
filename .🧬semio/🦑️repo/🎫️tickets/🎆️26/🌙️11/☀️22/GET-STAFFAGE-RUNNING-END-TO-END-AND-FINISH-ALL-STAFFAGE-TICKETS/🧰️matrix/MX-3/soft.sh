#!/bin/zsh
# 🧪️ The suite 🧊️model-bounds with recording assertions (`../ML-4/soft.ts`) in one queue grant: soft.sh <label> <MB> [<case filter regexp>]
source "${0:A:h}/env.sh"; label="$1"; mb="$2"; filter="${3:-.}"; mkdir -p "$GEN"
"${0:A:h}/q.sh" "$mb" "source ${0:A:h}/env.sh; cd $P && /usr/bin/time -l timeout 3000 $B $L/soft.ts 🧊️model-bounds $GEN/soft-$label '$filter' > $GEN/soft-$label.log 2>&1"
