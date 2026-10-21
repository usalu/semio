#!/bin/sh
# tool-iface: rebuilds the staged set from the current tree (regen.sh with a private scratch directory instead of /private/tmp/tool-iface; the tool directory is found from this script).
set -e
TOOL="$(cd "$(dirname "$0")/../../r6-tool-iface-tools" && pwd)"
cd "$TOOL"
SCRATCH="${R6_SCRATCH:-/private/tmp/r6}/tool-iface/ovl"
mkdir -p "$SCRATCH"
FIX="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/⚙️operations/🧪️tests/🧫️fixtures/📖️help/🔣️.json"
ROOT="$(cd ../../../../../../../.. && pwd)"
python3 build/registry.py
rm -rf src/generated
mkdir -p "src/generated/$(dirname "$FIX")"
cp "$ROOT/$FIX" "src/generated/$FIX"
python3 build/build.py
python3 build/overlay.py "$SCRATCH" >/dev/null
(cd "$SCRATCH" && nice -n 10 bun "$TOOL/build/regen-help.ts" "$SCRATCH")
cp "$SCRATCH/$FIX" "src/generated/$FIX"
python3 build/build.py
python3 build/overlay.py "$SCRATCH" >/dev/null
