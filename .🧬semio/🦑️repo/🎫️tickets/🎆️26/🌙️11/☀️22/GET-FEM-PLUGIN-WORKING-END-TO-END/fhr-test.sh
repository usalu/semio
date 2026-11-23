#!/usr/bin/env bash
# usage: fhr-test.sh <2d|3d> <tag> <test-target> [test filter...]
T="/c/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/GET-FEM-PLUGIN-WORKING-END-TO-END"
dim="$1"; tag="$2"; target="$3"; shift 3
if [ "$dim" = 2d ]; then
  cd "/c/semio/✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d" || exit 2
  export CARGO_TARGET_DIR='C:\t\2dh' CARGO_BUILD_BUILD_DIR='C:\t\2dhb'
else
  cd "/c/semio/✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d" || exit 2
  export CARGO_TARGET_DIR='C:\t\3d' CARGO_BUILD_BUILD_DIR='C:\t\3db'
fi
export TMP='C:\t\tmp' TEMP='C:\t\tmp' TMPDIR='C:\t\tmp' CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=0 CARGO_PROFILE_TEST_DEBUG=0
G="$T/🗑️generated"
cargo test --config 'build.rustc="rustc"' --config 'build.rustc-wrapper=""' -j 6 -p semio-s-artifact-fem-$dim-editor --test "$target" --no-fail-fast --message-format short -- "$@" > "$G/fhr-r2-$dim-$tag.log" 2>&1 </dev/null
echo "rc=$?" >> "$G/fhr-r2-$dim-$tag.log"
