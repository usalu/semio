#!/bin/sh
# 🔀 Puts mirror brw on the live base (base) or on batch selnear (batch) for red/green suite runs: swap.sh base|batch [mirror-root]
HERE=$(cd "$(dirname "$0")" && pwd); M=${2:-/var/folders/pm/zdg449hs6r33w_2gtsn23byc0000gn/T/sm/brw}; L="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
for r in "$L/🗂️workspaces/🦀️cargo/🟦️.ts" "$L/🏃️process/🦀️cargo/🎟️admission/🟦️.ts" "$L/🏃️process/🦀️cargo/🧹️lint/🟦️.ts"; do cp "/Users/ueli/Documents/semio/$r" "$M/$r"; done
[ "$1" = batch ] && python3 -I "$HERE/edit.py" "$M" apply
exit 0
