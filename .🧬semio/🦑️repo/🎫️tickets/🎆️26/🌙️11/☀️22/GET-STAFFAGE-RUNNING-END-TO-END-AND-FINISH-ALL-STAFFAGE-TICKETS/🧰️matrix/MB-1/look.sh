#!/bin/zsh
# 👀️ Renders the product 2D sheet (side, front, plan; line and solid view) of an asset at scales in its default pose with the r3-2d tool of the 3D ticket into 🗑️generated/matrix/MB-1/sheets/<id>/<tag>.<scale>.png and prints the paths: look.sh <tag> <id> <scale>…
source "${0:A:h}/env.sh"
T14="$U/../../☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE"
tag="$1"; id="$2"; shift 2
file="$(file_of "$id")"
pose="$(python3 -c 'import json,sys; print(json.load(open(sys.argv[1]))["asset"]["presentation"]["defaultPose"])' "$file")"
out="$GEN/sheets/$id"; work="$out/work/$id"; mkdir -p "$work"; cp "$file" "$work/🔣️.json"
cd "$T14" || exit 1
for scale in "$@"; do
  png=$(timeout 300 "$B" r3-2d.ts --file "$work/🔣️.json" --scale "$scale" --pose "${POSE:-$pose}" | tail -1)
  [[ -f "$png" ]] || { echo "no sheet: $png"; continue; }
  mv "$png" "$out/$tag.$scale.png"; sips -Z ${SIZE:-1000} "$out/$tag.$scale.png" --out "$out/$tag.$scale.s.png" > /dev/null; echo "$out/$tag.$scale.s.png"
done
rm -rf "$out/work"
