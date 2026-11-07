#!/bin/bash
# usage: rs.sh <label> <cargo test args...>   (scene-ink-tests twin, via slot.sh, keep-going)
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"; M="🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧶️scene-ink-tests/📦️packages/🦀️rust/Cargo.toml"; N=$1; shift
O="$C/🗑️generated/1105b/$N"; rm -f "$O.rc"
"$C/1105b-tools/slot.sh" "$N" env SEMIO_LANGUAGE=en repo cargo --store=shared --configuration=prototype -- "$@" > "$O.txt" 2>&1
echo $? > "$O.rc"
