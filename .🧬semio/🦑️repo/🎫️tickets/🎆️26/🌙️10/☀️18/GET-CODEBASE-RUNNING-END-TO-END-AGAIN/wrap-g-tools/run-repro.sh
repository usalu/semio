#!/bin/sh
# 🔬 usage: [SCRIPT=<ts>] run-repro.sh <bun-binary> [VAR=value ...]
# Cold lane + cold unit cache: Cargo builds `libc`, whose build script runs `$RUSTC_WRAPPER $RUSTC --version`.
ROOT=/Users/ueli/Documents/semio
C="$ROOT/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
T="$ROOT/.cache/b/t/m/r/toolchains/nightly-2026-07-20-aarch64-apple-darwin/bin"
UC="$ROOT/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache/📜️script.ts"
BUN="$1"; shift
R="$C/🗑️generated/wrap-g/cache-$$"; rm -rf "$R"; mkdir -p "$R/bin" "$R/lane"
LAUNCH="$ROOT/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache/🚪️launch/🟦️.ts"
"$BUN" -e "import {unitCacheShimSourceV1} from '$LAUNCH';process.stdout.write(unitCacheShimSourceV1('$BUN','${SCRIPT:-$UC}'))" > "$R/bin/rustc-wrapper"; chmod 755 "$R/bin/rustc-wrapper"
cd "$C/wrap-g-tools/${CRATE:-repro}" || exit 2
env "$@" RUSTC="${RUSTC_VALUE:-$T/rustc}" RUSTC_WRAPPER="$R/bin/rustc-wrapper" SEMIO_UNIT_CACHE_ROOT="$R" SEMIO_UNIT_CACHE_LANE=w \
 SEMIO_UNIT_CACHE_LANE_ROOT="$R/lane" SEMIO_UNIT_CACHE_INNER="[\"$T/rustc\"]" SEMIO_UNIT_CACHE_TOOLCHAIN=x \
 SEMIO_UNIT_CACHE_SLOTS=4 SEMIO_UNIT_CACHE_FOCUS= SEMIO_UNIT_CACHE_BUILD_SCRIPTS=on \
 CARGO_TARGET_DIR="$R/target" CARGO_BUILD_BUILD_DIR="$R/build" "${CARGO:-$T/cargo}" build --offline -j4 2>&1 | tail -${TAIL:-12}
rm -rf "$R"
