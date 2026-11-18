#!/bin/zsh
cd /Users/ueli/Documents/semio || exit 2
FT=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION"
S=/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R152
BUN=/Users/ueli/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
L="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
until mkdir "$FT/landing.lock" 2>/dev/null; do sleep 5; done
echo claude-R152 > "$FT/landing.lock/owner"; echo "lock $(date -u +%H:%M:%S)"
release(){ rm -f "$FT/landing.lock/owner"; rmdir "$FT/landing.lock"; }
rm -rf $S/live.bk
python3 "$FT/🧰️scripts/R152/land.py" apply "$L" $S/live.bk || { release; exit 3; }
run(){ ( cd "$L/$1" && env -u SEMIO_FLEET_GROUP -u SEMIO_FLEET_NFS_HOST -u SEMIO_FLEET_NETWORK -u SEMIO_REMOTE_CACHE_ENDPOINT -u SEMIO_REMOTE_CACHE_ACCESS -u SEMIO_REMOTE_CACHE_INSTANCE -u SEMIO_REMOTE_CACHE_TOKEN SEMIO_TEST_ARTIFACT_DIR=$S/art $BUN test ./🟦️.ts --timeout 120000 2>&1 | /usr/bin/grep -E "^ *[0-9]+ (pass|fail)" | tr '\n' ' ' > $S/out.$2; ) }
run 🏗️build/🚀️bootstrap/🔷️typescript-compiler/🧪️tests interface &
run 🏗️build/🚀️bootstrap/🔷️typescript-compiler/🧪️tests/🚫️bare-name law &
run 🕸️dependencies/🟦️typescript/🧪️tests deps &
run ⚡️caching/📥️inference/🧪️tests inference &
run 🏗️build/🕸️graph/📥️inference/🧭️script-inputs/🔀️router/🧪️tests router &
run 🎛️commands/✅️verify/🔷️typescript-project/🧪️tests verify &
wait
for n in interface law deps inference router verify; do echo "$n: $(cat $S/out.$n)"; done
echo "end $(date -u +%H:%M:%S)"
echo "(restore with: python3 land.py restore $L $S/live.bk; then release)"
