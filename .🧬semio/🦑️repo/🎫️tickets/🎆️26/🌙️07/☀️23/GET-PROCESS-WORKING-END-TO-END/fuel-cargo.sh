#!/bin/zsh
export PATH="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🚀️launcher/🚪️shims:$PATH"  # managed cargo entry
T="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️07/☀️23/GET-PROCESS-WORKING-END-TO-END"; N=$T/🗑️generated/native/build-fuel
mkdir -p $N
export PATH="$HOME/.bun/bin:$PATH"; export RUSTUP_TOOLCHAIN=nightly-2026-07-20-aarch64-apple-darwin
cd "$1" || exit 1
L=$2; shift 2
cargo "$@" > $T/🗑️generated/fuel-$L.txt 2>&1; echo exit $? >> $T/🗑️generated/fuel-$L.txt
