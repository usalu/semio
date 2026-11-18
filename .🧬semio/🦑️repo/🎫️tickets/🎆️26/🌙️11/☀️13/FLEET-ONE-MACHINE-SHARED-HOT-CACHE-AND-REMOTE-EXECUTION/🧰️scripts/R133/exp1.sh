#!/bin/zsh
# usage: exp1.sh <name> <seconds> -- repo args   (single machine, overlay seam, isolated state)
name=$1; secs=$2; shift 3
L=/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R133/$name; mkdir -p $L/s $L/tmp $L/run $L/cap $L/reg $L/xdg
cd /Users/ueli/Documents/semio || exit 1
env -u SEMIO_FLEET_GROUP -u SEMIO_FLEET_NFS_HOST -u SEMIO_FLEET_NETWORK -u SEMIO_REMOTE_CACHE_ENDPOINT -u SEMIO_REMOTE_CACHE_ACCESS -u SEMIO_REMOTE_CACHE_INSTANCE -u SEMIO_REMOTE_CACHE_TOKEN \
 SEMIO_SLICE_LAB=$name SEMIO_MACHINE_STATE_DIRECTORY=$L/s SEMIO_HOST_CAPACITY_TOTAL_MEGABYTES=16384 SEMIO_HOST_CAPACITY_CORES=8 SEMIO_HOST_CAPACITY_DIRECTORY=$L/cap SEMIO_LEASE_REGISTRY_DIRECTORY=$L/reg SEMIO_SERVICE_LAUNCHD=off TMPDIR=$L/tmp XDG_RUNTIME_DIR=$L/run XDG_CACHE_HOME=$L/xdg BUN_OPTIONS="--preload=/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION/🧰️scripts/R133/overlay.ts" \
 perl -e 'alarm shift; exec @ARGV' $secs "$@" > $L/out.log 2>&1
echo "exit $?" >> $L/out.log
