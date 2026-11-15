#!/bin/sh
# chk.sh <slug> [scale]  -> failing measurements + thermal areas
cd "$(dirname "$0")"
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
S=""; [ -n "$2" ] && S="--scale $2"
$BUN volumes-check.ts animals/$1 --product $S 2>&1 | grep -E "^(FAIL)" | grep -v "^FAIL:" | while read -r l; do echo "$l" | grep -oE "^FAIL [a-z-]+ +1:[0-9]+"; echo "$l" | grep -oE "!?(iou|p95M|p95SoftM|areaDeviation|outsideBody|extentDifference)[A-Za-z:.-]*=[0-9.e-]+(<=|>=)[0-9.e-]+|outerAreaM2:[a-z-]+=[0-9.]+|[a-zA-Z.]*areaDeviationTotal=[0-9.]+|outerArea[A-Za-z:-]*=[0-9.]+" | grep -E "^!|outer|Total" | tr '\n' ' '; echo; done
