#!/usr/bin/env bash
# 🧪️ FEM 2D interaction/results fix laws, native, private dirs C:\t\fx2. Usage: fx2d-law.sh <tag> <2d|3d> <package> <--lib|--test NAME> [filter]
T="/c/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/GET-FEM-PLUGIN-WORKING-END-TO-END"; mkdir -p "$T/🗑️generated/fx2" /c/t/fx2tmp
export CARGO_BUILD_BUILD_DIR='C:\t\fx2' CARGO_TARGET_DIR='C:\t\fx2' TMP='C:\t\fx2tmp' TEMP='C:\t\fx2tmp' TMPDIR='C:\t\fx2tmp' CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=0 CARGO_PROFILE_TEST_DEBUG=0 RUSTC_WRAPPER= RUST_MIN_STACK=134217728
LOG="$T/🗑️generated/fx2/$1.log"
A="/c/semio/✏️s/🔌️plugins/🏗️fem/🗿️artifacts"
if [ "$2" = 2d ]; then DIR="$A/◻️2d"; else DIR="$A/🧊️3d"; fi
shift 2; PKG="$1"; shift
if [ "$1" = "--lib" ]; then SEL="--lib"; shift; else SEL="$1 $2"; shift 2; fi
(cd "$DIR" && cargo test -j 3 --config 'unstable.build-dir-new-layout=false' --config 'build.rustc="rustc"' --config 'build.rustc-wrapper=""' -p "$PKG" $SEL -- --nocapture "$@") > "$LOG" 2>&1
echo "rc=$?" >> "$LOG"
