#!/bin/bash
# usage: nr2-build.sh <2d|3d> <test target> [test filter args...]
cd /c/semio
dim=$1; shift
A=$(ls -d ✏️s/🔌️plugins/🏗️fem/🗿️artifacts/*/ | grep -i "${dim}/$" | head -1)
case $dim in 2d) A="✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d";; 3d) A="✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d";; esac
export CARGO_BUILD_RUSTC=rustc RUSTC_WRAPPER= CARGO_TARGET_DIR='C:\t\nr' CARGO_BUILD_BUILD_DIR='C:\t\nrb' TMP='C:\t\nrtmp' TEMP='C:\t\nrtmp' TMPDIR='C:\t\nrtmp' CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=0 CARGO_PROFILE_TEST_DEBUG=0 RUST_MIN_STACK=268435456
cd "$A"
cargo test -j 3 -p semio-s-artifact-fem-${dim}-snapshot-tests --lib -- "$@"
