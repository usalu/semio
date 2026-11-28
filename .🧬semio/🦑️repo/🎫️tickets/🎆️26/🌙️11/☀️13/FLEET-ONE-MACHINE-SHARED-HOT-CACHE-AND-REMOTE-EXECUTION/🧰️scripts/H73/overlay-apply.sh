#!/bin/sh
# Rebuilds the scratch overlay from the live tree and applies the whole staged patch to it: overlay-apply.sh OUT
set -e
H="$(cd "$(dirname "$0")" && pwd)"; OUT="$1"
nice -n 10 python3 "$H/overlay.py" "$OUT" >/dev/null
L="$OUT/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"; UC="$L/⚡️caching/🦀️cargo/🗃️unit-cache"
rsync -a "$H/stage/🌱️seed/" "$UC/🛰️fleet/🌱️seed/"
python3 "$H/schema_edits.py" "$UC"
python3 "$H/patch.py" apply "$UC" "$L/🏃️process/🦀️cargo/🚪️launcher/🟦️.ts"
ln -sfn /Users/ueli/Documents/semio/node_modules "$OUT/node_modules"
