#!/bin/zsh
# usage: run.sh <test file abs> [pattern]
R=/Users/ueli/Documents/semio
B=$R/.cache/b/t/m/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
cd $R
if [ -n "$2" ]; then P=(--pattern="$2"); else P=(); fi
nice -n 10 $B "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️testing/🌱️genuine-birth/📜️script.ts" --file="$1" --birth=setup --root=$R $P 2>&1
