#!/bin/sh
TF=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️09/PROCEDURAL-3D-END-TO-END"
cd /Users/ueli/Documents/semio
for i in $(seq 1 30); do
  sh "$TF/wgpu-frame-job-test.sh" "$@"
  grep -q "could not compile" "$TF/🗑️generated/wgpu-frame-job-test.txt" || break
  sleep 120
done
