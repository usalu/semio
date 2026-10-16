#!/bin/zsh
# usage: cycscan.sh <phase> ; writes 🗑️generated/taf-cyc-<phase>-<label>.txt
cd /Users/ueli/Documents/semio
P="$1"
T="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/taf-tools"
G="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/🗑️generated"
run() {
  local label="$1"; shift
  perl -e 'alarm 1500; exec @ARGV' -- node node_modules/dependency-cruiser/bin/dependency-cruise.mjs "$@" --config "$T/nocycle.cjs" --output-type err > "$G/taf-cyc-$P-$label.txt" 2>&1
  echo "$label exit=$? $(date +%T)" >> "$G/taf-cyc-$P-status.txt"
}
: > "$G/taf-cyc-$P-status.txt"
run kernel "🧰️framework/🔨️modules/🎠️kernel"
run worker "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/👷️worker" "🧰️framework/🛍️products/💻️os/🧪️tests/📚️worker-law-suites" "🧰️framework/🛍️products/💻️os/🧪️tests/🧰️worker-law-fakes"
run rootscript "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands" "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws"
echo DONE >> "$G/taf-cyc-$P-status.txt"
