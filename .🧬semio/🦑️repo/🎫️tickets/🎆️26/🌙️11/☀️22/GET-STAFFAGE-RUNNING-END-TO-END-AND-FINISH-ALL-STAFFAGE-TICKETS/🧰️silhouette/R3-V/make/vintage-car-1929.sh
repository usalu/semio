#!/bin/zsh
# 🚗️ Writes the repair ops of `vintage-car-1929` (ops/vintage-car-1929.json): outlines from the body solids with outline.ts (every part is on the fixed chassis or on an axle that only travels up and down, so an outline in the plane of the view stays right in every pose).
source "${0:A:h}/../env.sh"
O="$W/ops/vintage-car-1929.json"; rm -f "$O"
cd "$S/📦️packages/🟦️typescript" || exit 1
ol() { timeout 900 "$B" "$W/outline.ts" vintage-car-1929 "$1" "$2" "$O" --base "$O" "${@:3}" || exit 1 }
ol 50 elevation "mascot=mascot@like=radiator-cap,z=9"
ol 50 plan "tie-rod=tie-rod+steering-arm-l+steering-arm-r@like=axle-f-mid"
ol 100 elevation "lamp-post=lamp-post-l+lamp-post-r@like=lamp-stalk"
ol 50 elevation "valve-f=valve-fl+valve-fr" "valve-r=valve-rl+valve-rr" "drum-r=drum-rl+drum-rr+differential-nose@tol=1" "damper=damper-l+damper-r+damper-lever-l+damper-lever-r@like=spring-front" "radiator-shell=radiator-shell+radiator-core@tol=1" "headlamp-bowl=headlamp-bowl-l+headlamp-bowl-r@tol=1"
ol 50 front "frame=frame-rail-l+frame-rail-r+crossmember-front+crossmember-radiator+crossmember-gearbox+crossmember-mid+crossmember-kick+crossmember-rear@tol=1" "fender-l=fender-front-l+fender-rear-l@tol=1" "fender-r=fender-front-r+fender-rear-r@tol=1" "axle-r=axle-tube+spring-seat-l+spring-seat-r@tol=1"
ol 50 plan "hood=hood-top+hood-side-l+hood-side-r@tol=1" "spring-front-l=spring-front-l@like=engine-bearer-2-l" "spring-front-r=spring-front-r@like=engine-bearer-2-r"
ol 100 elevation "radiator-shell=radiator-shell+radiator-core@tol=1" "headlamp-bowl=headlamp-bowl-l+headlamp-bowl-r@tol=1" "bracket-rear=bracket-rear-l+bracket-rear-r+bumper-clamp-l+bumper-clamp-r@tol=1"
ol 100 front "fender-l=fender-front-l+fender-rear-l@tol=1" "fender-r=fender-front-r+fender-rear-r@tol=1" "axle-f=axle-beam+stub-axle-l+stub-axle-r@tol=1" "axle-r=axle-tube+half-shaft-rl+half-shaft-rr@tol=1"
ol 100 plan "shackle-front-l=shackle-front-eye-l+shackle-front-l@like=bracket-front-l" "shackle-front-r=shackle-front-eye-r+shackle-front-r@like=bracket-front-r"
