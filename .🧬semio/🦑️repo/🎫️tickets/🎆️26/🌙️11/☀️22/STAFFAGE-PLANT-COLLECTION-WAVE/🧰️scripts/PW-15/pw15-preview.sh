#!/bin/sh
# 🖼️ Renders a PW-15 asset (default pose calm) at 1:50, 1:100, 1:200 and 1:500 as three-view sheets (dark line) plus the axonometric 1:50 view as SVG into 🗑️generated/PW-15/<id>/ and rasterises them with the system Quick Look thumbnailer (rsvg-convert is not installed on this machine). Usage: sh pw15-preview.sh <asset-id> [pose] [scales]
set -e
ID="$1"; POSE="${2:-calm}"; SCALES="${3:-50 100 200 500}"
S="$(cd "$(dirname "$0")/../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage" && pwd)"
G="$(cd "$(dirname "$0")/../.." && pwd)/🗑️generated/PW-15/$ID"
mkdir -p "$G"
cd "$S"
for SC in $SCALES; do
  repo tools exec bun -- 🔨️modules/🎨️depiction/🖨️preview/🟦️.ts --asset "$ID" --scale $SC --pose "$POSE" --views --projection first --no-png --flow none --out "$G" 2>&1 | grep -v "capacity\|still working" | tail -1
done
repo tools exec bun -- 🔨️modules/🎨️depiction/🖨️preview/🟦️.ts --asset "$ID" --scale 50 --pose "$POSE" --viewpoint axonometric --no-png --flow none --out "$G" 2>&1 | grep -v "capacity\|still working" | tail -1
cd "$G" && for F in *.svg; do qlmanage -t -s 2000 -o . "$F" >/dev/null 2>&1; done
