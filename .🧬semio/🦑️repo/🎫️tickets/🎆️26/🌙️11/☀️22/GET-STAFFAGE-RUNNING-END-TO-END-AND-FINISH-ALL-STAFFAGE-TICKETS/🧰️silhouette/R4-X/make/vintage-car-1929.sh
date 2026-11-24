#!/bin/zsh
# 🚗️ Writes the repair ops of `vintage-car-1929` of batch R4-X (ops/vintage-car-1929.json, applied after the ops of SAV and R3-V): the parts whose drawing differs from the body are drawn as the outline of their solids (outline.ts; every part is on the fixed chassis, on an axle that only travels up and down, or on a wheel that turns in the plane of the side elevation).
source "${0:A:h}/../env.sh"
O="$W/ops/vintage-car-1929.json"; rm -f "$O"
cd "$S/📦️packages/🟦️typescript" || exit 1
ol() { timeout 900 "$B" "$W/outline.ts" vintage-car-1929 "$1" "$2" "$O" --base "$O" "${@:3}" || exit 1 }
[[ -z "$ONLY" || "$ONLY" == *50e* ]] && ol 50 elevation \
  "hood-hinge=hood-hinge@like=radiator-neck" "drag-link=drag-link@axis=1" "drop-arm=drop-arm@like=damper" \
  "wiper=wiper@axis=1,like=handle-front" "wiper-motor=wiper-motor@like=handle-front" "wing-stay=wing-stay-l+wing-stay-r@axis=1" \
  "fender-front=fender-front-l+fender-front-r@tol=0.5" "fender-rear=fender-rear-l+fender-rear-r@tol=0.5" \
  "shackle-front=shackle-front-l+shackle-front-r@like=spring-front" "shackle-rear=shackle-rear-l+shackle-rear-r@like=spring-rear" \
  "lamp-bar-foot=lamp-bar-foot-l+lamp-bar-foot-r@like=lamp-bar" "headlamp-lens=headlamp-lens-l+headlamp-lens-r@tol=0.5" \
  "radiator-cap=radiator-cap+radiator-neck@tol=0.5" "tail-lamp=tail-lamp@tol=0.5" "tail-pipe=tail-pipe-1+tail-pipe-2+tail-pipe-3+tail-pipe-4@tol=0.5" \
  "wheelhouse=wheelhouse-l+wheelhouse-r@tol=0.5" "bumper-rear-lower=bumper-rear-lower@tol=0.5" "bumper-rear-upper=bumper-rear-upper@tol=0.5" \
  "tyre-f=tyre-fl+tyre-fr@tol=0.5" "tyre-r=tyre-rl+tyre-rr@tol=0.5" "rim-f=rim-fl+rim-fr@tol=0.5" "rim-r=rim-rl+rim-rr@tol=0.5" \
  "spokes-f=spokes-fl+spokes-fr@after=drum-f,bone=wheel-f,role=detail,z=5,tone=0.7,region=wheel-metal,material=steel-rim,tol=0.5" "spokes-r=spokes-rl+spokes-rr@after=drum-r,bone=wheel-r,role=detail,z=5,tone=0.7,region=wheel-metal,material=steel-rim,tol=0.5" \
  $(for w in f r; do for i in {1..12}; do echo "spoke-$w-$i=@rm=1"; done; done)
[[ -z "$ONLY" || "$ONLY" == *50f* ]] && ol 50 front \
  "roof-slab=roof-slab@tol=0.5" "hinge-l=hinge-front-l+hinge-rear-l@like=handle-l,z=2.15" "hinge-r=hinge-front-r+hinge-rear-r@like=handle-r,z=2.15" \
  "belt-moulding-l=belt-moulding-l@like=handle-l,z=2.15" "belt-moulding-r=belt-moulding-r@like=handle-r,z=2.15" \
  "spring-rear-l=spring-rear-l+spring-hanger-l+shackle-rear-l@like=spring-front-l" "spring-rear-r=spring-rear-r+spring-hanger-r+shackle-rear-r@like=spring-front-r" \
  "axle-f=axle-beam+stub-axle-l+stub-axle-r+kingpin-l+kingpin-r+steering-arm-l+steering-arm-r@tol=0.5" "tie-rod=tie-rod@axis=1,like=axle-f" \
  "damper-l=damper-l+damper-lever-l@like=spring-front-l" "damper-r=damper-r+damper-lever-r@like=spring-front-r" \
  "exhaust-downpipe=exhaust-downpipe-1+exhaust-downpipe-2@like=silencer" "lamp-bar-foot=lamp-bar-foot-l+lamp-bar-foot-r@like=lamp-bar" \
  "tyre-fl=tyre-fl@tol=0.5" "tyre-fr=tyre-fr@tol=0.5" "tyre-rl=tyre-rl@tol=0.5" "tyre-rr=tyre-rr@tol=0.5" \
  "drum-fl=drum-fl@tol=0.5" "drum-fr=drum-fr@tol=0.5" "hub-fl=hub-fl+hub-cap-fl@tol=0.5,z=3.8" "hub-fr=hub-fr+hub-cap-fr@tol=0.5,z=3.8"
