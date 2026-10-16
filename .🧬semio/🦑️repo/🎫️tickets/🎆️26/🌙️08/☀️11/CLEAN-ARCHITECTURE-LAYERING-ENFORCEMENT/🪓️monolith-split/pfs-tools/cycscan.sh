#!/bin/zsh
# usage: cycscan.sh <phase> ; writes 🗑️generated/pfs-cyc-<phase>-<label>.txt (dependency-cruiser, no-circular only)
. "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/pfs-tools/env.sh"
cd "$R"
PH="$1"
run() {
  local label="$1"; shift
  perl -e 'alarm 1500; exec @ARGV' -- node node_modules/dependency-cruiser/bin/dependency-cruise.mjs "$@" --config "$T/nocycle.cjs" --output-type err > "$G/pfs-cyc-$PH-$label.txt" 2>&1
  echo "$label exit=$? $(date +%T)" >> "$G/pfs-cyc-$PH-status.txt"
}
: > "$G/pfs-cyc-$PH-status.txt"
run pets "$P/🔨️modules/🚶️locomotion" "$P/🔨️modules/🚧️clearance" "$P/🔨️modules/🎪️stage"
run presentation "$PR/🎤️presentation/📦️packages/🟦️typescript/🟦️.ts" "$PR/🎤️presentation/📦️packages/🟦️typescript/🧱️vocabulary" "$PR/🎤️presentation/📦️packages/🟦️typescript/🎭️embodiments"
run grammar "$PR/📓️print/🧪️tests/🧬️native-chart-grammar" "$PR/📓️print/🧬️schema/💡️inferences/📦️packages/🟦️typescript/🔬️probes"
run quiz "$PR/❓️quiz/🧪️tests/🧰️learner-journey-kit" "$PR/❓️quiz/🧪️tests/🚶️journey-and-tabs" "$PR/❓️quiz/🧪️tests/🧩️deputy-and-contract" "$PR/❓️quiz/🧪️tests/🏔️challenge-session" "$PR/❓️quiz/🧪️tests/🧷️run-truth"
echo DONE >> "$G/pfs-cyc-$PH-status.txt"
