#!/bin/zsh
# WG8: runs the hub-less native guest journey law with a staged block2d release runtime.
cd /Users/ueli/Documents/semio
export CARGO_INCREMENTAL=0
export CARGO_TARGET_DIR=/Users/ueli/Documents/semio/.tmp-ticket/wp-wg8/target
export SEMIO_PLUGIN=block2d
export SEMIO_PLUGIN_MODULES="/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/dist/runtime/native/release/block2d"
cargo test -p semio-framework-os-renderer-wgpu --lib --no-fail-fast -- shell::hub_projection_workspace_tests::a_native_guest_ --ignored --nocapture --test-threads=1
echo "EXIT=$?"
