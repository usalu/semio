#!/bin/bash
# usage: all.sh <fresh-dst-copy>   (SRC = my finished temp)
cd "$(dirname "$0")/.." || exit 1
export SRC=${SRC:-$PWD/w} DST=$1
for s in boundary git replica replica2 attest attest_tests tests; do python3 land/$s.py || { echo "FAILED $s"; exit 1; }; done
echo landed-code-ok
SRC=$PWD/w land/patchfile.sh 🚧️boundary/🧪️tests/🟦️.ts 🚧️boundary/🧪️tests/🐍️.py 🚧️boundary/🧪️tests/🧫️fixtures/🔣️.json 🚧️boundary/🧬️schema/🔣️.json 🚧️boundary/🔤️fold/🔣️.json 🚧️boundary/🔤️fold/📜️script.ts 🗺️topology/🟦️.ts 🗺️topology/⌨️command/🟦️.ts 🗺️topology/📖️texts/🔣️.json 🗺️topology/🥾️provision/🔵️.ps1 🗺️topology/🧪️tests/🔵️.ts 🗺️topology/🧪️tests/🔵️.ps1 🗺️topology/🧪️tests/🧫️fixtures/🪟️task-schema.xsd || exit 1
SRC=$PWD/wt3 land/patchfile.sh 🗺️topology/🧪️tests/🟦️.ts || exit 1
M="🧰️framework/🛍️products/🦑️repo/🔨️modules/🤝️shared-worktree"
python3 guards-fixture.py "$DST/$M/🩺️attest/🧪️tests" >/dev/null || exit 1
echo landed-all-ok
