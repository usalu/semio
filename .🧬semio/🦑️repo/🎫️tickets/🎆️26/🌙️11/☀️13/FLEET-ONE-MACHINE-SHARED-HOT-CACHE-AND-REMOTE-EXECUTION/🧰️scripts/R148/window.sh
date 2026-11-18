#!/bin/zsh
cd /Users/ueli/Documents/semio || exit 2
FT=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION"
S=/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R148
BUN=/Users/ueli/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
L="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
until mkdir "$FT/landing.lock" 2>/dev/null; do sleep 5; done
echo claude-R148 > "$FT/landing.lock/owner"; start=$(date -u +%H:%M:%S); echo "lock $start"
restore(){ python3 "$FT/🧰️scripts/R148/land.py" restore $S/lib.backup; python3 "$FT/🧰️scripts/R148/land2.py" restore $S/bk2; }
python3 "$FT/🧰️scripts/R148/land.py" apply $S/lib.backup || { rmdir "$FT/landing.lock" 2>/dev/null; rm -f "$FT/landing.lock/owner"; rmdir "$FT/landing.lock"; exit 3; }
python3 "$FT/🧰️scripts/R148/land2.py" apply $S/bk2 || { python3 "$FT/🧰️scripts/R148/land.py" restore $S/lib.backup; rm -f "$FT/landing.lock/owner"; rmdir "$FT/landing.lock"; exit 4; }
run(){ ( cd "$1" && shift && env -u SEMIO_FLEET_GROUP -u SEMIO_FLEET_NFS_HOST -u SEMIO_FLEET_NETWORK -u SEMIO_REMOTE_CACHE_ENDPOINT -u SEMIO_REMOTE_CACHE_ACCESS -u SEMIO_REMOTE_CACHE_INSTANCE -u SEMIO_REMOTE_CACHE_TOKEN SEMIO_TEST_ARTIFACT_DIR=$S/art $BUN test ./🟦️.ts --timeout 120000 "$@" 2>&1 | tail -5 | /usr/bin/grep -E "pass|fail" | tr '\n' ' '; echo; ); }
ok=1
echo "module-references: $(run $L/🏗️build/🧮️task-model/🔎️module-references/🧪️tests)"
echo "same-cause: $(run $L/🏗️build/🏡️resident/🧪️tests/🧯️failure/🔁️same-cause)"
echo "failure: $(run $L/🏗️build/🏡️resident/🧪️tests/🧯️failure)"
echo "host: $(run $L/🏗️build/🏡️resident/🧪️tests/🌐️host -t 'unexpected failure|resident service that throws|full disk')"
echo "nx-engine: $(run $L/🏗️build/🧪️tests/🕸️graph/🔌️nx-engine)"
cd $S && $BUN digest.mjs after.json
cd /Users/ueli/Documents/semio
echo "end $(date -u +%H:%M:%S)"
