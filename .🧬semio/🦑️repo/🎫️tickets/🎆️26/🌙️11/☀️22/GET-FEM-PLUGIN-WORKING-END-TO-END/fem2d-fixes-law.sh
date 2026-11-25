#!/usr/bin/env bash
# 🧪️ Runs the FEM 2D editor fix laws natively in its own target dir. Usage: fem2d-fixes-law.sh <name> <package> <cargo test args...> -- <filters>
T="/c/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/GET-FEM-PLUGIN-WORKING-END-TO-END"; mkdir -p "$T/🗑️generated/p2" /c/t/tmp
export TMP='C:\t\tmp' TEMP='C:\t\tmp' TMPDIR='C:\t\tmp' CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=0 CARGO_PROFILE_TEST_DEBUG=0 RUSTC_WRAPPER= RUST_MIN_STACK=134217728
A="/c/semio/✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d"
name="$1"; pkg="$2"; shift 2
(cd "$A" && CARGO_TARGET_DIR='C:\t\p2' cargo test -j 3 --config 'build.rustc="rustc"' --config 'build.rustc-wrapper=""' -p "$pkg" "$@") > "$T/🗑️generated/p2/$name.log" 2>&1
echo "rc=$?" >> "$T/🗑️generated/p2/$name.log"
grep -E "^test |test result|^error(\[|:)|rc=|panicked" "$T/🗑️generated/p2/$name.log" | head -60
