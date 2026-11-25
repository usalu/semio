#!/bin/zsh
# 🧩️ The five suites that the capped run of 09:46 did not reach (it was stopped at 2,451 MB right after structure-projection), for the 20 assets of the batch: rest.sh <tag>   (run it through q.sh with 3200 MB: the measured 2,451 MB plus 30 %)
SUITES=multiview-sheet,model-skeleton,thermal-legend,plan-projection,model-roundness "${0:h}/suites.sh" "$1"
