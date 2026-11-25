#!/bin/bash
# usage: vt.sh <package-dir-rel> <config-rel-to-pkg> files...
export LC_ALL=en_US.UTF-8
ROOT=/Users/ueli/Documents/semio; PKG="$ROOT/$1"; CFG="$2"; shift 2
export SEMIO_VITEST_POLICY="{\"version\":1,\"cwd\":\"$PKG\",\"toolPath\":\"$ROOT/node_modules/vitest/vitest.mjs\",\"runtime\":\"$(which node)\",\"coverageRuntime\":\"$(which node)\",\"cacheRoot\":\"$ROOT/.cache/vt-ts\",\"coverageDirectory\":\"$ROOT/.cache/vt-cov\",\"budgetMs\":120000}"
cd "$PKG" && exec node "$ROOT/node_modules/vitest/vitest.mjs" run --config "$CFG" --configLoader native --maxWorkers=1 "$@"
