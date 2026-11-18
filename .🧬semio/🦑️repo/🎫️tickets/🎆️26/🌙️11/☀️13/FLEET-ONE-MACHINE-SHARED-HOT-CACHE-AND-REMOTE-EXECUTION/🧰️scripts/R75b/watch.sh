#!/bin/zsh
cd "$(dirname "$0")"
BUN=/Users/ueli/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
TMP=/private/var/folders/pm/zdg449hs6r33w_2gtsn23byc0000gn/T
python3 snap.py
echo "restarts of the pool seen on the server since 05:30Z: $(grep 'pool code changed, restarting' /Users/ueli/Documents/semio/.cache/f/sidecars/build-agent.log | awk '$1>"2026-11-16T05:30"' | wc -l | tr -d ' ')"
echo "orphans registered here: $(ls /Users/ueli/Documents/semio/.cache/f/owner-pool-children 2>/dev/null | wc -l | tr -d ' '); cores of the server: 10"
env -u SEMIO_FLEET_GROUP $BUN unstored.mjs $TMP 60 2>&1 | tail -1
