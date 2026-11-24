#!/usr/bin/env sh
# Exports the catalogue staffage of the cover axonometry as isometric light-line SVG (asset, pose, camera azimuth from ahead towards the asset's right, scale denominator) into the ticket's generated folder.
# Usage (from the repo root): sh <ticket>/cover-staffage-export.sh
set -eu
ticket="$(cd "$(dirname "$0")" && pwd)"
out="$ticket/🗑️generated/staffage"
cli="🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/📤️export/🎬️cli/🟦️.ts"
mkdir -p "$out"
while read -r asset pose azimuth scale; do
  [ -n "$asset" ] || continue
  mkdir -p "$out/$asset"
  repo tools exec bun -- "$cli" --asset "$asset" --pose "$pose" --view light-line --viewpoint axonometric --camera "$azimuth,35.2644" --scale "$scale" --format svg --lang en --out "$out/$asset" 2>&1 | tail -1
done <<LIST
${COVER_STAFFAGE:-person-drinking-coffee holding-cup -45 100
three-seat-sofa unoccupied 45 100
folding-deck-chair unfolded -45 100
domestic-cat sitting-tail-wrapped 0 100
japanese-maple summer-calm 45 200
miscanthus-grass calm 45 200
lavender-bush calm 45 100
boxwood-ball calm 45 100
potted-monstera still-air 45 100
used-floor-lamp in-use 45 100}
LIST
