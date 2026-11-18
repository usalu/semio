#!/bin/zsh
SC=/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R118
D="$(cd "$(dirname "$0")" && pwd)"
BUN=/Users/ueli/Documents/semio/.cache/b/t/m/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
export R118_STAGE=$SC/stage/L R118_LIVE=/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library
cd /Users/ueli/Documents/semio
env -u SEMIO_FLEET_GROUP -u SEMIO_REMOTE_CACHE_ENDPOINT -u SEMIO_REMOTE_CACHE_TOKEN $BUN test --preload "$D/preload.ts" "$R118_STAGE/${1:-🏗️build/👁️observation/📂️source-names/🧪️tests/🟦️.ts}" --timeout 180000 2>&1 | tail -${2:-40}
