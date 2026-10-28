#!/bin/sh
# 🔀 Mirror brf on its base (live + planmemo) or on batch nxmeta (batch); the new law stays: swap.sh base|batch
HERE=$(cd "$(dirname "$0")" && pwd); C=$(cd "$HERE/../.." && pwd); M=/var/folders/pm/zdg449hs6r33w_2gtsn23byc0000gn/T/sm/brf; S="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/🏗️compiler/🚪️ingress/⚙️configuration/🧪️tests/🪜️steps"
for r in "🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🕸️units/📥️capture/🟦️.ts" "🧰️framework/🔨️modules/🏃️process/🧪️testing/🦀️cargo/🟦️.ts" "$S/🔣️.json" "$S/🟦️.ts"; do cp "/Users/ueli/Documents/semio/$r" "$M/$r"; done
python3 -I "$C/bootreg-tools/planmemo/edit.py" "$M" apply >/dev/null
[ "$1" = batch ] && python3 -I "$HERE/edit.py" "$M" apply; exit 0
