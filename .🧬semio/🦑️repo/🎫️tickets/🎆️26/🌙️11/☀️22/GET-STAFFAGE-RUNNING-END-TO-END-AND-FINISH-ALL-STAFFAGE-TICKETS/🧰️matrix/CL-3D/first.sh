#!/bin/zsh
# 🥇️ The first measurements of lane CL-3D in one grant, one process after the other: the bounds diagnosis of category E (`diag-e.sh`), the trial of the staged roundness level on light and heavy assets (`trial.sh`), then the budget scan of the catalog (`scan.sh`, resumable).
source "${0:A:h}/env.sh"
zsh "$W/diag-e.sh" > "$GEN/diag-e.txt" 2>&1
zsh "$W/trial.sh" tealight standing-adult city-car person-with-rollator waiter-with-tray tennis-player rowan european-larch sweetgum european-beech small-leaved-lime used-climbing-net-pyramid saguaro-cactus > "$GEN/trial.txt" 2>&1
zsh "$W/scan.sh"
