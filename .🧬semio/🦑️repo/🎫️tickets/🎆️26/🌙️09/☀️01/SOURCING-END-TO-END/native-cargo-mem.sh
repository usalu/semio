#!/bin/zsh
# 🦀️ Native cargo for sourcing, one shared build dir, bypassing the repo ingress gate when it stalls.
# Usage: native-cargo.sh <label> <workspace-dir-relative-to-root> <cargo args...>
T="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️01/SOURCING-END-TO-END"; N=$T/🗑️generated/native-mem
mkdir -p $N
export CARGO_BUILD_RUSTC=rustc CARGO_BUILD_RUSTC_WRAPPER= CARGO_TARGET_DIR=$N/target CARGO_BUILD_BUILD_DIR=$N/build
export PATH="/Users/ueli/Documents/semio/.cache/b/t/m/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f:$HOME/.bun/bin:$PATH"
export CARGO_BUILD_JOBS=${JOBS:-4} CARGO_PROFILE_DEV_DEBUG=0 CARGO_PROFILE_TEST_DEBUG=0 CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_SPLIT_DEBUGINFO=off
L=$1; W=$2; shift 2
cd "/Users/ueli/Documents/semio/$W" || exit 1
cargo "$@" > $N/../$L.txt 2>&1; echo "exit $? -> $N/../$L.txt"
