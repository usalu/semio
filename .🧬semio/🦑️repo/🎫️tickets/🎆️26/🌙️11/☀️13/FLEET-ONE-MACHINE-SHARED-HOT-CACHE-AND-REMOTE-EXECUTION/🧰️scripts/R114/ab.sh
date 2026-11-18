#!/bin/zsh
cd /Users/ueli/Documents/semio
S="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION/🧰️scripts/R114"; W=/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R112/wave; B=.cache/b/t/m/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
run(){ env -u SEMIO_FLEET_GROUP -u SEMIO_REMOTE_CACHE_ENDPOINT -u SEMIO_REMOTE_CACHE_TOKEN R107_ALL=1 R111_PASS=owner "$@" $B "$S/look.ts" $W 6 1 2>&1 | grep "JOBS\|STEPS\|RESULT" | cut -c1-420; }
echo "-- 1 warm (child-style settle with derivation)"; run R111_DERIVE=1 R114_SKIP=0
echo "-- 2 before (parent settle+refs, fresh entries, take mode)"; run R111_DERIVE=0 R114_SKIP=0
echo "-- 3 after (parent refs only, cold cache)"; run R111_DERIVE=0 R114_SKIP=1
echo "-- 4 before again"; run R111_DERIVE=0 R114_SKIP=0
echo "-- 5 after again"; run R111_DERIVE=0 R114_SKIP=1
