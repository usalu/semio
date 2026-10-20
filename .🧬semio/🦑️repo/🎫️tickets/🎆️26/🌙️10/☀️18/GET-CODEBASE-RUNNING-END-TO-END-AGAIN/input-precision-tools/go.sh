#!/usr/bin/env bash
# 🚦 Post-GO sequence of the input precision set: drift check, apply, engine tests, law tests; graph census and Nx hash measurement follow once `bun repo show projects` has built the live graph (see result-input-precision.md).
set -euo pipefail
T="$(cd "$(dirname "$0")" && pwd)"; W=/Users/ueli/Documents/semio
LIB="$W/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
cd "$T"
python3 apply.py
python3 apply.py --apply
cd "$W"
SEMIO_INPUT_PRECISION_REPO=1 bun test "$LIB/🏗️build/📥️inputs/🎯️precision/🧪️tests/🟦️.ts"
bun test "$LIB/⚖️laws/🔒️cache-correctness/🧪️tests/🔒️rules/🟦️.ts"
