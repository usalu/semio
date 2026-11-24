#!/bin/zsh
# 🧫️ Tries the repair of one asset on a copy before the catalog file is touched: copies the catalog file to `🗑️generated/silhouette/SF/try.json`, applies `fix-<id>.ts --file` to it and prints the owned measure of every check, `validateAsset` and the verdict of the product check CLI of the copy: try.sh <id> [--volumes]
source "${0:A:h}/env.sh"
id="$1"; cp "$CAT/$id/🔣️.json" "$GEN/try.json"
cd "$W" && timeout 120 "$B" "fix-$id.ts" --file "$GEN/try.json" || exit 1
timeout 600 "$B" measure.ts "$GEN/try.json" $2 | cut -c1-260
N=4 timeout 300 "$B" validate.ts "$GEN/try.json"
cd "$S" && timeout 600 "$B" "🔨️modules/🗂️catalog/🩺️check/🟦️.ts" --file "$GEN/try.json" 2>&1 | grep -E "^(✓|✗)|UNSUITABLE|suitability-|^  [a-z-]+ " | head -8 | cut -c1-220
