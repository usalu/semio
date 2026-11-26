#!/usr/bin/env bash
set -eu
root=$PWD
ticket="$(cd "$(dirname "$0")" && pwd)"
output="${SEMIO_LAW_NATIVE_ROOT:-$ticket/🗑️generated/law-storage-native}"
toolchain="$root/.cache/b/t/m/r/699c2d5d27a9c904/toolchains/nightly-2026-07-20-x86_64-unknown-linux-gnu/bin"
export CARGO_HOME="$root/.cache/b/r/m/cargo/home"
export ZIG_GLOBAL_CACHE_DIR="$output/zig" ZIG_LOCAL_CACHE_DIR="$output/zig-local"
export RUSTC="$toolchain/rustc" RUSTC_WRAPPER="" CARGO_BUILD_BUILD_DIR="$output/b" CARGO_TARGET_DIR="$output/t"
export RUST_MIN_STACK=134217728 CARGO_PROFILE_TEST_DEBUG=0 CARGO_INCREMENTAL=0 RUSTFLAGS="-Z threads=1"
if [ "${1-}" = "--puzzle-2d" ]; then
  shift
  cd "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d"
  exec "$toolchain/cargo" test --locked --offline -j1 -p semio-s-artifact-puzzle-2d-connect-handles-preparation --lib "$@"
fi
if [ "${1-}" = "--puzzle-2d-runtime" ]; then
  shift
  cd "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d"
  exec "$toolchain/cargo" test --locked --offline -j1 -p semio-s-artifact-puzzle-2d-history-edit-runtime-tests --lib "$@"
fi
if [ "${1-}" = "--puzzle-2d-lock" ]; then
  shift
  cd "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d"
  exec "$toolchain/cargo" tree --offline --depth 0 "$@"
fi
if [ "${1-}" = "--lock-workspace" ]; then
  shift
  cd "$1"
  shift
  exec "$toolchain/cargo" tree --offline --depth 0 "$@"
fi
if [ "${1-}" = "--store-backbone" ]; then
  shift
  exec "$toolchain/cargo" test --locked --offline -j1 -p semio-os-store-backbone --lib "$@"
fi
if [ "${1-}" = "--actor-input" ]; then
  shift
  exec "$toolchain/cargo" test --locked --offline -j1 --manifest-path "🧰️framework/Cargo.toml" \
    -p semio-plugin-actor-input --lib "$@"
fi
if [ "${1-}" = "--actor-input-contract" ]; then
  shift
  exec "$toolchain/cargo" check --locked --offline -j1 -p semio-plugin-guest-contract \
    --features original-actor-context-testing --lib "$@"
fi
if [ "${1-}" = "--paged-integration" ]; then
  shift
  exec "$toolchain/cargo" test --locked --offline -j1 --manifest-path "🧰️framework/Cargo.toml" \
    -p semio-framework-value-paged-clone --test borrowed "$@"
fi
if [ "${1-}" = "--retained-integration" ]; then
  shift
  exec "$toolchain/cargo" test --locked --offline -j1 --manifest-path "🧰️framework/Cargo.toml" \
    -p semio-framework-value-retained-clone --test original-string-work --test original-paged-clone --test take "$@"
fi
exec "$toolchain/cargo" test --locked --offline -j1 --manifest-path "🧰️framework/Cargo.toml" \
  -p semio-framework-value-control-reply -p semio-framework-value-native-clone-engine -p semio-framework-value-paged-clone \
  -p semio-framework-value-retained-clone --lib "$@"
