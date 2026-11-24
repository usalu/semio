#!/bin/zsh
# 👀️ Renders the product 2D sheet (side, front, plan; line and solid view) of an asset document at a scale with the r3-2d tool of the 3D ticket into 🗑️generated/matrix/MF/look/<tag>.<scale>.<pose>.png and prints the path: look.sh <id|file.json> <tag> <scale> [pose]   (an id reads the catalog document)
export BATCH=MF; source "${0:A:h}/../MR/env.sh"
T14="$R/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE"
f="$1"; [[ "$f" == *.json ]] || f="$(ls "$S"/🗂️catalog/*/"$1"/🔣️.json)"
out="$GEN/look"; work="$out/work-$$/x"; mkdir -p "$work"; cp "$f" "$work/🔣️.json"
pose="${4:-$(python3 -c "import json,sys; print(json.load(open(sys.argv[1]))['asset']['presentation']['defaultPose'])" "$f")}"
png="$(cd "$T14" && timeout 300 "$B" r3-2d.ts --file "$work/🔣️.json" --scale "$3" --pose "$pose" 2>&1 | tail -1)"
rm -rf "$out/work-$$"
[[ -f "$png" ]] || { echo "no sheet: $png"; exit 1; }
mv "$png" "$out/$2.$3.$pose.png"; sips -Z ${SIZE:-1400} "$out/$2.$3.$pose.png" --out "$out/$2.$3.$pose.png" > /dev/null; echo "$out/$2.$3.$pose.png"
