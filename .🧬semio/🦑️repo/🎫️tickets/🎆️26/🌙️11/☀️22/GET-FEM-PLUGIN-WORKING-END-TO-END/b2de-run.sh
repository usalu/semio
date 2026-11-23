#!/usr/bin/env bash
# usage: [EXTRA="filter args"] b2de-run.sh <name> <cargo args...>
T="/c/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/GET-FEM-PLUGIN-WORKING-END-TO-END"
cd "/c/semio/✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d" || exit 2
export TMP='C:\t\2detmp' TEMP='C:\t\2detmp' TMPDIR='C:\t\2detmp' CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=0 CARGO_PROFILE_TEST_DEBUG=0 CARGO_TARGET_DIR='C:\t\2de' CARGO_BUILD_BUILD_DIR='C:\t\2deb'
N="$1"; shift
G="$T/🗑️generated"
cargo --config 'build.rustc="rustc"' --config 'build.rustc-wrapper=""' "$@" -j 6 --message-format short -- $EXTRA > "$G/b2de-$N.log" 2>&1
echo "rc=$?" >> "$G/b2de-$N.log"
