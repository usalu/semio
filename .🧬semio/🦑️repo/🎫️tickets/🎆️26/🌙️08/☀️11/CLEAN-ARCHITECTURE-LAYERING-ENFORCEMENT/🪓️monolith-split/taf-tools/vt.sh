#!/bin/zsh
# usage: vt.sh <phase> ; vitest json for kernel, framework, os-worker, gis configs
R=/Users/ueli/Documents/semio
G="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/🗑️generated"
P="$1"
cd $R
run() {
  local label="$1" cfg="$2"; shift 2
  perl -e 'alarm 1700; exec @ARGV' -- node node_modules/vitest/vitest.mjs run --config "$cfg" --testTimeout 120000 --reporter=json --outputFile="$G/taf-vt-$P-$label.json" "$@" > "$G/taf-vt-$P-$label.log" 2>&1
  echo "$label exit=$? $(date +%T)" >> "$G/taf-vt-$P-status.txt"
}
: > "$G/taf-vt-$P-status.txt"
run kernel "$R/🧰️framework/🔨️modules/🎠️kernel/🧪️tests/🎚️config/🟦️.ts"
run framework "$R/🧰️framework/🧪️tests/🎚️config/🟦️.ts"
run osworker "$R/🧰️framework/🛍️products/💻️os/🧪️tests/🎚️config/🟦️.ts"
run gis "$R/✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/💡️inference/🧪️tests/🎚️config/🟦️.ts"
echo DONE >> "$G/taf-vt-$P-status.txt"
