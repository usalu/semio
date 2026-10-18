#!/bin/zsh
# usage: verify.sh <origfile> <dir> <non-source-glob-exclude...>
orig="$1"; dir="$2"; shift 2
./gosplit norm "$orig" | sort > /tmp/gotix-a.$$ 
: > /tmp/gotix-b.$$
for f in "$dir"/*.go; do
  case "$f" in *_test.go) continue;; esac
  ./gosplit norm "$f" >> /tmp/gotix-b.$$
done
sort -o /tmp/gotix-b.$$ /tmp/gotix-b.$$
wc -l /tmp/gotix-a.$$ /tmp/gotix-b.$$ | head -2
diff /tmp/gotix-a.$$ /tmp/gotix-b.$$ | head -40
echo "diff-exit: $?"
rm -f /tmp/gotix-a.$$ /tmp/gotix-b.$$
