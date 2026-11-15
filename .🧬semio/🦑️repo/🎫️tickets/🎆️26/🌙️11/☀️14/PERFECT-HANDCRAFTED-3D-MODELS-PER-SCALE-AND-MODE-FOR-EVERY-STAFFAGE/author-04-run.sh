#!/bin/sh
cd "$(dirname "$0")"
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
slug=$1
python3 author-04-$slug.py >/dev/null || exit 1
$BUN volumes-check.ts machinery/$slug --quiet 2>&1 | grep -v "^ok\|^\[check"
