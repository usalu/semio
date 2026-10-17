#!/bin/zsh
# usage: parsecheck.sh <nul-list> <out>  (rustfmt parse-only; prints files whose parse fails)
cd /Users/ueli/Documents/semio
xargs -0 -n 1 -P 4 zsh -c 'out=$(rustfmt +nightly-2026-07-20 --edition 2021 --config skip_children=true --check "$0" 2>&1 >/dev/null | /usr/bin/grep -E "^error" | head -1); [ -n "$out" ] && echo "PARSE-ERROR $0 :: $out"; exit 0' < "$1" > "$2"
echo "done: $(wc -l < "$2") parse errors"
