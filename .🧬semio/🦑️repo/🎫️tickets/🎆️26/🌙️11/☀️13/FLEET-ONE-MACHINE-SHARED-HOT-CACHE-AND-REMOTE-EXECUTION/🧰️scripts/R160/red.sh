#!/bin/zsh
T=/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION
D=$T/🧰️scripts/R160
S=/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R160
BUN=/Users/ueli/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
LIB=/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library
run(){ (cd "$1" && env -u SEMIO_FLEET_GROUP -u SEMIO_FLEET_NFS_HOST -u SEMIO_FLEET_NETWORK -u SEMIO_REMOTE_CACHE_ENDPOINT -u SEMIO_REMOTE_CACHE_ACCESS -u SEMIO_REMOTE_CACHE_INSTANCE -u SEMIO_REMOTE_CACHE_TOKEN $BUN test ./🟦️.ts --timeout 180000 2>&1 | cut -c1-300); }
mkdir "$T/landing.lock" 2>/dev/null || { echo "lock held: $(cat $T/landing.lock/owner)"; exit 1; }
echo "claude-R160 $(date -u +%Y-%m-%dT%H:%M:%S)Z (tests only, red run)" > "$T/landing.lock/owner"
release(){ rm -f "$T/landing.lock/owner"; rmdir "$T/landing.lock"; }
rm -rf $S/bk-tests; python3 $D/land.py apply tests $S/bk-tests || { release; exit 2; }
run "$LIB/⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/🧪️tests/📈️growth" > $S/red-growth.txt
run "$LIB/🏗️build/🧭️invocation/🧪️tests/🛰️plan" > $S/red-plan.txt
python3 $D/land.py restore tests $S/bk-tests
release
echo "released $(date -u +%H:%M:%S)"; grep -E "^\(fail\)|pass$|fail$" $S/red-growth.txt $S/red-plan.txt | head -20
