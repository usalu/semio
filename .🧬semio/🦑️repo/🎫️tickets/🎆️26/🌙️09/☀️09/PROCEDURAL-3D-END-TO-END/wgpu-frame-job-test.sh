#!/bin/sh
cd /Users/ueli/Documents/semio
TF=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️09/PROCEDURAL-3D-END-TO-END"
W="🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu"
while [ "$(df -g /System/Volumes/Data | tail -1 | awk '{print $4}')" -lt 10 ] || [ "$(memory_pressure | tail -1 | grep -o '[0-9]*%' | tr -d %)" -lt 35 ]; do sleep 15; done
CARGO_INCREMENTAL=0 repo cargo --store=shared --configuration=prototype --parallelism=1 -- test --manifest-path "$W/👷️frame-job/📦️packages/🦀️rust/Cargo.toml" -j 1 "$@" > "$TF/🗑️generated/wgpu-frame-job-test.txt" 2>&1
echo "frame-job test exit $?" >> "$TF/🗑️generated/wgpu-frame-job-test.txt"
