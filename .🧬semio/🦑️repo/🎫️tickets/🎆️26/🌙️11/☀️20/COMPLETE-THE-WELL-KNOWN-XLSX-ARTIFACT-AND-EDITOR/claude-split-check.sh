#!/bin/zsh
# [DEBUG] cargo check of the runtime-host split crates for wasm32-wasip2, then (arg "native") the native hub crates
T="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️20/COMPLETE-THE-WELL-KNOWN-XLSX-ARTIFACT-AND-EDITOR"; N=$T/🗑️generated/claude-pane/native
export CARGO_BUILD_RUSTC=rustc CARGO_BUILD_RUSTC_WRAPPER= CARGO_TARGET_DIR=$N/target CARGO_BUILD_BUILD_DIR=$N/build
export CARGO_BUILD_JOBS=4 CARGO_PROFILE_DEV_DEBUG=0 CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_SPLIT_DEBUGINFO=off
export CARGO_UNSTABLE_FEATURE_UNIFICATION=true CARGO_RESOLVER_FEATURE_UNIFICATION=package
export CARGO_ENCODED_RUSTFLAGS=$(printf -- "-Z\x1fthreads=1\x1f-Z\x1fshare-generics=y\x1f-C\x1flink-arg=--max-memory=536870912\x1f-C\x1flink-arg=-zstack-size=8388608")
cd /Users/ueli/Documents/semio/🌎️hub || exit 1
cargo check -j 4 --target wasm32-wasip2 --profile dev --keep-going -p semio-hub-stdio-catalog-projection -p semio-hub-stdio-catalog -p semio-hub-stdio -p semio-hub-stdio-bim -p semio-hub-stdio-semio "$@"
