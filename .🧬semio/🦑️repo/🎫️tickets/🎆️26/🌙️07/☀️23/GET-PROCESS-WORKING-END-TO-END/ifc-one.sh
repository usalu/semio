#!/bin/zsh
T="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️07/☀️23/GET-PROCESS-WORKING-END-TO-END"
export CARGO_BUILD_RUSTC=rustc CARGO_BUILD_RUSTC_WRAPPER= CARGO_TARGET_DIR=/Users/ueli/Documents/semio/native/build-ifc
export CARGO_BUILD_JOBS=3 CARGO_PROFILE_DEV_DEBUG=0 CARGO_PROFILE_TEST_DEBUG=0 CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_SPLIT_DEBUGINFO=off
export PATH="$HOME/.bun/bin:$PATH"
cd /Users/ueli/Documents/semio || exit 1
L=$1; shift
cargo test --offline --manifest-path ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/Cargo.toml -p semio-s-artifact-stdio-ifc-4x3 --lib "$@" 2>&1 | awk '/work quote denied/{c[$0]++;next}{print substr($0,1,700)}END{for(k in c)print "[DEBUG-SUMMARY] " c[k] "x " k}' > "$T/🗑️generated/$L.txt"; echo exit >> "$T/🗑️generated/$L.txt"
