#!/bin/sh
cd /Users/ueli/Documents/semio
TF=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️09/PROCEDURAL-3D-END-TO-END"
while ! grep -q "runtime-mailbox-tests" $TF/🗑️generated/wgpu-summary.txt 2>/dev/null; do sleep 10; done
for c in 🥧️shell-tests-docs-1 🥪️shell-tests-docs-2 🪜️engine-attach-tests-2; do
  $TF/wgpu-check.sh check "$c" --tests
done
