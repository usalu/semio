#!/bin/zsh
# 🧾️ S20 faults overlay, row-12 proof in ONE overlay-lane ticket: L1's native train union (`check --keep-going --lib --tests`,
# 228 crates + 59 features) and then the wasm32-wasip2 guest union (`check --keep-going --lib`, 178 crates) — native cargo never
# compiles wasm-gated code — then the fault laws (record, fault text + catalog class scanner, MCP gateway mapping). Private build-dir + target-dir of the overlay (never the chain's), CARGO_BUILD_JOBS ≤ 4, load gate.
# usage: zsh overlay-union-wasm.sh <tag> [<round>]   → one log under `.🧬semio/🌐hub/s14-s20-overlay-build/logs/`
tag="$1"; round="${2:-T6R3a}"
L="/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-l1-logs"
O="/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-s20-overlay-faults"
B="/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-s20-overlay-build"
native=(); for c in ${(f)"$(/usr/bin/grep -v '^#' "$L/$round-native-crates.txt" | /usr/bin/grep -v '^ *$')"}; do native+=(-p "$c"); done
wasm=(); for c in ${(f)"$(/usr/bin/grep -v '^#' "$L/$round-wasm-crates.txt" | /usr/bin/grep -v '^ *$')"}; do wasm+=(-p "$c"); done
features="${(j:,:)${(f)"$(/usr/bin/grep -v '^#' "$L/$round-native-features.txt" | /usr/bin/grep -v '^ *$')"}}"
mkdir -p "$B/logs"
log="$B/logs/$(date +%m%d-%H%M%S)-$tag.txt"
echo "QUEUED $(date +%T) native ${#native} args + wasm32-wasip2 ${#wasm} args" > "$log"
cd "$O" || exit 2
export CARGO_INCREMENTAL=0 CARGO_BUILD_BUILD_DIR="$B/build" CARGO_TARGET_DIR="$B/target" NX_DAEMON=false CARGO_BUILD_JOBS=4
zsh /Users/ueli/Documents/semio/.tmp-ticket/📜️fleet-mutex.sh overlay s20 -- env NATIVE="${native[*]}" WASM="${wasm[*]}" FEATURES="$features" nice -n 15 zsh -c '
  echo "START native $(date +%T)"; cargo check --offline --keep-going --message-format short --lib --tests ${=NATIVE} --features "$FEATURES"; echo "EXIT native $? $(date +%T)"
  echo "START wasm32 $(date +%T)"; cargo check --offline --keep-going --message-format short --lib --target wasm32-wasip2 ${=WASM}; echo "EXIT wasm32 $? $(date +%T)"
  echo "START laws $(date +%T)"; cargo test --offline --no-fail-fast --lib -p semio-framework -p semio-framework-plugin -p semio-framework-os-mcp -p semio-framework-replication -- fault typed_operation catalog_classes map_fault gateway; echo "EXIT laws $? $(date +%T)"' >> "$log" 2>&1
echo "$log"
