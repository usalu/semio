#!/bin/bash
# usage: author-31-plane-run.sh <scale>   generator + lobe fit
cd "$(dirname "$0")"; BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1); S=$1
$BUN author-31-plane.ts $S
IDS=$(python3 -c "
import json;d=json.load(open('volumes/vegetation/london-plane-pollarded.json'))['volumes']['$S']
print(','.join(s['id'] for s in d['body'] if s['id'].startswith(('lobe','fill','crown'))))")
$BUN kit-fit.ts vegetation/london-plane-pollarded --scale $S --solid $IDS --rounds 2 --steps 0.12,0.06,0.03 --write 2>&1 | grep -E "IoU|written"
