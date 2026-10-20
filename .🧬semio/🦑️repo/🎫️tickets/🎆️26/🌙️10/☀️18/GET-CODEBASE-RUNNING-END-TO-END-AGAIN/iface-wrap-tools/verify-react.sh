#!/bin/bash
# React group verification, overlay only (never writes to the repository): (1) tsc over the whole mirrored elements tree with the staged files, (2) vitest suites in a copy-on-write overlay.
# usage: verify-react.sh tsc | vitest <path-substring>...
set -eu
A="$(cd "$(dirname "$0")" && pwd)"
E="🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine"
ROOT="$(cd "$A/../../../../../../../.." && pwd)"
case "$1" in
  tsc)
    python3 "$A/mk-tsc.py" r-all --mirror="$E/🧱️elements" --map="@semio-tech/framework-os/modules/renderer/engine/media/lifecycle=$E/🎬️media/🚚️lifecycle/🟦️.ts" "/🧱️elements/" "/🎬️media/🚚️lifecycle/"
    cd /private/tmp/iface-wrap/tsc/r-all
    nice -n 10 "$ROOT/node_modules/.bin/tsc" -p tsconfig.json --noEmit
    echo "tsc: 0 errors" ;;
  vitest)
    shift
    OV=/private/tmp/iface-wrap/vt
    nice -n 10 python3 "$A/mkovl.py" "$OV" --through react --real "$E/🧱️elements" --real "$E/🎯️targets/⚛️react/🧪️tests/🎚️config" --prefix "$E/" > /dev/null
    CD="$OV/$E/🎯️targets/⚛️react/🧪️tests/🎚️config"
    PKG="$OV/$E/🎯️targets/⚛️react/📦️packages/🟦️typescript"
    printf 'import base from "./🟦️.ts";\nexport default { ...base, server: { fs: { strict: false } } };\n' > "$CD/overlay.ts"
    mkdir -p /private/tmp/iface-wrap/vitest-cache
    export SEMIO_VITEST_POLICY="{\"version\":1,\"cwd\":\"$PKG\",\"toolPath\":\"$ROOT/node_modules/vitest/vitest.mjs\",\"runtime\":\"bun\",\"coverageRuntime\":\"bun\",\"cacheRoot\":\"/private/tmp/iface-wrap/vitest-cache\",\"coverageDirectory\":\"/private/tmp/iface-wrap/vitest-cov\",\"budgetMs\":600000}"
    cd "$PKG"
    SEMIO_TEST_LEVEL=long nice -n 10 bun "$ROOT/node_modules/vitest/vitest.mjs" run --config "$CD/overlay.ts" --configLoader native --testTimeout 120000 --hookTimeout 120000 "$@" ;;
  *) echo "usage: verify-react.sh tsc | vitest <filter>..."; exit 2 ;;
esac
