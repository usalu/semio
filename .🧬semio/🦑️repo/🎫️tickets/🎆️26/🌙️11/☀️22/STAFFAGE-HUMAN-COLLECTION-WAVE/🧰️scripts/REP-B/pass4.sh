#!/bin/zsh
# 🔁️ Fourth pass of the greedy silhouette repair (2026-11-23, with the revolved refit of limbs), directly on the repair files: every person and scale that still missed the guard, and all four scales of the people whose declared dimensions were corrected to the body.
source "${0:A:h}/env.sh"; cd "$W"
./fix.sh person-drinking-coffee 50 200 500
./fix.sh person-sweeping 50 100 200 500
./fix.sh person-signing 50 100 200 500
./fix.sh person-carrying-ladder 50 100 200 500
./fix.sh person-eating-standing 50
./fix.sh waving-person 50
./fix.sh photographer-with-camera 50
./fix.sh person-taking-selfie 50
./fix.sh standing-violinist 100
./fix.sh painter-at-easel 50 100
./fix.sh person-hanging-laundry 50 100
./fix.sh person-carrying-shopping-bags 50
./fix.sh person-with-guide-dog 50
./fix.sh person-checking-watch 50
./fix.sh parent-with-child-on-shoulders 50
./fix.sh worker-with-pallet-jack 50
./fix.sh dancer 50 100 200 500
echo PASS4-DONE
