#!/bin/zsh
D=$(cd "$(dirname "$0")" && pwd)
export BUN=/Users/ueli/Documents/semio/.cache/b/t/m/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
export LIB=/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build
export SC=/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R149/res-$1
mkdir -p $SC; rm -f $SC/res.*
cat $D/catalog.tests | xargs -P4 -n1 $D/one.sh
for f in $(cat $D/catalog.tests); do h=$(echo "$f" | md5 -q); echo "$f $(tr '\n' ' ' < $SC/res.$h)"; done
