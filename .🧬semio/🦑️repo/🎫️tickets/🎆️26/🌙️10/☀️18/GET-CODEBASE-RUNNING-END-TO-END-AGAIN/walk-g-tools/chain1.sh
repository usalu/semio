#!/bin/sh
# [walk-g] before/after on the two heavy roots, one at a time. usage: chain1.sh
cd /Users/ueli/Documents/semio || exit 1
C='.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN'
D='✏️s/🧑‍💻dev'
sh "$C/walk-g-tools/run.sh" before-sdev "ONLY_ROOTS=$D" RSS_CAP_MB=7500 SAMPLE_S=5 > "$C/🗑️generated/walk-g/before-sdev.out" 2>&1
sh "$C/walk-g-tools/run.sh" after-root ONLY_ROOTS=. RSS_CAP_MB=7500 SAMPLE_S=5 STAGED=1 > "$C/🗑️generated/walk-g/after-root.out" 2>&1
sh "$C/walk-g-tools/run.sh" after-sdev "ONLY_ROOTS=$D" RSS_CAP_MB=7500 SAMPLE_S=5 STAGED=1 > "$C/🗑️generated/walk-g/after-sdev.out" 2>&1
