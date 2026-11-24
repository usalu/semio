#!/bin/zsh
# 👀️ Renders the product 2D sheet (side, front, plan; line and solid view) of one asset document at scales in its default pose with the r3-2d tool of the 3D ticket and moves the PNGs to `🗑️generated/silhouette/SF/look/`: look.sh <tag> <asset document json> <id> <scale>…   Prints the paths.
source "${0:A:h}/env.sh"
T14="$R/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE"
tag="$1"; file="$2"; id="$3"; shift 3
out="$GEN/look"; work="$out/work/$id"; mkdir -p "$work"; cp "$file" "$work/🔣️.json"
pose=$(python3 -c "import json,sys; print(json.load(open(sys.argv[1]))['asset']['presentation']['defaultPose'])" "$file")
cd "$T14" || exit 1
for scale in "$@"; do
  png=$(timeout 300 "$B" r3-2d.ts --file "$work/🔣️.json" --scale "$scale" --pose "$pose" | tail -1)
  [ -f "$png" ] || { echo "no sheet: $png"; continue; }
  mv "$png" "$out/$id-$scale-$tag.png"; echo "$out/$id-$scale-$tag.png"
done
rm -rf "$out/work"
