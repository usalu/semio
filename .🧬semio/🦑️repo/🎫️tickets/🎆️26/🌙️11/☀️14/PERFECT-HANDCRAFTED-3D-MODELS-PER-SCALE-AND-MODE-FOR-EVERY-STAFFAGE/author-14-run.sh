#!/bin/sh
cd "$(dirname "$0")"
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
python3 author-14-$1.py > /dev/null || exit 1
if [ -n "$2" ]; then
  $BUN author-14-diag.ts vehicles/$1 $2
  $BUN volumes-check.ts vehicles/$1 --scale $2 --rules ${3:-volumes-silhouette,volumes-bounds} 2>&1 | grep -v '^\['
fi
