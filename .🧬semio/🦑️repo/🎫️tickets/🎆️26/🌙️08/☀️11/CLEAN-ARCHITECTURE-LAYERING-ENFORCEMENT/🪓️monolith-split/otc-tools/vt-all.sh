#!/bin/zsh
# usage: vt-all.sh <phase>
T="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/otc-tools"
G="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/🗑️generated"
O="/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os"
P="$1"; : > "$G/otc-vt-$P-status.txt"
$T/vt.sh $P os "$O/📦️packages/🟦️typescript" "$O/🧪️tests/🎚️config/🟦️.ts"
$T/vt.sh $P dev "$O/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript" "$O/🔨️modules/🧑‍💻dev/🧪️tests/🎚️config/🟦️.ts"
$T/vt.sh $P windowkits "$O/🔨️modules/🔌️plugin/🪟️window-kits/📦️packages/🦀️rust" "$O/🔨️modules/🔌️plugin/🪟️window-kits/🧪️tests/🎚️config/🟦️.ts"
$T/vt.sh $P shell "$O/🔨️modules/🖥️shell/📦️packages/🟦️typescript" "$O/🔨️modules/🖥️shell/🧪️tests/🎚️config/🟦️.ts"
$T/vt.sh $P r3f "$O/🔨️modules/♾️infinite/🌍️world/🎨️r3f/📦️packages/🟦️typescript" "$O/🔨️modules/♾️infinite/🌍️world/🎨️r3f/🧪️tests/🎚️config/🟦️.ts"
$T/vt.sh $P registry "$O/🔨️modules/🔌️plugin/📇️registry/🧪️tests" "$O/🔨️modules/🔌️plugin/📇️registry/🧪️tests/🎚️config/🟦️.ts"
$T/vt.sh $P mcp "$O/🔨️modules/🌉️mcp/📦️packages/🟦️typescript" "$O/🔨️modules/🌉️mcp/🧪️tests/🎚️config/🟦️.ts" "${MCP_FILTER:-../../🟦️.ts}"
echo DONE >> "$G/otc-vt-$P-status.txt"