[[ -z "$ONLY" || "$ONLY" == *50p* ]] && ol 50 plan \
  "drag-link=drag-link@axis=1" "fender-fl=fender-front-l@tol=0.5" "fender-fr=fender-front-r@tol=0.5" "fender-rl=fender-rear-l@tol=0.5" "fender-rr=fender-rear-r@tol=0.5" \
  "wing-stay-l=wing-stay-l@axis=1,like=drag-link" "wing-stay-r=wing-stay-r@axis=1,like=drag-link" "tie-rod=tie-rod@axis=1,bone=axle-f" \
  "steering-arm-l=steering-arm-l@like=axle-f-l" "steering-arm-r=steering-arm-r@like=axle-f-r" \
  "damper-l=damper-l+damper-lever-l@like=spring-front-l" "damper-r=damper-r+damper-lever-r@like=spring-front-r" \
  "bumper-front=bumper-front-lower+bumper-front-upper+bumper-front-clamp-c+bumper-front-clamp-l+bumper-front-clamp-r@tol=0.5" \
  "bumper-rear=bumper-rear-lower+bumper-rear-upper+bumper-rear-clamp-c+bumper-rear-clamp-l+bumper-rear-clamp-r@tol=0.5" \
  "headlamp-bowl-l=headlamp-bowl-l@tol=0.5" "headlamp-bowl-r=headlamp-bowl-r@tol=0.5" "headlamp-lens-l=headlamp-lens-l@tol=0.5" "headlamp-lens-r=headlamp-lens-r@tol=0.5" \
  "hood-catch-l=hood-catch-1-l+hood-catch-2-l@like=radiator-cap" "hood-catch-r=hood-catch-1-r+hood-catch-2-r@like=radiator-cap" \
  "radiator-shell=radiator-shell@tol=0.5" "frame-l=frame-rail-l@tol=0.5" "frame-r=frame-rail-r@tol=0.5" "tail-lamp=tail-lamp@tol=0.5"
[[ -z "$ONLY" || "$ONLY" == *100e* ]] && ol 100 elevation \
  "spokes-f=spokes-fl+spokes-fr@after=drum-f,bone=wheel-f,role=detail,z=5,tone=0.7,region=wheel-metal,material=steel-rim,tol=1" "spokes-r=spokes-rl+spokes-rr@after=drum-r,bone=wheel-r,role=detail,z=5,tone=0.7,region=wheel-metal,material=steel-rim,tol=1" \
  $(for w in f r; do for i in {1..8}; do echo "spoke-$w-$i=@rm=1"; done; done) \
  "tyre-f=tyre-fl+tyre-fr@tol=1" "tyre-r=tyre-rl+tyre-rr@tol=1" "rim-f=rim-fl+rim-fr@tol=1" "rim-r=rim-rl+rim-rr@tol=1" \
  "headlamp-lens=headlamp-lens-l+headlamp-lens-r@tol=1" "radiator-cap=radiator-cap@tol=1" \
  "shackle-front=shackle-front-l+shackle-front-r+shackle-front-eye-l+shackle-front-eye-r@like=spring-front" "shackle-rear=shackle-rear-l+shackle-rear-r@like=spring-rear"
[[ -z "$ONLY" || "$ONLY" == *100f* ]] && ol 100 front \
  "roof-slab=roof-slab@tol=1" "frame=frame-rail-l+frame-rail-r+crossmember-front+crossmember-mid+crossmember-rear@tol=1"
[[ -z "$ONLY" || "$ONLY" == *100p* ]] && ol 100 plan \
  "fender-fl=fender-front-l@tol=1" "fender-fr=fender-front-r@tol=1" "fender-rl=fender-rear-l@tol=1" "fender-rr=fender-rear-r@tol=1" \
  "spring-front-l=spring-front-l@like=shackle-front-l" "spring-front-r=spring-front-r@like=shackle-front-r" \
  "bumper-front=bumper-front-lower+bumper-front-upper@tol=1" "bumper-rear=bumper-rear-lower+bumper-rear-upper@tol=1" \
  "frame-l=frame-rail-l@tol=1" "frame-r=frame-rail-r@tol=1"
ASSET_FILE="$(catfile vintage-car-1929)" python3 "$W/texts.py" vintage-car-1929 '[["50", "omitted", "Door hinges, hood latches, mascot, tyre tread", "Tyre tread", "Türscharniere, Haubenverschlüsse, Kühlerfigur, Reifenprofil", "Reifenprofil"], ["50", "retained", "and tyre valve per wheel.", "and tyre valve per wheel; wiper with its motor, bonnet hinge, mascot, drop arm and drag link, spring shackles and the feet of the lamp bar (side), door hinges, belt moulding, dampers and tie rod (front), bonnet catches, wing stays and dampers (plan), each as the outline of its body.", "und Reifenventil je Rad.", "und Reifenventil je Rad; Wischer mit Motor, Haubenscharnier, Kühlerfigur, Lenkstockhebel und Lenkschubstange, Federlaschen und die Füsse der Lampenstange (Seite), Türscharniere, Gürtelleiste, Stossdämpfer und Spurstange (vorn), Haubenverschlüsse, Kotflügelstreben und Stossdämpfer (Grundriss), jeweils als Umriss ihres Körpers."]]'
