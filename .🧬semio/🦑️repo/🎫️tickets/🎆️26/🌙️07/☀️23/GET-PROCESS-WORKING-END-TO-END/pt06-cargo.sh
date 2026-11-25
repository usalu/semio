#!/bin/zsh
export PATH="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🚀️launcher/🚪️shims:$PATH"  # managed cargo entry
T="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️07/☀️23/GET-PROCESS-WORKING-END-TO-END"; N=$T/🗑️generated/native
export RUSTUP_TOOLCHAIN=nightly-2026-07-20-aarch64-apple-darwin
cd "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧷️companions/🍉️tests-06/📦️packages/🦀️rust" || exit 1
L=$1; shift
cargo "$@" > $N/../pt06-$L.txt 2>&1; echo exit $?
