#!/bin/zsh
# 🔍️ One line panel cut to a part inside the grant of its caller: `zoom.sh <tag> <file.json> <id> <scale> <pose> <view> <height> <left> <top> <width> <height as fractions>` writes `$GEN/sheets/<id>.s<scale>.<pose>.<tag>.<view>.cut<CUT>.png` with `../MV-L1/zoom.ts`.
source "${0:A:h}/env.sh"
tag="$1"; file="$2"; a="$3"; s="$4"; pose="$5"; v="$6"; h="$7"; shift 7; mkdir -p "$GEN/sheets"
cd "$T14" && DENSITY=${DENSITY:-500} timeout 900 "$B" "$U/🧰️matrix/MV-L1/zoom.ts" "$file" "$s" "$pose" "$v" "$GEN/sheets/$a.s$s.$pose.$tag.$v.cut${CUT:-}.png" "$h" "$@" | tail -1
