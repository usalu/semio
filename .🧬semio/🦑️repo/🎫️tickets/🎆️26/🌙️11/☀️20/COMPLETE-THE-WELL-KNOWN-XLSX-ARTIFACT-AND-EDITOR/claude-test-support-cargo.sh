#!/bin/zsh
# [DEBUG] runs one cargo check of the test-support split with the ingress bypass used by the other guarded builds
G="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️20/COMPLETE-THE-WELL-KNOWN-XLSX-ARTIFACT-AND-EDITOR/🗑️generated/claude-test-support"
export CARGO_BUILD_RUSTC=rustc CARGO_BUILD_RUSTC_WRAPPER= CARGO_TARGET_DIR=$G/target CARGO_BUILD_BUILD_DIR=$G/build
export CARGO_BUILD_JOBS=2 CARGO_PROFILE_DEV_DEBUG=0 CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_SPLIT_DEBUGINFO=off
export CARGO_UNSTABLE_FEATURE_UNIFICATION=true CARGO_RESOLVER_FEATURE_UNIFICATION=package
case "$*" in *wasm32*) export CARGO_ENCODED_RUSTFLAGS=$(printf -- "-Z\x1fthreads=1\x1f-Z\x1fshare-generics=y\x1f-C\x1flink-arg=--max-memory=536870912\x1f-C\x1flink-arg=-zstack-size=8388608");; *) export CARGO_ENCODED_RUSTFLAGS=$(printf -- "-Z\x1fthreads=1");; esac
cd ${CARGO_CWD:-/Users/ueli/Documents/semio} || exit 1
cargo ${CARGO_SUB:-check} -j 2 "$@"
