#!/usr/bin/env bash
# 🧪️ Runs cargo in the private scratch crate of work package A1 (the split stage of the scratch copy `after/`), away
# from the root workspace and its shared build directory.
#
#   bash <ticket>/a1_cargo.sh test --offline
#   bash <ticket>/a1_cargo.sh clippy --offline --all-targets --features sut -- -D warnings
#   bash <ticket>/a1_cargo.sh run --release --offline --features sut --example long_trace_digest
#
# The crate lives at <ticket>/🗑️generated/a1/crate/📦️packages/🦀️rust (`a1_scratch.ts prepare` writes it). The unit
# tests read `CARGO_MANIFEST_DIR/../../🧫️fixtures`: the fixtures of the scratch copy are placed there before every
# run, so the crate never reads what another agent is changing in the live tree.
set -euo pipefail

ticket="$(cd "$(dirname "$0")" && pwd)"
scratch="$ticket/🗑️generated/a1"
manifest="$scratch/crate/📦️packages/🦀️rust"
if [ ! -f "$manifest/Cargo.toml" ]; then
  echo "no scratch crate at $manifest" >&2
  exit 2
fi
rm -rf "$scratch/crate/🧫️fixtures"
cp -r "$scratch/after/🧰️framework/🛍️products/🐾️pets/🧫️fixtures" "$scratch/crate/🧫️fixtures"
mkdir -p "$scratch/build" "$scratch/target"
native() { if command -v cygpath >/dev/null 2>&1; then cygpath -m "$1"; else printf '%s' "$1"; fi; }
cd "$manifest"
RUSTC_WRAPPER="" \
CARGO_UNSTABLE_BUILD_DIR_NEW_LAYOUT=false \
CARGO_UNSTABLE_FINE_GRAIN_LOCKING=false \
CARGO_BUILD_BUILD_DIR="$(native "$scratch/build")" \
CARGO_TARGET_DIR="$(native "$scratch/target")" \
cargo "$@"
