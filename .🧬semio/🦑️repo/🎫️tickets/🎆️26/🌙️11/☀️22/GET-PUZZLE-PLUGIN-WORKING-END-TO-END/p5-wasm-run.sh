#!/usr/bin/env bash
T="$(cd "$(dirname "$0")" && pwd)"
LTO="${P5_LTO:-false}"
profile=()
while IFS= read -r s; do profile+=(--config "$s"); done < "$T/🗑️generated/p5-profile-selectors.txt"
profile+=(--config "profile.prototype.lto=$LTO")
cd "🌎️hub/🧩️compositions/🧩️puzzle/🧩️component/📦️packages/🦀️rust" || exit 1
cargo build --offline ${P5_LOCKED---locked} --lib --profile prototype --target wasm32-wasip2 --jobs "${P5_JOBS:-2}" "${profile[@]}"
