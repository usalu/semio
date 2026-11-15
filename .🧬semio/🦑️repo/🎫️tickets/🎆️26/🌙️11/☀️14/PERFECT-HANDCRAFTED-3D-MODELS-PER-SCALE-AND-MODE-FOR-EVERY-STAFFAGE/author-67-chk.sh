#!/bin/bash
# usage: author-67-chk.sh <asset> <scale...>  product check, failing lines to 🗑️generated/author-67/chk-<slug>.txt
cd "$(dirname "$0")"; BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1); A=$1; shift
O="🗑️generated/author-67/chk-$(basename $A).txt"; : > "$O"
for s in "$@"; do echo "== $s" >> "$O"; $BUN volumes-check.ts $A --product --scale $s --quiet 2>&1 | grep -E "FAIL|^ok" | cut -c1-1400 >> "$O"; done
echo finished >> "$O"
