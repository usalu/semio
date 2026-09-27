#!/bin/zsh
# 🧱️ C11: wasm32 checks of the guest-linked crates C11 changed (kernel directory schema, space-home Home guest), one cargo.
cd /Users/ueli/Documents/semio || exit 1
export CARGO_BUILD_BUILD_DIR="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/cargo/build-fleet-b" CARGO_INCREMENTAL=0
echo "START $(date '+%T')"
nice -n 10 cargo check -p semio-framework-os-kernel -p semio-s-artifact-space-home --features semio-s-artifact-space-home/component-app-assembly --lib --target wasm32-wasip2
echo "EXIT rc=$? $(date '+%T')"
