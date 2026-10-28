#!/bin/sh
# [walk-g] second before/after round with the project values written out for the equality oracle (cmp-projects.py). usage: chain2.sh
cd /Users/ueli/Documents/semio || exit 1
C='.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN'
G="$C/🗑️generated/walk-g"
D='✏️s/🧑‍💻dev'
for round in 2; do
sh "$C/walk-g-tools/run.sh" before-root$round ONLY_ROOTS=. RSS_CAP_MB=7500 SAMPLE_S=2 PROJECT_OUT="$G/before-root$round.project.json" > "$G/before-root$round.out" 2>&1
sh "$C/walk-g-tools/run.sh" after-root$round ONLY_ROOTS=. RSS_CAP_MB=7500 SAMPLE_S=2 STAGED=1 PROJECT_OUT="$G/after-root$round.project.json" > "$G/after-root$round.out" 2>&1
sh "$C/walk-g-tools/run.sh" before-sdev$round "ONLY_ROOTS=$D" RSS_CAP_MB=7500 SAMPLE_S=2 PROJECT_OUT="$G/before-sdev$round.project.json" > "$G/before-sdev$round.out" 2>&1
sh "$C/walk-g-tools/run.sh" after-sdev$round "ONLY_ROOTS=$D" RSS_CAP_MB=7500 SAMPLE_S=2 STAGED=1 PROJECT_OUT="$G/after-sdev$round.project.json" > "$G/after-sdev$round.out" 2>&1
done
