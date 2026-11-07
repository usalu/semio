#!/bin/bash
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"
while [ ! -f "$C/🗑️generated/1105b/renames-5.rc" ]; do sleep 20; done
i=6
for spec in "🔨️modules/🛂️manifest/🎞️media|semio-framework-manifest-media" "🔨️modules/📡️replication/📡️wire/🎮️command/📥️ingress|semio-framework-replication-command-ingress" "🔨️modules/🎭️actor/🎟️retained-turn|semio-framework-actor-retained-turn" "🛍️products/💻️os/🔨️modules/🏪️store/👥️presence/📸️snapshot|semio-os-presence-native-snapshot" "🔨️modules/🚪️io/🧬️schema|semio-framework-io-schema" "🔨️modules/🖱️ui/🧬️contract/🔤️text|semio-ui-contract-text"; do
 d="${spec%%|*}"; p="${spec##*|}"
 "$C/1105b-tools/chk.sh" "renames-$i" "🧰️framework/$d/📦️packages/🦀️rust/Cargo.toml" wasm -p "$p" >/dev/null 2>&1
 i=$((i+1))
done
touch "$C/🗑️generated/1105b/renames-chain-done"
