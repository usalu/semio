#!/bin/zsh
# [DEBUG] native wasm32-wasip2 build of the Space boot component, sharing the standalone-pane target directory
T="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️20/COMPLETE-THE-WELL-KNOWN-XLSX-ARTIFACT-AND-EDITOR"; N=$T/🗑️generated/claude-pane/native
mkdir -p $N
export CARGO_BUILD_RUSTC=rustc CARGO_BUILD_RUSTC_WRAPPER= CARGO_TARGET_DIR=$N/target CARGO_BUILD_BUILD_DIR=$N/build
export CARGO_BUILD_JOBS=${JOBS:-4} CARGO_PROFILE_DEV_DEBUG=0 CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_SPLIT_DEBUGINFO=off
export CARGO_UNSTABLE_FEATURE_UNIFICATION=true CARGO_RESOLVER_FEATURE_UNIFICATION=package
export CARGO_ENCODED_RUSTFLAGS=$(printf -- "-Z\x1fthreads=1\x1f-Z\x1fshare-generics=y\x1f-C\x1flink-arg=--max-memory=536870912\x1f-C\x1flink-arg=-zstack-size=8388608")
cd /Users/ueli/Documents/semio/🌎️hub || exit 1
cargo build -p semio-hub-space-component --lib --target wasm32-wasip2 --profile wasm-prototype \
 --config 'profile.wasm-prototype.inherits="dev"' --config 'profile.wasm-prototype.opt-level="s"' --config 'profile.wasm-prototype.codegen-units=16' --config 'profile.wasm-prototype.lto="off"' --config 'profile.wasm-prototype.incremental=false' --config 'profile.wasm-prototype.debug=0' --config 'profile.wasm-prototype.strip="symbols"' "$@"
cargo_exit=$?
echo "exit $cargo_exit"
exit $cargo_exit
