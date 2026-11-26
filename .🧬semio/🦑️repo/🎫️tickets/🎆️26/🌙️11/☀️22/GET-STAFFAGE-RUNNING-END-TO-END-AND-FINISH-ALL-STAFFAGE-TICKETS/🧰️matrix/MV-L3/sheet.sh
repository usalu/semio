#!/bin/zsh
# 👀️ Line panels of one asset document inside the grant of its caller: `sheet.sh <tag> <file.json> <id> <scale>:<pose>:<view>…` writes `$GEN/sheets/<id>.s<scale>.<pose>.<tag>.<view>.png` at 1300 pixels with `../MV-L1/zoom.ts` (the line view exactly as the product draws it).
source "${0:A:h}/env.sh"
tag="$1"; file="$2"; a="$3"; shift 3; mkdir -p "$GEN/sheets"; cd "$T14"
for item in "$@"; do
  s="${item%%:*}"; rest="${item#*:}"; pose="${rest%%:*}"; v="${rest#*:}"
  DENSITY=${DENSITY:-500} timeout 900 "$B" "$U/🧰️matrix/MV-L1/zoom.ts" "$file" "$s" "$pose" "$v" "$GEN/sheets/$a.s$s.$pose.$tag.$v.png" ${HEIGHT:-1300} | tail -1
done
