#!/bin/zsh
# 🧪️ F3 session 14c — region (1) history store + live panel tree: tsc of the shell closure, the shell/spawned-program tests and
# the ui Panel/ContextMenu/Layout tests, then vitest (spawned-program-session, ui Panel/Layout/ContextMenu). Native lane.
# usage: zsh f3-history-laws.sh <tag>
TAG="${1:?tag}"
OUT=/Users/ueli/Documents/semio/.tmp-ticket/wp-f3/generated
TSC=/Users/ueli/Documents/semio/node_modules/.bin/tsc
REACT="/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/⚛️react/📦️packages/🟦️typescript"
UI="/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/📦️packages/🟦️typescript"
cd /Users/ueli/Documents/semio/.tmp-ticket/wp-f3 && nice -n 15 $TSC -p tsc/tsconfig-shell.json > "$OUT/tsc-shell-$TAG.txt" 2>&1; echo "rc=$?" >> "$OUT/tsc-shell-$TAG.txt"
cd /Users/ueli/Documents/semio/.tmp-ticket/wp-f3 && nice -n 15 $TSC -p tsc/tsconfig-history.json > "$OUT/tsc-history-$TAG.txt" 2>&1; echo "rc=$?" >> "$OUT/tsc-history-$TAG.txt"
cd /Users/ueli/Documents/semio/.tmp-ticket/wp-f3 && nice -n 15 $TSC -p tsc/tsconfig-panel.json > "$OUT/tsc-panel-$TAG.txt" 2>&1; echo "rc=$?" >> "$OUT/tsc-panel-$TAG.txt"
cd "$REACT" && SEMIO_TEST_LEVEL=long nice -n 15 bunx vitest run --config "../../🧪️tests/🎚️config/🟦️.ts" "🪟️spawned-program-session" > "$OUT/test-spawned-program-$TAG.txt" 2>&1; echo "rc=$?" >> "$OUT/test-spawned-program-$TAG.txt"
cd "$UI" && nice -n 15 bunx vitest run --config "../../🧪️tests/🎚️config/🟦️.ts" "🖼️Panel" "📐️Layout" "🖱️ContextMenu" > "$OUT/test-ui-panel-$TAG.txt" 2>&1; echo "rc=$?" >> "$OUT/test-ui-panel-$TAG.txt"
