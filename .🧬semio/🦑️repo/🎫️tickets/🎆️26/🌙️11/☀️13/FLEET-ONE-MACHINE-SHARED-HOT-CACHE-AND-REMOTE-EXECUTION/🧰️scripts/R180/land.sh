#!/bin/zsh
cd /Users/ueli/Documents/semio || exit 1
T=/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION
S=/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R180
L="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/🔌️nx"
BUN=/Users/ueli/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
edits=("🟦️.ts|$S/staged-live/nx.ts|$S/base/nx.ts" "📦️graph/🟦️.ts|$S/staged-live/graph.ts|$S/base/graph.ts")
news=(
 "🤝️nested-custody/🟦️.ts|$S/stage/🤝️nested-custody/🟦️.ts"
 "🤝️nested-custody/🧬️schema/🔣️.json|$S/stage/🤝️nested-custody/🧬️schema/🔣️.json"
 "🤝️nested-custody/🧪️tests/🟦️.ts|$S/stage/🤝️nested-custody/🧪️tests/🟦️.ts"
 "🤝️nested-custody/🧪️tests/🔒️cross-process/🟦️.ts|$S/stage/🤝️nested-custody/🧪️tests/🔒️cross-process/🟦️.ts"
 "🤝️nested-custody/🧪️tests/🔒️cross-process/🧱️holder/🟦️.ts|$S/stage/🤝️nested-custody/🧪️tests/🔒️cross-process/🧱️holder/🟦️.ts"
 "📦️graph/🧯️release-failure/🟦️.ts|$S/stage/🧯️release-failure/🟦️.ts"
 "📦️graph/🧯️release-failure/🧬️schema/🔣️.json|$S/stage/🧯️release-failure/🧬️schema/🔣️.json"
 "📦️graph/🧯️release-failure/🧪️tests/🟦️.ts|$S/stage/🧯️release-failure/🧪️tests/🟦️.ts")
for r in $edits; do f=${r%%|*}; rest=${r#*|}; b=${rest#*|}; [ "$(shasum -a 256 < $b)" = "$(shasum -a 256 < "$L/$f")" ] || { echo "live file moved: $f"; exit 2; }; done
for r in $news; do f=${r%%|*}; [ ! -e "$L/$f" ] || { echo "new file exists: $f"; exit 2; }; done
M=$((10#$(date -u +%M))); { [ $M -ge 30 ] && [ $M -lt 35 ]; } || [ "$1" = "--any" ] || { echo "outside the window ($M)"; exit 4; }
waited=0; until mkdir "$T/landing.lock" 2>/dev/null; do sleep 2; waited=$((waited+2)); M=$((10#$(date -u +%M))); { [ $M -ge 30 ] && [ $M -lt 35 ]; } || [ "$1" = "--any" ] || { echo "window closed while waiting: $(cat $T/landing.lock/owner)"; exit 1; }; [ $waited -lt 280 ] || { echo "no lock: $(cat $T/landing.lock/owner)"; exit 1; }; done
echo "claude-R180 $(date -u +%Y-%m-%dT%H:%M:%S)Z" > "$T/landing.lock/owner"; START=$(date -u +%H:%M:%S); echo "lock $START after ${waited}s"
release(){ rm -f "$T/landing.lock/owner"; rmdir "$T/landing.lock"; }
restore(){ for r in $edits; do f=${r%%|*}; rest=${r#*|}; b=${rest#*|}; cp "$b" "$L/$f"; done; for r in $news; do f=${r%%|*}; rm -f "$L/$f"; done; for d in "🤝️nested-custody/🧪️tests/🔒️cross-process/🧱️holder" "🤝️nested-custody/🧪️tests/🔒️cross-process" "🤝️nested-custody/🧪️tests" "🤝️nested-custody/🧬️schema" "🤝️nested-custody" "📦️graph/🧯️release-failure/🧪️tests" "📦️graph/🧯️release-failure/🧬️schema" "📦️graph/🧯️release-failure"; do rmdir "$L/$d" 2>/dev/null; done; echo "restored $(date -u +%H:%M:%S)"; }
fail(){ echo "RED: $1"; restore; release; exit 3; }
for r in $news; do f=${r%%|*}; st=${r#*|}; mkdir -p "$(dirname "$L/$f")"; cp "$st" "$L/$f" || fail copy; done
for r in $edits; do f=${r%%|*}; rest=${r#*|}; st=${rest%%|*}; cp "$st" "$L/$f" || fail copy; done
export BUN_RUNTIME_TRANSPILER_CACHE_PATH=0
for f in "🤝️nested-custody/🧪️tests/🟦️.ts" "🤝️nested-custody/🧪️tests/🔒️cross-process/🟦️.ts" "📦️graph/🧯️release-failure/🧪️tests/🟦️.ts"; do
  d=$(dirname "$L/$f"); b=$(basename "$f"); out=$(cd "$d" && env -u SEMIO_FLEET_GROUP -u SEMIO_FLEET_NFS_HOST -u SEMIO_FLEET_NETWORK -u SEMIO_REMOTE_CACHE_ENDPOINT -u SEMIO_REMOTE_CACHE_ACCESS -u SEMIO_REMOTE_CACHE_INSTANCE -u SEMIO_REMOTE_CACHE_TOKEN nice -n 10 $BUN test ./$b --timeout 120000 2>&1); echo "$f $(echo "$out"|/usr/bin/grep -E '^ *[0-9]+ (pass|fail)'|tr -d ' '|tr '\n' ' ')"; echo "$out"|/usr/bin/grep -q "^ *0 fail" || fail "$f"; done
smoke=$(cd /tmp && $BUN -e "const a=await import('$L/🟦️.ts');const b=await import('$L/📦️graph/🟦️.ts');console.log(Object.keys(a).length,Object.keys(b).length,a.withOriginalNxFixedCacheV1.toString().includes('admit'))" 2>&1 | tail -1); echo "smoke $smoke"; [ "$smoke" = "7 41 true" ] || fail smoke
echo "GREEN $START $(date -u +%H:%M:%S)"; release
