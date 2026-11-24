#!/bin/zsh
# ✨️ Fit pass of SH-2 (2026-11-23): the people that miss the guard by little get the fit and the polish alone (lateral ends of extruded shapes, local offsets, shifts), no greedy pass; each is measured right after into `gate4`.
source "${0:A:h}/env.sh"; cd "$W"
export MAXITER=0 PRUNE=0 MAXSEC=1 FITSEC=${FITSEC:-200}
run() { ./fix.sh "$@"; grep -E "fit |polish " "$GEN/auto/$1-$2.log" | cut -c9-200; ./gate.sh gate4 "$1"; }
for job in "$@"; do run ${=job}; done
echo FIT1-DONE
