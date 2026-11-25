#!/bin/zsh
export PATH="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🚀️launcher/🚪️shims:$PATH"  # managed cargo entry
# Rebuilds the hub wasm with plain cargo and republishes it; the browser probes run separately (the boot check of rebuild-and-boot.sh needs a browser the machine often kills).
T="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️07/☀️23/GET-PROCESS-WORKING-END-TO-END"
B=/Users/ueli/Documents/semio/.cache/b/t/m/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
cd /Users/ueli/Documents/semio
rm -f $T/🗑️generated/hub-build.done $T/🗑️generated/hub-rebuild-v11.done
HUB_ATTEMPTS=${HUB_ATTEMPTS:-4} $T/hub-build.sh
grep -q "^exit 0" $T/🗑️generated/hub-build.done || { echo "build failed" > $T/🗑️generated/hub-rebuild-v11.done; exit 1; }
$B $T/publish-hub.ts > $T/🗑️generated/publish.txt 2>&1 && echo "published" > $T/🗑️generated/hub-rebuild-v11.done || echo "publish failed" > $T/🗑️generated/hub-rebuild-v11.done
