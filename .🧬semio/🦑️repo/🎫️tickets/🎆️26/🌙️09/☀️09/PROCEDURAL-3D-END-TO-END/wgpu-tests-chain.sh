#!/bin/sh
cd /Users/ueli/Documents/semio
TF=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️09/PROCEDURAL-3D-END-TO-END"
while [ "$(grep -c . $TF/🗑️generated/wgpu-summary.txt 2>/dev/null)" -lt 3 ]; do sleep 10; done
for c in 🪜️engine-attach-tests-2 🍰️shell-tests-input-3 🥧️shell-tests-docs-1 🥪️shell-tests-docs-2 👷️frame-job 🪤️runtime-mailbox-tests; do
  $TF/wgpu-check.sh check "$c" --tests
done
