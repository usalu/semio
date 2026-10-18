#!/bin/zsh
M="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/⌨️cli"
cd "$M" || exit 1
perl -e 'alarm 90; exec @ARGV' "$1" -test.run "^$2\$" -test.count=1 -test.timeout 80s >/dev/null 2>&1
echo "$2 $?"
