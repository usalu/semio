#!/bin/sh
# usage: serdechain.sh   -- serial cargo check --tests of every crate touched by serde wave 0 (logs serdechk-<n>.log)
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
W="🧰️framework/🛍️products/💻️os/🔨️modules"
T="$W/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu"
i=0
for m in \
 "$W/📡️spr/📦️packages/🦀️rust" \
 "$W/🔌️plugin/♻️cancellation/📦️packages/🦀️rust" \
 "$W/🏪️store/🔁️replay/🎮️operation/📦️packages/🦀️rust" \
 "$W/🔌️plugin/🧪️tests/🧪️history-edit-acceptance/📦️packages/🦀️rust" \
 "🧰️framework/🔨️modules/🖱️ui/🧬️contract/⚙️operations/📦️packages/🦀️rust" \
 "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧬️schema/🟦️projection/📦️packages/🦀️rust"
do
  i=$((i+1))
  sh "$C/ext-d-tools/run.sh" "serdechk-$i" cargo --store=shared --configuration=prototype -- check --tests --manifest-path "$m/Cargo.toml"
  echo "$i $m $(tail -1 "$C/🗑️generated/ext-d/serdechk-$i.rc")" >> "$C/🗑️generated/ext-d/serdechain.summary"
done
echo done >> "$C/🗑️generated/ext-d/serdechain.summary"
