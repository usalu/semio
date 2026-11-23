#!/bin/sh
# 🐍️ Runs a python script in the hash-locked geometry oracle environment of the staffage tests (shapely, numpy, scipy): `py-geometry.sh <script.py> [args…]`, from the directory of the caller.
O="/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧰️support/🐍️oracles"
UV_PYTHON_DOWNLOADS=never UV_PROJECT_ENVIRONMENT="$HOME/Library/Caches/semio/staffage-oracles/geometry" exec uv run --quiet --project "$O" --locked --only-group geometry python "$@"
