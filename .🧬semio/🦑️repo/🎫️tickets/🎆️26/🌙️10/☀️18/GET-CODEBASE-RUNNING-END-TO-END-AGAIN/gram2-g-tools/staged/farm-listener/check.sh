#!/bin/sh
# [gram2-g] verify of batch farm-listener: the farm capacity law (4 pass expected; red without the fix, see result-gram2-g.md)
cd /Users/ueli/Documents/semio || exit 1
BUN=.cache/b/t/m/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
$BUN test "./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌡️host-capacity/🧪️tests/🏊️farm/🟦️.ts" 2>&1 | tail -n 6
