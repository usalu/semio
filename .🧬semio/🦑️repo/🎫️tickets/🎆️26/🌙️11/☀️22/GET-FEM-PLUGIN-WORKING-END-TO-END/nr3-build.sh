#!/bin/bash
# usage: nr3-build.sh <2d|3d> [test filter args...]
cd /c/semio
dim=$1; shift
case $dim in 2d) A="✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d";; 3d) A="✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d";; esac
export CARGO_BUILD_RUSTC=rustc RUSTC_WRAPPER= CARGO_TARGET_DIR='C:\t\nr3' CARGO_BUILD_BUILD_DIR='C:\t\nr3b' TMP='C:\t\nr3tmp' TEMP='C:\t\nr3tmp' TMPDIR='C:\t\nr3tmp' CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=0 CARGO_PROFILE_TEST_DEBUG=0 RUST_MIN_STACK=268435456
cd "$A"
cargo test -j 3 -p semio-s-artifact-fem-${dim}-snapshot-tests --lib -- "$@"
