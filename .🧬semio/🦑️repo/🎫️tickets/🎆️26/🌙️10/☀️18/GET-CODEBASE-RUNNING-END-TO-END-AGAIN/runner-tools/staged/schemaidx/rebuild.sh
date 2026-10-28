#!/bin/sh
# 🔁 schemaidx rebuild on the current live tree (the index is hot): refresh base from live, regenerate the new bytes, reseal. `sh rebuild.sh`
HERE=$(cd "$(dirname "$0")" && pwd); ROOT=$(cd "$HERE/../../../../../../../../../.." && pwd)
python3 -I "$HERE/../../mkbatch.py" rebase schemaidx && bun "$HERE/regen.ts" "$ROOT" "$HERE/files/f01.json" && python3 -I "$HERE/../../mkbatch.py" seal schemaidx
