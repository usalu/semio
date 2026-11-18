#!/bin/zsh
cd /Users/ueli/Documents/semio || exit 1
T=/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION
D=$T/🧰️scripts/R168
S=/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R168
LIBR="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📦️dependencies"
BUN=/Users/ueli/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
FILES=("$LIBR/🧾️incremental/💾️storage/🟦️.ts" "$LIBR/🧾️generation/💾️storage/🟦️.ts")
until [ $((10#$(date -u +%M))) -lt 5 ] || { [ $((10#$(date -u +%M))) -ge 30 ] && [ $((10#$(date -u +%M))) -lt 35 ]; }; do sleep 1; done
waited=0
until mkdir "$T/landing.lock" 2>/dev/null; do sleep 2; waited=$((waited+2)); [ $waited -lt 240 ] || { echo "never got the lock: $(cat $T/landing.lock/owner)"; exit 1; }; done
echo "claude-R168 $(date -u +%Y-%m-%dT%H:%M:%S)Z" > "$T/landing.lock/owner"
START=$(date -u +%H:%M:%S); echo "lock taken $START after ${waited}s"
rm -rf $S/bk; mkdir -p $S/bk; for f in "${FILES[@]}"; do mkdir -p "$S/bk/$(dirname $f)"; cp "$f" "$S/bk/$f"; done
release(){ rm -f "$T/landing.lock/owner"; rmdir "$T/landing.lock"; }
restore(){ for f in "${FILES[@]}"; do cp "$S/bk/$f" "$f"; done; }
fail(){ echo "RED: $1"; restore; for f in "${FILES[@]}"; do cmp "$S/bk/$f" "$f" || echo "RESTORE MISMATCH $f"; done; release; echo "restored $(date -u +%H:%M:%S)"; exit 3; }
python3 $D/patch-a.py /Users/ueli/Documents/semio || fail apply
for f in "${FILES[@]}"; do cmp "$f" "$S/new/${f#$LIBR/}" || fail "differs from staged $f"; done
ENVV=(env -u SEMIO_FLEET_GROUP -u SEMIO_FLEET_NFS_HOST -u SEMIO_FLEET_NETWORK -u SEMIO_REMOTE_CACHE_ENDPOINT -u SEMIO_REMOTE_CACHE_ACCESS -u SEMIO_REMOTE_CACHE_INSTANCE -u SEMIO_REMOTE_CACHE_TOKEN SEMIO_TEST_ARTIFACT_DIR=$S/art)
(cd "$LIBR/🧾️incremental/🧪️tests/🩹️evidence" && "${ENVV[@]}" nice -n 10 $BUN test ./🟦️.ts --timeout 180000 2>&1 | grep -E "^ *[0-9]+ (pass|fail)") | tee $S/win-ev.txt; grep -q " 3 pass" $S/win-ev.txt && grep -q " 0 fail" $S/win-ev.txt || fail evidence-law
(cd "$LIBR/🧾️incremental/🧪️tests/🏎️ports" && "${ENVV[@]}" nice -n 10 $BUN test ./🟦️.ts --timeout 180000 2>&1 | grep -E "^ *[0-9]+ (pass|fail)") | tee $S/win-ports.txt; grep -q " 3 pass" $S/win-ports.txt && grep -q " 0 fail" $S/win-ports.txt || fail ports
(cd "$LIBR/🧾️generation/🧪️tests" && "${ENVV[@]}" nice -n 10 $BUN test ./🟦️.ts --timeout 180000 2>&1 | grep -E "^ *[0-9]+ (pass|fail)") | tee $S/win-gen.txt; grep -q " 15 pass" $S/win-gen.txt && grep -q " 1 fail" $S/win-gen.txt || fail generation-battery
echo "GREEN started $START ended $(date -u +%H:%M:%S)"
release
