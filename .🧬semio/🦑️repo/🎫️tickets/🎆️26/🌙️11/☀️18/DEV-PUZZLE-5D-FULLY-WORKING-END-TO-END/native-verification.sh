#!/usr/bin/env bash
set -eu
ticket=$(cd "$(dirname "$0")" && pwd)
repo_root=$(cd "$ticket/../../../../../../.." && pwd)
build="$ticket/🗑️generated/${SEMIO_5D_NATIVE_TARGET:-native-target}"
mkdir -p "$build"
cd "$repo_root"
export SEMIO_5D_NATIVE_BUILD="$build"
export SEMIO_5D_NATIVE_REPO="$repo_root"
exec unshare --user --map-root-user --mount --propagation private bash -c '
  mount -t tmpfs -o mode=1777 tmpfs /tmp
  mkdir /tmp/build
  mount --bind "$SEMIO_5D_NATIVE_BUILD" /tmp/build
  cd "$SEMIO_5D_NATIVE_REPO"
  toolbin="$PWD/.cache/b/t/m/r/699c2d5d27a9c904/toolchains/nightly-2026-07-20-x86_64-unknown-linux-gnu/bin"
  runtime=(/home/deck/.cache/semio/runtimes/bun/*/bun)
  export CARGO_HOME="$PWD/.cache/b/r/m/cargo/home"
  export CARGO_TARGET_DIR=/tmp/build/t CARGO_BUILD_BUILD_DIR=/tmp/build/b
  export CARGO_INCREMENTAL=0 CARGO_PROFILE_TEST_DEBUG=0 CARGO_PROFILE_DEV_DEBUG=0 RUSTC_WRAPPER=""
  export PATH="$toolbin:${runtime[0]%/bun}:$PATH" RUSTC="$toolbin/rustc" RUST_MIN_STACK=67108864
  export RUSTFLAGS="-Z threads=1 -C linker-features=+lld -C link-self-contained=+linker -Z unstable-options"
  cargo_lock="$SEMIO_5D_NATIVE_REPO/✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/Cargo.lock"
  if [ "${SEMIO_5D_NATIVE_UNLOCKED:-0}" = 1 ]; then
    lock_copy="$(dirname "$SEMIO_5D_NATIVE_BUILD")/native-puzzle5d-Cargo.lock"
    cp "$cargo_lock" "$lock_copy"
    mount --bind "$lock_copy" "$cargo_lock"
    cargo metadata --offline --format-version=1 --manifest-path "${cargo_lock%Cargo.lock}📦️packages/🦀️rust/Cargo.toml" --filter-platform x86_64-unknown-linux-gnu > /dev/null
  fi
  exec cargo test --jobs "${SEMIO_5D_NATIVE_JOBS:-4}" --config "profile.test.package.semio-s-artifact-puzzle-5d-v1.strip=false" --locked --offline "$@"
' bash "$@"
