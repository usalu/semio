#!/bin/bash
export SEMIO_OFFLOAD=never
cd "/Users/ueli/Documents/semio"
run(){ n=$1; m=$2; p=$3; repo cargo --store=shared --configuration=prototype --parallelism=2 -- check --message-format=short --manifest-path "$m" -p "$p" > "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️19/DEV-WFC-WORKING-FOR-ALL-ARTIFACT-EDITORS/🗑️generated/check-$n.log" 2>&1; echo "rc=$?" >> "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️19/DEV-WFC-WORKING-FOR-ALL-ARTIFACT-EDITORS/🗑️generated/check-$n.log"; }
run bitmap "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🖼️bitmap/Cargo.toml" semio-s-artifact-wfc-bitmap-bitmap
run grid2d "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🔲️grid2d/Cargo.toml" semio-s-artifact-wfc-grid2d-v1
run wfc2d "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/◻️2d/Cargo.toml" semio-s-artifact-wfc-2d-v1
run grid3d "$HOME/Documents/semio/✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🧱️grid3d/Cargo.toml" semio-s-artifact-wfc-grid3d-editor
run wfc3d "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🧊️3d/Cargo.toml" semio-s-artifact-wfc-3d-wfc3d
echo done > "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️19/DEV-WFC-WORKING-FOR-ALL-ARTIFACT-EDITORS/🗑️generated/check.status"
