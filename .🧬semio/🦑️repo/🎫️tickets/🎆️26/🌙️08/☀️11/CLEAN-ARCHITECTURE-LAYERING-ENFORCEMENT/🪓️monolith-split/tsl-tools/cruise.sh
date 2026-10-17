#!/bin/zsh
# usage: cruise.sh <phase>
cd /Users/ueli/Documents/semio
F="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
G="$F/🗑️generated"; T="$F/otc-tools"; P="$1"
: > "$G/tsl-cyc-$P-status.txt"
perl -e 'alarm 2400; exec @ARGV' -- node node_modules/dependency-cruiser/bin/dependency-cruise.mjs "🧰️framework/🛍️products/💻️os/🔨️modules" "🧰️framework/🛍️products/💻️os/🎚️config" --config "$T/nocycle.cjs" --output-type json > "$G/tsl-cyc-$P-os.json" 2> "$G/tsl-cyc-$P-os.err"
echo "os exit=$? $(date +%T)" >> "$G/tsl-cyc-$P-status.txt"
echo DONE >> "$G/tsl-cyc-$P-status.txt"
