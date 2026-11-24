#!/bin/zsh
# 🚌️ Writes the repair ops of `city-bus` (ops/city-bus.json): the body repair of make/city-bus.py first, then the 1:50 drawings from the body solids.
source "${0:A:h}/../env.sh"
O="$W/ops/city-bus.json"
python3 "$W/make/city-bus.py" || exit 1
cd "$S/📦️packages/🟦️typescript" || exit 1
ol() { timeout 900 "$B" "$W/outline.ts" city-bus "$1" "$2" "$O" --base "$O" "${@:3}" || exit 1 }
ol 50 front "leaf-f-fwd=leaf-f-fwd+leaf-f-fwd-upper+leaf-f-fwd-seal"
python3 "$W/close-gap.py" city-bus 50 front leaf-f-fwd x gt -1.28 0.006
ol 50 plan "headlamp-l=headlamp-l" "headlamp-r=headlamp-r"
SHELL_IDS=$(python3 "$W/ids.py" city-bus 50 body mirror,seat,grab-rail,stanchion,driver,dashboard,steering)
ol 50 elevation "body-shell=$SHELL_IDS@tol=1"
ol 50 front "body-shell=$SHELL_IDS@tol=1"
ol 50 plan "body-shell=$SHELL_IDS@tol=1"
ol 50 elevation "battery-pack=battery-pack+battery-module@tol=1" "hvac-unit=$(python3 "$W/ids.py" city-bus 50 hvac)@tol=1"
ol 50 front "tyre-rr=rr-tyre+rr-inner-tyre+rr-rim+rr-inner-rim+rr-hub+rr-brake-disc@tol=1" "tyre-rl=rl-tyre+rl-inner-tyre+rl-rim+rl-inner-rim+rl-hub+rl-brake-disc@tol=1" "tyre-fr=fr-tyre+fr-rim+fr-hub+fr-brake-disc@tol=1" "tyre-fl=fl-tyre+fl-rim+fl-hub+fl-brake-disc@tol=1"
python3 - <<'P' | python3 "$W/hand.py" city-bus
import json
print(json.dumps([{"op": "shape-set", "scale": "50", "view": "plan", "id": i, "set": {"w": 0.625}} for i in ("leaf-m-aft", "leaf-f-aft")]))
P
ol 50 front "mirror-arm-r=mirror-arm-r+mirror-base-r" "mirror-arm-l=mirror-arm-l"
