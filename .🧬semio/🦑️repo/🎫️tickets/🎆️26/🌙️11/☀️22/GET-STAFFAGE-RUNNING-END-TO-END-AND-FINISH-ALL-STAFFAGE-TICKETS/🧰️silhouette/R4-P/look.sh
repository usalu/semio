#!/bin/zsh
# 👀️ Renders the product 2D sheet (side, front, plan; line and solid view) of a plant at a scale in its default pose with the r3-2d tool of the 3D ticket and moves the PNG to 🗑️generated/silhouette/R4-P/sheets: look.sh <tag> <id> <scale> [asset.json]   (the catalog file by default). Prints the path.
source "${0:A:h}/env.sh"
tag="$1"; id="$2"; scale="$3"; file="${4:-$CAT/$id/🔣️.json}"
T14="$R/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE"
mkdir -p "$GEN/sheets" "$GEN/work/$id"
if [[ "$file" != "$GEN/work/$id/🔣️.json" ]]; then cp "$file" "$GEN/work/$id/🔣️.json"; fi
png="$(cd "$T14" && timeout 600 "$B" r3-2d.ts --file "$GEN/work/$id/🔣️.json" --scale "$scale" 2>&1 | grep -o '/.*\.png' | tail -1)"
[[ -f "$png" ]] || { echo "no sheet for $id 1:$scale"; exit 1; }
mv "$png" "$GEN/sheets/$tag-$id-$scale.png"; echo "$GEN/sheets/$tag-$id-$scale.png"
