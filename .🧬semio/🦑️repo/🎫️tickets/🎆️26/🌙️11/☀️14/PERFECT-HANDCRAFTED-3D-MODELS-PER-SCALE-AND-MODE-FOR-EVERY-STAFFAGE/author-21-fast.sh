#!/bin/bash
# usage: author-21-fast.sh <scale>  -> gen + hard p95 + ious for elevation/front/plan
cd "$(dirname "$0")"
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
for v in elevation front plan; do echo -n "$v: "; $BUN author-21-hard.ts vegetation/japanese-maple $1 $v 2>&1 | head -1 | awk '{print "p95",$4,"soft",$7,"iou",$9}'; done
