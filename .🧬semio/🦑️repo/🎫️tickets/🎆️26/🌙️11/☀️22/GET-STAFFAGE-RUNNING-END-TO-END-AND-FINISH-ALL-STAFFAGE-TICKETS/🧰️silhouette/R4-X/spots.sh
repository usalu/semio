#!/bin/zsh
# 🖼️ Several places of one view as the guard rasterises them (tile-accurate), tiled into one picture, with the worst places listed: spots.sh <id|file> <scale> <view> <ops|-> <count> <name:x:y:r>…
source "${0:A:h}/env.sh"
id="$1"; s="$2"; v="$3"; ops="$4"; n="$5"; shift 5
D="$GEN/png/${id:t:r}-$s-$v"; mkdir -p "$D"
cd "$S/📦️packages/🟦️typescript" && SPOTS="${(j:;:)@}" SPOTDIR="$D" timeout ${TIMEOUT:-2400} "$B" "$W/where.ts" "$id" "$s" "$v" "$ops" "$n" | cut -c1-${CUT:-400}
files=(); for q in "$@"; do files+=("$D/${q%%:*}.png"); done
python3 "$W/tile.py" "$D/all.png" ${COLS:-3} ${SIZE:-400} $files > /dev/null && echo "$D/all.png"
