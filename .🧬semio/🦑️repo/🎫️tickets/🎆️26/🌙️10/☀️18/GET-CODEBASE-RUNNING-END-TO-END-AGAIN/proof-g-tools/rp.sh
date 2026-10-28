#!/bin/sh
# `repo <args>` without the stale installed Go launcher (its registry contract predates the `restart` field): the hand-over the launcher performs, verbatim.
cd "$(dirname "$0")/../../../../../../../.." || exit 1
BUN=".cache/b/t/m/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun"
export BUN_RUNTIME_TRANSPILER_CACHE_PATH=0
exec "$BUN" "./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🚪️door/📜️script.ts" repo "$@"
