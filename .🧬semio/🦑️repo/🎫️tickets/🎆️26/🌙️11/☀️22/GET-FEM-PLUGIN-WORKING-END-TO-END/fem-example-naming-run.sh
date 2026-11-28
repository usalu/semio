#!/usr/bin/env bash
T="/c/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/GET-FEM-PLUGIN-WORKING-END-TO-END"
export CARGO_BUILD_BUILD_DIR='C:\x' CARGO_TARGET_DIR='C:\x' TMP='C:\xtmp' TEMP='C:\xtmp' TMPDIR='C:\xtmp' CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=0 CARGO_PROFILE_TEST_DEBUG=0 RUSTC_WRAPPER= RUST_MIN_STACK=134217728
A="/c/semio/✏️s/🔌️plugins/🏗️fem/🗿️artifacts"
for d in 3d 2d; do
  if [ $d = 2d ]; then DIR="$A/◻️2d"; PKG=semio-s-artifact-fem-2d-editor; else DIR="$A/🧊️3d"; PKG=semio-s-artifact-fem-3d-editor; fi
  (cd "$DIR" && cargo test -j 3 --config 'build.rustc="rustc"' --config 'build.rustc-wrapper=""' -p "$PKG" --lib -- set_active_example_options_equal) > "$T/🗑️generated/ex/$d.log" 2>&1
  echo "rc=$?" >> "$T/🗑️generated/ex/$d.log"
done
echo done > "$T/🗑️generated/ex/done4"
