#!/bin/zsh
export PATH="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🚀️launcher/🚪️shims:$PATH"  # managed cargo entry
T="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️07/☀️23/GET-PROCESS-WORKING-END-TO-END"
export RUSTUP_TOOLCHAIN=nightly-2026-07-20-aarch64-apple-darwin
cd /Users/ueli/Documents/semio
for p in value-retirement value-clone-source value-native-receiver-protocol value-native-owned job pack-format; do
  $T/stale-cargo.sh swapc-$p test --offline --manifest-path 🧰️framework/Cargo.toml --no-fail-fast -p semio-framework-$p
done
