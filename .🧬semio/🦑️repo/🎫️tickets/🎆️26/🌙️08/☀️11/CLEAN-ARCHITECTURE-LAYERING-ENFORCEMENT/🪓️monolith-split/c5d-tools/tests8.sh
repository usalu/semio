#!/bin/zsh
# usage: tests8.sh <label> ; bun tests that read the package 8 and 9 sources
R=/Users/ueli/Documents/semio
F="$R/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
PL="🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin"
L=$1
cd $R
i=0
for t in \
  "🧰️framework/🔨️modules/🧵️job/🏃️work/🎟️grant/🧪️tests/🟦️.ts" \
  "$PL/🛠️tool-machine/♻️retirement/🧪️tests/🟦️.ts" \
  "$PL/🧪️tests/🔬️plugin-runtime-runtime-close-budget/🟦️.ts" \
  "🧰️framework/🔨️modules/🧬️schema/🧩️composition/📥️admission/🧪️tests/🟦️.ts" \
  "$PL/🧪️tests/📦️units/🟦️.ts" ; do
  i=$((i+1))
  perl -e 'alarm 900; exec @ARGV' -- nice -n 10 bun test "./$t" > "$F/🗑️generated/c5d-ts8-$L-$i.log" 2>&1
  echo "$i $t exit=$? $(grep -E '^ *[0-9]+ (pass|fail)' "$F/🗑️generated/c5d-ts8-$L-$i.log" | tr '\n' ' ')"
done
