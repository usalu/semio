#!/bin/bash
# usage: terrain-run.sh NAME -- repo cargo args...
T="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/GIS-PLUGIN-END-TO-END"
N="$1"; shift
L="$T/🗑️generated/compile.lock"
until mkdir "$L" 2>/dev/null; do if [ $(( $(date +%s) - $(stat -f %m "$L") )) -gt 7200 ]; then rmdir "$L"; fi; sleep 20; done
echo terrain-lane > "$L/owner"
trap 'rm -rf "$L"' EXIT
cd /Users/ueli/Documents/semio
env -u SEMIO_REMOTE_CACHE_TOKEN -u SEMIO_REMOTE_CACHE_ENDPOINT SEMIO_OFFLOAD=distribute repo cargo --lang=en --store=shared --configuration=prototype -- "$@" > "$T/🗑️generated/terrain-$N.log" 2>&1
echo "exit $?" >> "$T/🗑️generated/terrain-$N.log"
