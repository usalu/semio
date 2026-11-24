#!/bin/zsh
# 8️⃣ Eighth pass of the batch: what the first pass traced onto another bone than the part's own (stroller parts on the parent's forearm, hands on the arm) is taken out again, then the greedy repair, which no longer climbs to a parent bone and refuses a plate on a bone askew to the view, repairs those views; the three-views sweep of the person follows.
source "${0:A:h}/env.sh"
export ANYBONE=1
cd "$W"
python3 shapedrop.py parent-with-stroller 50 front chassis-cross fittings-l chassis-l chassis-r fittings-r basket hand-r hand-l
python3 shapedrop.py parent-with-stroller 50 plan hand-r hand-l
python3 shapedrop.py parent-with-stroller 100 front chassis-cross fittings-r hand-r
python3 shapedrop.py parent-with-stroller 500 plan seat-fabric
python3 shapedrop.py parent-with-stroller 500 front stroller-sil torso
"$W/keep.sh" parent-with-stroller
"$W/fix.sh" parent-with-stroller 50 100 500
"$W/tv.sh" parent-with-stroller
