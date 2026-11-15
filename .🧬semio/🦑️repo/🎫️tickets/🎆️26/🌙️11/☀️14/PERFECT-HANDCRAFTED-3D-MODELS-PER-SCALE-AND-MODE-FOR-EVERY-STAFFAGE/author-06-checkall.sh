#!/bin/bash
cd "$(dirname "$0")"
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
for a in "$@"; do $BUN volumes-check.ts animals/$a > 🗑️generated/author-06/$a.check.txt 2>&1; done
echo finished > 🗑️generated/author-06/done.txt
