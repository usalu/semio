#!/usr/bin/env bash
# usage: [EXTRA="filter args"] b2d-run.sh <name> <cargo args...>
T="/c/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/GET-FEM-PLUGIN-WORKING-END-TO-END"
cd "/c/semio/✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d" || exit 2
export TMP='C:\t\tmp' TEMP='C:\t\tmp' TMPDIR='C:\t\tmp' CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=0 CARGO_PROFILE_TEST_DEBUG=0 CARGO_TARGET_DIR='C:\t\2d-a' CARGO_BUILD_BUILD_DIR='C:\t\2d-ab'
N="$1"; shift
G="$T/🗑️generated"
cargo --config 'build.rustc="rustc"' --config 'build.rustc-wrapper=""' "$@" -j 6 --message-format short -- $EXTRA > "$G/b2d-$N.log" 2>&1
echo "rc=$?" >> "$G/b2d-$N.log"
