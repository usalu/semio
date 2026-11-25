#!/usr/bin/env bash
set -eu

ticket="$(cd "$(dirname "$0")" && pwd)"
root="$(cd "$ticket/../../../../../../.." && pwd)"
output="$ticket/🗑️generated"
lane="$output/builder/lane-wasip2"
tools="$root/.cache/b/t/m/r/699c2d5d27a9c904/toolchains/nightly-2026-07-20-x86_64-unknown-linux-gnu/bin"
bun=(/home/deck/.cache/semio/runtimes/bun/*/bun)

export PATH="$tools:$(dirname "${bun[0]}"):$PATH"
export RUSTC="$tools/rustc" RUSTC_WRAPPER=""
export CARGO_HOME="$root/.cache/b/r/m/cargo/home" CARGO_TARGET_DIR="$lane/t" CARGO_BUILD_BUILD_DIR="$lane/b"
export CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=0 CARGO_PROFILE_TEST_DEBUG=0
export CARGO_PROFILE_DEV_OPT_LEVEL=1 CARGO_PROFILE_TEST_OPT_LEVEL=1
export CARGO_PROFILE_DEV_CODEGEN_UNITS=16 CARGO_PROFILE_TEST_CODEGEN_UNITS=16
export CARGO_PROFILE_DEV_DEBUG_ASSERTIONS=true CARGO_PROFILE_DEV_OVERFLOW_CHECKS=true
export RUST_MIN_STACK=134217728
export RUSTFLAGS="-Z threads=1 -C linker-features=+lld -C link-self-contained=+linker -Z unstable-options -C embed-bitcode=no -C strip=symbols -Zembed-metadata=no"
export TMPDIR="$output/guest-tmp" ZIG_GLOBAL_CACHE_DIR="$output/guest-zig" ZIG_LOCAL_CACHE_DIR="$output/guest-zig-local"
mkdir -p "$TMPDIR" "$ZIG_GLOBAL_CACHE_DIR" "$ZIG_LOCAL_CACHE_DIR"

cd "$root"
mapfile -t selectors < <("${bun[0]}" "$ticket/p5-profile.ts")
test "${#selectors[@]}" -gt 0
profile=()
for selector in "${selectors[@]}"; do
  profile+=(--config "$selector")
done

cd '🌎️hub/🧩️compositions/🧩️puzzle/🧩️component/📦️packages/🦀️rust'
exec cargo build --offline --locked --lib --profile prototype --target wasm32-wasip2 --jobs 1 "${profile[@]}" --config profile.prototype.lto=false
