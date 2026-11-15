#!/bin/bash
# usage: author-21-chk.sh <slug> <scale...>  -> failing lines only
cd "$(dirname "$0")"
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
slug=$1; shift
for s in "$@"; do $BUN volumes-check.ts vegetation/$slug --scale $s --quiet 2>&1 | grep -v "^ok\|^\[check\|^vegetation\|^FAIL:" | cut -c1-1100; done
