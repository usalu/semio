#!/bin/bash
# Builds one hub component cold with a given profile and codegen-unit count; records wall time, peak RSS, wasm size and function count.
# usage: 🧪️cgu-experiment.sh <package> <profile> <cgu> <tag>
cd /Users/ueli/Documents/semio || exit 2
PKG="$1"; PROFILE="$2"; CGU="$3"; TAG="$4"
T=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️23/BUILD-SEMIO-TECH-PLAY-FOR-CDN-DEPLOYMENT"
OUT="$T/🗑️generated/cgu-$TAG"; mkdir -p "$OUT"
export CARGO_TARGET_DIR="$PWD/.🧬semio/🦑️repo/⚡️cache/x$TAG"
export CARGO_INCREMENTAL=0
start=$(date +%s)
/usr/bin/time -l cargo rustc --locked --manifest-path 🌎️hub/Cargo.toml -p "$PKG" --profile "$PROFILE" --lib --crate-type cdylib --target wasm32-wasip2 --config "profile.$PROFILE.codegen-units=$CGU" --config "profile.$PROFILE.package.semio-framework-os-kernel.codegen-units=$CGU" --message-format short > "$OUT/build.log" 2> "$OUT/time.log"
rc=$?
end=$(date +%s)
W=$(ls "$CARGO_TARGET_DIR/wasm32-wasip2/$PROFILE/"*.wasm 2>/dev/null | head -1)
echo "rc=$rc wall=$((end-start))s maxrss=$(grep 'maximum resident' "$OUT/time.log" | awk '{print $1}') wasm=$W size=$(stat -f %z "$W" 2>/dev/null)" > "$OUT/result.txt"
grep -E "error|ElemSection|LLVM ERROR|panicked" "$OUT/time.log" "$OUT/build.log" | head -5 >> "$OUT/result.txt"
cat "$OUT/result.txt"
