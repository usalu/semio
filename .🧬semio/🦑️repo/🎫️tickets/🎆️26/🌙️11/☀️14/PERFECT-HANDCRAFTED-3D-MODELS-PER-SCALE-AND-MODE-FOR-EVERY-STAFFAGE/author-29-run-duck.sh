#!/bin/bash
S=$1
M="body-plumage=body,neck-plumage=neck-lower+neck-upper,head-plumage=head,bill=bill,wing-plumage=near-wing+far-wing,tail-plumage=tail,legs-skin=near-shank+near-tarsus+far-shank+far-tarsus,web-feet=near-foot+far-foot"
./author-29-bun.sh author-29-duck.ts $S --write && ./author-29-bun.sh kit-thermal.ts animals/mallard-duck --scale $S --map "$M" --write 2>&1 | grep -E "^(legs|bill|web|plumage|NO|kit|body|neck|head|wing|tail)|MODEL" | cut -c1-120 && python3 author-29-emit.py animals/mallard-duck $S trunk 12 0.004 && ./author-29-bun.sh kit-structure.ts animals/mallard-duck --scale $S $NJ --write 2>&1 | grep -E "^kit-structure|outside" | cut -c1-140 && ./author-29-bun.sh volumes-check.ts animals/mallard-duck --scale $S --quiet 2>&1 | grep -v "^\[check" | cut -c1-900
