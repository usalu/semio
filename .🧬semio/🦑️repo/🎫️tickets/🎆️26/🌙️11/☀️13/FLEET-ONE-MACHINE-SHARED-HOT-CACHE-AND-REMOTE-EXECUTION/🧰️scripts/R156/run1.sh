#!/bin/zsh
S=/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R156
BUN=/Users/ueli/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
B="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build"
cd "$B/🧪️tests/🕸️graph/🩹️repair-rounds" || exit 1
sed 's#from"typescript"#from"/Users/ueli/Documents/semio/node_modules/typescript/lib/typescript.js"#' $S/new/repair-rounds.ts > $S/stage/repair-rounds.test.ts
R151_NODES="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/📦️project-nodes/🟨️.mjs" R151_LOAD_SOURCE=$S/new/load.ts R151_SCHEMA=$S/new/graph-schema.json R156_RETRY=file://$S/stage/🔁retry/🟨️.mjs NODE_PATH=/Users/ueli/Documents/semio/node_modules nice -n 10 $BUN test $S/stage/repair-rounds.test.ts --timeout 180000 2>&1 | grep -v "^$" | head -60
