#!/usr/bin/env bash
# build dir = tmpfs overlay (read-only lower: prior ticket native build cache); usage: l2-native.sh <log-tag> '<shell snippet using $CARGO>'  ; build dir lives on tmpfs because /home is full
set -eu
ticket=$(cd "$(dirname "$0")" && pwd)
repo_root=$(cd "$ticket/../../../../../../.." && pwd)
mkdir -p "$ticket/🗑️generated"
export L2_LOWER="/home/deck/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️18/DEV-PUZZLE-5D-FULLY-WORKING-END-TO-END/🗑️generated/native-target/b" L2_REPO="$repo_root" L2_SNIPPET="$2" L2_SIZE="${L2_SIZE:-6g}"
exec unshare --user --map-root-user --mount --propagation private bash -c '
  mount -t tmpfs -o mode=1777,size=$L2_SIZE tmpfs /tmp
  mkdir -p /tmp/build/t /tmp/ov/u /tmp/ov/w /tmp/build/b
  mount -t overlay overlay -o lowerdir="$L2_LOWER",upperdir=/tmp/ov/u,workdir=/tmp/ov/w,userxattr /tmp/build/b
  cd "$L2_REPO"
  toolbin="$PWD/.cache/b/t/m/r/699c2d5d27a9c904/toolchains/nightly-2026-07-20-x86_64-unknown-linux-gnu/bin"
  runtime=(/home/deck/.cache/semio/runtimes/bun/*/bun)
  export CARGO_HOME="$PWD/.cache/b/r/m/cargo/home"
  export CARGO_TARGET_DIR=/tmp/build/t CARGO_BUILD_BUILD_DIR=/tmp/build/b
  export CARGO_INCREMENTAL=0 CARGO_PROFILE_TEST_DEBUG=0 CARGO_PROFILE_DEV_DEBUG=0 RUSTC_WRAPPER=""
  export PATH="$toolbin:${runtime[0]%/bun}:$PATH" RUSTC="$toolbin/rustc" RUST_MIN_STACK=67108864
  export RUSTFLAGS="-Z threads=1 -C linker-features=+lld -C link-self-contained=+linker -Z unstable-options"
  export P5="$PWD/✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/📦️packages/🦀️rust"
  eval "$L2_SNIPPET"
' bash
