#!/bin/zsh
# 🧾️ Every proof of one asset but its matrix, on the catalog file as it is (gate run anew: the trial copy of the asset is removed so that no older gate log is cited), appended to $GEN/proof/<id>.out; the matrix runs beside it in the other worker (mx.sh): pre.sh <id>
export BATCH=MH-6; source "${0:A:h}/../MR/env.sh"; a="$1"
rm -f "$GEN/try/$a.json"
STEPS="gate scan measure check tests" "$W/proof.sh" "$a"
