#!/usr/bin/env bash
# 🧪️ Single-item wallet laws, private dirs C:\t\sw. Usage: sw-law.sh <tag> <2d|3d|fw> <package> <--lib|--test NAME> [filter]
T="/c/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/GET-FEM-PLUGIN-WORKING-END-TO-END"; mkdir -p "$T/🗑️generated/sw" /c/t/swtmp
export CARGO_BUILD_BUILD_DIR='C:\t\sw' CARGO_TARGET_DIR='C:\t\sw' TMP='C:\t\swtmp' TEMP='C:\t\swtmp' TMPDIR='C:\t\swtmp' CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=0 CARGO_PROFILE_TEST_DEBUG=0 RUSTC_WRAPPER= RUST_MIN_STACK=134217728
LOG="$T/🗑️generated/sw/$1.log"
A="/c/semio/✏️s/🔌️plugins/🏗️fem/🗿️artifacts"
case "$2" in 2d) DIR="$A/◻️2d";; 3d) DIR="$A/🧊️3d";; fw) DIR="/c/semio/🧰️framework/🔨️modules/🧵️job/📬️outcome/🪙️budget/📦️packages/🦀️rust";; esac
shift 2; PKG="$1"; shift
if [ "$1" = "--lib" ]; then SEL="--lib"; shift; else SEL="$1 $2"; shift 2; fi
(cd "$DIR" && cargo test -j 3 --config 'unstable.build-dir-new-layout=false' --config 'build.rustc="rustc"' --config 'build.rustc-wrapper=""' -p "$PKG" $SEL -- --nocapture "$@") > "$LOG" 2>&1
echo "rc=$?" >> "$LOG"
