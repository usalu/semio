#!/bin/zsh
# 🧪️ F3 session 14c — region (2) panel body stores: tsc of the shell closure and the engine-contract/spawned-program tests,
# then vitest (engine-contract built-node/panel/reducer laws, ShellHelpers panel suites). Native lane. usage: zsh f3-panel-laws.sh <tag>
TAG="${1:?tag}"
OUT=/Users/ueli/Documents/semio/.tmp-ticket/wp-f3/generated
TSC=/Users/ueli/Documents/semio/node_modules/.bin/tsc
REACT="/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/⚛️react/📦️packages/🟦️typescript"
cd /Users/ueli/Documents/semio/.tmp-ticket/wp-f3 && nice -n 15 $TSC -p tsc/tsconfig-shell.json > "$OUT/tsc-shell-$TAG.txt" 2>&1; echo "rc=$?" >> "$OUT/tsc-shell-$TAG.txt"
cd /Users/ueli/Documents/semio/.tmp-ticket/wp-f3 && nice -n 15 $TSC -p tsc/tsconfig-history.json > "$OUT/tsc-history-$TAG.txt" 2>&1; echo "rc=$?" >> "$OUT/tsc-history-$TAG.txt"
cd "$REACT" && SEMIO_TEST_LEVEL=long nice -n 15 bunx vitest run --config "../../🧪️tests/🎚️config/🟦️.ts" "🔬️engine-contract" -t "built-node store|shell store reducer|panelTabDefinitionToNode|panel|History|leftover" > "$OUT/test-contract-panel-$TAG.txt" 2>&1; echo "rc=$?" >> "$OUT/test-contract-panel-$TAG.txt"
cd "$REACT" && SEMIO_TEST_LEVEL=long nice -n 15 bunx vitest run --config "../../🧪️tests/🎚️config/🟦️.ts" "🛠️ShellHelpers" "🪟️spawned-program-session" > "$OUT/test-shellhelpers-$TAG.txt" 2>&1; echo "rc=$?" >> "$OUT/test-shellhelpers-$TAG.txt"
