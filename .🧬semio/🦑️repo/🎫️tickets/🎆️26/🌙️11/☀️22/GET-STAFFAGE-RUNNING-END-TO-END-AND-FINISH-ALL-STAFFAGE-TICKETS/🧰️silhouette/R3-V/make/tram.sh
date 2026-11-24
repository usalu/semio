#!/bin/zsh
# 🚋️ Writes the repair ops of `tram` (ops/tram.json): the hand-made shapes of make/tram.py, then outlines from the body solids with outline.ts.
source "${0:A:h}/../env.sh"
O="$W/ops/tram.json"; rm -f "$O"
python3 "$W/make/tram.py" || exit 1
cd "$S/📦️packages/🟦️typescript" || exit 1
ol() { timeout 900 "$B" "$W/outline.ts" tram "$1" "$2" "$O" --base "$O" "${@:3}" || exit 1 }
ol 50 front "articulation-arm=articulation-arm+articulation-bearing"
ol 100 front "articulation-arm=articulation-arm+articulation-bearing"
ol 50 elevation "pan-head=pan-apex-l+pan-apex-r+pan-strip-1+pan-carrier-1+pan-strip-2+pan-carrier-2+pan-head-shaft" "pan-horns=pan-horn-1-l+pan-horn-1-r+pan-horn-2-l+pan-horn-2-r"
ol 50 elevation "pan-base=$(python3 "$W/ids.py" tram 50 body-end | tr '+' '\n' | grep '^pan-' | paste -sd+ -)"
ol 100 elevation "pan-base=$(python3 "$W/ids.py" tram 100 body-end | tr '+' '\n' | grep '^pan-' | paste -sd+ -)"
ol 50 elevation "hvac-box=hvac-box@tol=1" "wiper=wiper-blade+wiper-arm"
ol 100 elevation "hvac-box=hvac-box@tol=1"
ol 50 elevation "articulation-arm=articulation-arm+articulation-bearing"
ol 100 elevation "pan-head=pan-apex+pan-strip-1+pan-strip-2+pan-head-shaft" "pan-horns=pan-horn-1-l+pan-horn-1-r+pan-horn-2-l+pan-horn-2-r"
ol 50 front "leaf-a1=leaf-a1+leaf-seal-a1+leaf-button-a1@after=shell-end,bone=door-a1,role=body,z=3,tone=0.6,region=doors,material=paint"
ol 50 plan "headlamp-l=headlamp-l" "headlamp-r=headlamp-r"
