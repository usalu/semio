#!/bin/zsh
# usage: bun-tests.sh <phase>   runs the four grammar source-text readers under bun test, logs in $G/pfs-bun-<phase>-<n>.log
. "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/pfs-tools/env.sh"
PH=$1; PRT="$PR/📓️print"
i=0
for f in "$PRT/🧪️tests/🧬️native-chart-grammar/🎨️paint/🧪️tests/🟦️.ts" "$PRT/🔨️modules/🖨️tectonic-template-compilation/📥️refusal/🧪️tests/🟦️.ts" "$PRT/📦️artifacts/📖️pdf/🧪️tests/🟦️.ts" "$PRT/📦️artifacts/🎛️operation/🧪️tests/📥️grammar/🟦️.ts"; do
  i=$((i+1)); cd "$(dirname "$f")"
  perl -e 'alarm 600; exec @ARGV' -- bun test "$f" > "$G/pfs-bun-$PH-$i.log" 2>&1
  echo "exit=$?" >> "$G/pfs-bun-$PH-$i.log"
done
echo done > "$G/pfs-bun-$PH-DONE"
