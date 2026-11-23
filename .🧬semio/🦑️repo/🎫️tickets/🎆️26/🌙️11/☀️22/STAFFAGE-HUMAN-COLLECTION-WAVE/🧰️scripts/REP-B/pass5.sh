#!/bin/zsh
# 🔁️ Fifth pass of the greedy silhouette repair (2026-11-23, with the fit of the axis offsets of revolved limbs and their redraw about the own bone axis): the people and scales that still miss the guard; then the declared dimensions of the four people whose body and drawings disagreed are fitted again.
source "${0:A:h}/env.sh"; cd "$W"
export MAXSEC=${MAXSEC:-150}
./fix.sh person-signing 100
./fix.sh person-eating-standing 50
./fix.sh waving-person 50
./fix.sh person-taking-selfie 50
./fix.sh person-drinking-coffee 50
./fix.sh photographer-with-camera 50
./fix.sh standing-violinist 100
./fix.sh person-sweeping 50
./fix.sh person-hanging-laundry 50 100
./fix.sh person-carrying-shopping-bags 50
./fix.sh person-with-guide-dog 50
./fix.sh person-checking-watch 50
./fix.sh worker-with-pallet-jack 50
./fix.sh painter-at-easel 50
./fix.sh person-carrying-ladder 50 200 500
cd "$S/📦️packages/🟦️typescript"
for a in person-drinking-coffee person-sweeping person-signing person-carrying-ladder; do "$B" "$W/dims.ts" "$a" --fit | grep "declared\|disagree\|redeclared"; (cd "$W" && "$B" apply.ts "$a" | tail -1); done
echo PASS5-DONE
