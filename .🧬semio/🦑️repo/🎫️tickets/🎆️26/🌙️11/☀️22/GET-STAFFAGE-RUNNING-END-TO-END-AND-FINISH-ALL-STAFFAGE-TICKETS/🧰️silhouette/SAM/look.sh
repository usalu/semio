#!/bin/zsh
# 👀️ Renders the product 2D sheet (side, front, plan; line and solid view) of an asset document at one scale in the default pose with the r3-2d tool of the 3D ticket and moves the PNG to 🗑️generated/silhouette/SAM/sheets: look.sh <tag> <id> <scale> [<asset document json>]   (without a file: the catalog asset). VIEWS=light-solid draws one row only. Prints the path of the PNG.
source "${0:A:h}/env.sh"
T14="$U/../../☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE"
tag="$1"; id="$2"; scale="$3"; file="${4:-$(ls "$S/🗂️catalog"/*/"$id"/🔣️.json)}"
out="$GEN/sheets"; work="$GEN/work/sam-$id"; mkdir -p "$out" "$work"; cp "$file" "$work/🔣️.json"
png=$(cd "$T14" && timeout 300 "$B" r3-2d.ts --file "$work/🔣️.json" --scale "$scale" ${VIEWS:+--views $VIEWS} | tail -1)
rm -rf "$work"
[[ -f "$png" ]] || { echo "no sheet: $png"; exit 1; }
mv "$png" "$out/$id.$scale.$tag.png"; echo "$out/$id.$scale.$tag.png"
