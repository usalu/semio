#!/bin/sh
# usage: nxenv.sh <repo-relative test file> [bun test args]   runs one Nx-gated library test with the Nx fixed-storage env the Nx adapter would set (pinned Bun)
cd /Users/ueli/Documents/semio || exit 1
B=$(ls "$PWD"/.cache/b/t/m/bun/*/bun | head -1)
export SEMIO_NX_FIXED_STORAGE="$PWD/.🧬semio/🦑️repo/⚡️cache/rust/fixed/m" NX_CACHE_DIRECTORY="$PWD/.🧬semio/🦑️repo/⚡️cache/rust/fixed/m/nx/artifacts"
f="$1"; shift
exec "$B" test "./$f" "$@"
