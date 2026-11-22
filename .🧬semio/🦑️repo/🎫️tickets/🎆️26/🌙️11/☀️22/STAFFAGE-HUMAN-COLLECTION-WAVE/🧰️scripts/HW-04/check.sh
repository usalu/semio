#!/bin/sh
# 🩺️ Runs the staffage check CLI on an asset JSON (draft or catalog file): check.sh <asset.json> [samples]
F="$(cd "$(dirname "$1")" && pwd)/$(basename "$1")"
B=/Users/ueli/Documents/semio/.cache/b/t/m/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
cd "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage" && $B 🔨️modules/🗂️catalog/🩺️check/🟦️.ts --file "$F" --samples "${2:-9}"
