#!/bin/zsh
# 👀️ Renders the product 2D sheet (side, front, plan; line and solid view) of an asset with the r3-2d tool of the 3D ticket, from the catalog file (TAG=after, default) or from the backup `$GEN/backup/<id>.json` (TAG=before), in the default pose or POSE=<id>, into $GEN/sheets: look.sh <id> <scale>…   (BATCH=<batch> selects the fixer folder)
source "${0:A:h}/env.sh"
T14="$R/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE"
a="$1"; shift; tag="${TAG:-after}"
out="$GEN/sheets"; work="$GEN/sheets/work-$tag/$a"; mkdir -p "$work"
if [[ "$tag" == before ]]; then cp "$GEN/backup/$a.json" "$work/🔣️.json"; else cp "$(ls "$S"/🗂️catalog/*/"$a"/🔣️.json)" "$work/🔣️.json"; fi
pose="${POSE:-$(python3 -c "import json,sys; print(json.load(open(sys.argv[1]))['asset']['presentation']['defaultPose'])" "$work/🔣️.json")}"
for s in "$@"; do
  png="$(cd "$T14" && timeout 300 "$B" r3-2d.ts --file "$work/🔣️.json" --scale "$s" --pose "$pose" 2>&1 | tail -1)"
  [[ -f "$png" ]] && mv "$png" "$out/$a.s$s.$pose.$tag.png" && echo "$out/$a.s$s.$pose.$tag.png" || echo "no sheet: $png"
done
rm -rf "$GEN/sheets/work-$tag"
