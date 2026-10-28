#!/bin/sh
# 🔄 [optcache] resets the two edited files of mirror <name> to the live bytes and re-applies the anchored edits: usage mirror-sync.sh <mirror root>
C="$(cd "$(dirname "$0")/.." && pwd)"; M="$1"; L="$(cd "$C/../../../../../../.." && pwd)"
S="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧪️tests/🕸️graph"
cp "$L/$S/🧰️support/🟦️.ts" "$M/$S/🧰️support/🟦️.ts" && cp "$L/$S/🧊️optional-cache/🟦️.ts" "$M/$S/🧊️optional-cache/🟦️.ts" && python3 -I "$C/optcache-tools/edit_optcache.py" "$M"
