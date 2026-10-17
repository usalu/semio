#!/bin/zsh
# usage: worker-cruise.sh <phase>
cd /Users/ueli/Documents/semio
F="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
G="$F/🗑️generated"; T="$F/otc-tools"; P="$1"
perl -e 'alarm 1500; exec @ARGV' -- node node_modules/dependency-cruiser/bin/dependency-cruise.mjs "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/👷️worker" "🧰️framework/🛍️products/💻️os/🧪️tests/📚️worker-law-suites" "🧰️framework/🛍️products/💻️os/🧪️tests/🧰️worker-law-fakes" "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🧪️tests/🔬️backbone-parity" --config "$T/nocycle.cjs" --output-type err > "$G/tsl-cyc-$P-worker.txt" 2>&1
echo "worker exit=$? $(date +%T)" >> "$G/tsl-cyc-$P-status.txt"
