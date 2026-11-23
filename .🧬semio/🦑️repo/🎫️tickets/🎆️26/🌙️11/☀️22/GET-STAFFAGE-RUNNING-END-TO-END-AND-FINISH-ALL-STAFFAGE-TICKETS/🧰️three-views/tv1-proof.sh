#!/usr/bin/env bash
# 🧾️ The final proof lines of one catalog asset, one tool after the other: three-views sweep, validator (schema and validateAsset on the catalog file), product check CLI and the silhouette measure: `tv1-proof.sh <id>`.
set -uo pipefail
here="$(cd "$(dirname "$0")" && pwd)"
tickets="$(cd "$here/../../../.." && pwd)"
t14="$tickets/🌙️11/☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE"
staffage="$(cd "$here/../../../../../../../.." && pwd)/🎓️teaching/🏛️architecture/🧍️staffage"
bun="$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)"
file="$(ls "$staffage"/🗂️catalog/*/"$1"/🔣️.json)"
(cd "$here" && timeout 900 "$bun" tv1-sweep.ts "$1" | tail -3 | cut -c1-400)
(cd "$t14" && timeout 900 "$bun" r3-validate.ts --file "$file" --volumes 2>&1 | tail -3 | cut -c1-400)
(cd "$staffage" && timeout 600 "$bun" 🔨️modules/🗂️catalog/🩺️check/🟦️.ts --asset "$1" 2>&1 | head -1 | cut -c1-400)
(cd "$here" && timeout 900 "$bun" tv1-silhouette.ts "$1" 2>&1 | tail -5 | cut -c1-600)
