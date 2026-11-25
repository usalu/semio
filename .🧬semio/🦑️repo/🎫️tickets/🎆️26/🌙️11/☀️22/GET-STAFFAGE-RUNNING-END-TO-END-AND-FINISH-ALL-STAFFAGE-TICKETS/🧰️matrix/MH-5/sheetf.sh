#!/bin/zsh
# 🖼️ Product sheets of any trial copy beside the landed file: sheetf.sh <file.json> <tag> <pose|-> <scale>… renders into $GEN/sheets/<id>.s<scale>.<pose>.<tag>.png with the r3-2d tool of the 3D ticket (inside the worker grant or under its own).
export BATCH=MH-5
source "${0:A:h}/../MR/env.sh"
T14="$R/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE"
f="$1"; tag="$2"; pose="$3"; shift 3; a="$(python3 -c "import json,sys; print(json.load(open(sys.argv[1]))['asset']['id'])" "$f")"
[[ "$pose" == "-" ]] && pose="$(python3 -c "import json,sys; print(json.load(open(sys.argv[1]))['asset']['presentation']['defaultPose'])" "$f")"
work="$GEN/sheets/w-$tag-$$/$a"; mkdir -p "$work"; cp "$f" "$work/🔣️.json"
for s in "$@"; do
  png="$(cd "$T14" && "$W/q.sh" timeout 300 "$B" r3-2d.ts --file "$work/🔣️.json" --scale "$s" --pose "$pose" 2>&1 | tail -1)"
  [[ -f "$png" ]] && mv "$png" "$GEN/sheets/$a.s$s.$pose.$tag.png" && echo "$GEN/sheets/$a.s$s.$pose.$tag.png" || echo "no sheet: $png"
done
rm -rf "$GEN/sheets/w-$tag-$$"
