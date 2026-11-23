#!/bin/sh
cd /Users/ueli/Documents/semio
for c in "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧾️frame-transaction" "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📨️runtime-mailbox" "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🪀️shell-view-panels"; do
  n=$(basename "$c")
  while [ "$(memory_pressure | tail -1 | grep -o '[0-9]*%' | tr -d %)" -lt 40 ]; do sleep 10; done
  repo cargo --store=shared --configuration=prototype -- check --manifest-path "$c/📦️packages/🦀️rust/Cargo.toml" -j 2 > ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️09/PROCEDURAL-3D-END-TO-END/🗑️generated/p5a-check-$n.txt" 2>&1
  echo "$n exit $?" >> ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️09/PROCEDURAL-3D-END-TO-END/🗑️generated/p5a-check-summary.txt"
done
