#!/bin/zsh
# 👀️ The look.sh of MR through the memory queue: the product 2D sheet of an asset from the catalog file (TAG=after, default) or from the backup (TAG=before), default pose or POSE=<id>, into $GEN/sheets: look.sh <id> <scale>…
source "${0:A:h}/env.sh"
T14="$R/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE"
a="$1"; shift; tag="${TAG:-after}"
out="$GEN/sheets"; work="$GEN/sheets/work-$tag/$a"; mkdir -p "$work"
if [[ "$tag" == before ]]; then cp "${BEFORE_DIR:-$GEN/backup}/$a.json" "$work/🔣️.json"; else cp "$S/🗂️catalog/👤️humans/$a/🔣️.json" "$work/🔣️.json"; fi
pose="${POSE:-$(python3 -c "import json,sys; print(json.load(open(sys.argv[1]))['asset']['presentation']['defaultPose'])" "$work/🔣️.json")}"
for s in "$@"; do
  png="$("$W/cap.sh" 800 300 "$T14" "$B" r3-2d.ts --file "$work/🔣️.json" --scale "$s" --pose "$pose" 2>&1 | tail -1)"
  [[ -f "$png" ]] && mv "$png" "$out/$a.s$s.$pose.$tag.png" && echo "$out/$a.s$s.$pose.$tag.png" || echo "no sheet: $png"
done
rm -rf "$work"
