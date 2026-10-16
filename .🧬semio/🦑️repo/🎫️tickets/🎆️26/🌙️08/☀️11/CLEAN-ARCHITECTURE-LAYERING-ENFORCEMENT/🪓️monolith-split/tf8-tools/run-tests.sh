#!/bin/zsh
# usage: run-tests.sh <phase>   (phase = base|after)
export LC_ALL=en_US.UTF-8
ROOT=/Users/ueli/Documents/semio
F="$ROOT/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
G="$F/🗑️generated"
R="$ROOT/🧰️framework/🛍️products/🦑️repo/🔨️modules"
PK="$R/🧪️test/📦️packages/🟦️typescript"
phase=$1
cd "$PK"
for n in test-platform fault-notices-gate mutation-caps mutation-history-gates mutation-input-declarations mutation-payload-parity; do
  d="🧪️tests/🧪️$n"
  perl -e 'alarm 1500; exec @ARGV' -- bun test "../../$d/🟦️.ts" > "$G/tf8-$phase-bun-$n.log" 2>&1
  echo "$n exit=$?" >> "$G/tf8-$phase-bun-exit.txt"
done
echo done >> "$G/tf8-$phase-bun-exit.txt"
