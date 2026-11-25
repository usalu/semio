#!/bin/zsh
export RUSTUP_TOOLCHAIN=nightly-2026-07-20-aarch64-apple-darwin
export PATH="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🚀️launcher/🚪️shims:$HOME/.bun/bin:$PATH"
T="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️07/☀️23/GET-PROCESS-WORKING-END-TO-END"; N=$T/🗑️generated/native/build-re2
mkdir -p $N
export RUSTUP_TOOLCHAIN=nightly-2026-07-20-aarch64-apple-darwin
export PATH="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🚀️launcher/🚪️shims:$HOME/.bun/bin:$PATH"
cd "$1" || exit 1
L=$2; shift 2
cargo "$@" > $N/../re2-$L.txt 2>&1; echo exit $? >> $N/../re2-$L.txt
