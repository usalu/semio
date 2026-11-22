#!/usr/bin/env bash
# usage: l1-native.sh <2d|3d> <cargo args...>   (e.g. check --workspace --all-targets)
set -eu
ticket=$(cd "$(dirname "$0")" && pwd)
repo_root=$(cd "$ticket/../../../../../../.." && pwd)
dim=$1; shift
case "$dim" in 2d) art="◻️2d";; 3d) art="🧊️3d";; esac
build="$ticket/🗑️generated/l1d-target-$dim"
mkdir -p "$build"
cd "$repo_root"
export L1_TICKET="$ticket" L1_BUILD="$build" L1_REPO="$repo_root" L1_MANIFEST="$repo_root/✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/$art/Cargo.toml"
exec unshare --user --map-root-user --mount --propagation private bash -c '
  mount -t tmpfs -o mode=1777,size=4g tmpfs /tmp
  mkdir /tmp/build
  :
  cd "$L1_REPO"
  toolbin="$PWD/.cache/b/t/m/r/699c2d5d27a9c904/toolchains/nightly-2026-07-20-x86_64-unknown-linux-gnu/bin"
  runtime=(/home/deck/.cache/semio/runtimes/bun/*/bun)
  export CARGO_HOME="$PWD/.cache/b/r/m/cargo/home"
  export CARGO_TARGET_DIR=/tmp/build/t CARGO_BUILD_BUILD_DIR=/tmp/build/b
  export CARGO_INCREMENTAL=0 CARGO_PROFILE_TEST_DEBUG=0 CARGO_PROFILE_DEV_DEBUG=0 RUSTC_WRAPPER=""
  export PATH="$toolbin:${runtime[0]%/bun}:$PATH" RUSTC="$toolbin/rustc" RUST_MIN_STACK=67108864
  export RUSTFLAGS="-Z threads=1 -C linker-features=+lld -C link-self-contained=+linker -Z unstable-options -C embed-bitcode=no"
  G="$L1_TICKET/🗑️generated"
  while :; do
    while [ ! -s "$G/l1d-req" ]; do sleep 2; done
    mapfile -t a < "$G/l1d-req"; rm -f "$G/l1d-req"
    [ "${a[0]}" = quit ] && exit 0
    out="${a[0]}"; unset "a[0]"
    cargo "${a[@]}" --manifest-path "$L1_MANIFEST" --jobs 2 --offline > "$G/$out" 2>&1
    echo "exit=$?" >> "$G/$out"; echo done >> "$G/$out"
  done
' bash "$@"
