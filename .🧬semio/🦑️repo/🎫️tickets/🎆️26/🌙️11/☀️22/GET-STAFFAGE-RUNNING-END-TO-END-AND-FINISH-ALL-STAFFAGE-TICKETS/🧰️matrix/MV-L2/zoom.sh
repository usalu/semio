#!/bin/zsh
# 🔍️ One line panel of a sheet at a larger size inside the grant of the worker: `zoom.sh <tag> <file.json> <id> <scale> <pose> <view> [height] [left top width height]` writes `$GEN/sheets/<id>.s<scale>.<pose>.<tag>.<view>[.cut].png` with `../MV-L1/zoom.ts`.
source "${0:A:h}/env.sh"
T14="$R/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE"
tag="$1"; file="$2"; a="$3"; s="$4"; pose="$5"; v="$6"; shift 6; h="${1:-1500}"; cut=""; [[ $# -gt 1 ]] && cut=".cut${CUT:-}"
cd "$T14" && DENSITY=${DENSITY:-500} timeout 900 "$B" "$U/🧰️matrix/MV-L1/zoom.ts" "$file" "$s" "$pose" "$v" "$GEN/sheets/$a.s$s.$pose.$tag.$v$cut.png" "$h" "${@:2}" | tail -1
