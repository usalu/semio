#!/bin/bash
BUN=${BUN:-/Users/ueli/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun}
R=/Users/Shared/semio; M="$R/🧰️framework/🛍️products/🦑️repo/🔨️modules/🤝️shared-worktree"
for v in $(env | sed -n 's/^\(SEMIO_[A-Za-z0-9_]*\)=.*/\1/p'); do unset "$v"; done
cd "$M"
for f in $(ls */🧪️tests/*.ts */*/🧪️tests/*.ts 2>/dev/null); do
  case "$f" in *harness.ts|*sweep.ts) continue;; esac
  d=$(dirname "$f"); b=$(basename "$f"); echo "=== $f"
  ( cd "$d" && env -i PATH="$PATH" HOME="$HOME" TMPDIR=/tmp "$BUN" test "./$b" --timeout 180000 2>&1 | grep -E "^\(fail\)|^ *[0-9]+ (pass|fail)|^error" )
done
