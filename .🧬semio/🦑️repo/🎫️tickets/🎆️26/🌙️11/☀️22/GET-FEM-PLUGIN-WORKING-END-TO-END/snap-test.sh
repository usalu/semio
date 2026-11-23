#!/usr/bin/env bash
# usage: snap-test.sh 2d|3d logname [test args...]
T="/c/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/GET-FEM-PLUGIN-WORKING-END-TO-END"
D=$1; L=$2; shift 2
if [ "$D" = 2d ]; then cd "/c/semio/✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d"; export CARGO_TARGET_DIR='C:\t\2d-a' CARGO_BUILD_BUILD_DIR='C:\t\2d-ab'; P=semio-s-artifact-fem-2d-snapshot-tests
else cd "/c/semio/✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d"; export CARGO_TARGET_DIR='C:\t\3d' CARGO_BUILD_BUILD_DIR='C:\t\3db'; P=semio-s-artifact-fem-3d-snapshot-tests; fi
export TMP='C:\t\tmp' TEMP='C:\t\tmp' TMPDIR='C:\t\tmp' CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=0 CARGO_PROFILE_TEST_DEBUG=0 CARGO_BUILD_RUSTC=rustc CARGO_BUILD_RUSTC_WRAPPER= RUSTC_WRAPPER=
cargo test -j 6 -p $P --lib --message-format short -- "$@" > "$T/🗑️generated/snap-$L.log" 2>&1
echo "rc=$?" >> "$T/🗑️generated/snap-$L.log"
