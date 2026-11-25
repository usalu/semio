#!/bin/sh
# usage: run-kernel-vitest.sh <suite path relative to the kernel module root> <log> [extra vitest args]
S=/Users/ueli/Documents/semio
R="$S/🧰️framework/🔨️modules/🎠️kernel/📦️packages/🟦️typescript"
cd "$R" || exit 2
SUITE="$1"; LOG="$2"; shift 2
export NODE_OPTIONS=--max-old-space-size=2048
G="$S/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/GET-PDF-ARTIFACT-EDITOR-WORKING-END-TO-END/🗑️generated/vitest"
mkdir -p "$G"
export SEMIO_TEST_ARTIFACT_DIR="$G/artifacts" SEMIO_VITEST_POLICY="{\"version\":1,\"cwd\":\"$R\",\"toolPath\":\"$S/node_modules/vitest/vitest.mjs\",\"runtime\":\"$(command -v node)\",\"coverageRuntime\":\"$(command -v node)\",\"cacheRoot\":\"$G/cache\",\"coverageDirectory\":\"$G/coverage\",\"budgetMs\":600000,\"maxWorkers\":1}"
exec "$(command -v node)" "$S/node_modules/vitest/vitest.mjs" run --config ../../🧪️tests/🎚️config/🟦️.ts --configLoader native --pool=forks --maxWorkers=1 --no-file-parallelism --reporter=verbose "$SUITE" "$@" > "$LOG" 2>&1
