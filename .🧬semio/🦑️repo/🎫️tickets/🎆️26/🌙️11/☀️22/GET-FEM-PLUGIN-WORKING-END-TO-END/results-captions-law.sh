#!/usr/bin/env bash
# 🏷️ Runs the FEM 2D results caption placement law natively in its own target dir.
T="/c/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/GET-FEM-PLUGIN-WORKING-END-TO-END"; mkdir -p "$T/🗑️generated" /c/t/caption /c/t/captmp
export TMP='C:\t\captmp' TEMP='C:\t\captmp' CARGO_TARGET_DIR='C:\t\caption' CARGO_INCREMENTAL=0 RUST_MIN_STACK=134217728
cd "/c/semio/✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📊️results/📦️packages/🦀️rust"
cargo test -j 3 --config 'build.rustc="rustc"' --config 'build.rustc-wrapper=""' -p semio-s-artifact-fem-2d-editor-results-window ${1:-results_} > "$T/🗑️generated/results-captions-law.log" 2>&1
echo "rc=$?" >> "$T/🗑️generated/results-captions-law.log"; grep -E "^test |test result|^error|rc=" "$T/🗑️generated/results-captions-law.log" | head -40
