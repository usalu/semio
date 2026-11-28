#!/bin/sh
T="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/GIS-PLUGIN-END-TO-END"
L="$T/🗑️generated/compile.lock6"
name="$1"; shift
until mkdir "$L" 2>/dev/null; do if [ $(( $(date +%s) - $(stat -f %m "$L") )) -gt 7200 ]; then rmdir "$L"; fi; sleep 20; done
echo conformance-lane > "$L/owner"
cd /Users/ueli/Documents/semio
env -u SEMIO_REMOTE_CACHE_TOKEN -u SEMIO_REMOTE_CACHE_ENDPOINT SEMIO_OFFLOAD=distribute repo cargo --lang=en --store=shared --configuration=prototype -- "$@" > "$T/🗑️generated/conformance-$name.log" 2>&1
echo "exit=$?" >> "$T/🗑️generated/conformance-$name.log"
rm -rf "$L"
