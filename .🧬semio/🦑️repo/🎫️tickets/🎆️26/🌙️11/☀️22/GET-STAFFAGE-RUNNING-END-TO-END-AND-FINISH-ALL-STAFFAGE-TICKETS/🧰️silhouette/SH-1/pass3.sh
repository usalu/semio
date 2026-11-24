#!/bin/zsh
# 3️⃣ Third pass of the batch: the greedy repair with plates on every bone and on whole hands (ANYBONE=1) where the second pass ran out of time or of single-bone steps.
source "${0:A:h}/env.sh"
export ANYBONE=1
while read -r a scales; do "$W/fix.sh" "$a" ${=scales}; done <<'LIST'
courier-with-hand-truck 50
person-carrying-shopping-bags 50
person-cooking 50
person-eating-standing 50
painter-at-easel 50
leaning-person 50
person-pushing-bicycle 50
person-signing 50
cyclist 50 500
dancer 50 500
LIST
