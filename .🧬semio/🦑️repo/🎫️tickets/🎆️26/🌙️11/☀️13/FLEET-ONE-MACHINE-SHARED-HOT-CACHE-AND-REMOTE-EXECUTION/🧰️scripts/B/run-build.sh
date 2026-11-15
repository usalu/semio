#!/bin/sh
# usage: run-build.sh <checkout> <cache root> <lane> [extra cargo args...]   (one cargo build through the unit-cache wrapper, with its own CAS next to the cache root)
set -e
BUN=/Users/ueli/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
SCRIPT="/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache/📜️script.ts"
[ -n "$UNIT_SCRIPT" ] && SCRIPT="$UNIT_SCRIPT"
CHECKOUT="$1"; CACHE="$2"; LANE="$3"; shift 3
LANE_ROOT="$CACHE/l/$LANE"
mkdir -p "$LANE_ROOT"
cd "$CHECKOUT"
exec env -u CARGO_MAKEFLAGS -u MAKEFLAGS -u MFLAGS -u CARGO_BUILD_JOBS RUSTC_WRAPPER="$BUN" RUSTC="$SCRIPT" \
  SEMIO_UNIT_CACHE_ROOT="$CACHE" SEMIO_UNIT_CACHE_LANE="$LANE" SEMIO_UNIT_CACHE_LANE_ROOT="$LANE_ROOT" \
  SEMIO_UNIT_CACHE_SLOTS=4 SEMIO_UNIT_CACHE_INNER='["rustc"]' SEMIO_UNIT_CACHE_BUILD_SCRIPTS=on \
  SEMIO_CAS_ROOT="$CACHE/../cas-$(basename "$CACHE")" CARGO_TARGET_DIR="$LANE_ROOT/t" CARGO_TERM_COLOR=never \
  cargo build --offline "$@"
