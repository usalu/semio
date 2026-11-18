#!/bin/zsh
# usage: t.sh <stage|base> <dir relative to build library> ; runs ./🟦️.ts or given file
SC=/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R139
BUN=/Users/ueli/Documents/semio/.cache/b/t/m/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
LIB=/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build
mode=$1; dir=$2; file=${3:-🟦️.ts}
P=(); [ "$mode" = stage ] && P=(--preload $SC/overlay.ts)
cd "$LIB/$dir" && env -u SEMIO_FLEET_GROUP -u SEMIO_FLEET_NFS_HOST -u SEMIO_FLEET_NETWORK -u SEMIO_REMOTE_CACHE_ENDPOINT -u SEMIO_REMOTE_CACHE_ACCESS -u SEMIO_REMOTE_CACHE_INSTANCE -u SEMIO_REMOTE_CACHE_TOKEN R139_STAGE=$SC/m $BUN test $P ./$file --timeout 120000 2>&1
