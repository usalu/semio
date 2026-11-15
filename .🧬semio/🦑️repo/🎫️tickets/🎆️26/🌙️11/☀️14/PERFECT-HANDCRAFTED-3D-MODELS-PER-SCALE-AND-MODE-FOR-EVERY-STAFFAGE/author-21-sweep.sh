#!/bin/bash
# usage: env... author-21-sweep.sh <scale>  -> gen scale and print compact silhouette numbers (product engine)
cd "$(dirname "$0")"
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
$BUN author-21-maple.ts $1 >/dev/null 2>&1
$BUN volumes-check.ts vegetation/japanese-maple --scale $1 --rules volumes-silhouette 2>&1 | grep "silhouette" | grep -o "iou:[a-z]*=[0-9.]*\|p95SoftM:[a-z]*=[0-9.]*" | sed 's/p95SoftM/s95/;s/iou:elevation/E/;s/iou:front/F/;s/iou:plan/P/;s/:elevation//;s/:front//;s/:plan//' | tr '\n' ' '; echo
