#!/bin/sh
# 🖼️ Renders the three views of a pose (or clip instant) of an asset file at one scale and view mode to SVG and PNG with sharp.
# prev.sh <file.json> <asset-id> <scale> <pose|clip:id:time> [view] [width]
B=/Users/ueli/Documents/semio/.cache/b/t/m/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
H="$(cd "$(dirname "$0")" && pwd)"
S="$H/../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage"
F="$(cd "$(dirname "$1")" && pwd)/$(basename "$1")"
O="$H/../../🗑️generated/HW-03/prev"
mkdir -p "$O"
case "$4" in clip:*) Q="--clip $(echo "$4" | cut -d: -f2) --time $(echo "$4" | cut -d: -f3)";; *) Q="--pose $4";; esac
cd "$S" && $B 🔨️modules/🎨️depiction/🖨️preview/🟦️.ts --asset "$2" --file "$F" --scale "$3" $Q --view "${5:-light-solid}" --views elevation,front,plan --projection third --out "$O" 2>&1 | grep -v "^ *at \|rsvg\|ToolMissing\|tool:\|detail:\|^$\|Bun v\|^[0-9]* |\|^ *\^" | tail -3
cd "$H" && $B png.ts "${6:-1600}" "$O"/*.svg >/dev/null && ls -t "$O"/*.png | head -3
