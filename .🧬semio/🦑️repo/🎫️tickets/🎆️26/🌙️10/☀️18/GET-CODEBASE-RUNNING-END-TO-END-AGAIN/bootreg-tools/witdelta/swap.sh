#!/bin/sh
# 🔀 Mirror brf on the live witness code (base) or on batch witdelta (batch); law, fixture and oracle stay the batch's: swap.sh base|batch
HERE=$(cd "$(dirname "$0")" && pwd); M=/var/folders/pm/zdg449hs6r33w_2gtsn23byc0000gn/T/sm/brf; D="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🛠️preparation/👁️witness"
for r in "$D/🟦️.ts" "$D/🧬️schema/🔣️.json"; do cp "/Users/ueli/Documents/semio/$r" "$M/$r"; done
[ "$1" = batch ] && python3 -I "$HERE/edit.py" "$M" apply; exit 0
