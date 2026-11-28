#!/usr/bin/env bash
set -u
# 🧪️ Lib tests of the gismap, generation2d and puzzle2d pdf export crates and the crates they were carved from.
cd "$(git rev-parse --show-toplevel)"
T="$(dirname "$0")"
P="✏️s/🔌️plugins"

"$T/cargo-lane.sh" 4 test --manifest-path "$P/🌀️procedural/🗿️artifacts/🌀️generation2d/Cargo.toml" --no-fail-fast --lib --message-format short -p semio-s-artifact-procedural-generation2d-pdf-export -p semio-s-artifact-procedural-2d-io || exit $?
"$T/cargo-lane.sh" 4 test --manifest-path "$P/🧩️puzzle/🗿️artifacts/◻️2d/Cargo.toml" --no-fail-fast --lib --message-format short -p semio-s-artifact-puzzle-2d-pdf-export -p semio-s-artifact-puzzle-2d-text-inverse || exit $?
