#!/bin/zsh
# 🧪️ T13 landing gate: native `cargo check` (lib + tests) of every guest crate T13 touched, then one wasm32-wasip2 check
# through the fleet wasm mutex. Rule 26 build-dir, private target dir, no incremental.
cd /Users/ueli/Documents/semio || exit 1
export CARGO_BUILD_BUILD_DIR=/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/cargo/build-fleet-b
export CARGO_TARGET_DIR=/Users/ueli/Documents/semio/.tmp-ticket/wp-t13/target
export CARGO_INCREMENTAL=0
CRATES=(-p semio-s-artifact-wfc-2d -p semio-s-artifact-wfc-grid2d -p semio-s-artifact-wfc-bitmap -p semio-s-artifact-wfc-3d -p semio-s-artifact-wfc-grid3d -p semio-framework-surface -p semio-s-artifact-reasoning-wires)
echo "START native $(date '+%T')"
nice -n 10 cargo check "${CRATES[@]}" --lib --tests --message-format short; echo "EXIT-NATIVE $? $(date '+%T')"
nice -n 10 cargo check -p semio-s-artifact-wfc-bitmap -p semio-s-artifact-wfc-2d -p semio-s-artifact-wfc-grid2d -p semio-s-artifact-wfc-3d -p semio-s-artifact-wfc-grid3d --features component-app-assembly --lib --tests --message-format short; echo "EXIT-NATIVE-APP $? $(date '+%T')"
nice -n 10 cargo check --manifest-path "✏️s/🔌️plugins/🗄️stdio/🔮️oracles/📦️packages/🦀️rust/Cargo.toml" --lib --tests --message-format short; echo "EXIT-ORACLE-CRATE $? $(date '+%T')"
[ "$1" = "--native-only" ] && exit 0
zsh /Users/ueli/Documents/semio/.tmp-ticket/📜️fleet-mutex.sh wasm t13 -- zsh -c 'nice -n 10 cargo check -p semio-s-artifact-wfc-2d -p semio-s-artifact-wfc-grid2d -p semio-s-artifact-wfc-bitmap -p semio-s-artifact-wfc-3d -p semio-s-artifact-wfc-grid3d --features component-app-assembly --lib --target wasm32-wasip2 --message-format short; echo "EXIT-WASM-WFC $? $(date +%T)"; nice -n 10 cargo check -p semio-s-artifact-reasoning-wires -p semio-framework-surface --lib --target wasm32-wasip2 --message-format short; echo "EXIT-WASM-WIRES-SURFACE $? $(date +%T)"'
echo ALL_DONE
