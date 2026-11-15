#!/bin/bash
# 🥾 Runs the fleet provision script from a topology copy outside the worktree, as root, for worker g1-m07 of group g1.
# Usage: provision-g1-m07.sh <log> [provision flags]
log=$1; shift
origin=g1-m07
export SEMIO_LANGUAGE=en SEMIO_FLEET_GROUP=g1 SEMIO_FLEET_NFS_HOST=192.168.178.70 SEMIO_FLEET_NETWORK=192.168.178.0/24
unset SEMIO_FLEET_CACHE_ENDPOINT
export PATH="/Users/ueli/.local/bin:/usr/local/bin:/usr/bin:/bin:/usr/sbin:/sbin"
cd /
: > "$log"; chown ueli:staff "$log"; chmod 644 "$log"
machine=/Users/ueli/Documents/semio/.cache/f/machine.json
if [ ! -f "$machine" ] && [ -e "/Volumes/semio-store/v1/origins/$origin" ]; then echo "origin $origin is already taken in the store: stop" >> "$log"; echo "provision_rc=98" >> "$log"; exit 0; fi
bash "/private/tmp/claude-501/-Users-ueli--claude-spares-spare-VItXlS/5a89d7c0-12b5-46db-835c-af2f18b7f4ac/scratchpad/topo/🗺️topology/🥾️provision/🐚️.sh" g1 "$origin" --user ueli "$@" >> "$log" 2>&1
echo "provision_rc=$?" >> "$log"
exit 0
