#!/bin/zsh
# 7️⃣ Seventh pass of the batch: the greedy repair (plates on every bone, hands, pairs of limbs; print suitability guarded) on the views that are still red.
source "${0:A:h}/env.sh"
export ANYBONE=1
while read -r a scales; do "$W/fix.sh" "$a" ${=scales}; done <<'LIST'
person-signing 100
person-stretching 500
person-carrying-shopping-bags 50
person-cooking 50
person-eating-standing 50
person-pushing-bicycle 50
painter-at-easel 50
cyclist 50
LIST
