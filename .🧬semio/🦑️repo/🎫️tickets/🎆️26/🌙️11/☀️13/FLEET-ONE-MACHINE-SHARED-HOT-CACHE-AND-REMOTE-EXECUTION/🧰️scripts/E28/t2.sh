#!/bin/zsh
# usage: t2.sh <path relative to library root>  runs repo test-file in the second scratch checkout and prints failure details
L="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
cd /private/tmp/e28/co2 && repo test-file "$L/$1" 2>&1 | grep -vE '^\[script\]|^\[test-file\]|^bun test|^$' | cut -c1-${W:-300} | head -${N:-60}
