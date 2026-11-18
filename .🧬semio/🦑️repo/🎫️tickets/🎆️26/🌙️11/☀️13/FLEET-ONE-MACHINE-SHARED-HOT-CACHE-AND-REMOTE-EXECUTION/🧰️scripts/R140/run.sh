#!/bin/zsh
cd /Users/ueli/Documents/semio || exit 1
export R140_OUT=${R140_OUT:-/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R140}
BUN=.cache/b/t/m/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
S="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️testing/🌱️genuine-birth/📜️script.ts"
F="$1"; shift
$BUN "$@" "$S" --file="$F" --birth=setup --root=/Users/ueli/Documents/semio
