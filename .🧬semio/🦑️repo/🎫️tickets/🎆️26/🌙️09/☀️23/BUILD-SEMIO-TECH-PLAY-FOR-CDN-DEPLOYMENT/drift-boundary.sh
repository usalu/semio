#!/bin/bash
cd /Users/ueli/Documents/semio
for d in ✏️s 🌎️hub 🧰️framework; do .cache/b/t/m/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️23/BUILD-SEMIO-TECH-PLAY-FOR-CDN-DEPLOYMENT/native-source-boundary-preflight.ts" /Users/ueli/Documents/semio "$d" 2>&1 | grep -aE "OVER|exceed" | cut -c1-250; done
echo boundary-done
