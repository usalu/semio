#!/bin/zsh
# 🧪️ S20 14c: runs the raster archive-door repro law in the native lane (build-fleet-b, private target).
cd /Users/ueli/Documents/semio
export CARGO_BUILD_BUILD_DIR=/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/cargo/build-fleet-b
export CARGO_INCREMENTAL=0
export CARGO_TARGET_DIR=/Users/ueli/Documents/semio/.tmp-ticket/wp-s20/target
export NX_DAEMON=false
zsh /Users/ueli/Documents/semio/.tmp-ticket/📜️fleet-mutex.sh native s20 -- nice -n 15 cargo test -p semio-s-artifact-raster-raster --lib --no-fail-fast -- a_demo_edit_archive_loads_back_through_the_document_archive_door --nocapture
echo "rc=$?"
