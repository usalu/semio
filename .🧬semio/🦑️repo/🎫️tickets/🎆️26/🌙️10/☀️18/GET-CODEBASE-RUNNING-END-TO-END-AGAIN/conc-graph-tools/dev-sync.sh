#!/bin/sh
# copies the staged generation files into a scratch layout that mirrors the live member (for quick test runs without the full overlay)
# usage: dev-sync.sh [DEV_DIR]   (default /private/tmp/conc-graph/dev)
HERE=$(cd "$(dirname "$0")" && pwd)
DEV=${1:-/private/tmp/conc-graph/dev}
S="$HERE/staged/new/gen"
G="$DEV/🕰️generations"
mkdir -p "$G/🧪️tests/🏃️world" "$G/🧪️tests/🧬️schema" "$G/🧬️schema" "$DEV/🧭️file-owner"
LIVE="$HERE/../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph"
cp "$LIVE/🧭️file-owner/🟨️.mjs" "$DEV/🧭️file-owner/🟨️.mjs"
cp "$S/module.mjs" "$G/🟨️.mjs"; cp "$S/module.d.mts" "$G/🟨️.d.mts"; cp "$S/schema.json" "$G/🧬️schema/🔣️.json"
cp "$S/test.ts" "$G/🧪️tests/🟦️.ts"; cp "$S/fixture.json" "$G/🧪️tests/🔣️.json"; cp "$S/fixture-schema.json" "$G/🧪️tests/🧬️schema/🔣️.json"
cp "$S/oracle.py" "$G/🧪️tests/🐍️oracle.py"; cp "$S/world.ts" "$G/🧪️tests/🏃️world/🟦️.ts"
L="$G/🔌️load"; mkdir -p "$L/🧪️tests/🧬️schema"; mkdir -p "$DEV/⏱️phases"
cp "$LIVE/⏱️phases/🟨️.mjs" "$DEV/⏱️phases/🟨️.mjs"
[ -e "$HERE/staged/new/load/fixture.json" ] && { mkdir -p "$L/🧪️tests/🧬️schema"; cp "$HERE/staged/new/load/fixture.json" "$L/🧪️tests/🔣️.json"; cp "$HERE/staged/new/load/fixture-schema.json" "$L/🧪️tests/🧬️schema/🔣️.json"; }
cp "$HERE/staged/new/load/module.mjs" "$L/🟨️.mjs"; cp "$HERE/staged/new/load/module.d.mts" "$L/🟨️.d.mts"
[ -e "$HERE/staged/new/load/test.ts" ] && cp "$HERE/staged/new/load/test.ts" "$L/🧪️tests/🟦️.ts"
E="$G/🌍️environment"; mkdir -p "$E/🧪️tests/🧬️schema"
cp "$HERE/staged/new/env/module.mjs" "$E/🟨️.mjs"; cp "$HERE/staged/new/env/module.d.mts" "$E/🟨️.d.mts"
cp "$HERE/staged/new/env/test.ts" "$E/🧪️tests/🟦️.ts"; cp "$HERE/staged/new/env/fixture.json" "$E/🧪️tests/🔣️.json"; cp "$HERE/staged/new/env/fixture-schema.json" "$E/🧪️tests/🧬️schema/🔣️.json"
[ -e "$DEV/node_modules" ] || ln -s "$HERE/../../../../../../../../node_modules" "$DEV/node_modules"
echo "synced to $G"
