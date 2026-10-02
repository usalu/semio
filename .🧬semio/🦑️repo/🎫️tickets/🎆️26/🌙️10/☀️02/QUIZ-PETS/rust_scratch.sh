#!/usr/bin/env bash
# 🧪️ Runs cargo in a private scratch crate of the pets Rust twins, away from the root workspace and its shared build directory.
#
#   bash <ticket>/rust_scratch.sh <work package folder> <cargo arguments…>
#   bash <ticket>/rust_scratch.sh wp-k test --offline
#   bash <ticket>/rust_scratch.sh wp-k clippy --offline --all-targets --features sut -- -D warnings
#   SCRATCH_CRATE=host bash <ticket>/rust_scratch.sh wp-k build --offline --features sut   (another crate folder of the scratch area)
#
# The crate lives at <ticket>/🗑️generated/<work package folder>/crate/📦️packages/🦀️rust (Cargo.toml + glue 🦀️.rs with
# `#[path]` mounts of the repository files). The unit tests read `CARGO_MANIFEST_DIR/../../🧫️fixtures`, so the product's
# fixtures are copied to <…>/crate/🧫️fixtures before every run. See 🗒️rust-scratch-notes.md beside this script.
set -euo pipefail

ticket="$(cd "$(dirname "$0")" && pwd)"
root="$(cd "$ticket/../../../../../../.." && pwd)"
package="$1"
shift
scratch="$ticket/🗑️generated/$package"
manifest="$scratch/${SCRATCH_CRATE:-crate/📦️packages/🦀️rust}"
if [ ! -f "$manifest/Cargo.toml" ]; then
  echo "no scratch crate at $manifest" >&2
  exit 2
fi
rm -rf "$scratch/crate/🧫️fixtures"
cp -r "$root/🧰️framework/🛍️products/🐾️pets/🧫️fixtures" "$scratch/crate/🧫️fixtures"
mkdir -p "$scratch/build" "$scratch/target"
native() { if command -v cygpath >/dev/null 2>&1; then cygpath -m "$1"; else printf '%s' "$1"; fi; }
cd "$manifest"
RUSTC_WRAPPER="" \
CARGO_UNSTABLE_BUILD_DIR_NEW_LAYOUT=false \
CARGO_UNSTABLE_FINE_GRAIN_LOCKING=false \
CARGO_BUILD_BUILD_DIR="$(native "$scratch/build")" \
CARGO_TARGET_DIR="$(native "$scratch/target")" \
cargo "$@"
