#!/bin/bash
D="$(cd "$(dirname "$0")" && pwd)"
"$D/hub.sh" cold-hub-1
bash "$D/gates/cext.sh" 1
echo done > "$D/../🗑️generated/1105b/cold-done"
