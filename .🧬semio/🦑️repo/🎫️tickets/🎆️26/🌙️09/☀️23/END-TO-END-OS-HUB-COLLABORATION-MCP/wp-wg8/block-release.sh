#!/bin/zsh
# WG8: rebuild the block guest (component-release + materialize-release) from the post-H9 tree, one wasm hold.
cd /Users/ueli/Documents/semio
unset CARGO_TARGET_DIR CARGO_BUILD_TARGET_DIR
export CARGO_INCREMENTAL=0
echo "START $(date '+%F %T')"
zsh /Users/ueli/Documents/semio/.tmp-ticket/📜️fleet-mutex.sh wasm wg8 -- bun nx run @semio-tech/block-plugin:materialize-release --outputStyle=stream
echo "EXIT=$? $(date '+%F %T')"
