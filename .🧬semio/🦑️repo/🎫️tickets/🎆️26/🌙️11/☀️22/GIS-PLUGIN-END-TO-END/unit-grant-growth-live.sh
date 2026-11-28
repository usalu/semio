#!/bin/bash
T="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/GIS-PLUGIN-END-TO-END"
cd /Users/ueli/Documents/semio
env -u SEMIO_REMOTE_CACHE_TOKEN -u SEMIO_REMOTE_CACHE_ENDPOINT SEMIO_OFFLOAD=distribute repo cargo --lang=en --store=shared --configuration=prototype -- build --manifest-path "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/Cargo.toml" -p semio-plugin-actor-input-decoder -j 4 > "$T/🗑️generated/growth-live-2.log" 2>&1
echo "exit $?" >> "$T/🗑️generated/growth-live-2.log"
