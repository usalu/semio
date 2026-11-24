#!/bin/zsh
# 🚩️ Repair of `tour-guide` 1:50 (drawing only): writes ops/tour-guide.json from the catalog file. Sleeves and forearms drawn as bodies of revolution stood up to 28 mm beside their lofts, the jacket prism 26 mm, the hands lacked thumb and thenar; these parts are drawn by the outline of their solids as prisms on their bones, the skirt of the jacket with its hem, the badge is drawn where its solid hangs (it stood 2 mm off and closed the slit beside the hand), the flag of the plan by its outline. Volume: the flag pole of 24 mm was declared `slender` although it is three smallest features thick; the guard then compares it as a stroke of the width of the widest drawn part (33 mm, the flag sleeve) that runs 13 mm past the ferrule. It is compared as the body it is. make/tour-guide.sh
source "${0:A:h:h}/env.sh"; cd "$W"
a=tour-guide; o="$W/ops/$a.json"; rm -f "$o"
P="@prism=1,fit=2"
arms=("sleeve-l=sleeve-l$P" "sleeve-r=sleeve-r$P" "forearm-l=forearm-l+cuff-l$P" "forearm-r=forearm-r$P" "wrist-r=wrist-r+cuff-r$P")
handl="hand-l=hand-l+hand-l-thenar+hand-l-index+hand-l-middle+hand-l-ring+hand-l-little+hand-l-thumb$P"
handr="hand-r=hand-r+hand-r-thenar+hand-r-index+hand-r-middle+hand-r-ring+hand-r-little+hand-r-thumb$P"
coat=("jacket=jacket$P" "jacket-skirt=jacket-skirt+jacket-hem$P")
./t.sh outline.ts $a 50 elevation "$o" "${arms[@]}" "$handl" "$handr" "${coat[@]}" "badge-edge=badge+badge-clip@keep=1"
./t.sh outline.ts $a 50 front "$o" "${arms[@]}" "$handl" "${coat[@]}" --base "$o"
./t.sh outline.ts $a 50 plan "$o" "${arms[@]}" "$handr" "${coat[@]}" "flag=flag$P" --base "$o"
python3 make/tour-guide.py
