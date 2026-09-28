#!/bin/zsh
# 🔴️ F3 session 14c — red proof of the closed-menu render law: reverse the ContextMenu source hunks (patch -R), run the law,
# re-apply them. Run inside the native lane. usage: zsh f3-contextmenu-red.sh <tag>
TAG="${1:?tag}"
OUT=/Users/ueli/Documents/semio/.tmp-ticket/wp-f3/generated
PATCH=/Users/ueli/Documents/semio/.tmp-ticket/wp-f3/patches/f3-contextmenu-closed-reset.diff
UI="/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/📦️packages/🟦️typescript"
cd /Users/ueli/Documents/semio && patch -p1 -R < "$PATCH" > "$OUT/red-patch-$TAG.txt" 2>&1 || exit 1
cd "$UI" && nice -n 15 bunx vitest run --config "../../🧪️tests/🎚️config/🟦️.ts" "🖱️ContextMenu" -t "closed context menu" > "$OUT/test-contextmenu-red-$TAG.txt" 2>&1; echo "rc=$?" >> "$OUT/test-contextmenu-red-$TAG.txt"
cd /Users/ueli/Documents/semio && patch -p1 < "$PATCH" >> "$OUT/red-patch-$TAG.txt" 2>&1; echo "reapply rc=$?" >> "$OUT/red-patch-$TAG.txt"
cd "$UI" && nice -n 15 bunx vitest run --config "../../🧪️tests/🎚️config/🟦️.ts" "🖱️ContextMenu" > "$OUT/test-contextmenu-$TAG.txt" 2>&1; echo "rc=$?" >> "$OUT/test-contextmenu-$TAG.txt"
