#!/bin/zsh
# 🥈️ The second measurements of lane CL-3D in one grant, one process after the other: where the canopy of `person-with-umbrella` lies in the 1:200 and 1:500 body scenes (`extent.ts --match pleat`), then the trial of the staged roundness levels on light, heavy and growing assets (`trial.sh`).
source "${0:A:h}/env.sh"
zsh "$W/run.sh" extent.ts person-with-umbrella s200-body s500-body --match pleat 2>&1 | cut -c1-300 > "$GEN/diag-pleat.txt"
zsh "$W/trial.sh" tealight thermostatic-radiator-valve standing-adult city-car nurse-walking person-with-rollator waiter-with-tray tennis-player standing-desk-worker parent-carrying-toddler rowan european-larch sweetgum european-beech small-leaved-lime used-upright-piano ceiling-diffuser walking-toddler running-child used-park-armchair used-steel-string-acoustic-guitar > "$GEN/trial2.txt" 2>&1
