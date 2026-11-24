#!/bin/zsh
# 🛹️ Repair of `skateboarder` 1:50 (drawing only): writes ops/skateboarder.json from the catalog file. The figure stands turned on the board; limbs drawn as bodies of revolution and torso, head and shoes drawn as thick prisms stood 19 to 36 mm beside their lofts in all three views. Every part of the figure is drawn in each view by the outline of its solids as a plate on its bone (`sheet`), neck and nose get shapes of their own, and the stacked slices of hoodie, head and beanie in the plan give way to the one outline. make/skateboarder.sh
source "${0:A:h:h}/env.sh"; cd "$W"
a=skateboarder; o="$W/ops/$a.json"; rm -f "$o"
parts=()
for p in shank-r shank-l forearm-r forearm-l thigh-r thigh-l sleeve-l sleeve-r trousers-seat hoodie; do parts+=("$p=$p@prism=1,fit=2"); done
for s in l r; do parts+=("wrist-$s=wrist-$s+cuff-$s@prism=1,fit=2" "shoe-$s=shoe-$s+sole-$s@prism=1,fit=2" "shoe-toe-$s=shoe-toe-$s+sole-toe-$s@prism=1,fit=2"); done
parts+=("hood=hood+hoodie-collar@prism=1,fit=2" "beanie=beanie+beanie-cuff@prism=1,fit=2")
hands=("hand-r=palm-r+finger-r-0+finger-r-1+finger-r-2+finger-r-3+thumb-r@prism=1,fit=2" "hand-l=palm-l+finger-l-0+finger-l-1+finger-l-2+finger-l-3+thumb-l@prism=1,fit=2")
face=("head=head+jaw+lips+ear-l+ear-r@prism=1,fit=2" "nose=nose@prism=1,fit=2,like=head,after=head" "neck=neck@prism=1,fit=2,like=head,after=head")
./t.sh outline.ts $a 50 elevation "$o" "${parts[@]}" "${face[@]}"
./t.sh outline.ts $a 50 front "$o" "${parts[@]}" "${hands[@]}" "${face[@]}" --base "$o"
gone=(); for x in hoodie-under-{1..5} head-under-{1..4} beanie-under-{1..3} trousers-seat-l trousers-seat-r; do gone+=("$x=@rm=1"); done
# The legs stand almost upright and the plan sees them end-on: there they stay the bodies of revolution they were, which reach as far sideways in the plan as in the front view when a knee lifts (pose `ollie-air`); a plate or a short prism does not.
planparts=(); for p in "${parts[@]}"; do [[ "$p" == thigh-* || "$p" == shank-* ]] || planparts+=("$p"); done
./t.sh outline.ts $a 50 plan "$o" "${planparts[@]}" "${hands[@]}" "head=head+jaw+lips+ear-l+ear-r+nose@prism=1,fit=2" "${gone[@]}" --base "$o"
