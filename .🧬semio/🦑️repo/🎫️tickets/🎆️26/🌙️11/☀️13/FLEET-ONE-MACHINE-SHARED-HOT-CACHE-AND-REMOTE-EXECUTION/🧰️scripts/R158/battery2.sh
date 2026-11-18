#!/bin/zsh
D=$(dirname "$0"); out="$1"; mkdir -p "$out"
cat "$D/tests2.list" | xargs -P6 -I{} "$D/one.sh" {} "$out" | sort > "$out/summary.txt"
