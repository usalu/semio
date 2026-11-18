#!/bin/zsh
# usage: deps-grant.sh [staged]
S=/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R135
R=/Users/ueli/Documents/semio
D=${0:A:h}
BUN=$R/.cache/b/t/m/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
PRE=()
if [ "$1" = staged ]; then node $D/mkovl.mjs; export R135_OVER=$(cat $S/over.resolved); PRE=(--preload $D/overlay.ts); fi
cd $R || exit 1
env -u SEMIO_FLEET_GROUP $BUN ${PRE[@]} $D/deps-grant.ts
