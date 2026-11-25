#!/bin/zsh
# 👀️ Renders the product sheet of a trial copy and writes a copy 1600 px wide: look-file.sh <id> <file.json> <scale> <pose|-> <tag>   (inside a grant that is held, lane.sh)
export BATCH=MH-6; source "${0:A:h}/../MR/env.sh"
T14="$R/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE"
work="$GEN/sheets/lf/$1"; mkdir -p "$work"; cp "$2" "$work/🔣️.json"
pose="$4"; [[ "$pose" == "-" ]] && pose="$(python3 -c "import json,sys; print(json.load(open(sys.argv[1]))['asset']['presentation']['defaultPose'])" "$work/🔣️.json")"
png="$(cd "$T14" && timeout 300 "$B" r3-2d.ts --file "$work/🔣️.json" --scale "$3" --pose "$pose" 2>&1 | tail -1)"
out="$GEN/sheets/$1.s$3.$pose.$5.png"
[[ -f "$png" ]] && mv "$png" "$out" && sips -Z 1600 "$out" --out "${out%.png}.small.png" >/dev/null 2>&1 && echo "${out%.png}.small.png" || echo "no sheet: $png"
