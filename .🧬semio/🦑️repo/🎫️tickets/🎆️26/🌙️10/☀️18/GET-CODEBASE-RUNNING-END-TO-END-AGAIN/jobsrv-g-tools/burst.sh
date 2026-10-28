#!/bin/sh
# 🔬 usage: [UC=<unit-cache dir>] [JOBS=4] [FILES=8] [TAIL=6] [CARGO_FROM=<dir with a jobserver parent>] burst.sh <plain|cache> <tag>
# Real Cargo (pinned toolchain) builds 4 crates whose build scripts compile FILES C files each with the `cc` crate (parallel feature) through $J/slowcc,
# which records how many compile jobs were alive at once. plain = no wrapper; cache = the unit-cache wrapper + build script stand-ins from $UC (cold lane, cold cache).
# Prints: max concurrent compile jobs, the grant (-j), wall seconds.
ROOT=/Users/ueli/Documents/semio
C="$ROOT/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
J="$C/jobsrv-g-tools"
T="$ROOT/.cache/b/t/m/r/toolchains/nightly-2026-07-20-aarch64-apple-darwin/bin"
UC="${UC:-$ROOT/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache}"
BUN="${BUN:-$(command -v bun)}"
mode="$1"; tag="${2:-x}"; JOBS="${JOBS:-4}"; FILES="${FILES:-8}"
R="$C/🗑️generated/jobsrv-g/b-$tag-$$"; rm -rf "$R"; mkdir -p "$R/bin" "$R/lane" "$R/js"
: > "$R/js.log"
set -- env RUSTC="$T/rustc" JS_DIR="$R/js" JS_LOG="$R/js.log" JS_SLEEP="${JS_SLEEP:-0.6}" CC="$J/slowcc" BURST_FILES="$FILES" CARGO_TARGET_DIR="$R/target" CARGO_BUILD_BUILD_DIR="$R/build" CARGO_INCREMENTAL=0
if [ "$mode" = cache ]; then
 "$BUN" -e "import {unitCacheShimSourceV1} from '$UC/🚪️launch/🟦️.ts';process.stdout.write(unitCacheShimSourceV1('$BUN','$UC/📜️script.ts'))" > "$R/bin/rustc-wrapper"; chmod 755 "$R/bin/rustc-wrapper"
 set -- "$@" RUSTC_WRAPPER="$R/bin/rustc-wrapper" SEMIO_UNIT_CACHE_ROOT="$R" SEMIO_UNIT_CACHE_LANE=w SEMIO_UNIT_CACHE_LANE_ROOT="$R/lane" SEMIO_UNIT_CACHE_INNER="[\"$T/rustc\"]" SEMIO_UNIT_CACHE_TOOLCHAIN=x SEMIO_UNIT_CACHE_SLOTS=4 SEMIO_UNIT_CACHE_FOCUS= SEMIO_UNIT_CACHE_BUILD_SCRIPTS=on
else
 set -- "$@" RUSTC_WRAPPER=
fi
cd "$J/burst" || exit 2
start=$(date +%s)
if [ -n "$OUTER" ]; then python3 "$J/outer.py" "$OUTER" "$JOBS" -- "$@" $T/cargo build --offline ${CARGO_ARGS:-} > "$R/cargo.log" 2>&1; rc=$?
else "$@" $T/cargo build --offline -j"$JOBS" ${CARGO_ARGS:-} > "$R/cargo.log" 2>&1; rc=$?; fi
end=$(date +%s)
max=$(sort -n "$R/js.log" | tail -1); jobs=$(wc -l < "$R/js.log" | tr -d ' ')
echo "mode=$mode outer=${OUTER:-cargo} tag=$tag rc=$rc grant=$JOBS files/crate=$FILES x4 compile-jobs=$jobs max-concurrent=${max:-0} wall=$((end-start))s log=$R/cargo.log"
[ "$rc" = 0 ] || tail -${TAIL:-15} "$R/cargo.log"
[ -n "$KEEP" ] || rm -rf "$R/target" "$R/build" "$R/lane" "$R/u" "$R/bin"
