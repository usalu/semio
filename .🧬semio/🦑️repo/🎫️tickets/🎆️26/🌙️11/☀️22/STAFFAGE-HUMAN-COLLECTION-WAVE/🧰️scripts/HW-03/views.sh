#!/bin/sh
# 🖼️ Renders elevation, front and plan of one pose (or clip instant) at one scale and view mode as single drawings, trimmed and placed side by side in one PNG.
# views.sh <file.json> <asset-id> <scale> <pose|clip:id:time> [view] [height-px]
B=/Users/ueli/Documents/semio/.cache/b/t/m/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
H="$(cd "$(dirname "$0")" && pwd)"
S="$H/../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage"
F="$(cd "$(dirname "$1")" && pwd)/$(basename "$1")"
O="$H/../../🗑️generated/HW-03/views/$2-$3-$(echo "$4" | tr ':' '_')-${5:-light-solid}"
mkdir -p "$O"
case "$4" in clip:*) Q="--clip $(echo "$4" | cut -d: -f2) --time $(echo "$4" | cut -d: -f3)";; *) Q="--pose $4";; esac
for v in elevation front plan; do (cd "$S" && $B 🔨️modules/🎨️depiction/🖨️preview/🟦️.ts --asset "$2" --file "$F" --scale "$3" $Q --view "${5:-light-solid}" --viewpoint $v --out "$O/$v" >/dev/null 2>&1); done
cd "$H" && $B join.ts "$O" "${6:-900}"
