#!/bin/zsh
# 👁️ Product 2D sheets (side, front, plan; line and solid) inside ONE grant (start it with qsh), one process per sheet, one after the other: each argument is `<id>:<source>:<scale>[:<pose>]` with source `try` ($GEN/try/<id>.json), `before` ($GEN/backup/<id>.json) or `now` (the catalog file); the PNG goes to $GEN/sheets/<id>.s<scale>.<pose>.<source>.png: QMEM=2500 qsh in-look.sh person-x:try:50 person-x:before:50:some-pose
[[ -n "$MH3_GRANT" ]] || { echo "in-look.sh runs only inside a grant: use qsh"; exit 2; }
export BATCH=MH-3
source "${0:A:h}/../MR/env.sh"
T14="$R/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE"
out="$GEN/sheets"; mkdir -p "$out"
for spec in "$@"; do
  parts=("${(@s/:/)spec}"); a="$parts[1]"; tag="$parts[2]"; s="$parts[3]"
  case "$tag" in try) file="$GEN/try/$a.json";; before) file="$GEN/backup/$a.json";; *) file="$S/🗂️catalog/👤️humans/$a/🔣️.json";; esac
  work="$out/work-$tag-$a"; mkdir -p "$work"; cp "$file" "$work/🔣️.json"
  pose="${parts[4]:-$(python3 -c "import json,sys; print(json.load(open(sys.argv[1]))['asset']['presentation']['defaultPose'])" "$work/🔣️.json")}"
  png="$(cd "$T14" && timeout 300 "$B" r3-2d.ts --file "$work/🔣️.json" --scale "$s" --pose "$pose" 2>&1 | tail -1)"
  [[ -f "$png" ]] && mv "$png" "$out/$a.s$s.$pose.$tag.png" && echo "$out/$a.s$s.$pose.$tag.png" || echo "no sheet for $spec: $png"
  rm -rf "$work"
done
