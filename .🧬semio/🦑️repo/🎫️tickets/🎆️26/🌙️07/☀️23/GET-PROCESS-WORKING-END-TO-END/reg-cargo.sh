#!/bin/zsh
T="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️07/☀️23/GET-PROCESS-WORKING-END-TO-END"; N=$T/🗑️generated/native
export CARGO_BUILD_RUSTC=rustc CARGO_BUILD_RUSTC_WRAPPER= CARGO_TARGET_DIR=$N/target-reg CARGO_BUILD_BUILD_DIR=$N/build-reg
export CARGO_BUILD_JOBS=3 CARGO_PROFILE_DEV_DEBUG=0 CARGO_PROFILE_TEST_DEBUG=0 CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_SPLIT_DEBUGINFO=off
export PATH="$HOME/.bun/bin:$PATH"
cd "${REG_CWD:-/Users/ueli/Documents/semio/🌎️hub}" || exit 1
L=$1; shift
cargo "$@" > $T/🗑️generated/reg-$L.txt 2>&1; echo exit $? >> $T/🗑️generated/reg-$L.txt
