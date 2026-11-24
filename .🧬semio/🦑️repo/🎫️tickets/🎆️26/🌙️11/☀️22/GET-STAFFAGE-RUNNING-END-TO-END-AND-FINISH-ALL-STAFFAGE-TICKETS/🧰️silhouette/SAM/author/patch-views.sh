#!/bin/zsh
# 🩹️ The common recipe of the machines whose volume has parts that no view drew: after the ops that `ops/<id>.json` already holds (declared dimensions), every authored scale and view gets the missing parts as patch.ts finds them: patch-views.sh <id> [<scales, default "50 100 200 500">]   (PATCH_FLAGS="--grow 0.001" passes flags on)
source "${0:A:h}/../env.sh"; cd "$W"
id="$1"; scales="${2:-50 100 200 500}"
for s in ${=scales}; do for v in elevation front plan; do "$B" patch.ts "$id" $s $v ${=PATCH_FLAGS} | tail -1; done; done
