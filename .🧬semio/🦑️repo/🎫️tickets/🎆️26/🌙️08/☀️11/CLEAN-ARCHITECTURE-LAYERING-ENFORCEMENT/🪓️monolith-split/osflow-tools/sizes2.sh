#!/bin/zsh
# 🧮️ lib and test closure rows of the planned crates. usage: sizes2.sh <node> ...
FL=/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow
cd "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/osflow-tools"
for n in "$@"; do
  for mode in lib test; do
    if [ $mode = test ]; then x=test; else x=; fi
    printf "%s %s: " "$n" $mode
    EXTRA=$x TOTAL_ONLY=1 bun closure.ts "$FL/$n/🦀️.rs" "$FL/$n" 2>&1 | tail -1
  done
done
