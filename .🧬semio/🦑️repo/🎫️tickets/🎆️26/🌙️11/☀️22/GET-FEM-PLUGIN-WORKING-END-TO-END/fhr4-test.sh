#!/usr/bin/env bash
# usage: fhr4-test.sh <2d|3d> <tag> <test-target|--lib> [test filter...]
T="/c/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/GET-FEM-PLUGIN-WORKING-END-TO-END"
dim="$1"; tag="$2"; target="$3"; shift 3
if [ "$dim" = 2d ]; then
  cd "/c/semio/✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d" || exit 2
  export CARGO_TARGET_DIR='C:\t\hr4c' CARGO_BUILD_BUILD_DIR='C:\t\hr4c'
else
  cd "/c/semio/✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d" || exit 2
  export CARGO_TARGET_DIR='C:\t\hr4' CARGO_BUILD_BUILD_DIR='C:\t\hr4'
fi
export CARGO_UNSTABLE_BUILD_DIR_NEW_LAYOUT=false CARGO_UNSTABLE_FINE_GRAIN_LOCKING=false
export TMP='C:\t\hr4tmp' TEMP='C:\t\hr4tmp' TMPDIR='C:\t\hr4tmp' CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=0 CARGO_PROFILE_TEST_DEBUG=0
G="$T/🗑️generated"
if [ "$target" = --lib ]; then sel="--lib"; else sel="--test $target"; fi
cargo test --config 'build.rustc="rustc"' --config 'build.rustc-wrapper=""' $FHR4_EXTRA -j 3 -p semio-s-artifact-fem-$dim-editor $sel --no-fail-fast --message-format short -- "$@" > "$G/fhr4-$dim-$tag.log" 2>&1 </dev/null
echo "rc=$?" >> "$G/fhr4-$dim-$tag.log"
