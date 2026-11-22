#!/bin/zsh
# 🩺 Runs the fast validator on the candidate; extra arguments are passed through (for example --volumes --scale 50 --rules silhouette).
K="${0:A:h}"
T="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE"
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
python3 "$K/author-double-bass.py" > /dev/null || exit 1
cd "$T" && $BUN r3-validate.ts --file "$K/🔣️.json" "$@"
