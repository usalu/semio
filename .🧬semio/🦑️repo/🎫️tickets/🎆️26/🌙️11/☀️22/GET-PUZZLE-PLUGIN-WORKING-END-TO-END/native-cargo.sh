#!/usr/bin/env bash
# 🧩 Native Rust runner with private tmpfs overlay (zero /home disk writes) + 2-slot compile semaphore.
# Usage:
#   native-cargo.sh <workdir-rel-or-abs> '<shell commands using cargo>'
# Env overrides:
#   TMPFS_SIZE=4500m   (default 4500m)
#   LOWER_DIR=...      (default warm x86_64 build cache)
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
export NATIVE_TMPFS_SIZE="${TMPFS_SIZE:-4500m}"
export NATIVE_LOWER_DIR="${LOWER_DIR:-$REPO_ROOT/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️18/DEV-PUZZLE-5D-FULLY-WORKING-END-TO-END/🗑️generated/native-target/b}"
export NATIVE_SLOT_LOCK_DIR="/dev/shm/semio-puzzle-cargo-slots"
mkdir -p "$NATIVE_SLOT_LOCK_DIR" "$TICKET_DIR/🗑️generated"

# Acquire one of 3 host-wide puzzle cargo slots so 15 GiB RAM never OOMs when 4+ agents run in parallel.
acquire_slot_and_run() {
  for slot in 1 2 3; do
    exec 9>"$NATIVE_SLOT_LOCK_DIR/slot-$slot.lock"
    if flock -n 9; then
      run_in_namespace
      return $?
    fi
  done
  # All slots busy: wait fairly on slot $((RANDOM % 3 + 1))
  slot=$((RANDOM % 3 + 1))
  exec 9>"$NATIVE_SLOT_LOCK_DIR/slot-$slot.lock"
  flock 9
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
    export CARGO_TARGET_DIR=/tmp/build/t
    export CARGO_BUILD_BUILD_DIR=/tmp/build/b
    export CARGO_INCREMENTAL=0
    export CARGO_PROFILE_DEV_DEBUG=0
    export CARGO_PROFILE_TEST_DEBUG=0
    export CARGO_PROFILE_DEV_OPT_LEVEL="${CARGO_PROFILE_DEV_OPT_LEVEL:-1}"
    export CARGO_PROFILE_TEST_OPT_LEVEL="${CARGO_PROFILE_TEST_OPT_LEVEL:-1}"
    export CARGO_PROFILE_DEV_CODEGEN_UNITS="${CARGO_CGU:-16}"
    export CARGO_PROFILE_TEST_CODEGEN_UNITS="${CARGO_CGU:-16}"
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
