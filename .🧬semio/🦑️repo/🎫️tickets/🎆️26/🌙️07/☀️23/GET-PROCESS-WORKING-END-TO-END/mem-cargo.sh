#!/bin/zsh
# Usage: mem-cargo.sh <workspace-dir> <label> <cargo args...>  -- measurement only (needs the bare compiler for its RUSTC_WRAPPER); admitted into the machine ledger through `repo capacity run`, private target, per-crate peak RSS TSV.
T="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️07/☀️23/GET-PROCESS-WORKING-END-TO-END"; G=$T/🗑️generated
export PATH="$HOME/.bun/bin:$PATH"
export CARGO_TARGET_DIR=$G/native/build-mem/target CARGO_BUILD_BUILD_DIR=$G/native/build-mem/build
export CARGO_BUILD_JOBS=2 CARGO_PROFILE_DEV_DEBUG=0 CARGO_PROFILE_TEST_DEBUG=0 CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_SPLIT_DEBUGINFO=off
export CARGO_BUILD_RUSTC=$HOME/.rustup/toolchains/nightly-2026-07-20-aarch64-apple-darwin/bin/rustc CARGO_BUILD_RUSTC_WRAPPER=$T/rustc-mem-wrap.sh RUSTC_WRAPPER=$T/rustc-mem-wrap.sh SEMIO_MEM_TSV=$G/mem-$2.tsv
mkdir -p $G/native/build-mem
cd "$1" || exit 1
d=$1; l=$2; shift 2
repo capacity run --operation=cargo --jobs=2 -- $HOME/.cargo/bin/cargo "$@" > $G/mem-$l.txt 2>&1; echo exit $? >> $G/mem-$l.txt; tail -3 $G/mem-$l.txt
