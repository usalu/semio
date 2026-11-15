#!/bin/sh
cd "$(dirname "$0")"
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
python3 author-17-$1.py || exit 1
$BUN volumes-check.ts components/$2 --quiet 2>&1 | grep -v "^ok" | cut -c1-900
$BUN volumes-render.ts components/$2 >/dev/null 2>&1
