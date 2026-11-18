#!/bin/zsh
S=/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R117
T=/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION/🧰️scripts/R117
BUN=/Users/ueli/Documents/semio/.cache/b/t/m/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
H=/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity
cd $S/run || exit 1
sed -e "s#\"\.\./\.\./🟦️\.ts\"#\"$H/🟦️.ts\"#" -e "s#\"\.\./🧫️fixtures/🔣️\.json\"#\"$H/🧪️tests/🧫️fixtures/🔣️.json\"#" -e "s#\"\.\./🤲️lend/🧫️fixtures/🔣️\.json\"#\"$H/🧪️tests/🤲️lend/🧫️fixtures/🔣️.json\"#" $S/tests/essential.test.ts > essential.test.ts
F=/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/📦️project-nodes/🏊️project-farm
B=/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference
sed -e "s#\"\.\./\.\./🟨️\.mjs\"#\"$F/🟨️.mjs\"#" -e "s#\"\.\./\.\./🏊️pool/🟨️\.mjs\"#\"$F/🏊️pool/🟨️.mjs\"#" -e "s#\"\.\./\.\./\.\./\.\./\.\./\.\./👁️observation/🟨️\.mjs\"#\"$B/../../👁️observation/🟨️.mjs\"#" -e "s#\"\.\./\.\./\.\./\.\./\.\./\.\./🧠️memory/🦶️footprint/🟨️\.mjs\"#\"$B/../../🧠️memory/🦶️footprint/🟨️.mjs\"#" -e "s#\"\.\./\.\./\.\./\.\./\.\./\.\./🔣️policy\.json\"#\"$B/../../🔣️policy.json\"#" -e "s#\"\.\./\.\./🪆️captured/🟨️\.mjs\"#\"$F/🪆️captured/🟨️.mjs\"#" $S/tests/firstchild.test.ts > firstchild.test.ts
env -u SEMIO_FLEET_GROUP -u SEMIO_REMOTE_CACHE_ENDPOINT -u SEMIO_REMOTE_CACHE_TOKEN $BUN test --preload ${PRELOAD:-$T/preload.ts} ./${TESTFILE:-essential.test.ts} 2>&1 | tail -40
