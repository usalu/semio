#!/bin/zsh
cd /Users/ueli/Documents/semio || exit 1
T=/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION
D=$T/🧰️scripts/R161
SCR=/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R161
B=/Users/ueli/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
LIB="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
LA="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🚀️launcher"
H=$(date -u +%H); M=$(date -u +%M)
[ "$M" -lt 5 ] || { echo "outside the hh:00-hh:05 window ($H:$M)"; exit 1; }
waited=0
until mkdir "$T/landing.lock" 2>/dev/null; do sleep 2; waited=$((waited+2)); [ $waited -lt 240 ] || { echo "never got the lock: $(cat $T/landing.lock/owner)"; exit 1; }; done
echo "claude-R161 $(date -u +%Y-%m-%dT%H:%M:%S)Z" > "$T/landing.lock/owner"
START=$(date -u +%H:%M:%S); echo "lock taken $START after ${waited}s"
release(){ rm -f "$T/landing.lock/owner"; rmdir "$T/landing.lock"; }
fail(){ echo "RED: $1"; python3 $D/land5.py restore $SCR/bk5; python3 $D/land2.py restore $SCR/bk2; release; echo "restored $(date -u +%H:%M:%S)"; exit 3; }
rm -rf $SCR/bk2 $SCR/bk5
python3 $D/land2.py apply $SCR/bk2 || fail apply2
python3 $D/land5.py apply $SCR/bk5 || fail apply5
for f in "$LIB/⚡️caching/🚀️bootstrap/🛠️tools/🟦️.ts" "$LIB/🏗️build/🏡️resident/🔭️watch/🏗️provision/🟨️.mjs" "$LIB/🏗️build/🏡️resident/🔭️watch/🏗️provision/🔌️tools/🟨️.mjs"; do $B build --no-bundle "$f" >/dev/null 2>$SCR/err.txt || { cat $SCR/err.txt | head -3; fail "syntax $f"; }; done
(cd "$LIB/🏗️build/🏡️resident/🧪️tests/🔎️installed-watcher" && env -u SEMIO_FLEET_GROUP -u SEMIO_REMOTE_CACHE_ENDPOINT $B test ./🟦️.ts --timeout 120000 2>&1 | tail -12 > $SCR/after.txt; tail -7 $SCR/after.txt; grep -q " 0 fail" $SCR/after.txt) || fail "watcher test"
rm -rf $SCR/go2; mkdir -p $SCR/go2/mini/flat; cp "$LA/📦️packages/🐹️go/go.mod" $SCR/go2/
n=0; find "$LA/📦️packages/🐹️go" -mindepth 2 -maxdepth 2 -name '*.go' -not -path '*📦️main*' | sort | while read f; do n=$((n+1)); cp "$f" "$SCR/go2/mini/flat/src$n.go"; done
m=0; find "$LA/🧪️tests" -name '*.go' | sort | while read f; do m=$((m+1)); cp "$f" "$SCR/go2/mini/flat/t${m}_test.go"; done
cp "$LA/📦️packages/🐹️go/🐹️.go" $SCR/go2/mini/flat/root.go; cp $SCR/go2/go.mod $SCR/go2/mini/flat/
echo '{}' > "$SCR/go2/mini/📋️project.json"; echo '{"workspaces":[]}' > $SCR/go2/mini/package.json
mkdir -p "$SCR/go2/mini/🧰️framework/🛍️products/🦑️repo/🔨️modules/🚀️launcher"; cp -R "$LA/🗂️registry" "$LA/🧫️fixtures" "$SCR/go2/mini/🧰️framework/🛍️products/🦑️repo/🔨️modules/🚀️launcher/"
(cd $SCR/go2/mini/flat && GOTOOLCHAIN=local GOFLAGS=-mod=mod GOPROXY=off GOWORK=off /Users/ueli/.local/go/bin/go test -run 'MachinePath|LauncherReads|FleetStore|Provisioning' ./... 2>&1 | tail -6 > $SCR/gotest.txt; cat $SCR/gotest.txt; grep -q "^ok" $SCR/gotest.txt) || fail "go tests"
echo "GREEN started $START ended $(date -u +%H:%M:%S)"
release
