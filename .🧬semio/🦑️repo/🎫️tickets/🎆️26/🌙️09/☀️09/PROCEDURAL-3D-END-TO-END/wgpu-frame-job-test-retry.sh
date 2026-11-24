#!/bin/sh
TF=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️09/PROCEDURAL-3D-END-TO-END"
cd /Users/ueli/Documents/semio
for i in 1 2 3 4 5 6 7 8 9 10 11 12; do
  "$TF/wgpu-frame-job-test.sh" "$@"
  tail -1 "$TF/🗑️generated/wgpu-frame-job-test.txt" | grep -q "exit 75" || break
  sleep 20
done
