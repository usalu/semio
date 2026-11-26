#!/bin/zsh
# 👀️ Product sheets of asset documents inside a grant that is already held (after ML-2/sheets.sh): sheets.sh <label> <file.json> <scale> <pose>… writes 🗑️generated/matrix/ML-4/sheets/<asset>.s<scale>.<pose>.<label>.png with the r3-2d tool of the 3D ticket (side, front, plan; line and solid).
source "${0:A:h}/env.sh"
T14="$R/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE"
label="$1"; file="$2"; scale="$3"; shift 3
work="$GEN/sheets/work-$label"; mkdir -p "$work"; cp "$file" "$work/🔣️.json"; id="$(python3 -c "import json,sys; print(json.load(open(sys.argv[1]))['asset']['id'])" "$work/🔣️.json")"
for pose in "$@"; do
  png="$(cd "$T14" && timeout 600 "$B" r3-2d.ts --file "$work/🔣️.json" --scale "$scale" --pose "$pose" 2>&1 | tail -1)"
  [[ -f "$png" ]] && mv "$png" "$GEN/sheets/$id.s$scale.$pose.$label.png" && echo "$GEN/sheets/$id.s$scale.$pose.$label.png" || echo "no sheet: $png"
done
rm -rf "$work"
