#!/bin/zsh
# usage: vtchain.sh <phase> -> os (node) then mcp registry composition shell (node) then dev (bun)
. /Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/tso-tools/env.sh
PH="$1"
"$T/vitest.sh" "$PH-os" "🧰️framework/🛍️products/💻️os/📦️packages/🟦️typescript" "../../🧪️tests/🎚️config/🟦️.ts" > "$G/tso-run-$PH-os.out" 2>&1
"$T/vtall.sh" "$PH" mcp registry composition shell
TSO_RUNTIME=bun "$T/vtall.sh" "$PH-b" dev
echo done > "$G/tso-vtchain-$PH.done"
