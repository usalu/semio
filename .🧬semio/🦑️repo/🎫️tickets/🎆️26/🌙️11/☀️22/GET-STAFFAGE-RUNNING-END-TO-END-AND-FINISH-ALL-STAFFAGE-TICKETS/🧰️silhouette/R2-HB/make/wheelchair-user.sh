#!/bin/zsh
# ♿️ Repair of `wheelchair-user`: writes ops/wheelchair-user.json from the catalog file: the hand-written part (`make/wheelchair-user.py`: tabs of the hand rims, drawstrings) and the outlines drawn from the solids. make/wheelchair-user.sh
source "${0:A:h:h}/env.sh"; cd "$W"
a=wheelchair-user; o="$W/ops/$a.outline.json"; rm -f "$o"
python3 make/$a.py
P="@prism=1,fit=2"
# 1:50 side elevation: the hoodie stood 21 mm beside its loft at the neck.
./t.sh outline.ts $a 50 elevation "$o" "hoodie=hoodie$P" --base "$W/ops/$a.hand.json"
# 1:200 side elevation: the axle plates (72 x 150 mm) and the seat sling were not drawn; the side tubes, bent lofts compared as strokes, are drawn along the centre line the guard compares them by (it rounds the bends), so the pocket between thigh, shank and tube is the same pocket in drawing and body (it was 439 against 461 cells at a limit of 455 for a pocket that counts as closed: 112.5 mm).
./t.sh outline.ts $a 200 elevation "$o" "axle-plate-l=axle-plate-l@like=castor-fork-l,after=castor-fork-l,tone=0.5" "axle-plate-r=axle-plate-r@like=castor-fork-r,after=castor-fork-r,tone=0.5" "seat-sling=seat-sling@like=seat-cushion,after=seat-cushion,tone=0.3,material=foam" --base "$o"
./t.sh centre.ts $a 200 elevation "$o" side-tube-l side-tube-r --base "$o"
python3 join.py $a hand outline
