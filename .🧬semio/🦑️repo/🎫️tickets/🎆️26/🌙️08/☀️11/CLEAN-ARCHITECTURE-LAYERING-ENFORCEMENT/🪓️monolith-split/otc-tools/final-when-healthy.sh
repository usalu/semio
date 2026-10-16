#!/bin/zsh
# polls the health probe every 60 s (at most 90 minutes), then runs the dev, registry and mcp vitest configs as phase "final"
T="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/otc-tools"
G="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/🗑️generated"
O="/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os"
: > "$G/otc-final-status.txt"
for i in $(seq 1 90); do
  out=$($T/healthy.sh)
  echo "$(date +%T) try $i: ${out##*$'\n'}" >> "$G/otc-final-status.txt"
  if [[ "$out" == *healthy* ]]; then
    : > "$G/otc-vt-final-status.txt"
    $T/vt.sh final dev "$O/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript" "$O/🔨️modules/🧑‍💻dev/🧪️tests/🎚️config/🟦️.ts"
    $T/vt.sh final registry "$O/🔨️modules/🔌️plugin/📇️registry/🧪️tests" "$O/🔨️modules/🔌️plugin/📇️registry/🧪️tests/🎚️config/🟦️.ts"
    $T/vt.sh final mcp "$O/🔨️modules/🌉️mcp/📦️packages/🟦️typescript" "$O/🔨️modules/🌉️mcp/🧪️tests/🎚️config/🟦️.ts" "resolvemcpbinarypath"
    echo DONE >> "$G/otc-vt-final-status.txt"
    echo "ran final at $(date +%T)" >> "$G/otc-final-status.txt"
    exit 0
  fi
  sleep 60
done
echo "gave up" >> "$G/otc-final-status.txt"
