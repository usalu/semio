#!/usr/bin/env bash
set -euo pipefail
ticket=$(cd "$(dirname "$0")" && pwd)
export SEMIO_WASM_NATIVE_REPO=$(cd "$ticket/../../../../../../.." && pwd)
export SEMIO_WASM_NATIVE_BUILD="$ticket/🗑️generated/w"
exec unshare --user --map-root-user --mount --propagation private bash -c '
  mount -t tmpfs -o mode=1777 tmpfs /tmp
  mkdir /tmp/build
  mount --bind "$SEMIO_WASM_NATIVE_BUILD" /tmp/build
  cd "$SEMIO_WASM_NATIVE_REPO"
  toolbin="$PWD/.cache/b/t/m/r/699c2d5d27a9c904/toolchains/nightly-2026-07-20-x86_64-unknown-linux-gnu/bin"
  export PATH="$toolbin:$PATH" RUSTC="$toolbin/rustc" RUSTC_WRAPPER=""
  export CARGO_HOME="$PWD/.cache/b/r/m/cargo/home"
  export CARGO_BUILD_BUILD_DIR=/tmp/build/b CARGO_TARGET_DIR=/tmp/build/t
  profile=()
  while IFS= read -r selector; do
    profile+=(--config "$selector")
  done < /tmp/build/profile-selectors
  test "${#profile[@]}" -gt 0
  cd "🌎️hub/🧩️compositions/🧩️puzzle/🧩️component/📦️packages/🦀️rust"
  exec "$toolbin/cargo" build --offline --locked --lib --profile prototype --target wasm32-unknown-unknown --jobs 1 "${profile[@]}"
'
