#!/bin/zsh
# 🪑️ Repair of `seated-office-worker` (drawing only): writes ops/seated-office-worker.json from the catalog file. The castors and arms of the chair base that stand turned to the view (72 and 144 degrees) were drawn in the side elevation as profiles of the bone plane and landed up to 70 mm beside their solids; at every scale each part is drawn by the outline of its solid (the bones of the base never move, so a flat outline holds in every pose), and the summary outline batch SH-2 had laid over one castor is removed. make/seated-office-worker.sh
source "${0:A:h:h}/env.sh"; cd "$W"
a=seated-office-worker; o="$W/ops/$a.json"; rm -f "$o"
base() {
  local d=$1 specs=(); shift
  for c in back-a back-b side-a side-b; do
    for p in "$@"; do
      case $p in
        arm+socket) specs+=("chair-arm-$c=chair-arm-$c+chair-socket-$c");;
        stem!) specs+=("chair-stem-$c=chair-stem-$c@like=chair-hood-$c,after=chair-hood-$c,tone=0.62,material=steel-painted,region=chair-steel");;
        */*) specs+=("chair-${p%/*}-$c-${p#*/}=chair-${p%/*}-$c-${p#*/}");;
        *) specs+=("chair-$p-$c=chair-$p-$c");;
      esac
    done
  done
  [[ $d != 500 ]] && specs+=("chair-castor-side-a-sil=@rm=1")
  ./t.sh outline.ts $a $d elevation "$o" "${specs[@]}" --base "$o"
}
base 50 arm+socket wheel/a tyre/a wheel/b tyre/b hood stem!
base 100 arm wheel tyre hood
base 200 arm wheel tyre stem
base 500 arm wheel
# The hands of the 1:100 side elevation drew the fingers 15 mm thinner than the body (26.5 mm, 53 mm once the raster moved with the chair base): the profile of each hand sheet is the outline of hand, fingers and thumb.
for h in l r; do ./t.sh outline.ts $a 100 elevation "$o" "hand-$h=hand-$h+fingers-proximal-$h+fingers-middle-$h+fingers-distal-$h+thumb-$h+thumb-tip-$h@keep=1" --base "$o"; done
