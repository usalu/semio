#!/bin/bash
R=/Users/ueli/Documents/semio
T="$R/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION"
B=/Users/ueli/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
cd "$R" || exit 2
G="$(ls -d "$R"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️testing/🌱️genuine-birth/📜️script.ts)"
F="$R/🧰️framework/🔨️modules/🏃️process"
H="$R/🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🧪️tests"
D="$R/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🩺️doctor/🧪️tests"
L="$R/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🧭️script-inputs"
FILES=("$F/🧪️testing/🏖️sandbox/🧹️image-sweep/🧪️tests/🟦️.ts" "$H/💿️leaked-images/🟦️.ts" "$H/🚪️gate/🟦️.ts" "$F/📦️artifacts/🏗️native-build/💾️storage/🍎️image/📡️roster/🧪️tests/🟦️.ts" "$D/🟦️.ts" "$D/🔎️probes/🟦️.ts" "$L/🔌️provider-closures/🧪️tests/🛃️preflight/🟦️.ts" "$L/🪆️heavy-questions/🧪️tests/🟦️.ts")
mkdir "$T/landing.lock" 2>/dev/null || { echo "lock held: $(cat "$T/landing.lock/owner")"; exit 3; }
echo claude-R123 > "$T/landing.lock/owner"
trap 'rm -rf "$T/landing.lock"' EXIT
start=$(date -u +%H:%M:%S); red=0
for f in "${FILES[@]}"; do
  out=$("$B" "$G" --file="$f" --birth=setup --root="$R" 2>&1 | grep -E "^ *[0-9]+ (pass|fail)" | tr '\n' ' ')
  echo "$(basename "$(dirname "$(dirname "$f")")")/$(basename "$(dirname "$f")"): $out"
  case "$out" in *" 0 fail"*) ;; *) red=1;; esac
done
echo "start $start end $(date -u +%H:%M:%S) red=$red"
exit $red
