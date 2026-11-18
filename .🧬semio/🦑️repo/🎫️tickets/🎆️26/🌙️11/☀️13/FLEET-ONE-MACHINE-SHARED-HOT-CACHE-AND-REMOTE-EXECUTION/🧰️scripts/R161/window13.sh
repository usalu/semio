#!/bin/zsh
cd /Users/ueli/Documents/semio || exit 1
T=/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION
D=$T/🧰️scripts/R161
SCR=/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R161
B=/Users/ueli/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
FW=/Users/ueli/Documents/semio/🧰️framework
waited=0
until mkdir "$T/landing.lock" 2>/dev/null; do sleep 2; waited=$((waited+2)); [ $waited -lt 200 ] || { echo "no lock"; exit 1; }; done
echo "claude-R161 $(date -u +%Y-%m-%dT%H:%M:%S)Z" > "$T/landing.lock/owner"
START=$(date -u +%H:%M:%S); echo "lock taken $START after ${waited}s"
release(){ rm -f "$T/landing.lock/owner"; rmdir "$T/landing.lock"; }
fail(){ echo "RED: $1"; python3 $D/land14.py restore $SCR/bk14; python3 $D/land13.py restore $SCR/bk13; release; echo "restored $(date -u +%H:%M:%S)"; exit 3; }
rm -rf $SCR/bk13 $SCR/bk14
python3 $D/land13.py apply $SCR/bk13 || fail apply13
python3 $D/land14.py apply $SCR/bk14 || fail apply14
for f in "$FW/🔨️modules/🏃️process/🧰️tools/🔎️resolve/🟦️.ts" "$FW/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏡️resident/🔭️watch/🏗️provision/🔌️tools/🟨️.mjs"; do $B build --no-bundle "$f" >/dev/null 2>$SCR/err.txt || { head -3 $SCR/err.txt; fail "syntax $f"; }; done
(cd "$FW/🔨️modules/🏃️process/🧰️tools/🧪️tests/🔎️resolve" && env -u SEMIO_FLEET_GROUP $B test ./🟦️.ts --timeout 120000 2>&1 | tail -12 > $SCR/after.txt; tail -6 $SCR/after.txt; grep -q " 0 fail" $SCR/after.txt) || fail "resolve tests"
(cd "$FW/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏡️resident/🧪️tests/🔎️installed-watcher" && $B test ./🟦️.ts --timeout 120000 2>&1 | tail -8 > $SCR/after2.txt; tail -5 $SCR/after2.txt; grep -q " 0 fail" $SCR/after2.txt) || fail "watcher tests"
echo "GREEN started $START ended $(date -u +%H:%M:%S)"
release
