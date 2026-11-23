#!/usr/bin/env bash
# 🖼️ Draws the three 2D views of one instant with the product pipeline (r3-2d.ts of the 3D ticket) and moves the picture to 🗑️generated/tv1: `tv1-draw.sh <category>/<slug>|--file <json> <scale> [--pose ID | --clip ID --t S]`; prints the path of the PNG.
set -euo pipefail
here="$(cd "$(dirname "$0")" && pwd)"
tickets="$(cd "$here/../../../.." && pwd)"
t14="$tickets/🌙️11/☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE"
out="$here/../🗑️generated/tv1/draw"
bun="$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)"
mkdir -p "$out"
if [ "$1" = "--file" ]; then spec=(--file "$2"); tag="after"; shift 2; else spec=("$1"); tag="before"; shift; fi
scale="$1"; shift
made="$(cd "$t14" && timeout 600 "$bun" r3-2d.ts "${spec[@]}" --scale "$scale" --views light-solid "$@" | grep -o '/.*\.png' | tail -1)"
target="$out/$tag-$(basename "$made")"
mv "$made" "$target"
echo "$target"
