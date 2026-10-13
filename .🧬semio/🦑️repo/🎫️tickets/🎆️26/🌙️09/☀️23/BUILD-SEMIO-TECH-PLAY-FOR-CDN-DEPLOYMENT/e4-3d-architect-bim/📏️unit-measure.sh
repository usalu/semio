#!/bin/bash
# 📏️ Measures one library unit alone: builds its closure (private E4 dirs), cleans only that package, rebuilds it and
# reports the rebuild wall (the unit is the only thing compiled) and the unit's sampled peak RSS.
# usage: 📏️unit-measure.sh <tag> <manifest> <package> [profile=wasm-release] [target=wasm32-wasip2]
TAG=$1; MANIFEST=$2; PKG=$3; PROFILE=${4:-wasm-release}; TARGET=${5:-wasm32-wasip2}
HERE="$(cd "$(dirname "$0")" && pwd)"; E4=/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/e4
CARGO="$HERE/🏗️cargo.sh"
rm -f "$E4/logs/$TAG.log" "$E4/peaks/$TAG.tsv"
bash "$CARGO" "$TAG-deps" 540 build --offline --manifest-path "$MANIFEST" -p "$PKG" --lib --target "$TARGET" --profile "$PROFILE" >/dev/null; RC=$?; [ $RC = 130 ] && { echo "closure build interrupted after its slice: rerun to resume"; exit 3; }; [ $RC != 0 ] && { echo "closure build failed"; grep -m3 -A6 '^error' "$E4/logs/$TAG-deps.log"; exit 1; }
env CARGO_TARGET_DIR="$E4/t" CARGO_BUILD_BUILD_DIR="$E4/b" cargo clean --offline --manifest-path "/Users/ueli/Documents/semio/$MANIFEST" -p "$PKG" --target "$TARGET" --profile "$PROFILE" 2>&1 | tail -1
bash "$CARGO" "$TAG" 540 build --offline --manifest-path "$MANIFEST" -p "$PKG" --lib --target "$TARGET" --profile "$PROFILE" | head -1
echo "units compiled: $(grep -cE '^\s+Compiling' "$E4/logs/$TAG.log") ($(grep -E '^\s+Compiling' "$E4/logs/$TAG.log" | awk '{print $2}' | tr '\n' ' '))"
echo "peak: $(awk -F"\t" -v n="$(echo "$PKG" | tr - _)" '$2==n' "$E4/peaks/$TAG.tsv")"
