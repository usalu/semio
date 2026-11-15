#!/bin/sh
# usage: verify-export.sh  exports the five verification assets (GLB all scenes, FBX per clip, scale and view, model JSON, ZIP) under 🗑️generated/verify/models
ROOT=/Users/ueli/Documents/semio
T="$ROOT/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE"
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
cd "$T" && exec nice -n 10 "$BUN" verify-export.ts --out "$T/🗑️generated/verify/models" --asset jogger,city-car,patio-heater,stair-flight,cooking-pot --view light-solid,thermal,forces --lang en
