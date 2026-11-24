#!/bin/sh
# 🧪 Runs the preview-eval on-demand law natively (gated on memory pressure; stops below 5 GiB free).
cd /Users/ueli/Documents/semio || exit 1
TF=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️09/PROCEDURAL-3D-END-TO-END"
P="✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧵️preview-eval/📦️packages/🦀️rust/Cargo.toml"
while [ "$(memory_pressure | tail -1 | grep -o '[0-9]*%' | tr -d %)" -lt 35 ]; do sleep 15; done
repo cargo --store=shared --configuration=prototype -- test --manifest-path "$P" -j 2 --lib -- a_parked_preview --nocapture > "$TF/🗑️generated/lazy-activation-rust-test.txt" 2>&1
echo "exit $?" >> "$TF/🗑️generated/lazy-activation-rust-test.txt"
