#!/bin/bash
# 📏️ Like 📏️unit-measure.sh, but for a scratch workspace (a symlinked copy of a workspace root with one broken upstream
# crate replaced by its HEAD copy): builds the closure once, then cleans and rebuilds each package alone, reporting the
# rebuild wall and the sampled peak RSS and rustc seconds of each unit.
# usage: 📏️scratch-unit-measure.sh <tag> <scratch Cargo.toml> <package>... (env PROFILE=wasm-release TARGET=wasm32-wasip2)
TAG=$1; MAN=$2; shift 2; PROFILE=${PROFILE:-wasm-release}; TARGET=${TARGET:-wasm32-wasip2}
HERE="$(cd "$(dirname "$0")" && pwd)"; E4=/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/e4
cd "$(dirname "$MAN")" || exit 2
ARGS=(); for p in "$@"; do ARGS+=(-p "$p"); done
CARGO_TARGET_DIR="$E4/t" CARGO_BUILD_BUILD_DIR="$E4/b" bash "$HERE/../🚦️cargo-slot.sh" "e4-$TAG" -- cargo build --offline --manifest-path "$MAN" "${ARGS[@]}" --lib --target "$TARGET" --profile "$PROFILE" > "$E4/logs/$TAG-deps.log" 2>&1 || { grep -m3 -A5 '^error' "$E4/logs/$TAG-deps.log"; exit 1; }
for p in "$@"; do
  CARGO_TARGET_DIR="$E4/t" CARGO_BUILD_BUILD_DIR="$E4/b" cargo clean --offline --manifest-path "$MAN" -p "$p" --target "$TARGET" --profile "$PROFILE" >/dev/null 2>&1
  rm -f "$E4/peaks/$TAG-$p.tsv" "$E4/logs/$TAG-$p.log"
  out=$(cd "$(dirname "$MAN")" && CARGO_TARGET_DIR="$E4/t" CARGO_BUILD_BUILD_DIR="$E4/b" bash "$HERE/🏗️cargo.sh" "$TAG-$p" 540 build --offline --manifest-path "$MAN" -p "$p" --lib --target "$TARGET" --profile "$PROFILE" 2>&1 | grep -m1 '^rc=')
  echo "$p | $out | units=$(grep -cE '^\s+Compiling' "$E4/logs/$TAG-$p.log") | $(awk -F'\t' -v n="$(echo "$p" | tr - _)" '$2==n{print $1" "$3}' "$E4/peaks/$TAG-$p.tsv")"
done
