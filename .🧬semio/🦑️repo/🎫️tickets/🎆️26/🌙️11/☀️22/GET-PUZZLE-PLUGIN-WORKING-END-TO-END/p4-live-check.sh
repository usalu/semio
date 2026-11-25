#!/usr/bin/env bash
# Read-only check of LIVE sources (hooks masked in a private namespace), target on disk in lane-probe.
set -euo pipefail
T="$(cd "$(dirname "$0")" && pwd)"; R="$(cd "$T/../../../../../../.." && pwd)"
export NATIVE_REPO_ROOT="$R" P4T="$T/🗑️generated/builder/lane-probe/p4-live"
mkdir -p "$P4T/t" "$P4T/b"
exec 8>/dev/shm/semio-puzzle-builder/lockwasm; flock 8
unshare --user --map-root-user --mount --propagation private bash -c '
  set -euo pipefail
  mount -t tmpfs -o mode=1777,size=3000m tmpfs /tmp
  source "$NATIVE_REPO_ROOT/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/GET-PUZZLE-PLUGIN-WORKING-END-TO-END/mask-dbg.sh"
  tb="$NATIVE_REPO_ROOT/.cache/b/t/m/r/699c2d5d27a9c904/toolchains/nightly-2026-07-20-x86_64-unknown-linux-gnu/bin"
  export PATH="$tb:$PATH" RUSTC="$tb/rustc" RUSTC_WRAPPER="" CARGO_HOME="$NATIVE_REPO_ROOT/.cache/b/r/m/cargo/home" CARGO_TARGET_DIR="$P4T/t" CARGO_BUILD_BUILD_DIR="$P4T/b" CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=0 TMPDIR=/tmp
  export RUSTFLAGS="-C linker-features=+lld -C link-self-contained=+linker -Z unstable-options -C embed-bitcode=no"
  cd "$NATIVE_REPO_ROOT/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧊️brep/🧬️schema/📸️snapshot/📦️packages/🦀️rust"
  cargo check --offline --jobs 2 -p semio-s-artifact-stdio-semio-brep-snapshot
'
