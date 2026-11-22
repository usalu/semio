#!/bin/sh
# 🖼️ PW-08 previews: the product's own 2D drawings (r3-2d) of a built candidate at the given scales, printing the PNG paths.
HERE="$(cd "$(dirname "$0")" && pwd)"
T3="$HERE/../../../../☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE"
ID="$1"; shift
cd "$T3" && for s in "$@"; do repo tools exec bun -- r3-2d.ts --file "$HERE/../../🗑️generated/PW-08/$ID/🔣️.json" --scale "$s" 2>&1 | grep -e png -e rror; done
