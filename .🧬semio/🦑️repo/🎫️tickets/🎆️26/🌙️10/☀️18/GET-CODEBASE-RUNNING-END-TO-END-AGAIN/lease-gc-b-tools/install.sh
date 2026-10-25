#!/bin/sh
# Installs the reviewed leases-b tree into the live leases module; refuses when the live core changed since the snapshot.
set -e
cd /Users/ueli/Documents/semio
L="🧰️framework/🔨️modules/🏃️process/🔒️leases"; B="${L}-b"
S=/private/tmp/claude-501/-Users-ueli-Documents-semio/08ddb7d5-f718-486f-900f-04b37cd0c5c0/scratchpad
cmp "$L/🟦️.ts" "$S/leases-core.orig.ts"
put() { mkdir -p "$(dirname "$L/$1")"; cp "$B/$1" "$L/$1.install-tmp"; mv "$L/$1.install-tmp" "$L/$1"; echo "installed $1"; }
put "♻️retirement/🧬️schema/🔣️.json"
put "♻️retirement/🧱️nested/🟦️.ts"
put "♻️retirement/🧹️sweep/🟦️.ts"
put "♻️retirement/🧹️sweep/🧪️tests/🐍️.py"
put "♻️retirement/🧹️sweep/🧪️tests/🔣️.json"
put "♻️retirement/🧹️sweep/🧪️tests/🟦️.ts"
put "♻️retirement/🧹️sweep/🧪️tests/🧬️schema/🔣️.json"
put "🟦️.ts"
