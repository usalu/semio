#!/usr/bin/env bash
T="$(cd "$(dirname "$0")" && pwd)"
profile=()
while IFS= read -r s; do profile+=(--config "$s"); done < "$T/🗑️generated/p5-profile-selectors.txt"
profile+=(--config "profile.prototype.lto=false")
cd "🌎️hub/🧩️compositions/🧩️puzzle/🧩️component/📦️packages/🦀️rust" || exit 1
cargo rustc --offline --locked --lib -p semio-s-artifact-stdio-semio-brep-snapshot --profile prototype --target wasm32-wasip2 --jobs 1 "${profile[@]}" -- -Zmacro-backtrace
