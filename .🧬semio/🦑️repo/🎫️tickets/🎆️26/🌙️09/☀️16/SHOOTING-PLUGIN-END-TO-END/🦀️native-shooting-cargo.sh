#!/usr/bin/env bash
# 🦀️ Native Rust runner for 🎥️shooting with private tmpfs overlay (zero /home disk writes) + host-wide slot semaphore.
set -euo pipefail

TICKET_DIR="$(cd "$(dirname "$0")" && pwd)"
REPO_ROOT="$(cd "$TICKET_DIR/../../../../../../.." && pwd)"
WORKDIR_ARG="${1:?missing workdir}"
SNIPPET="${2:?missing shell snippet}"

if [[ "$WORKDIR_ARG" = /* ]]; then
  TARGET_WORKDIR="$WORKDIR_ARG"
else
  TARGET_WORKDIR="$REPO_ROOT/$WORKDIR_ARG"
fi

export NATIVE_REPO_ROOT="$REPO_ROOT"
export NATIVE_WORKDIR="$TARGET_WORKDIR"
export NATIVE_SNIPPET="$SNIPPET"
export NATIVE_TMPFS_SIZE="${TMPFS_SIZE:-512m}"
export NATIVE_LOWER_DIR="${LOWER_DIR:-$REPO_ROOT/.cache/b/r/m/b/p}"
export NATIVE_SLOT_LOCK_DIR="/dev/shm/semio-shooting-cargo-slots"
export NATIVE_PERSIST_DIR="${PERSIST_DIR:-/dev/shm/semio-shooting-persist}"
mkdir -p "$NATIVE_SLOT_LOCK_DIR" "$NATIVE_PERSIST_DIR" "$TICKET_DIR/🗑️generated"

acquire_slot_and_run() {
  exec 7>"$NATIVE_SLOT_LOCK_DIR/exclusive.lock"
  flock 7
  exec 8>"$NATIVE_SLOT_LOCK_DIR/slot-1.lock"
  exec 9>"$NATIVE_SLOT_LOCK_DIR/slot-2.lock"
  exec 6>"$NATIVE_SLOT_LOCK_DIR/slot-3.lock"
  flock 8
  flock 9
  flock 6
  run_in_namespace
}

run_in_namespace() {
  unshare --user --map-root-user --mount --propagation private bash -c '
    set -euo pipefail
    mount -t tmpfs -o "mode=1777,size=$NATIVE_TMPFS_SIZE" tmpfs /tmp
    mkdir -p /tmp/build/t /tmp/ov/u /tmp/ov/w /tmp/build/b /tmp/zig /tmp/zig-local
    if [ -d "$NATIVE_LOWER_DIR" ]; then
      mount -t overlay overlay -o "lowerdir=$NATIVE_LOWER_DIR,upperdir=/tmp/ov/u,workdir=/tmp/ov/w,userxattr" /tmp/build/b
    fi
    toolbin="$NATIVE_REPO_ROOT/.cache/b/t/m/r/699c2d5d27a9c904/toolchains/nightly-2026-07-20-x86_64-unknown-linux-gnu/bin"
    bun_candidates=(/home/deck/.cache/semio/runtimes/bun/*/bun)
    bun_dir="$(dirname "${bun_candidates[0]}")"
    export PATH="$toolbin:$bun_dir:$PATH"
    export RUSTC="$toolbin/rustc"
    export RUSTC_WRAPPER=""
    export CARGO_HOME="$NATIVE_REPO_ROOT/.cache/b/r/m/cargo/home"
    export CARGO_TARGET_DIR="${CARGO_TARGET_DIR:-$NATIVE_PERSIST_DIR/target}"
    export CARGO_BUILD_BUILD_DIR="${CARGO_BUILD_BUILD_DIR:-$NATIVE_PERSIST_DIR/build}"
    mkdir -p "$CARGO_TARGET_DIR" "$CARGO_BUILD_BUILD_DIR"
    export CARGO_BUILD_JOBS="${CARGO_BUILD_JOBS:-1}"
    export CARGO_INCREMENTAL="${CARGO_INCREMENTAL:-1}"
    export CARGO_PROFILE_DEV_DEBUG=0
    export CARGO_PROFILE_TEST_DEBUG=0
    export CARGO_PROFILE_DEV_OPT_LEVEL="${CARGO_PROFILE_DEV_OPT_LEVEL:-0}"
    export CARGO_PROFILE_TEST_OPT_LEVEL="${CARGO_PROFILE_TEST_OPT_LEVEL:-0}"
    export CARGO_PROFILE_DEV_CODEGEN_UNITS="${CARGO_PROFILE_DEV_CODEGEN_UNITS:-16}"
    export CARGO_PROFILE_TEST_CODEGEN_UNITS="${CARGO_PROFILE_TEST_CODEGEN_UNITS:-16}"
    export CARGO_PROFILE_DEV_DEBUG_ASSERTIONS=true
    export CARGO_PROFILE_DEV_OVERFLOW_CHECKS=true
    export ZIG_GLOBAL_CACHE_DIR=/tmp/zig
    export ZIG_LOCAL_CACHE_DIR=/tmp/zig-local
    export TMPDIR=/tmp
    export RUST_MIN_STACK="${RUST_MIN_STACK:-134217728}"
    export RUSTFLAGS="${RUSTFLAGS:--Z threads=1 -C linker-features=+lld -C link-self-contained=+linker -Z unstable-options -C embed-bitcode=no -C strip=symbols -Zembed-metadata=no}"
    cd "$NATIVE_WORKDIR"
    eval "$NATIVE_SNIPPET"
  '
}

acquire_slot_and_run
