#!/bin/zsh
# usage: cargo.sh <cargo args...>  (plain toolchain cargo, no rustc wrapper, scratch target, offline)
S=/private/tmp/claude-501/-Users-ueli--claude-spares-spare-1dOdZf/35823786-c219-418d-81dc-737e033e7539/scratchpad/M4
TC=/Users/ueli/Documents/semio/.cache/b/t/m/r/9171f2f44c79a380/toolchains/nightly-2026-07-20-aarch64-apple-darwin/bin
export PATH=$TC:$PATH CARGO_HOME=/Users/ueli/Documents/semio/.cache/b/r/m/cargo/home CARGO_NET_OFFLINE=true CARGO_TARGET_DIR=$S/target RUSTC_WRAPPER= CARGO_BUILD_RUSTC=$TC/rustc CARGO_BUILD_RUSTC_WRAPPER= CARGO_BUILD_TARGET_DIR=$S/target CARGO_BUILD_BUILD_DIR=$S/build CARGO_BUILD_JOBS=4
cd /Users/ueli/Documents/semio
cargo "$@"
