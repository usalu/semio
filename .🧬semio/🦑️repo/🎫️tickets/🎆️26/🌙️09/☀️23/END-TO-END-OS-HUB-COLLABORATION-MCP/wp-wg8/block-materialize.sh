#!/bin/zsh
# WG8: materializes block's release descriptor from the already-built catalog component (no component rebuild), one wasm hold.
cd /Users/ueli/Documents/semio
unset CARGO_TARGET_DIR CARGO_BUILD_TARGET_DIR
export CARGO_INCREMENTAL=0
echo "START $(date '+%F %T') component=$(shasum -a 256 '✏️s/🔌️plugins/🧱️block/📦️packages/🦀️rust/dist/component-release/semio_s_plugin_block.wasm' | cut -c1-16)"
zsh /Users/ueli/Documents/semio/.tmp-ticket/📜️fleet-mutex.sh wasm wg8 -- bun nx run @semio-tech/block-plugin:materialize-release --exclude-task-dependencies --outputStyle=stream
echo "EXIT=$? $(date '+%F %T') component=$(shasum -a 256 '✏️s/🔌️plugins/🧱️block/📦️packages/🦀️rust/dist/component-release/semio_s_plugin_block.wasm' | cut -c1-16)"
