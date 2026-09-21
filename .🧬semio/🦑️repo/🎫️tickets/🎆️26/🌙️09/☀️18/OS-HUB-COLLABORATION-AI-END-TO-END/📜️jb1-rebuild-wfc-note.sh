#!/bin/zsh
# 🧱️ JB1: rebuild the two plugin components the builtin-job state-machine change must reach
# (`note` drives every `client-e2e` dispatch row, `wfc` drives the inference row that PZ1 §3
# measured as `job.explicit-state-machine-required`), then stage each into the SHARED uplift dir the
# MCP gateway resolves. Built into a PRIVATE uplift dir per preamble rule 25 and staged with rm+cp
# (macOS SIGKILLs a process whose binary was overwritten in place). Runs under the fleet wasm
# mutex; the caller wraps it.
set -u
cd /Users/ueli/Documents/semio || exit 1
export CARGO_PROFILE_WASM_DEV_DEBUG=false
export RUST_MIN_STACK=67108864
PRIVATE="$PWD/.🧬semio/🦑️repo/⚡️cache/cargo/target-jb1-wasm"
SHARED="$PWD/.🧬semio/🦑️repo/⚡️cache/cargo/target/wasm32-wasip2/wasm-dev"
build_rc=0
for plugin in note wfc; do
  crate="semio-s-plugin-$plugin"
  echo "=== [jb1] building $crate at $(date +%H:%M:%S) ==="
  CARGO_TARGET_DIR="$PRIVATE" cargo build -p "$crate" --target wasm32-wasip2 --profile wasm-dev || { build_rc=1; echo "=== [jb1] $crate FAILED ==="; continue; }
  built="$PRIVATE/wasm32-wasip2/wasm-dev/semio_s_plugin_$plugin.wasm"
  if [ ! -f "$built" ]; then echo "=== [jb1] $crate produced no component at $built ==="; build_rc=1; continue; fi
  rm -f "$SHARED/semio_s_plugin_$plugin.wasm"
  cp "$built" "$SHARED/semio_s_plugin_$plugin.wasm"
  echo "=== [jb1] staged $plugin: $(stat -f %z "$SHARED/semio_s_plugin_$plugin.wasm") bytes at $(date +%H:%M:%S) ==="
done
exit $build_rc
