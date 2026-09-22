#!/bin/zsh
# 🪶️ NB1: rebuild 📕️norm's wasm-dev component under the strip override, weigh it against the
# admission bound with the plugin's own law, then describe it. Runs INSIDE the ordered wasm mutex.
set -u
repo="/Users/ueli/Documents/semio"
ticket="$repo/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️18/OS-HUB-COLLABORATION-AI-END-TO-END"
gen="$ticket/🗑️generated"
crate="$repo/✏️s/🔌️plugins/📕️norm/📦️packages/🦀️rust"
comp="$repo/.🧬semio/🦑️repo/⚡️cache/cargo/target/wasm32-wasip2/wasm-dev/semio_s_plugin_norm.wasm"
json="$repo/✏️s/🔌️plugins/📕️norm/🔣️.json"
pack="$repo/✏️s/🔌️plugins/📕️norm/🛂️.descriptor.semio"
mkdir -p "$gen"
export CARGO_PROFILE_WASM_DEV_DEBUG=false CARGO_INCREMENTAL=0 NX_DAEMON=false
size() { [ -f "$1" ] && stat -f %z "$1" || echo 0; }
before_comp=$(size "$comp"); before_json=$(size "$json"); before_pack=$(size "$pack")
echo "nb1 hold start $(date '+%F %T') load=$(uptime | sed 's/.*averages: //')" | tee -a "$gen/nb1-ledger.txt"
echo "component before: $before_comp" | tee -a "$gen/nb1-ledger.txt"

start=$SECONDS
cd "$repo" && cargo build -p semio-s-plugin-norm --target wasm32-wasip2 --profile wasm-dev > "$gen/nb1-build.txt" 2>&1
build_rc=$?
build_s=$((SECONDS-start))
after_comp=$(size "$comp")
echo "build rc=$build_rc ${build_s}s component $before_comp -> $after_comp" | tee -a "$gen/nb1-ledger.txt"
tail -5 "$gen/nb1-build.txt" | tee -a "$gen/nb1-ledger.txt"

cd "$crate" && bun ./📜️script.ts component-budget-check > "$gen/nb1-law-after.txt" 2>&1
law_rc=$?
echo "component-budget-check rc=$law_rc" | tee -a "$gen/nb1-ledger.txt"
tail -3 "$gen/nb1-law-after.txt" | tee -a "$gen/nb1-ledger.txt"

start=$SECONDS
cd "$crate" && bun ./📜️script.ts describe > "$gen/nb1-describe-norm.txt" 2>&1
desc_rc=$?
desc_s=$((SECONDS-start))
echo "describe rc=$desc_rc ${desc_s}s json $before_json -> $(size "$json") pack $before_pack -> $(size "$pack")" | tee -a "$gen/nb1-ledger.txt"
tail -8 "$gen/nb1-describe-norm.txt" | tee -a "$gen/nb1-ledger.txt"
echo "nb1 hold end $(date '+%F %T')" | tee -a "$gen/nb1-ledger.txt"
