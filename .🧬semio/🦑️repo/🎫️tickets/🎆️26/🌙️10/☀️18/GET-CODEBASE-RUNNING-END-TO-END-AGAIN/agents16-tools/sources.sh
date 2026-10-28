#!/bin/sh
# 🔁 agents16: puts the live source files of the three batches into <mirror> and applies the anchored edits in stack order (allrun, farmfree, behind); test files and fixtures in the mirror are left as they are.
# Usage: sh sources.sh <mirror-root>
M=$1; HERE=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd); L=$(cd "$HERE/../../../../../../../.." && pwd)
HC="🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity"; LIB="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build"
for f in "$HC/📏️decision/🟦️.ts" "$HC/🧬️schema/🔣️.json" "$HC/🎫️gate/🟦️.ts" "$HC/🔼️upgrade/🟦️.ts" "$HC/📐️estimates/🟦️.ts" "$HC/🌦️forecast/🟦️.ts" "$LIB/🌡️host-capacity/🟦️.ts" "$LIB/🕸️graph/📥️inference/📦️project-nodes/🏊️project-farm/🟨️.mjs"; do cp -p "$L/$f" "$M/$f"; done
python3 -I "$HERE/edit_allrun.py" "$M" && python3 -I "$HERE/edit_farmfree.py" "$M" && python3 -I "$HERE/edit_behind.py" "$M"
