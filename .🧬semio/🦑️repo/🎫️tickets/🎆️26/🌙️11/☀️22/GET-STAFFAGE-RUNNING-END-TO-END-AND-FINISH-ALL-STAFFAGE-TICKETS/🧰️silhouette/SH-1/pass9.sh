#!/bin/zsh
# 9️⃣ Ninth pass of the batch: the single shapes the first pass replaced by the outline of their whole bone (grown.ts names them: weed leaves over the trug, a tyre, a head, a wheel, a finger) are drawn again as authored, then the greedy repair, which no longer makes that replacement, repairs those views, and the new outlines take the look of what they replace (restyle.ts).
source "${0:A:h}/env.sh"
export ANYBONE=1
cd "$W"
python3 shapedrop.py child-on-kick-scooter 50 plan tyre-rear
python3 shapedrop.py child-on-kick-scooter 200 elevation head
python3 shapedrop.py kneeling-gardener 50 front weed-leaf-1 weed-leaf-2 weed-leaf-6 weed-leaf-10
python3 shapedrop.py parent-with-stroller 500 front wheel-rear-l
python3 shapedrop.py person-checking-watch 50 plan finger-l-3b
"$W/keep.sh" child-on-kick-scooter kneeling-gardener parent-with-stroller person-checking-watch
"$W/fix.sh" child-on-kick-scooter 50 200
"$W/fix.sh" kneeling-gardener 50
"$W/fix.sh" parent-with-stroller 500
"$W/fix.sh" person-checking-watch 50
"$B" restyle.ts "$GEN/backup" child-on-kick-scooter kneeling-gardener parent-with-stroller person-checking-watch
"$W/keep.sh" child-on-kick-scooter kneeling-gardener parent-with-stroller person-checking-watch
"$B" measure.ts child-on-kick-scooter kneeling-gardener parent-with-stroller person-checking-watch | grep "!\|OK\|failing"
"$B" grown.ts "$GEN/backup" child-on-kick-scooter kneeling-gardener parent-with-stroller person-checking-watch
echo done
