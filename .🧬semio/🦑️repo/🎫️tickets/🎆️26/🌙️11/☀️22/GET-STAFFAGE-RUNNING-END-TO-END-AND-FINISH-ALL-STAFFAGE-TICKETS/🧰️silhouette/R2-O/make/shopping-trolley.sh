#!/bin/zsh
# 🛒️ Writes the repair ops of `shopping-trolley` (ops/shopping-trolley.json) from the catalog file before the repair: parts of the body that the front view and the plan left out or drew elsewhere are drawn from the body solids (outline.ts): hubs and axles of the four castors, the crate tray with its floor frame, lips and posts, the brackets between frame and basket.
source "${0:A:h}/../env.sh"
A=shopping-trolley; O="$W/ops/$A.json"; rm -f "$O"
cd "$S/📦️packages/🟦️typescript" || exit 1
ol() { timeout 900 "$B" "$W/outline.ts" $A "$1" "$2" "$O" --base "$O" "${@:3}" || exit 1 }
castors=()
for c in rear-l rear-r front-l front-r; do castors+=("hub-$c=hub-$c@after=wheel-$c,z=2.41" "castor-axle-$c=castor-axle-$c@after=fork-$c,z=2.56"); done
ol 50 front "${castors[@]}"
ol 50 front "tray=x@rm=1" "tray-frame=tray-frame-4@axis=1,pen=0.13,after=chassis-line,z=2.65,tone=0.3" "tray-lip-rear=tray-lip-rear@axis=1,pen=0.13,after=chassis-line,z=2.65,tone=0.3" "tray-post-rear-l=tray-post-rear-l@after=chassis-line,z=2.66,tone=0.3" "tray-post-rear-r=tray-post-rear-r@after=chassis-line,z=2.66,tone=0.3" "tray-post-front-l=tray-post-front-l@after=chassis-line,z=2.66,tone=0.3" "tray-post-front-r=tray-post-front-r@after=chassis-line,z=2.66,tone=0.3"
ol 50 front "bracket-floor-l=bracket-floor-l@after=chassis-line,z=2.75" "bracket-floor-r=bracket-floor-r@after=chassis-line,z=2.75" "bracket-rim-l=bracket-rim-l@after=chassis-line,z=2.75" "bracket-rim-r=bracket-rim-r@after=chassis-line,z=2.75"
ol 50 plan "bracket-floor-l=bracket-floor-l@after=chassis-line,z=2.75" "bracket-floor-r=bracket-floor-r@after=chassis-line,z=2.75" "bracket-rim-l=bracket-rim-l@after=chassis-line,z=2.75" "bracket-rim-r=bracket-rim-r@after=chassis-line,z=2.75"
castors=()
for c in rear-l rear-r front-l front-r; do castors+=("fork-$c=castor-fork-$c-o+castor-fork-$c-i" "hub-$c=hub-$c@after=wheel-$c,z=2.41"); done
ol 100 front "${castors[@]}"
ol 100 front "tray=x@rm=1" "tray-frame=tray-frame-4@axis=1,pen=0.13,after=chassis-line,z=2.65,tone=0.3" "tray-lip-rear=tray-lip-rear@axis=1,pen=0.13,after=chassis-line,z=2.65,tone=0.3" "tray-post-rear-l=tray-post-rear-l@after=chassis-line,z=2.66,tone=0.3" "tray-post-rear-r=tray-post-rear-r@after=chassis-line,z=2.66,tone=0.3" "tray-post-front-l=tray-post-front-l@after=chassis-line,z=2.66,tone=0.3" "tray-post-front-r=tray-post-front-r@after=chassis-line,z=2.66,tone=0.3"
python3 "$W/texts.py" shopping-trolley '[
["50","silhouette","struts, tray lip, all four castors (the rear pair outside the front pair),","struts, the basket brackets, the crate tray with its floor frame, lip and posts, all four castors with hubs and axles (the rear pair outside the front pair),","Stützen, Ablagekante, alle vier Rollen (das hintere Paar außerhalb des vorderen),","Stützen, die Korblaschen, die Kistenablage mit Bodenrahmen, Kante und Pfosten, alle vier Rollen mit Naben und Achsen (das hintere Paar außerhalb des vorderen),"],
["50","silhouette","the rail trapezoid converging to the front,","the rail trapezoid converging to the front, the basket brackets,","das nach vorn zusammenlaufende Holmtrapez,","das nach vorn zusammenlaufende Holmtrapez, die Korblaschen,"],
["100","silhouette","struts, tray lip, four castors,","struts, the crate tray with its floor frame, lip and posts, four castors with their fork plates and hubs,","Stützen, Ablagekante, vier Rollen,","Stützen, die Kistenablage mit Bodenrahmen, Kante und Pfosten, vier Rollen mit Gabelblechen und Naben,"]
]'
