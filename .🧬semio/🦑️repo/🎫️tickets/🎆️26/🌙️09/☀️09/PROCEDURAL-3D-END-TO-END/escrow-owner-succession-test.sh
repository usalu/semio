#!/bin/sh
# 🧪 Runs the escrow owner-succession law natively (gated on memory pressure).
cd /Users/ueli/Documents/semio || exit 1
TF=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️09/PROCEDURAL-3D-END-TO-END"
P="🧰️framework/🔨️modules/🧵️job/⏱️context/📦️packages/🦀️rust/Cargo.toml"
while [ "$(memory_pressure | tail -1 | grep -o '[0-9]*%' | tr -d %)" -lt 35 ]; do sleep 15; done
repo cargo --store=shared --configuration=prototype --parallelism=1 -- test --manifest-path "$P" -j 1 --features testkit --test original-native-progress -- ${1:-successive_owners} --nocapture > "$TF/🗑️generated/escrow-owner-succession-test.txt" 2>&1
echo "exit $?" >> "$TF/🗑️generated/escrow-owner-succession-test.txt"
