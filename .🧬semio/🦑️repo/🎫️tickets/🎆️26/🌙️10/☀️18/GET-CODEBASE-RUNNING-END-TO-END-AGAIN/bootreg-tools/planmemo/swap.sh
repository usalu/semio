#!/bin/sh
# 🔀 Mirror brf on the live capture/launcher/facade/unit-cache schema (base) or on batch planmemo (batch); the memo module and its law stay: swap.sh base|batch
HERE=$(cd "$(dirname "$0")" && pwd); M=/var/folders/pm/zdg449hs6r33w_2gtsn23byc0000gn/T/sm/brf
for r in "🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🕸️units/📥️capture/🟦️.ts" "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🦀️cargo/🚪️launcher/🟦️.ts" "🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/🕸️units/📥️capture/🟦️.ts" "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache/🧬️schema/🔣️.json"; do cp "/Users/ueli/Documents/semio/$r" "$M/$r"; done
[ "$1" = batch ] && python3 -I "$HERE/edit.py" "$M" apply; exit 0
