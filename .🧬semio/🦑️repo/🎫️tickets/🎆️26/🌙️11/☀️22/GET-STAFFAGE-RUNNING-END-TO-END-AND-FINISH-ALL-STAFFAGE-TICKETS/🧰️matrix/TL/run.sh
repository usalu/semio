#!/bin/zsh
# 🧪️ Runs the thermal-legend suite through the capacity queue (1950 MB).
cd /Users/ueli/Documents/semio
B=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
cd 🎓️teaching/🏛️architecture/🧍️staffage/📦️packages/🟦️typescript || exit 1
cd /Users/ueli/Documents/semio
repo capacity run --memory=1950 -- sh -c 'cd 🎓️teaching/🏛️architecture/🧍️staffage/📦️packages/🟦️typescript && timeout 1800 '"$B"' x vitest run --reporter verbose --config ../../🧪️tests/🎚️config/🟦️.ts ../../🧪️tests/🎨️thermal-legend/🟦️.ts'
