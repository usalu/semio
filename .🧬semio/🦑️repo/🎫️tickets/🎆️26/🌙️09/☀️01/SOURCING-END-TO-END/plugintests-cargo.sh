#!/bin/zsh
# 🧪️ Native cargo for the final sourcing verification, output under 🗑️generated/verify.
# Usage: verify-cargo.sh <label> <workspace-dir-relative-to-root> <cargo args...>
R="/Users/ueli/Documents/semio"; T="$R/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️01/SOURCING-END-TO-END"; N=$T/🗑️generated/native; V=$T/🗑️generated/plugintests
mkdir -p $N $V
TC=$(echo $R/.cache/b/t/m/r/*/toolchains/nightly-2026-07-20-*/bin | cut -d' ' -f1)
export PATH="$TC:$(echo $R/.cache/b/t/m/bun/*/ | cut -d' ' -f1):$HOME/.bun/bin:$PATH"
export CARGO_BUILD_RUSTC=rustc CARGO_BUILD_RUSTC_WRAPPER= CARGO_TARGET_DIR=$N/target CARGO_BUILD_BUILD_DIR=$N/build
export CARGO_BUILD_JOBS=${JOBS:-4} CARGO_PROFILE_DEV_DEBUG=0 CARGO_PROFILE_TEST_DEBUG=0 CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_SPLIT_DEBUGINFO=off
export SEMIO_TEST_ARTIFACT_DIR=$V/artifacts; mkdir -p $SEMIO_TEST_ARTIFACT_DIR
L=$1; W=$2; shift 2
cd "$R/$W" || exit 1
cargo "$@" > $V/$L.txt 2>&1; echo "exit $? -> $V/$L.txt"
