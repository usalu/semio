#!/bin/zsh
# 🧱️ WR4: rebuild the two plugin components the two-phase typed-command SDK change must reach
# (`note` drives `live-agent-loop-check`, `draw` drives `client-e2e`), then stage each one into the
# SHARED uplift dir the gateway resolves (`.cargo/config.toml`'s `target-dir`). Built into a PRIVATE
# uplift dir per rule 25 — a binary-producing cargo run starves on the shared one — and staged with
# rm+cp (macOS SIGKILLs a process whose binary was overwritten in place).
# Runs under the fleet wasm mutex; the caller wraps it.
set -u
cd /Users/ueli/Documents/semio || exit 1
export CARGO_PROFILE_WASM_DEV_DEBUG=false
PRIVATE="$PWD/.🧬semio/🦑️repo/⚡️cache/cargo/target-wr4-wasm"
SHARED="$PWD/.🧬semio/🦑️repo/⚡️cache/cargo/target/wasm32-wasip2/wasm-dev"
build_rc=0
# 🎯️ `animate` is what `client-e2e`'s own `capabilities_search(kind=[mutation])` hit 0 resolves to
# today (`animate.…#editor.setFrame`), `note` is what `live-agent-loop-check` drives, and `draw` is
# WR3's export witness — the three components both permanent gates actually touch.
for plugin in note animate draw; do
  crate="semio-s-plugin-$plugin"
  echo "=== [wr4] building $crate ==="
  CARGO_TARGET_DIR="$PRIVATE" cargo build -p "$crate" --target wasm32-wasip2 --profile wasm-dev || { build_rc=1; echo "=== [wr4] $crate FAILED ==="; continue; }
  built="$PRIVATE/wasm32-wasip2/wasm-dev/semio_s_plugin_$plugin.wasm"
  if [ ! -f "$built" ]; then echo "=== [wr4] $crate produced no component at $built ==="; build_rc=1; continue; fi
  rm -f "$SHARED/semio_s_plugin_$plugin.wasm"
  cp "$built" "$SHARED/semio_s_plugin_$plugin.wasm"
  echo "=== [wr4] staged $plugin: $(stat -f %z "$SHARED/semio_s_plugin_$plugin.wasm") bytes ==="
done
exit $build_rc
