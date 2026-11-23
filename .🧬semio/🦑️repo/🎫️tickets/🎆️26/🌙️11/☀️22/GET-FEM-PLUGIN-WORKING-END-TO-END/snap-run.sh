#!/usr/bin/env bash
# usage: snap-run.sh 2d|3d logname [test args...]
# Builds with --no-run (lock held only while compiling), then runs the test exe directly outside cargo.
T="/c/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/GET-FEM-PLUGIN-WORKING-END-TO-END"
D=$1; L=$2; shift 2
if [ "$D" = 2d ]; then cd "/c/semio/✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d"; export CARGO_TARGET_DIR='C:\t\snap2' CARGO_BUILD_BUILD_DIR='C:\t\snap2b'; P=semio-s-artifact-fem-2d-snapshot-tests; X=semio_s_artifact_fem_2d_snapshot_tests; TD=/c/t/snap2
else cd "/c/semio/✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d"; export CARGO_TARGET_DIR='C:\t\3d' CARGO_BUILD_BUILD_DIR='C:\t\3db'; P=semio-s-artifact-fem-3d-snapshot-tests; X=semio_s_artifact_fem_3d_snapshot_tests; TD=/c/t/3d; fi
export TMP='C:\t\tmp' TEMP='C:\t\tmp' TMPDIR='C:\t\tmp' CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=0 CARGO_PROFILE_TEST_DEBUG=0 CARGO_BUILD_RUSTC=rustc CARGO_BUILD_RUSTC_WRAPPER= RUSTC_WRAPPER=
LOG="$T/🗑️generated/snap-$L.log"
cargo test -j 6 -p $P --lib --no-run --message-format short > "$LOG" 2>&1
echo "build rc=$?" >> "$LOG"
EXE=$(ls -t $TD/debug/deps/$X-*.exe 2>/dev/null | head -1)
echo "exe=$EXE" >> "$LOG"
"$EXE" "$@" >> "$LOG" 2>&1
echo "rc=$?" >> "$LOG"
