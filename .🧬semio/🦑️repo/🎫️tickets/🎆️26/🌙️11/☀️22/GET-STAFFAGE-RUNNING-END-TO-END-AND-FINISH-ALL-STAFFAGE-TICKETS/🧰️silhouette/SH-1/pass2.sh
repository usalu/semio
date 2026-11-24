#!/bin/zsh
# 2️⃣ Second pass of the batch: the greedy repair with traced plates on every bone (ANYBONE=1) at the scales that still failed after the first pass, one process after the other.
source "${0:A:h}/env.sh"
export ANYBONE=1
while read -r a scales; do "$W/fix.sh" "$a" ${=scales}; done <<'LIST'
barista 100
courier-with-hand-truck 50
kneeling-gardener 50
leaning-person 50
painter-at-easel 50
person-carrying-shopping-bags 50
person-checking-watch 50
person-cooking 50
person-drinking-coffee 50
person-eating-standing 50
person-hanging-laundry 50 100
person-on-crutches 50
person-pushing-bicycle 50
person-signing 50 100
person-carrying-ladder 50 200 500
person-stretching 500
cyclist 50 100 500
dancer 50 100 200 500
LIST
