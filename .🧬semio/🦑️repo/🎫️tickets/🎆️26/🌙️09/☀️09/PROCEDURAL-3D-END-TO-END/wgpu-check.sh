#!/bin/sh
# usage: wgpu-check.sh <check|test> <crate-dir-name> [extra cargo args after --]
cd /Users/ueli/Documents/semio
TF=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️09/PROCEDURAL-3D-END-TO-END"
W="🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu"
mode=$1; n=$2; shift 2
while [ "$(memory_pressure | tail -1 | grep -o '[0-9]*%' | tr -d %)" -lt 40 ] || [ "$(uptime | sed 's/.*averages: //' | cut -d. -f1)" -gt 20 ]; do sleep 15; done
repo cargo --store=shared --configuration=prototype -- $mode --manifest-path "$W/$n/📦️packages/🦀️rust/Cargo.toml" -j 2 "$@" > "$TF/🗑️generated/wgpu-$mode-$n.txt" 2>&1
echo "$mode $n exit $?" >> "$TF/🗑️generated/wgpu-summary.txt"
