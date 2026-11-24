#!/bin/zsh
# 📦️ Repair of `worker-carrying-box` 1:50: writes ops/worker-carrying-box.json from the catalog file: the load of the carton (`make/worker-carrying-box.py`, volume) and the outlines drawn from the solids: the boots of the front view (drawn 17 mm narrower than shaft and sole) and the upper arms of the plan (bodies of revolution 19 mm beside the sleeves) are prisms on their bones with the outline of their solids. make/worker-carrying-box.sh
source "${0:A:h:h}/env.sh"; cd "$W"
a=worker-carrying-box; o="$W/ops/$a.outline.json"; rm -f "$o"
python3 make/$a.py
P="@prism=1,fit=2"
./t.sh outline.ts $a 50 front "$o" "shoe-r=boot-r+boot-shaft-r+sole-r$P" "shoe-l=boot-l+boot-shaft-l+sole-l$P" "shoe-toe-r=toe-cap-r+toe-sole-r$P" "shoe-toe-l=toe-cap-l+toe-sole-l$P" --base "$W/ops/$a.hand.json"
./t.sh outline.ts $a 50 plan "$o" "upper-arm-r=sleeve-r$P" "upper-arm-l=sleeve-l$P" --base "$o"
python3 join.py $a hand outline
