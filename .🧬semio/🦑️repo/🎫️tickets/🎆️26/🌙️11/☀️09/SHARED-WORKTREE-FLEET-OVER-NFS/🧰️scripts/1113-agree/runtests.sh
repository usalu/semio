#!/bin/bash
# usage: runtests.sh <module root> [file filter]; runs every test file of the module, one bun process each
BUN=/Users/ueli/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
MOD="$1"; FILTER="${2:-}"
cd "$MOD" || exit 2
fail=0
for dir in $(find . -type d -name '🧪️tests' | sort); do
  for f in "$dir"/*.ts; do
    [ -f "$f" ] || continue
    case "$f" in *"$FILTER"*) ;; *) continue;; esac
    out=$(cd "$dir" && env -u SEMIO_FLEET_GROUP -u SEMIO_FLEET_NFS_HOST -u SEMIO_FLEET_NETWORK -u SEMIO_REMOTE_CACHE_ENDPOINT -u SEMIO_REMOTE_CACHE_ACCESS -u SEMIO_REMOTE_CACHE_INSTANCE "$BUN" test "./$(basename "$f")" --timeout 180000 2>&1)
    line=$(echo "$out" | grep -E "^ *[0-9]+ (pass|fail)" | tr '\n' ' ')
    echo "$f: $line"
    echo "$out" | grep -q "^ *0 fail" || { fail=1; echo "$out" | grep -E "^\(fail\)|error:" | head -5; }
  done
done
exit $fail
