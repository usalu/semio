#!/bin/zsh
# usage: vt.sh <phase> [vitest filters...]   (cad-js vitest project, in-source tests of the spatial kernel)
export LC_ALL=en_US.UTF-8
R=/Users/ueli/Documents/semio
G="$R/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/🗑️generated"
mkdir -p "$G/tf8-vitest-cache"
phase=$1; shift
P="$R/✏️s/🧑‍💻dev/📐️cad/📦️packages/🟦️typescript"
export SEMIO_TEST_LEVEL=long
export SEMIO_VITEST_POLICY="{\"version\":1,\"cwd\":\"$P\",\"toolPath\":\"$R/node_modules/vitest/vitest.mjs\",\"runtime\":\"$(which bun)\",\"coverageRuntime\":\"$(which bun)\",\"cacheRoot\":\"$G/tf8-vitest-cache\",\"coverageDirectory\":\"$G/tf8-vitest-cov\",\"budgetMs\":1500000}"
cd "$P"
perl -e 'alarm 1700; exec @ARGV' -- node "$R/node_modules/vitest/vitest.mjs" run --config ../../🧪️tests/🎚️config/🟦️.ts --configLoader runner --testTimeout 120000 --reporter=verbose "$@" > "$G/tf8-$phase-vitest.log" 2>&1
echo "exit=$?" >> "$G/tf8-$phase-vitest.log"
