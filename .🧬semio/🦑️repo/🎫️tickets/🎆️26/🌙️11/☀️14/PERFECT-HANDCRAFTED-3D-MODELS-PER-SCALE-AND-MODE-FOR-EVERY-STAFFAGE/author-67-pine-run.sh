#!/bin/bash
# usage: author-67-pine-run.sh <scale> [fit]  builder (slab lobes), decisions
cd "$(dirname "$0")"; BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1); S=$1
SLABS=${SLABS:-1} $BUN author-31-pine.ts $S | tail -1
python3 author-31-decisions.py scots-pine >/dev/null
