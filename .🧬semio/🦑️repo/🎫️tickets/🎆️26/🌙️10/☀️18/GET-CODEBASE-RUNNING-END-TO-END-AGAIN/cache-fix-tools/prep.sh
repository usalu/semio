#!/usr/bin/env bash
# usage: prep.sh   regenerates the tracked-manifest list and the manifest operations, rebuilds the staged set from the live base (read-only on the repo)
set -euo pipefail
here="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"; root="$(git -C "$here" rev-parse --show-toplevel)"
mkdir -p "$here/../🗑️generated/cachefix-x"
git -C "$root" ls-files '*project.json' | command grep -v '^\.🧬semio/' > "$here/../🗑️generated/cachefix-x/pj-list.txt"
bun "$here/mk-ops.ts"
bun "$here/build-staged.ts"
