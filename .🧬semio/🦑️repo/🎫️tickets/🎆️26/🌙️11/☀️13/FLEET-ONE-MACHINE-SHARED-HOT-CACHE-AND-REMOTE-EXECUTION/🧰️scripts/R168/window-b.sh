#!/bin/zsh
cd /Users/ueli/Documents/semio || exit 1
T=/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION
D=$T/🧰️scripts/R168
S=/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R168
W="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🟦️bun"
BUN=/Users/ueli/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
FILES=("$W/🟨️.cjs" "$W/🟦️.ts" "$W/🧬️schema/🔣️.json")
STAGED=("$S/new-b/cjs" "$S/new-b/ts" "$S/new-b/json")
until [ $((10#$(date -u +%M))) -lt 5 ] || { [ $((10#$(date -u +%M))) -ge 30 ] && [ $((10#$(date -u +%M))) -lt 35 ]; }; do sleep 1; done
waited=0
until mkdir "$T/landing.lock" 2>/dev/null; do sleep 2; waited=$((waited+2)); [ $waited -lt 240 ] || { echo "never got the lock: $(cat $T/landing.lock/owner)"; exit 1; }; done
echo "claude-R168 $(date -u +%Y-%m-%dT%H:%M:%S)Z" > "$T/landing.lock/owner"
START=$(date -u +%H:%M:%S); echo "lock taken $START after ${waited}s"
rm -rf $S/bkb; mkdir -p $S/bkb; for f in "${FILES[@]}"; do mkdir -p "$S/bkb/$(dirname $f)"; cp "$f" "$S/bkb/$f"; done
release(){ rm -f "$T/landing.lock/owner"; rmdir "$T/landing.lock"; }
restore(){ for f in "${FILES[@]}"; do cp "$S/bkb/$f" "$f"; done; rm -rf "$W/🧪️tests/🧭️closure"; }
fail(){ echo "RED: $1"; restore; for f in "${FILES[@]}"; do cmp "$S/bkb/$f" "$f" || echo "RESTORE MISMATCH $f"; done; release; echo "restored $(date -u +%H:%M:%S)"; exit 3; }
python3 $D/patch-b.py /Users/ueli/Documents/semio || fail apply
i=1; for f in "${FILES[@]}"; do cmp "$f" "${STAGED[$i]}" || fail "differs from staged $f"; i=$((i+1)); done
mkdir -p "$W/🧪️tests/🧭️closure"; cp $S/closure-test/test.ts "$W/🧪️tests/🧭️closure/🟦️.ts"
ENVV=(env -u SEMIO_FLEET_GROUP -u SEMIO_FLEET_NFS_HOST -u SEMIO_FLEET_NETWORK -u SEMIO_REMOTE_CACHE_ENDPOINT -u SEMIO_REMOTE_CACHE_ACCESS -u SEMIO_REMOTE_CACHE_INSTANCE -u SEMIO_REMOTE_CACHE_TOKEN SEMIO_TEST_ARTIFACT_DIR=$S/art)
run(){ (cd "$W/🧪️tests/$1" && "${ENVV[@]}" nice -n 10 $BUN test ./🟦️.ts --timeout 180000 2>&1 | grep -E "^ *[0-9]+ (pass|fail)" | tr -d '\n'); echo; }
r=$(run 🧭️closure); echo "closure: $r"; echo "$r" | grep -q "5 pass" && echo "$r" | grep -q " 0 fail" || fail closure-law
r=$(run ""); echo "root: $r"; echo "$r" | grep -q " 2 fail" || echo "$r" | grep -q " 0 fail" || fail root-battery
for d in ⏩️frontier ♻️refresh 🌊️churn; do r=$(run $d); echo "$d: $r"; echo "$r" | grep -q " 0 fail" || fail $d; done
echo "GREEN started $START ended $(date -u +%H:%M:%S)"
release
