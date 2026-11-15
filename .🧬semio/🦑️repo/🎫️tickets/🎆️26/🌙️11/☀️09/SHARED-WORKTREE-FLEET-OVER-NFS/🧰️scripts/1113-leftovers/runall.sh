#!/bin/bash
# usage: runall.sh <module-root> <logfile>   runs every test file of the module, one line per file
ROOT="$1"; OUT="$2"; : > "$OUT"
BUN=/Users/ueli/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
cd "$ROOT" || exit 1
find . -path '*🧪️tests/*' -name '*.ts' ! -name '🌪️harness.ts' ! -name '🌪️sweep.ts' | sort | while read -r f; do
  d=$(dirname "$f"); b=$(basename "$f")
  res=$(cd "$ROOT/$d" && env -u SEMIO_FLEET_GROUP -u SEMIO_FLEET_NFS_HOST -u SEMIO_FLEET_NETWORK -u SEMIO_REMOTE_CACHE_ENDPOINT -u SEMIO_REMOTE_CACHE_ACCESS -u SEMIO_REMOTE_CACHE_INSTANCE "$BUN" test "./$b" --timeout 180000 2>&1)
  echo "$f: $(echo "$res" | grep -E '^ *[0-9]+ (pass|fail|skip)$' | tr -s ' ' | tr '\n' ' ')" >> "$OUT"
  echo "$res" | grep -E "^\(fail\)" | head -10 >> "$OUT"
done
echo DONE >> "$OUT"
