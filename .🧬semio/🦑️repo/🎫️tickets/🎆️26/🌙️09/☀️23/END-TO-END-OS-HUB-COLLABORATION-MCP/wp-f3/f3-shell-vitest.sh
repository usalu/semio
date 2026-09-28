#!/bin/zsh
# 🧪️ F3 — vitest of this session's shell laws with the react package's own config (engine-contract "shell store reducer" +
# "built-node store" blocks, tutorial-bridge). Run inside the native lane. usage: zsh f3-shell-vitest.sh <tag>
TAG="${1:?tag}"
OUT=/Users/ueli/Documents/semio/.tmp-ticket/wp-f3/generated
REACT="/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/⚛️react/📦️packages/🟦️typescript"
cd "$REACT" && nice -n 15 bunx vitest run --config "../../🧪️tests/🎚️config/🟦️.ts" "🔬️engine-contract" -t "shell store reducer|built-node store" > "$OUT/test-shell-reducer-$TAG.txt" 2>&1; echo "rc=$?" >> "$OUT/test-shell-reducer-$TAG.txt"
cd "$REACT" && nice -n 15 bunx vitest run --config "../../🧪️tests/🎚️config/🟦️.ts" "🎥️tutorial-bridge" > "$OUT/test-tutorial-bridge-$TAG.txt" 2>&1; echo "rc=$?" >> "$OUT/test-tutorial-bridge-$TAG.txt"
