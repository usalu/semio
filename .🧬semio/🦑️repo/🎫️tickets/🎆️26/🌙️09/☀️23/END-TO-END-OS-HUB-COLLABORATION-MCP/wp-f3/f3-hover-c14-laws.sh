#!/bin/zsh
# 🧪️ F3 session 14c — laws of the hover re-render fix set (leftover overlay structural sharing + per-pane snapshot in
# World3dHost, ContextMenuController closed-reset bail-out): tsc of the shell closure, the two test files, then vitest
# (engine-contract "leftover" laws, ui-react ContextMenu component test). Run inside the native lane. usage: zsh f3-hover-c14-laws.sh <tag>
TAG="${1:?tag}"
OUT=/Users/ueli/Documents/semio/.tmp-ticket/wp-f3/generated
TSC=/Users/ueli/Documents/semio/node_modules/.bin/tsc
REACT="/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/⚛️react/📦️packages/🟦️typescript"
UI="/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/📦️packages/🟦️typescript"
cd /Users/ueli/Documents/semio/.tmp-ticket/wp-f3 && nice -n 15 $TSC -p tsc/tsconfig-shell.json > "$OUT/tsc-shell-$TAG.txt" 2>&1; echo "rc=$?" >> "$OUT/tsc-shell-$TAG.txt"
cd /Users/ueli/Documents/semio/.tmp-ticket/wp-f3 && nice -n 15 $TSC -p tsc/tsconfig-shell-tests.json > "$OUT/tsc-shell-tests-$TAG.txt" 2>&1; echo "rc=$?" >> "$OUT/tsc-shell-tests-$TAG.txt"
cd /Users/ueli/Documents/semio/.tmp-ticket/wp-f3 && nice -n 15 $TSC -p tsc/tsconfig-contextmenu.json > "$OUT/tsc-contextmenu-$TAG.txt" 2>&1; echo "rc=$?" >> "$OUT/tsc-contextmenu-$TAG.txt"
cd "$REACT" && SEMIO_TEST_LEVEL=long nice -n 15 bunx vitest run --config "../../🧪️tests/🎚️config/🟦️.ts" "🔬️engine-contract" -t "leftover" > "$OUT/test-leftover-$TAG.txt" 2>&1; echo "rc=$?" >> "$OUT/test-leftover-$TAG.txt"
cd "$UI" && nice -n 15 bunx vitest run --config "../../🧪️tests/🎚️config/🟦️.ts" "🖱️ContextMenu" > "$OUT/test-contextmenu-$TAG.txt" 2>&1; echo "rc=$?" >> "$OUT/test-contextmenu-$TAG.txt"
