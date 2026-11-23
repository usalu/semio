#!/bin/zsh
# ✨️ Polish pass of the greedy silhouette repair (2026-11-23): the people and scales that miss the guard by little get the whole time budget for small offsets and shifts of the shapes next to the worst mismatch.
source "${0:A:h}/env.sh"; cd "$W"
export PARAM=0 PRUNE=0 MAXSEC=${MAXSEC:-200}
./fix.sh person-drinking-coffee 50
./fix.sh person-signing 50
./fix.sh photographer-with-camera 50
./fix.sh person-taking-selfie 50
./fix.sh standing-violinist 100
./fix.sh painter-at-easel 50
./fix.sh worker-with-pallet-jack 50
./fix.sh waving-person 50
./fix.sh person-hanging-laundry 100 50
./fix.sh person-carrying-ladder 50
./fix.sh person-with-guide-dog 50
./fix.sh person-carrying-shopping-bags 50
./fix.sh person-checking-watch 50
./fix.sh person-eating-standing 50
echo POLISH-DONE
