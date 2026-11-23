#!/usr/bin/env bash
# usage: [EXTRA="filter args"] b2dg-run.sh <name> <cargo args...>
T="/c/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/GET-FEM-PLUGIN-WORKING-END-TO-END"
cd "/c/semio/✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d" || exit 2
export CARGO_UNSTABLE_BUILD_DIR_NEW_LAYOUT=false CARGO_UNSTABLE_FINE_GRAIN_LOCKING=false
export TMP='C:\t\2dgtmp' TEMP='C:\t\2dgtmp' TMPDIR='C:\t\2dgtmp' CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=0 CARGO_PROFILE_TEST_DEBUG=0 CARGO_TARGET_DIR='C:\t\2dg' CARGO_BUILD_BUILD_DIR='C:\t\2dgb'
N="$1"; shift
G="$T/🗑️generated"
cargo --config 'build.rustc="rustc"' --config 'build.rustc-wrapper=""' "$@" -j 3 --message-format short -- $EXTRA > "$G/b2dg-$N.log" 2>&1
echo "rc=$?" >> "$G/b2dg-$N.log"
