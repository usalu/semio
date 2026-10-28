#!/bin/sh
# 🧷 [td-graph] applies every td-graph edit set to <root> (a mirror; base = live + D3): sh apply-mine.sh <root>
R="${1:?root}"; T="$(cd "$(dirname "$0")" && pwd)"; B="$R/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build"
python3 -I "$T/edits/derived_walks.py" "$R" || exit 2
python3 -I "$T/anchors.py" apply "$T/anchors/source-demand.json" "$B/🧪️tests/🕸️graph/🎯️source-demand/🟦️.ts" || exit 2
python3 -I "$T/anchors.py" apply "$T/anchors/graph-fixture.json" "$B/🧪️tests/🧫️fixtures/🕸️graph/🔣️.json" || exit 2
python3 -I "$T/anchors.py" apply "$T/anchors/inference-source.json" "$B/🕸️graph/📥️inference/🧪️tests/📖️source/🟦️.ts" || exit 2
for a in "$T"/anchors/extra-*.json; do [ -f "$a" ] || continue; rel=$(python3 -I -c "import json,sys;print(json.load(open(sys.argv[1]))['rel'])" "$a.rel" 2>/dev/null); done
echo applied
