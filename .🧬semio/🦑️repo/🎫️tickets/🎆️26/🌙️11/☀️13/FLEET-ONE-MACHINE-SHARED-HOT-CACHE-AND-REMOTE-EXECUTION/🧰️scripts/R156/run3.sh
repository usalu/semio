#!/bin/zsh
S=/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R156
BUN=/Users/ueli/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
M=/Users/ueli/Documents/semio/node_modules
sed -e "s#from\"typescript\"#from\"$M/typescript/lib/typescript.js\"#" -e "s#from\"ajv\"#from\"$M/ajv/dist/ajv.js\"#" -e "s#\"../../../🧬️schema/📥️authority.json\"#\"$S/new/authority.json\"#" -e "s#\"../../../🕸️graph/🔁retry/🧬️schema/🔣️.json\"#\"$S/new/retry-schema.json\"#" $S/new/$1.ts > $S/stage/$1.test.ts
cd $S/stage && R151_LOAD_SOURCE=${LOADSRC:-$S/new/load.ts} R151_SCHEMA=${SCHEMASRC:-$S/new/graph-schema.json} R156_RETRY=file://$S/stage/🔁retry/🟨️.mjs nice -n 10 $BUN test ./$1.test.ts --timeout 180000 2>&1 | grep -E "^\(fail\)|^\(pass\)| pass| fail|error:|Expected|Received"
