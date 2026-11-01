#!/bin/bash
# sequential driver: retained -> transients -> gen3d etc
T=/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS/1101-tools
A="✏️s/🔌️plugins"

"$T/chk.sh" mutpolicy-fem2d "$A/🏗️fem/🗿️artifacts/◻️2d/Cargo.toml" native --lib -p semio-s-artifact-fem-2d-editor-transient
"$T/chk.sh" mutpolicy-wfc2d "$A/🀄️wfc/🗿️artifacts/◻️2d/Cargo.toml" native --lib -p semio-s-artifact-wfc-2d-transient
"$T/chk.sh" mutpolicy-wfc3d "$A/🀄️wfc/🗿️artifacts/🧊️3d/Cargo.toml" native --lib -p semio-s-artifact-wfc-3d-transient
"$T/chk.sh" mutpolicy-wfcbmp "$A/🀄️wfc/🗿️artifacts/🖼️bitmap/Cargo.toml" native --lib -p semio-s-artifact-wfc-bitmap-transient
echo done > "$T/../🗑️generated/1101/mutpolicy-batch1.rc"
