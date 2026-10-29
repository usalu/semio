#!/bin/bash
H=$(dirname "$0")
while ps -axo command | rg -q 'chain3\.sh'; do sleep 15; done
"$H/retry.sh" c gchk4 "✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap" --workspace
