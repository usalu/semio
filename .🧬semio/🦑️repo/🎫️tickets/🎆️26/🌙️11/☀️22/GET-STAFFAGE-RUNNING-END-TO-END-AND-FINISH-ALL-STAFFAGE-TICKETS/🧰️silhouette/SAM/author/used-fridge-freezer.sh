#!/bin/zsh
# 🧊️ Repair ops of `used-fridge-freezer`, in order: the mains lead as a stroke (used-fridge-freezer.py), then the copper suction pipe that runs up the back between cabinet and condenser (solids `pipe-a`, `pipe-b`, 8 mm, drawn in no view; it stood 15.0 mm outside the drawing, exactly at the limit) as its traced outline behind the side panel.
source "${0:A:h}/../env.sh"; cd "$W/author" && python3 used-fridge-freezer.py && cd "$W"
"$B" trace.ts used-fridge-freezer 50 elevation pipe-a,pipe-b --id pipe --bone cabinet --like cable-tape --after cable-lead --material copper-pipe --tone 0.4 --z 2.9
