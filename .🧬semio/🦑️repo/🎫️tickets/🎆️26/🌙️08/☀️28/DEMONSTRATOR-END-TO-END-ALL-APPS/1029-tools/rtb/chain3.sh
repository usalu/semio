#!/bin/bash
H=$(dirname "$0")
while ps -axo command | rg -q 'chain2\.sh'; do sleep 15; done
"$H/retry.sh" c pchk8 "✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d" --workspace
