#!/bin/zsh
# 🧪️ Trial of the ops of one person on a work copy (the catalog file stays): applies every stage to the generator output and prints the silhouette guard and the validator. try.sh <slug> [scales] [views]
source "${0:A:h}/env.sh"
python3 "$W/apply.py" $1 "$GEN/base/$1.json" "$GEN/work/$1.json" > /dev/null || exit 1
(cd "$W" && $B diag.ts "$GEN/work/$1.json" ${2:-500,200,100,50} ${3:-elevation,front,plan}) | cut -c1-${COLS:-700}
(cd "$T/🧰️scripts/HW-03" && $B validate.ts "$GEN/work/$1.json") | cut -c1-400
