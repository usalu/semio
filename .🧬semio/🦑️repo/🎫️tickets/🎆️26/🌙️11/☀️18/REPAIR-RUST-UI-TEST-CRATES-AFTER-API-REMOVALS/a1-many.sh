#!/bin/zsh
# usage: a1-many.sh <tag> <package>...; tests several workspace packages in one run; log in 🗑️generated/<tag>.log
F="/Users/ueli/Documents/semio/🧰️framework"; G="${0:A:h}/🗑️generated"; tag=$1; shift; args=(); for p in "$@"; do args+=(-p "$p"); done; cd "$F"
timeout 5400 repo cargo --store=isolated --configuration=prototype -- test --offline --no-fail-fast --manifest-path "$F/Cargo.toml" $args > "$G/$tag.log" 2>&1
echo "exit $?" >> "$G/$tag.log"
