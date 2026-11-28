#!/usr/bin/env bash
# 🧪️ Runs only the 3D viewer law (private C:\t\vw), after viewer-fixes-law.sh warmed the shared target.
T="/c/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/GET-FEM-PLUGIN-WORKING-END-TO-END"; G="$T/🗑️generated/viewer-fixes"
export CARGO_TARGET_DIR='C:\t\vw' CARGO_BUILD_BUILD_DIR='C:\t\vw' TMP='C:\t\vwtmp' TEMP='C:\t\vwtmp' TMPDIR='C:\t\vwtmp' CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=0 CARGO_PROFILE_TEST_DEBUG=0 RUSTC_WRAPPER= RUST_MIN_STACK=134217728
(cd "/c/semio/✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d" && cargo test -j 3 --config 'build.rustc="rustc"' --config 'build.rustc-wrapper=""' -p semio-s-artifact-fem-3d-viewer --lib) > "$G/law-3d-only.log" 2>&1; echo "rc=$?" >> "$G/law-3d-only.log"
