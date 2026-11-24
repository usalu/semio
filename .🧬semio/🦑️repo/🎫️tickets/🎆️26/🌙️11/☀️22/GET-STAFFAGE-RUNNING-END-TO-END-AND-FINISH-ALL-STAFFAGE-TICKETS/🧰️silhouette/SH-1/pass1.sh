#!/bin/zsh
# 1️⃣ First pass of the batch: the greedy repair of every person at the scales that failed in the baseline, one process after the other.
source "${0:A:h}/env.sh"
while read -r a scales; do "$W/fix.sh" "$a" ${=scales}; done <<'LIST'
barista 50 100
basketball-player 50
child-on-kick-scooter 50 200
construction-worker-with-shovel 50
courier-with-hand-truck 50 200 500
cyclist 50 100 500
dancer 50 100 200 500
elderly-with-cane 50
jogger 50 100
kneeling-gardener 50
leaning-person 50 100 200
painter-at-easel 50
parent-carrying-toddler 50
parent-with-stroller 50 100 200 500
person-carrying-ladder 50 200 500
person-carrying-shopping-bags 50
person-checking-watch 50
person-cooking 50
person-drinking-coffee 50
person-eating-standing 50
person-hanging-laundry 50 100
person-on-crutches 50
person-pushing-bicycle 50
person-signing 50 100
person-stretching 500
LIST
