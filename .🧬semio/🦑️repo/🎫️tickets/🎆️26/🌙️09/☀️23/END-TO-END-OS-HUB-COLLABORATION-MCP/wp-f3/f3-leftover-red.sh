#!/bin/zsh
# 🔴️ F3 session 14c — red proof of the leftover structural-sharing law: reverse the World3dHost hunks (patch -R), run the
# engine-contract "leftover" laws, re-apply, run them again. Run inside the native lane. usage: zsh f3-leftover-red.sh <tag>
TAG="${1:?tag}"
OUT=/Users/ueli/Documents/semio/.tmp-ticket/wp-f3/generated
PATCH=/Users/ueli/Documents/semio/.tmp-ticket/wp-f3/patches/f3-leftover-structural-sharing.diff
REACT="/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/⚛️react/📦️packages/🟦️typescript"
cd /Users/ueli/Documents/semio && patch -p1 -R < "$PATCH" > "$OUT/red-leftover-patch-$TAG.txt" 2>&1 || exit 1
cd "$REACT" && SEMIO_TEST_LEVEL=long nice -n 15 bunx vitest run --config "../../🧪️tests/🎚️config/🟦️.ts" "🔬️engine-contract" -t "leftover" > "$OUT/test-leftover-red-$TAG.txt" 2>&1; echo "rc=$?" >> "$OUT/test-leftover-red-$TAG.txt"
cd /Users/ueli/Documents/semio && patch -p1 < "$PATCH" >> "$OUT/red-leftover-patch-$TAG.txt" 2>&1; echo "reapply rc=$?" >> "$OUT/red-leftover-patch-$TAG.txt"
cd "$REACT" && SEMIO_TEST_LEVEL=long nice -n 15 bunx vitest run --config "../../🧪️tests/🎚️config/🟦️.ts" "🔬️engine-contract" -t "leftover" > "$OUT/test-leftover-$TAG.txt" 2>&1; echo "rc=$?" >> "$OUT/test-leftover-$TAG.txt"
