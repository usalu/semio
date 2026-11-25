#!/usr/bin/env bash
set -uo pipefail
T="$(cd "$(dirname "$0")" && pwd)"
R="$(cd "$T/../../../../../../.." && pwd)"
SCRATCH="${P5_SCRATCH:?set P5_SCRATCH to a local directory with enough space; no default}"
mkdir -p "$SCRATCH/t" "$SCRATCH/b"
toolbin="$R/.cache/b/t/m/r/699c2d5d27a9c904/toolchains/nightly-2026-07-20-x86_64-unknown-linux-gnu/bin"
export PATH="$toolbin:$PATH" RUSTC="$toolbin/rustc" RUSTC_WRAPPER=""
export CARGO_HOME="$R/.cache/b/r/m/cargo/home" CARGO_TARGET_DIR="$SCRATCH/t" CARGO_BUILD_BUILD_DIR="$SCRATCH/b" TMPDIR="$SCRATCH/tmp"
mkdir -p "$TMPDIR"
profile=()
while IFS= read -r s; do profile+=(--config "$s"); done < "$T/🗑️generated/p5-profile-selectors.txt"
cd "$R/🌎️hub/🧩️compositions/🧩️puzzle/🧩️component/📦️packages/🦀️rust"
exec "$toolbin/cargo" build --offline --locked --lib --profile prototype --target wasm32-wasip2 --jobs "${P5_JOBS:-4}" "${profile[@]}"
