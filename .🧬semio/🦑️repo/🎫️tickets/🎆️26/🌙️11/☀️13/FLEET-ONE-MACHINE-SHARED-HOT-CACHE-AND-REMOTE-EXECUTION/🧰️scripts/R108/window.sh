#!/bin/bash
R=/Users/ueli/Documents/semio; T="$R/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION"
B=$R/.cache/b/t/m/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
L=$R/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library; S="$L/🧪️testing/🌱️genuine-birth/📜️script.ts"
P=$L/🏗️build/🕸️graph/📥️inference/📦️project-nodes; I=$L/🏗️build/🕸️graph/📥️inference/🧭️script-inputs
mkdir "$T/landing.lock" 2>/dev/null || { echo "held: $(cat $T/landing.lock/owner)"; exit 1; }
echo "claude-R108 $(date -u +%Y-%m-%dT%H:%M:%S)Z" > "$T/landing.lock/owner"
red=0
for f in "$P/🌐️owner-pool/🧪️tests/🪦️reaper/🟦️.ts" "$P/🌐️owner-pool/🧪️tests/🟦️.ts" "$P/🏊️project-farm/🧪️tests/🪦️busy-orphan/🟦️.ts" "$P/🏊️project-farm/🧪️tests/🪦️orphans/🟦️.ts" "$I/🔌️provider-closures/🧪️tests/🛃️preflight/🟦️.ts" "$I/🪆️heavy-questions/🧪️tests/🟦️.ts"; do
  out=$(cd $R && $B $S --file="$f" --birth=setup --root=$R 2>&1 | grep -E "^ *[0-9]+ (pass|fail)" | tr '\n' ' '); echo "$out"
  case "$out" in *" 0 fail"*) ;; *) red=1;; esac
done
rm -r "$T/landing.lock"; if [ $red = 1 ]; then echo RED; else echo GREEN; fi; date -u
