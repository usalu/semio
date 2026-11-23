#!/bin/sh
cd /Users/ueli/Documents/semio
TF=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️09/PROCEDURAL-3D-END-TO-END"
while [ "$(grep -c "engine-attach-tests-2 exit" $TF/🗑️generated/wgpu-summary.txt)" -lt 2 ]; do sleep 10; done
$TF/wgpu-check.sh check 🥪️shell-tests-docs-2 --tests
