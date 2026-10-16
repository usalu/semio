#!/bin/zsh
# 🧪️ Runs the slice D vitest scope with the renderer's own policy (copied from the nodegraph runner); logs go to 🗑️generated.
R=/Users/ueli/Documents/semio
G=$R/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/🗑️generated
T=$R/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/relo-board-tools
P=$R/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/⚛️react/📦️packages/🟦️typescript
mkdir -p "$G/relo-board-vitest-cache"
export SEMIO_TEST_LEVEL=long
export SEMIO_VITEST_POLICY="{\"version\":1,\"cwd\":\"$P\",\"toolPath\":\"$R/node_modules/vitest/vitest.mjs\",\"runtime\":\"$(which bun)\",\"coverageRuntime\":\"$(which bun)\",\"cacheRoot\":\"$G/relo-board-vitest-cache\",\"coverageDirectory\":\"$G/relo-board-vitest-cov\",\"budgetMs\":900000}"
cd "$P"
perl -e 'alarm 1500; exec @ARGV' -- node "$R/node_modules/vitest/vitest.mjs" run --config "${RELO_VITEST_CONFIG:-$T/vitest.config.ts}" --testTimeout 120000 --reporter=default "$@"
