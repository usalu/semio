#!/bin/bash
# usage: runall.sh <moduleRoot> [filter]
source /private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/boot1113/env.sh
MOD="${1:-$W}"
cd "$MOD"
find . -path ./node_modules -prune -o -path '*/🧪️tests/*' -type f -name '*.ts' -print | grep -v fixtures | sort | while read f; do
  case "$f" in *"${2:-}"*) ;; *) continue;; esac
  d=$(dirname "$f"); b=$(basename "$f")
  out=$(cd "$MOD/$d" && env -u SEMIO_FLEET_GROUP -u SEMIO_FLEET_NFS_HOST -u SEMIO_FLEET_NETWORK -u SEMIO_REMOTE_CACHE_ENDPOINT -u SEMIO_REMOTE_CACHE_ACCESS -u SEMIO_REMOTE_CACHE_INSTANCE $BUN test "./$b" --timeout 180000 2>&1)
  pass=$(echo "$out" | grep -E '^ *[0-9]+ pass' | head -1); fail=$(echo "$out" | grep -E '^ *[0-9]+ fail' | head -1); skip=$(echo "$out" | grep -E '^ *[0-9]+ skip' | head -1)
  echo "$f :: $pass | $fail | $skip"
  echo "$out" | grep -E '^\(fail\)|error:' | head -5
done
