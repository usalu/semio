#!/bin/zsh
D=$(dirname "$0"); export BUN=/Users/ueli/Documents/semio/.cache/b/t/m/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
export LIB=/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build
export SC=$1; mkdir -p $SC; rm -f $SC/res.*
tr "\n" "\0" < ${2:-$D/tests.list} | xargs -0 -P4 -n1 $D/one.sh
while IFS= read -r f; do h=$(echo "$f" | md5 -q); echo "$f :: $(tr '\n' ' ' < $SC/res.$h)"; done < ${2:-$D/tests.list} > $SC/summary.txt
cat $SC/summary.txt
