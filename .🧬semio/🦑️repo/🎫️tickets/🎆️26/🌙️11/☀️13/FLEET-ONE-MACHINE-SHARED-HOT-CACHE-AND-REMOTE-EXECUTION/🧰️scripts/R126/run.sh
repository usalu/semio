#!/bin/zsh
# usage: run.sh <t-name.ts | live test path relative to the root> [base]  - one test file as a bare bun test in its own directory; staged overlay (modules and new tests as the host file) unless "base"
S=/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R126
R=/Users/ueli/Documents/semio
T=${0:h}
BUN=$R/.cache/b/t/m/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
HOST=$(python3 $T/stage.py host $1)
[ -z "$HOST" ] && HOST=$1
P=()
if [ "$2" = live ]; then :
elif [ "$2" = base ]; then export R126_OVER=$(python3 $T/stage.py tests); else export R126_OVER=$(python3 $T/stage.py overlay); fi
[ "$2" != live ] && P=(--preload $T/overlay.ts)
cd "$R/$(dirname $HOST)" || exit 1
env -u SEMIO_FLEET_GROUP -u SEMIO_FLEET_NFS_HOST -u SEMIO_FLEET_NETWORK -u SEMIO_REMOTE_CACHE_ENDPOINT -u SEMIO_REMOTE_CACHE_ACCESS -u SEMIO_REMOTE_CACHE_INSTANCE -u SEMIO_REMOTE_CACHE_TOKEN SEMIO_WORKSPACE_ROOT=$R $BUN test $P ./$(basename $HOST) --timeout 180000 2>&1 | grep -v '^\[script\]'
