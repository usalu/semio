#!/bin/bash
# usage: run-all.sh <module root> <log dir> [path-substring]  -- runs EVERY ts test file under every 🧪️tests directory of the module, two at a time
W="$1"; LOG="$2"; ONLY="$3"
BUN=/Users/ueli/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
mkdir -p "$LOG"; rm -f "$LOG"/*.txt
one(){
  f="$1"; dir=$(dirname "$f"); name=$(basename "$f"); tag=$(echo "${f#$W/}" | tr '/' '_')
  cd "$dir" || exit
  env -u SEMIO_FLEET_GROUP -u SEMIO_FLEET_NFS_HOST -u SEMIO_FLEET_NETWORK -u SEMIO_REMOTE_CACHE_ENDPOINT -u SEMIO_REMOTE_CACHE_ACCESS -u SEMIO_REMOTE_CACHE_INSTANCE "$BUN" test "./$name" --timeout 180000 > "$LOG/$tag.txt" 2>&1
  echo "${f#$W/}: $(grep -E '^ *[0-9]+ (pass|fail)$' "$LOG/$tag.txt" | tr -s ' ' | tr '\n' ' ') exit=$?"
}
export -f one; export W LOG BUN
find "$W" -path '*🧪️tests/*' -name '*.ts' -not -path '*/node_modules/*' -not -path '*fixtures*' | grep -F "${ONLY}" | sort | xargs -P 2 -I{} bash -c 'one "{}"'
