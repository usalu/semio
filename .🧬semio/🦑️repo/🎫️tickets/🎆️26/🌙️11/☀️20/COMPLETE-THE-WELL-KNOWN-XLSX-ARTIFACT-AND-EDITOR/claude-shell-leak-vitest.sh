#!/bin/zsh
# Runs named renderer-react engine suites through the repo-managed vitest under the repo-managed bun. Usage: claude-shell-leak-vitest.sh <suite-dir-name>...
ROOT=/Users/ueli/Documents/semio
BUN=$ROOT/.cache/b/t/m/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
PKG="$ROOT/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/⚛️react/📦️packages/🟦️typescript"
OUT="$ROOT/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️20/COMPLETE-THE-WELL-KNOWN-XLSX-ARTIFACT-AND-EDITOR/🗑️generated/claude-shell-leak"
mkdir -p "$OUT/cache" "$OUT/cov"
export SEMIO_TEST_LEVEL=long
export SEMIO_VITEST_POLICY="{\"version\":1,\"cwd\":\"$PKG\",\"toolPath\":\"$ROOT/node_modules/vitest/vitest.mjs\",\"runtime\":\"$BUN\",\"coverageRuntime\":\"$BUN\",\"cacheRoot\":\"$OUT/cache\",\"coverageDirectory\":\"$OUT/cov\",\"budgetMs\":300000}"
cd "$PKG" && exec "$BUN" "$ROOT/node_modules/vitest/vitest.mjs" run --config ../../🧪️tests/🎚️config/🟦️.ts --configLoader native "$@"
