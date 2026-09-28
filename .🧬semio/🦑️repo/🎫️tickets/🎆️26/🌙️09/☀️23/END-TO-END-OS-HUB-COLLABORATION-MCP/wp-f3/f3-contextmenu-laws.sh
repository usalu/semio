#!/bin/zsh
# 🧪️ F3 session 14c — ContextMenu closed-reset fix: tsc of the shell closure (the shell imports ContextMenu) and the ui-react
# ContextMenu component test. Run inside the native lane. usage: zsh f3-contextmenu-laws.sh <tag>
TAG="${1:?tag}"
OUT=/Users/ueli/Documents/semio/.tmp-ticket/wp-f3/generated
UI="/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/📦️packages/🟦️typescript"
cd "$UI" && nice -n 15 bunx vitest run --config "../../🧪️tests/🎚️config/🟦️.ts" "🖱️ContextMenu" > "$OUT/test-contextmenu-$TAG.txt" 2>&1; echo "rc=$?" >> "$OUT/test-contextmenu-$TAG.txt"
cd /Users/ueli/Documents/semio/.tmp-ticket/wp-f3 && nice -n 15 /Users/ueli/Documents/semio/node_modules/.bin/tsc -p tsc/tsconfig-shell.json > "$OUT/tsc-shell-$TAG.txt" 2>&1; echo "rc=$?" >> "$OUT/tsc-shell-$TAG.txt"
