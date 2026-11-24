#!/bin/zsh
# 🎯️ Directed steps of SH-2 (2026-11-23) for the people whose remaining mismatch is one named part: the shoes of the violinist in the front of 1:100, the right sleeve and the gloves of the pallet-jack worker, the fretboard of the guitarist in the plan, sleeves and hoodie of the selfie taker; each is measured right after into `gate5`.
source "${0:A:h}/env.sh"; cd "$W"
export PRUNE=0 MAXSEC=1 FITSEC=${FITSEC:-60}
show() { grep -E "directed|group|refit|fit |polish " "$GEN/auto/$1-$2.log" | cut -c9-190; ./gate.sh gate5 "$1"; }
CMD="refit|front|shoe-r;refit|front|shoe-l" ./fix.sh standing-violinist 100; show standing-violinist 100
CMD="refit|plan|sleeve-r" ./fix.sh worker-with-pallet-jack 50; show worker-with-pallet-jack 50
GROUP=1 CMD="$(python3 handcmd.py worker-with-pallet-jack 50 hand-r elevation);$(python3 handcmd.py worker-with-pallet-jack 50 hand-l elevation)" PARAM=0 POLISH=0 ./fix.sh worker-with-pallet-jack 50; show worker-with-pallet-jack 50
CMD="refit|plan|guitar-fretboard;refit|plan|guitar-neck" ./fix.sh seated-guitarist 50; show seated-guitarist 50
CMD="refit|elevation|sleeve-r;refit|front|sleeve-r;refit|front|hoodie;refit|plan|sleeve-l" ./fix.sh person-taking-selfie 50; show person-taking-selfie 50
echo QUICK1-DONE
