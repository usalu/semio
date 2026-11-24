#!/bin/zsh
# 👀️ Renders the product 2D sheet (side, front, plan; line and solid view) of an object document at a scale in its default pose with the r3-2d tool of the 3D ticket into 🗑️generated/silhouette/R4-X/look/<id>/<tag>.<scale>.png and prints the path: look.sh <asset document json> <id> <scale> <tag>
source "${0:A:h}/env.sh"
T14="$U/../../☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE"
OUT="$GEN/look/$2"; WORK="$OUT/work/$2"; mkdir -p "$WORK"; cp "$1" "$WORK/🔣️.json"
POSE=$(python3 -c "import json,sys; print(json.load(open(sys.argv[1]))['asset']['presentation']['defaultPose'])" "$1")
cd "$T14" || exit 1
PNG=$(timeout ${LOOK_TIMEOUT:-900} "$B" r3-2d.ts --file "$WORK/🔣️.json" --scale "$3" --pose "$POSE" | tail -1)
rm -rf "$OUT/work"
[[ -f "$PNG" ]] || { echo "no sheet: $PNG"; exit 1; }
mv "$PNG" "$OUT/$4.$3.png"; sips -Z ${SIZE:-800} "$OUT/$4.$3.png" --out "$OUT/$4.$3.s.png" > /dev/null; echo "$OUT/$4.$3.s.png"
