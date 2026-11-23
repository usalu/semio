#!/usr/bin/env bash
# usage: pb-run.sh <3d|2d> <step>
T="/c/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/GET-FEM-PLUGIN-WORKING-END-TO-END"
if [ "$1" = 3d ]; then D="🧊️3d"; P=semio-s-artifact-fem-3d-editor; else D="◻️2d"; P=semio-s-artifact-fem-2d-editor; fi
cd "/c/semio/✏️s/🔌️plugins/🏗️fem/🗿️artifacts/$D" || exit 2
export TMP='C:\t\pbtmp' TEMP='C:\t\pbtmp' TMPDIR='C:\t\pbtmp' CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=0 CARGO_PROFILE_TEST_DEBUG=0 CARGO_TARGET_DIR='C:\t\pb' CARGO_BUILD_BUILD_DIR='C:\t\pbb' CARGO_BUILD_RUSTC=rustc CARGO_BUILD_RUSTC_WRAPPER= RUSTC_WRAPPER=
G="$T/🗑️generated"
PK="-p $P"
cargo test -j 3 $PK --test editor_app_2 --message-format short -- --nocapture playback_presses > "$G/playback-$2.log" 2>&1
echo "rc=$?" >> "$G/playback-$2.log"
