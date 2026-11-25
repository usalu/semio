#!/bin/zsh
# [DEBUG] native wasm32-wasip2 build of the standalone XLSX pane boot components, stdio first and stdio-office second so the two giant crates never compile together
T="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️20/COMPLETE-THE-WELL-KNOWN-XLSX-ARTIFACT-AND-EDITOR"; N=$T/🗑️generated/claude-pane/native
mkdir -p $N
export CARGO_BUILD_RUSTC=rustc CARGO_BUILD_RUSTC_WRAPPER= CARGO_TARGET_DIR=$N/target CARGO_BUILD_BUILD_DIR=$N/build
export CARGO_BUILD_JOBS=${JOBS:-4} CARGO_PROFILE_DEV_DEBUG=0 CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_SPLIT_DEBUGINFO=off
export CARGO_UNSTABLE_FEATURE_UNIFICATION=true CARGO_RESOLVER_FEATURE_UNIFICATION=package
cd /Users/ueli/Documents/semio/🌎️hub || exit 1
build() {
 cargo build -p "$1" --lib --target wasm32-wasip2 --profile wasm-prototype \
 --config 'profile.wasm-prototype.inherits="dev"' --config 'profile.wasm-prototype.opt-level="s"' --config 'profile.wasm-prototype.codegen-units=16' --config 'profile.wasm-prototype.lto="thin"' --config 'profile.wasm-prototype.incremental=false' --config 'profile.wasm-prototype.debug=0' --config 'profile.wasm-prototype.strip="symbols"' "${@:2}"
}
cargo_exit=0
build semio-hub-stdio-component "$@" || cargo_exit=$?
if [ $cargo_exit -eq 0 ]; then build semio-hub-stdio-office-component "$@" || cargo_exit=$?; fi
echo "exit $cargo_exit"
exit $cargo_exit
