#!/bin/zsh
# usage: vt-all.sh <phase>
T="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/tsl-tools"
G="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/🗑️generated"
O="/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os"
P="$1"; : > "$G/tsl-vt-$P-status.txt"
$T/vt.sh $P os "$O/📦️packages/🟦️typescript" "$O/🧪️tests/🎚️config/🟦️.ts"
$T/vt.sh $P windowkits "$O/🔨️modules/🔌️plugin/🪟️window-kits/📦️packages/🦀️rust" "$O/🔨️modules/🔌️plugin/🪟️window-kits/🧪️tests/🎚️config/🟦️.ts"
$T/vt.sh $P pluginstore "$O/🔨️modules/🔌️plugin/🏪️store" "$O/🔨️modules/🔌️plugin/🏪️store/🧪️tests/🎚️config/🟦️.ts"
echo DONE >> "$G/tsl-vt-$P-status.txt"
