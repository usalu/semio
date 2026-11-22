#!/bin/zsh
S="$(dirname "$0")"
for f in "$@"; do
  echo "#### ${f#*library/}"
  "$S/t.sh" "$f" 2>&1 | grep -v "^\[script\]" | grep -E "^error|^\s+at .*🧪️tests|^(Expected|Received)|^ *[0-9]+ (pass|fail)$" | awk '!seen[$0]++' | head -${N:-8} | cut -c1-220
done
