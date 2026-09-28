#!/bin/zsh
# 🧪️ F3 — the shell-closure laws of this session's host TS fixes: tsc of the two test files, then vitest (engine-contract
# "shell store reducer" + "built-node store reloads" blocks, tutorial-bridge). Run inside the native lane. usage: zsh f3-shell-tests.sh <tag>
TAG="${1:?tag}"
OUT=/Users/ueli/Documents/semio/.tmp-ticket/wp-f3/generated
REACT="/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/⚛️react/📦️packages/🟦️typescript"
cd /Users/ueli/Documents/semio/.tmp-ticket/wp-f3 && nice -n 15 /Users/ueli/Documents/semio/node_modules/.bin/tsc -p tsc/tsconfig-shell-tests.json > "$OUT/tsc-shell-tests-$TAG.txt" 2>&1; echo "rc=$?" >> "$OUT/tsc-shell-tests-$TAG.txt"
cd "$REACT" && nice -n 15 bunx vitest run "../../../../🧪️tests/🔬️engine-contract/🟦️.ts" -t "shell store reducer|built-node store" > "$OUT/test-shell-reducer-$TAG.txt" 2>&1; echo "rc=$?" >> "$OUT/test-shell-reducer-$TAG.txt"
cd "$REACT" && nice -n 15 bunx vitest run "../../../../🧪️tests/🎥️tutorial-bridge/🟦️.ts" > "$OUT/test-tutorial-bridge-$TAG.txt" 2>&1; echo "rc=$?" >> "$OUT/test-tutorial-bridge-$TAG.txt"
DEV="/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript"
cd "$DEV" && nice -n 15 bunx vitest run --config "../../🧪️tests/🎚️config/🟦️.ts" "🥾️boot-budget" > "$OUT/test-boot-budget-$TAG.txt" 2>&1; echo "rc=$?" >> "$OUT/test-boot-budget-$TAG.txt"
