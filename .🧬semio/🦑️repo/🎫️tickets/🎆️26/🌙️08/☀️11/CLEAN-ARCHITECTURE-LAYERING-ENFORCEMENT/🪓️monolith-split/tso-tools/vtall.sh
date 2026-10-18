#!/bin/zsh
# usage: vtall.sh <phase> [only-label...] -> runs mcp registry composition shell dev (os is run separately as <phase>-os)
. /Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/tso-tools/env.sh
PH="$1"; shift
O="🧰️framework/🛍️products/💻️os"
typeset -A CWD CFG
CWD[mcp]="$O/🔨️modules/🌉️mcp/📦️packages/🟦️typescript";  CFG[mcp]="../../🧪️tests/🎚️config/🟦️.ts"
CWD[registry]="$O/🔨️modules/🔌️plugin/📇️registry";  CFG[registry]="🧪️tests/🎚️config/🟦️.ts"
CWD[composition]="$O/🔨️modules/🔌️plugin/📇️registry/🧪️tests/🧩️composition"; CFG[composition]="🎚️config/🟦️.ts"
CWD[shell]="$O/🔨️modules/🖥️shell/📦️packages/🟦️typescript"; CFG[shell]="../../🧪️tests/🎚️config/🟦️.ts"
CWD[dev]="$O/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript"; CFG[dev]="../../🧪️tests/🎚️config/🟦️.ts"
LABELS=(mcp registry composition shell dev); [[ $# -gt 0 ]] && LABELS=($@)
for l in $LABELS; do
  "$T/vitest.sh" "$PH-$l" "${CWD[$l]}" "${CFG[$l]}" > "$G/tso-run-$PH-$l.out" 2>&1
  echo "$l $(date +%T)" >> "$G/tso-vtall-$PH.status"
done
