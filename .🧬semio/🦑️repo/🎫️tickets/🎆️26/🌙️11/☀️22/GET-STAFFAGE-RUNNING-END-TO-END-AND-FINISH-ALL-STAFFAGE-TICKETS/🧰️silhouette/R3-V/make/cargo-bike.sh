#!/bin/zsh
# 🚲️ Writes the repair ops of `cargo-bike` (ops/cargo-bike.json): the hand-made ops of make/cargo-bike.py, then outlines from the body solids with outline.ts.
source "${0:A:h}/../env.sh"
O="$W/ops/cargo-bike.json"; rm -f "$O"
python3 "$W/make/cargo-bike.py" || exit 1
cd "$S/📦️packages/🟦️typescript" || exit 1
ol() { timeout 900 "$B" "$W/outline.ts" cargo-bike "$1" "$2" "$O" --base "$O" "${@:3}" || exit 1 }
ol 50 elevation "display=display@keep=1" "fender-r=fender-r@keep=1,tol=1"
ol 100 elevation "battery-case=battery-case@keep=1" "motor-bracket=motor-bracket@keep=1" "fender-f=fender-f@keep=1,tol=1" "fork-crown=fork-crown@keep=1"
ol 200 elevation "battery-case=battery-case@keep=1" "motor-bracket=motor-bracket@keep=1"
ol 50 elevation "tyre-r=tyre-r@keep=1,tol=0.5"
ol 50 plan "lever-l=lever-body-l+lever-l" "lever-r=lever-body-r+lever-r"
ol 100 front "stand-cross=stand-cross@axis=1,after=stand-pivot,z=42.5,region=frame"
ol 100 elevation "lamp-bracket=lamp-bracket-f@axis=1,after=head-lamp,z=54.5,region=frame"
ol 100 elevation "fender-bracket-f=fender-bracket-f@after=head-lamp,z=54.6,region=fenders"
