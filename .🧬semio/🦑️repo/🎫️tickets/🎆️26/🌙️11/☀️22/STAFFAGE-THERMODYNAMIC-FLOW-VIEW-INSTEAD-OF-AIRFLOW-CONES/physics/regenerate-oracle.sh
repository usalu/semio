#!/bin/sh
# 🔁 Regenerates the expectations of the 🌊️flow-obstacles fixture: kernel triangles (bun) -> oracle (python: numpy, scipy, trimesh) -> digest of the unobstructed parcels (bun). Run from the repository root.
set -eu
T="$(cd "$(dirname "$0")" && pwd)"
S="🎓️teaching/🏛️architecture/🧍️staffage"
BUN="$(ls "$HOME"/Library/Caches/semio/runtimes/bun/*/bun | head -1)"
mkdir -p "$T/🗑️generated"
"$BUN" "$S/🧪️tests/🌊️flow-obstacles/🧱️mesh.ts" "$T/🗑️generated/figure-mesh.json"
UV_PYTHON_DOWNLOADS=never UV_PROJECT_ENVIRONMENT="$HOME/Library/Caches/semio/staffage-oracles/geometry" uv run --quiet --project "$S/🧪️tests/🧰️support/🐍️oracles" --locked --only-group geometry python -I "$S/🧪️tests/🌊️flow-obstacles/🐍️.py" --write --mesh "$T/🗑️generated/figure-mesh.json"
"$BUN" "$T/unobstructed-digest.ts"
