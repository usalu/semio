#!/bin/bash
ROOT=/Users/ueli/Documents/semio
BUN=$ROOT/.cache/b/t/m/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
H=$(ls -d "$ROOT"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️testing/🌱️genuine-birth/📜️script.ts)
cd "$ROOT" || exit 2
for f in "$@"; do
  s=$(date +%s)
  out=$(env -u SEMIO_FLEET_GROUP -u SEMIO_FLEET_NFS_HOST -u SEMIO_FLEET_NETWORK -u SEMIO_REMOTE_CACHE_ENDPOINT -u SEMIO_REMOTE_CACHE_ACCESS -u SEMIO_REMOTE_CACHE_INSTANCE -u SEMIO_REMOTE_CACHE_TOKEN "$BUN" "$H" --file="$f" --birth=setup --root="$ROOT" 2>&1)
  code=$?
  echo "== exit $code $(( $(date +%s)-s )) s  $f"
  echo "$out" | grep -E "^ *[0-9]+ (pass|fail|skip)|error:|\(fail\)" | head -12
done
