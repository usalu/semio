#!/bin/zsh
# 5️⃣ Fifth pass of the batch: the repaired shapes that broke the print suitability of the check CLI are taken back (takeback.sh), then the greedy repair, which now also refuses such shapes, runs again where a view is red.
source "${0:A:h}/env.sh"
"$W/takeback.sh" barista 50
"$W/takeback.sh" child-on-kick-scooter 50
"$W/takeback.sh" cyclist 500
export ANYBONE=1
while read -r a scales; do "$W/fix.sh" "$a" ${=scales}; done <<'LIST'
barista 50
child-on-kick-scooter 50
cyclist 500 50
person-stretching 500
LIST
