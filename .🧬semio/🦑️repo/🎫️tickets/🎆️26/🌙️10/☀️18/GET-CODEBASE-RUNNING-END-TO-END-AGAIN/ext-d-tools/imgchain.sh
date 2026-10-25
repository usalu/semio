#!/bin/sh
# usage: imgchain.sh   -- serial cargo check --tests of every crate touched by image wave A (logs imgchk-<n>.log)
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
W="🧰️framework/🛍️products/💻️os/🔨️modules"
T="$W/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu"
i=0
for m in \
 "🧰️framework/🔨️modules/🖼️canvas/🪪️icon-codec/📦️packages/🦀️rust" \
 "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🪢️text-edit/📦️packages/🦀️rust" \
 "$W/🔌️plugin/🗃️document/📦️packages/🦀️rust" \
 "$W/♾️infinite/🌍️world/🧩️components/📦️packages/🦀️rust" \
 "$W/♾️infinite/🌍️world/🧱️scene/📦️packages/🦀️rust" \
 "$W/♾️infinite/🌍️world/🎛️host/📦️packages/🦀️rust" \
 "$W/♾️infinite/📦️packages/🦀️rust" \
 "$T/🎨️scene-canvas2d/📦️packages/🦀️rust" \
 "$T/👆️ui-engine-focus/📦️packages/🦀️rust" \
 "$T/✒️scene-ink/📦️packages/🦀️rust" \
 "$T/🖱️ui-engine/📦️packages/🦀️rust" \
 "$T/🧶️scene-ink-tests/📦️packages/🦀️rust" \
 "$T/🧿️ui-engine-tests/📦️packages/🦀️rust" \
 "$T/🌈️shell-tests-chrome-1/📦️packages/🦀️rust" \
 "$T/📦️packages/🦀️rust"
do
  i=$((i+1))
  sh "$C/ext-d-tools/run.sh" "imgchk-$i" cargo --store=shared --configuration=prototype -- check --tests --manifest-path "$m/Cargo.toml"
  echo "$i $m $(tail -1 "$C/🗑️generated/ext-d/imgchk-$i.rc")" >> "$C/🗑️generated/ext-d/imgchain.summary"
done
echo done >> "$C/🗑️generated/ext-d/imgchain.summary"
