#!/usr/bin/env bash
# 📋️ Runs the FEM 3D clipboard laws, checks the 2D editor and reruns the store alternative law natively in vc3.
T="/c/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/GET-FEM-PLUGIN-WORKING-END-TO-END"; mkdir -p "$T/🗑️generated/vc3" /c/t/tmp
export TMP='C:\t\tmp' TEMP='C:\t\tmp' TMPDIR='C:\t\tmp' CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=0 CARGO_PROFILE_TEST_DEBUG=0 RUSTC_WRAPPER= RUST_MIN_STACK=134217728
A="/c/semio/✏️s/🔌️plugins/🏗️fem/🗿️artifacts"; C=(--config 'build.rustc="rustc"' --config 'build.rustc-wrapper=""')
G="$T/🗑️generated/vc3"
(cd "$A/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/📦️packages/🦀️rust" && CARGO_TARGET_DIR='C:\t\vc3' cargo test -j 3 "${C[@]}" -p semio-s-artifact-fem-3d-editor --lib -- clipboard) > "$G/clip3d.log" 2>&1; echo "rc=$?" >> "$G/clip3d.log"
(cd "$A/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/📦️packages/🦀️rust" && CARGO_TARGET_DIR='C:\t\vc3' cargo check -j 3 "${C[@]}" -p semio-s-artifact-fem-2d-editor --lib) > "$G/clip2d.log" 2>&1; echo "rc=$?" >> "$G/clip2d.log"
"$T/history-alternative-law.sh" > /dev/null 2>&1
for f in clip3d clip2d alternative; do echo "== $f"; grep -E "^test |test result|^error(\[|:)|rc=" -A6 "$G/$f.log" | cut -c1-220 | head -40; done
