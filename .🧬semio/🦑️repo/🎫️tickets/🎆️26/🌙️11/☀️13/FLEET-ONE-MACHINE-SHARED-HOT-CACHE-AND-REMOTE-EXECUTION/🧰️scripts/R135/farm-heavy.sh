#!/bin/zsh
# usage: farm-heavy.sh [staged]   (declared 8 GB / 4 cores host; staged = overlay of the not yet landed R135 files)
S=/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R135
R=/Users/ueli/Documents/semio
D=${0:A:h}
BUN=$R/.cache/b/t/m/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
PRE=()
if [ "$1" = staged ]; then node $D/mkovl.mjs; export R135_OVER=$(cat $S/over.resolved); PRE=(--preload $D/overlay.ts); fi
cd $R || exit 1
env -u SEMIO_FLEET_GROUP SEMIO_HOST_CAPACITY_TOTAL_MEGABYTES=${R135_TOTAL:-8192} SEMIO_HOST_CAPACITY_CORES=${R135_CORES:-4} nice -n 5 /usr/bin/time -l $BUN ${PRE[@]} $D/farm-heavy.ts
