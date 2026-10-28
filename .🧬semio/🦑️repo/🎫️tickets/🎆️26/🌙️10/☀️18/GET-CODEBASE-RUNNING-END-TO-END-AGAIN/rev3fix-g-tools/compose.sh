#!/bin/sh
# compose.sh <overlay> <order: mine-last|mine-first>
set -e
cd /Users/ueli/Documents/semio
S=/private/tmp/claude-501/-Users-ueli-Documents-semio/de5d5cb4-dca7-469c-b840-78655a0500ed/scratchpad/rev3fix; OV=$1; ORDER=$2
B="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build"; L="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"; P="🧰️framework/🔨️modules/🏃️process"
rm -rf "$OV"
python3 ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/rev3fix-g-tools/overlay.py" "$OV" @".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/rev3fix-g-tools/staged/rev3fix/edits.json" @".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/rev3-g-tools/staged/upgrade-before-custody/edits.json" @".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/rev3-g-tools/staged/win-withheld/edits.json" @".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/gram2-g-tools/staged/index-chunks/edits.json" "$B/🕸️graph" "$B/🌡️host-capacity" "$B/🔧️tools/🏗️bazel" "$L/⚡️caching/🦀️cargo/🗃️unit-cache" "$P/🔒️leases/📁️physical" "$P/🏗️build/🌡️host-capacity" 2>&1 | tail -2
python3 ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/rev3-g-tools/batch.py" ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/rev3-g-tools/staged/win-withheld" apply "$OV" | tail -1
python3 ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/rev3-g-tools/batch.py" ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/rev3-g-tools/staged/upgrade-before-custody" apply "$OV" | tail -1
if [ "$ORDER" = mine-first ]; then python3 ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/rev3fix-g-tools/batch.py" ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/rev3fix-g-tools/staged/rev3fix" apply "$OV" | grep -v "^before\|^absent\|^after"; fi
BATCH_ROOT="$OV" python3 ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/gram2-g-tools/staged/batch_generic.py" ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/gram2-g-tools/staged/index-chunks" apply | tail -3
if [ "$ORDER" = mine-last ]; then python3 ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/rev3fix-g-tools/batch.py" ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/rev3fix-g-tools/staged/rev3fix" apply "$OV" | grep -v "^before\|^absent\|^after"; fi
