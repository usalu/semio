#!/bin/bash
# R110: rehearsal of the fleet on this machine. usage: rehearse.sh [options of the command, see 📜️script.ts]; default: 3 machines of 16 GB, 470 owners
R=/Users/ueli/Documents/semio
B=/Users/ueli/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
K="$R/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️testing/🏟️fleet-rehearsal/📜️script.ts"
cd "$R" && exec "$B" "$K" rehearse "$@"
