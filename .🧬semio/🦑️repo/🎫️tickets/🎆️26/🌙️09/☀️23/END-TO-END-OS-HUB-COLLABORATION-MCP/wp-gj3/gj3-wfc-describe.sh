#!/usr/bin/env zsh
set -u
ROOT=/Users/ueli/Documents/semio
WP="$ROOT/.tmp-ticket/wp-gj3"
GEN="$WP/generated"
mkdir -p "$GEN"
WFC_RUST=$(ls -d "$ROOT"/*s/*plugins/*wfc/*packages/*rust 2>/dev/null | head -1)
SCRIPT=$(ls "$WFC_RUST"/*script.ts | head -1)
echo "WFC_RUST=$WFC_RUST SCRIPT=$SCRIPT" | tee "$GEN/gj3-wfc-describe.txt"
export NX_DAEMON=false CARGO_PROFILE_WASM_DEV_DEBUG=false CARGO_INCREMENTAL=0 CARGO_BUILD_JOBS=4
unset SEMIO_BUILD_BUDGET_MS 2>/dev/null || true
cd "$WFC_RUST"
echo "=== describe start $(date -Iseconds) ===" >> "$GEN/gj3-wfc-describe.txt"
bun "$SCRIPT" describe >> "$GEN/gj3-wfc-describe.txt" 2>&1
rc=$?
echo "=== describe exit $rc $(date -Iseconds) ===" >> "$GEN/gj3-wfc-describe.txt"
STAGED=$(ls "$ROOT"/.*semio/*repo/*cache/cargo/target/wasm32-wasip2/wasm-dev/semio_s_plugin_wfc.wasm 2>/dev/null | head -1)
ls -la "$STAGED" >> "$GEN/gj3-wfc-describe.txt" 2>&1
shasum -a 256 "$STAGED" | cut -c1-32 >> "$GEN/gj3-wfc-describe.txt" 2>&1
exit $rc
