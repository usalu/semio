#!/bin/zsh
# usage: r5-one.sh <tag> <manifest> [cargo test args]
T="${0:A:h}/🗑️generated"; tag=$1; m=$2; shift 2
timeout 1500 repo cargo --store=isolated --configuration=prototype -- test --offline --no-fail-fast --manifest-path "$m" "$@" > "$T/r5-$tag.log" 2>&1
echo "exit $?" >> "$T/r5-$tag.log"
