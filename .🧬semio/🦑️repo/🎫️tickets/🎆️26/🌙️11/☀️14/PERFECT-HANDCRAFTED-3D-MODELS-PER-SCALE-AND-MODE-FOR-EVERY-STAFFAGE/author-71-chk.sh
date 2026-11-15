#!/bin/bash
# usage: author-71-chk.sh category/slug "scales" -> 🗑️generated/author-71/<slug>-chk-<scale>.txt (product check, one scale after the other)
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1); T="$(cd "$(dirname "$0")" && pwd)"; cd "$T"
a=$1; slug=${a#*/}
for s in ${2:-50 100 200}; do $BUN volumes-check.ts $a --product --scale $s > "$T/🗑️generated/author-71/$slug-chk-$s.txt" 2>&1; done
echo done > "$T/🗑️generated/author-71/$slug-chk-done.txt"
