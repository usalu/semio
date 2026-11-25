#!/bin/zsh
# 👁️ The product sheet of a document that is not in the catalog (a dry run): peek.sh <file.json> <name> <scale> [pose]   writes $GEN/sheets/<name>.s<scale>.<pose>.peek.png with the r3-2d tool that look.sh uses.
source "${0:A:h}/env.sh"
T14="$R/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE"
work="$GEN/sheets/peek-$2"; mkdir -p "$work"; cp "$1" "$work/🔣️.json"
pose="${4:-$(python3 -c "import json,sys; print(json.load(open(sys.argv[1]))['asset']['presentation']['defaultPose'])" "$1")}"
png="$("$W/cap.sh" 800 300 "$T14" "$B" r3-2d.ts --file "$work/🔣️.json" --scale "$3" --pose "$pose" 2>&1 | tail -1)"
[[ -f "$png" ]] && mv "$png" "$GEN/sheets/$2.s$3.$pose.peek.png" && echo "$GEN/sheets/$2.s$3.$pose.peek.png" || echo "no sheet: $png"
rm -rf "$work"
