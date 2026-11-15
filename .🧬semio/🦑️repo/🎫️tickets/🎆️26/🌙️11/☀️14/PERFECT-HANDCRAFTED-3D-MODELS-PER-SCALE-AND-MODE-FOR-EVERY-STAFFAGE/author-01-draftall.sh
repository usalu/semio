#!/bin/zsh
cd "$(dirname "$0")"
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
for a in "$@"; do
  for s in 50 100 200 500; do
    $BUN volumes-draft.ts vegetation/$a --scale $s --apply --force > /dev/null 2>&1
  done
done
