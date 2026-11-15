#!/bin/bash
S=$1
./author-29-bun.sh author-29-pigeon.ts $S --write && ./author-29-thermal-pigeon.sh $S --write 2>&1 | grep -E "^(legs|beak|cere|eye|plumage|NO|kit)|MODEL" | cut -c1-110 && ./author-29-bun.sh author-29-pigeon.ts $S --extra && python3 author-29-emit.py animals/pigeon $S trunk 16 0.004 && ./author-29-bun.sh kit-structure.ts animals/pigeon --scale $S --write 2>&1 | grep -E "^kit-structure|outside" | cut -c1-140 && ./author-29-bun.sh volumes-check.ts animals/pigeon --scale $S --quiet 2>&1 | grep -v "^\[check" | cut -c1-900
