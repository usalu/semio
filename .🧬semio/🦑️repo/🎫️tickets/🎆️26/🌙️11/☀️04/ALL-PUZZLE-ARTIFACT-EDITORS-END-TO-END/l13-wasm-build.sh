#!/usr/bin/env bash
# native wasm build of @semio-tech/puzzle-plugin:wasm: tmpfs(4g) + overlay on the older on-disk wasm build dir; output copied to pkg/
set -eu
ticket=$(cd "$(dirname "$0")" && pwd)
export L13_REPO=$(cd "$ticket/../../../../../../.." && pwd) L13_TICKET="$ticket"
export L13_LOWER="$L13_REPO/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️18/DEV-PUZZLE-5D-FULLY-WORKING-END-TO-END/🗑️generated/w"
mkdir -p "$ticket/🗑️generated"
exec unshare --user --map-root-user --mount --propagation private bash -c '
  set -eu
  mount -t tmpfs -o mode=1777,size=4g tmpfs /tmp
  mkdir -p /tmp/build/t /tmp/ov/u /tmp/ov/w /tmp/build/b
  mount -t overlay overlay -o lowerdir="$L13_LOWER/b",upperdir=/tmp/ov/u,workdir=/tmp/ov/w,userxattr /tmp/build/b
  cd "$L13_REPO"
  toolbin="$PWD/.cache/b/t/m/r/699c2d5d27a9c904/toolchains/nightly-2026-07-20-x86_64-unknown-linux-gnu/bin"
  export PATH="$toolbin:$PATH" RUSTC="$toolbin/rustc" RUSTC_WRAPPER="" CARGO_HOME="$PWD/.cache/b/r/m/cargo/home"
  export CARGO_BUILD_BUILD_DIR=/tmp/build/b CARGO_TARGET_DIR=/tmp/build/t CARGO_INCREMENTAL=0
  profile=(); while IFS= read -r s; do profile+=(--config "$s"); done < "$L13_LOWER/profile-selectors"
  cd "🌎️hub/🧩️compositions/🧩️puzzle/🧩️component/📦️packages/🦀️rust"
  for try in $(seq 1 60); do
    if "$toolbin/cargo" build --offline --locked --lib --profile prototype --target wasm32-unknown-unknown --jobs 2 --message-format short "${profile[@]}" 2>&1 | grep -E "^(error|\s+(Compiling|Finished))|could not compile|SIGKILL" ; [ "${PIPESTATUS[0]}" = 0 ]; then break; fi
    echo "[retry $try] $(df -h /tmp | tail -1) $(free -m | sed -n 2p)"; sleep 20
  done
  out=/tmp/build/t/wasm32-unknown-unknown/prototype/semio_hub_puzzle_component.wasm
  pkg="$L13_REPO/🌎️hub/🧩️compositions/🧩️puzzle/📦️packages/🦀️rust/pkg"
  "$CARGO_HOME/bin/wasm-bindgen" "$out" --out-dir /tmp/bg --typescript --target web --out-name semio_puzzle
  cp /tmp/bg/semio_puzzle* "$pkg/"
  ls -la "$pkg"
'
