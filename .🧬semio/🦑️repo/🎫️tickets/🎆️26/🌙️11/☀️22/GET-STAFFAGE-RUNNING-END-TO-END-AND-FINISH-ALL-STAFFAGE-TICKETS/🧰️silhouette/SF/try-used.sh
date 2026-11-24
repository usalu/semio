#!/bin/zsh
# 🧪️ The owned measure of every check, `validateAsset` and the verdict of the product check CLI for the generator output `assets/<id>/🔣️.json` of a used-object collection ticket, before it is shipped: try-used.sh <id> [--volumes]
source "${0:A:h}/env.sh"
id="$1"; src=$(ls -d "$R/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22"/USED-*/assets/"$id" | head -1)/🔣️.json
cd "$W" && timeout 600 "$B" measure.ts "$src" $2 | cut -c1-260
N=4 timeout 300 "$B" validate.ts "$src"
cd "$S" && timeout 600 "$B" "🔨️modules/🗂️catalog/🩺️check/🟦️.ts" --file "$src" 2>&1 | grep -E "^(✓|✗)|suitability-|^  [a-z-]+ " | head -8 | cut -c1-220
