#!/bin/zsh
# usage: a2-one.sh (one.sh with a 90-minute bound for queued isolated builds) <tag> <manifest-relative-to-framework> [cargo test args]; log in 🗑️generated/<tag>.log
F="/Users/ueli/Documents/semio/🧰️framework"; G="${0:A:h}/🗑️generated"; tag=$1; m=$2; shift 2; cd "$F"
timeout 5400 repo cargo --store=isolated --configuration=prototype -- test --offline --no-fail-fast --manifest-path "$F/$m" "$@" > "$G/$tag.log" 2>&1
echo "exit $?" >> "$G/$tag.log"
grep -E '^error|^test result|FAILED|panicked' "$G/$tag.log" | head -60; tail -1 "$G/$tag.log"
