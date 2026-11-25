#!/bin/zsh
# 🖼️ One product sheet (side, front, plan; line and solid) of any asset document through the memory queue: sheet.sh <file.json> <asset-id> <scale> <pose|-> <out.png> renders the file (copied into a work folder named after the asset) with the r3-2d tool of the 3D ticket in the pose given (`-`: the default pose) and moves the image to <out.png>.
export BATCH=MH-2; source "${0:A:h}/../MR/env.sh"
T14="$R/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE"
src="$1"; a="$2"; s="$3"; pose="$4"; out="$5"; mkdir -p "$GEN/sheets"
[[ "$pose" == "-" ]] && pose="$(python3 -c "import json,sys; print(json.load(open(sys.argv[1]))['asset']['presentation']['defaultPose'])" "$src")"
work="$GEN/sheets/work-$$/$a"; mkdir -p "$work"; cp "$src" "$work/🔣️.json"
png="$("$W/q.sh" 1000 600 "$T14" "$B" r3-2d.ts --file "$work/🔣️.json" --scale "$s" --pose "$pose" 2>&1 | tail -1)"
[[ -f "$png" ]] && mv "$png" "$out" && echo "$out" || echo "no sheet: $png"
rm -rf "$GEN/sheets/work-$$"
