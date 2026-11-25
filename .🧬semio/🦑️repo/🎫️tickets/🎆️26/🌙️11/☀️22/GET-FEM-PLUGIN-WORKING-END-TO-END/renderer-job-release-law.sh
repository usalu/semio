#!/usr/bin/env bash
# 🧹️ Runs the plugin-runtime spawned-job laws (orphaned job cancel, guest cancel-job, ledger cancel) natively with an explicit Vitest policy. Log: 🗑️generated/stock/renderer-vitest.log
T="$(cd "$(dirname "$0")" && pwd)"; G="$T/🗑️generated/stock"; mkdir -p "$G" /c/t/stock-vitest
P="/c/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/⚛️react/📦️packages/🟦️typescript"
cd "$P" || exit 2
W="$(cygpath -m "$P")"; TOOL="$(cygpath -m /c/semio/node_modules/.bin/vitest.exe)"
export SEMIO_TEST_LEVEL=standard SEMIO_VITEST_POLICY="{\"version\":1,\"cwd\":\"$W\",\"toolPath\":\"$TOOL\",\"runtime\":\"node\",\"coverageRuntime\":\"node\",\"cacheRoot\":\"C:/t/stock-vitest\",\"coverageDirectory\":\"C:/t/stock-vitest/coverage\",\"budgetMs\":120000}"
timeout 590 "$TOOL" run --configLoader native --config "../../🧪️tests/🎚️config/🟦️.ts" "../../../../🧪️tests/🔌️plugin-runtime-suite/🟦️.tsx" --testNamePattern="${1:-failed drive still owns|cancel-job only while|routes a cancel to the guest|steps it to its terminal}" > "$G/renderer-vitest.log" 2>&1
echo "rc=$?" >> "$G/renderer-vitest.log"
