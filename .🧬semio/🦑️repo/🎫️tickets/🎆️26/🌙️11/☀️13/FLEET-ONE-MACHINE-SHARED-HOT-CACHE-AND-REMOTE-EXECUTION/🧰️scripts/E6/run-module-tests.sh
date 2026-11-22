#!/bin/zsh
M="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🤝️shared-worktree"
OUT="$1"
BUN=/Users/ueli/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
: >> "$OUT"
cd "$M" || exit 1
find . -path '*🧪️tests*' -name '*.ts' | grep -v fixtures | sort | sed -n "${2},${3}p" | while read -r f; do
  d=$(dirname "$f"); b=$(basename "$f")
  line=$(cd "$M/$d" && env -u SEMIO_FLEET_GROUP -u SEMIO_FLEET_NFS_HOST -u SEMIO_FLEET_NETWORK -u SEMIO_REMOTE_CACHE_ENDPOINT -u SEMIO_REMOTE_CACHE_ACCESS -u SEMIO_REMOTE_CACHE_INSTANCE -u SEMIO_REMOTE_CACHE_TOKEN BUN_RUNTIME_TRANSPILER_CACHE_PATH=0 "$BUN" test "./$b" --timeout 180000 2>&1 | grep -E "^ *[0-9]+ (pass|fail)|error:" | tr '\n' ' ')
  echo "$f :: $line" >> "$OUT"
done
echo DONE >> "$OUT"
