#!/bin/zsh
# 👀️ Renders the product 2D sheet (side, front, plan; line and solid view) of a component at one scale in its default pose with the r3-2d tool of the 3D ticket and moves the PNG to 🗑️generated/silhouette/SC/sheets/<id>/<tag>.<name>; prints the path: look.sh <tag> <id|file.json> <scale>…
source "${0:A:h}/env.sh"
T14="$U/../../☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE"
tag="$1"; arg="$2"; shift 2
file="$arg"; [[ "$arg" == *.json ]] || file="$CAT/$arg/🔣️.json"
id="$(python3 -c 'import json,sys; print(json.load(open(sys.argv[1]))["asset"]["id"])' "$file")"
pose="$(python3 -c 'import json,sys; print(json.load(open(sys.argv[1]))["asset"]["presentation"]["defaultPose"])' "$file")"
out="$GEN/sheets/$id"; work="$out/work/$id"; mkdir -p "$work"; cp "$file" "$work/🔣️.json"
cd "$T14" || exit 1
for scale in "$@"; do
  png=$(timeout 300 "$B" r3-2d.ts --file "$work/🔣️.json" --scale "$scale" --pose "$pose" | tail -1)
  [[ -f "$png" ]] || { echo "no sheet: $png"; continue; }
  mv "$png" "$out/$tag.$(basename "$png")"; echo "$out/$tag.$(basename "$png")"
done
rm -rf "$out/work"
