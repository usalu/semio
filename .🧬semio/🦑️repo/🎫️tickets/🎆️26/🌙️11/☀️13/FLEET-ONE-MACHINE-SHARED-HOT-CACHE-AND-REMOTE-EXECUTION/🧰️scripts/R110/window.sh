#!/bin/bash
# R110 landing: adds the folder 🧪️testing/🏟️fleet-rehearsal (new, additive). Runs only in the first 5 minutes of a half hour UTC, under landing.lock, restores (removes the folder) on red.
R=/Users/ueli/Documents/semio; T="$R/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION"
B=/Users/ueli/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
H=$R/.cache/b/t/m/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
L=$R/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library; S="$L/🧪️testing/🌱️genuine-birth/📜️script.ts"
I=$L/🏗️build/🕸️graph/📥️inference/🧭️script-inputs
SRC="$T/🧰️scripts/R110/kit"; DST="$L/🧪️testing/🏟️fleet-rehearsal"
minute=$(date -u +%M); if [ $((10#$minute % 30)) -ge 5 ]; then echo "outside the first 5 minutes of a half hour (UTC minute $minute)"; exit 2; fi
[ -e "$DST" ] && { echo "target exists"; exit 3; }
mkdir "$T/landing.lock" 2>/dev/null || { echo "held: $(cat $T/landing.lock/owner)"; exit 1; }
echo "claude-R110 $(date -u +%Y-%m-%dT%H:%M:%S)Z" > "$T/landing.lock/owner"
start=$(date -u +%H:%M:%S); red=0
mkdir -p "$DST" && cp -R "$SRC/." "$DST/"
out=$(cd "$DST/🧪️tests" && env -u SEMIO_FLEET_GROUP -u SEMIO_FLEET_NFS_HOST -u SEMIO_FLEET_NETWORK -u SEMIO_REMOTE_CACHE_ENDPOINT -u SEMIO_REMOTE_CACHE_ACCESS -u SEMIO_REMOTE_CACHE_INSTANCE -u SEMIO_REMOTE_CACHE_TOKEN $B test ./🟦️.ts --timeout 180000 2>&1 | grep -E "^ *[0-9]+ (pass|fail)" | tr '\n' ' '); echo "fleet-rehearsal: $out"
case "$out" in *" 0 fail"*) ;; *) red=1;; esac
for f in "$I/🔌️provider-closures/🧪️tests/🛃️preflight/🟦️.ts" "$I/🪆️heavy-questions/🧪️tests/🟦️.ts"; do
  o=$(cd $R && $H $S --file="$f" --birth=setup --root=$R 2>&1 | grep -E "^ *[0-9]+ (pass|fail)" | tr '\n' ' '); echo "$o"
  case "$o" in *" 0 fail"*) ;; *) red=1;; esac
done
if [ $red = 1 ]; then [ "$(realpath "$DST")" = "$DST" ] && rm -r "$DST"; echo RED; else echo GREEN; fi
rm -r "$T/landing.lock"; echo "window $start to $(date -u +%H:%M:%S)"
