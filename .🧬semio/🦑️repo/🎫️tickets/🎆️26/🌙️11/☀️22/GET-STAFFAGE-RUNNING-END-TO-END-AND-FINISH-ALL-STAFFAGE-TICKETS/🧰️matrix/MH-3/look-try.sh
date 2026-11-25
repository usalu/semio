#!/bin/zsh
# 👁️ Renders the product 2D sheet (side, front, plan; line and solid) of a dry-run document $GEN/try/<id>.json (TAG=try) or of the backup (TAG=before) at the given scales, in the default pose or POSE=<id>, into $GEN/sheets: look-try.sh <id> <scale>…
export BATCH=MH-3
source "${0:A:h}/../MR/env.sh"
B="${0:A:h}/qbun"
T14="$R/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE"
a="$1"; shift; tag="${TAG:-try}"; folder=try; [[ "$tag" == before ]] && folder=backup
out="$GEN/sheets"; work="$out/work-$tag-$a"; mkdir -p "$work"
cp "$GEN/$folder/$a.json" "$work/🔣️.json"
pose="${POSE:-$(python3 -c "import json,sys; print(json.load(open(sys.argv[1]))['asset']['presentation']['defaultPose'])" "$work/🔣️.json")}"
for s in "$@"; do
  png="$(cd "$T14" && QMEM=2500 QTIME=300 "$B" r3-2d.ts --file "$work/🔣️.json" --scale "$s" --pose "$pose" 2>&1 | tail -1)"
  [[ -f "$png" ]] && mv "$png" "$out/$a.s$s.$pose.$tag.png" && echo "$out/$a.s$s.$pose.$tag.png" || echo "no sheet: $png"
done
