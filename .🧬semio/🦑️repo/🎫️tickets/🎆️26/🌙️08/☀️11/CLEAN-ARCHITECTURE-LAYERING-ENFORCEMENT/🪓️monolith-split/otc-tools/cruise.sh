#!/bin/zsh
# usage: cruise.sh <phase>
cd /Users/ueli/Documents/semio
F="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
G="$F/🗑️generated"; T="$F/otc-tools"; P="$1"
: > "$G/otc-$P-status.txt"
run() { local label="$1"; shift
  perl -e 'alarm 1700; exec @ARGV' -- node node_modules/dependency-cruiser/bin/dependency-cruise.mjs "$@" --config "$T/nocycle.cjs" --output-type json > "$G/otc-$P-$label.json" 2> "$G/otc-$P-$label.err"
  echo "$label exit=$? $(date +%T)" >> "$G/otc-$P-status.txt"; }
run os "🧰️framework/🛍️products/💻️os/🔨️modules"
run repo "🧰️framework/🛍️products/🦑️repo/🔨️modules"
run config "🧰️framework/🛍️products/💻️os/🎚️config"
echo DONE >> "$G/otc-$P-status.txt"
