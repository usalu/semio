#!/bin/zsh
# usage: run-tests.sh ; one repo test-file per listed file, sequential
cd /Users/ueli/Documents/semio
B="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build"
for f in \
 "$B/👁️observation/🧪️tests/🪪️witness/🟦️.ts" \
 "$B/🕸️graph/📥️inference/🧪️tests/🪪️verification-stamps/🟦️.ts" \
 "$B/🪪️graph-state/📦️project-records/🚚️entry-lane/🧪️tests/🟦️.ts" \
 "$B/🪪️graph-state/📦️project-records/🚚️entry-lane/🧪️tests/📤️background-offer/🟦️.ts" \
 "$B/🪪️graph-state/📦️project-records/🚚️entry-lane/🧪️tests/🛑️cancel/🟦️.ts" \
 "$B/🪪️graph-state/📦️project-records/🚚️entry-lane/🧪️tests/🌊️streamed-intake/🟦️.ts" \
 "$B/🪪️graph-state/📦️project-records/🚚️entry-lane/🧪️tests/🪢️shared-intake/🟦️.ts" \
 "$B/🕸️graph/📥️inference/🔎️owner-discovery/🕸️wasm-outputs/🧪️tests/🟦️.ts" \
 "$B/🕸️graph/📥️inference/🔎️owner-discovery/🕸️wasm-outputs/🧪️tests/🌱️flow-manifest/🟦️.ts" \
 "$B/📥️inputs/🦀️resolution/🧪️tests/🟦️.ts" \
 "$B/📥️inputs/🧪️tests/🟦️.ts" \
 "$B/🕸️graph/📥️inference/📦️project-nodes/🌐️owner-pool/🧪️tests/🗃️shared-closures/🟦️.ts" \
 "$B/🕸️graph/📥️inference/🧪️tests/♻️owner-reuse/🟦️.ts" ; do
  while [ $(sysctl -n kern.num_files) -gt 60000 ]; do sleep 20; done
  [ -f "$f" ] || { echo "MISSING $f"; continue; }
  repo test-file "$f" 2>&1 | grep -E "^(passed|FAILED) |^\(fail\)" | sed -E 's#🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/##' | cut -c1-200
done
echo TESTS-DONE
