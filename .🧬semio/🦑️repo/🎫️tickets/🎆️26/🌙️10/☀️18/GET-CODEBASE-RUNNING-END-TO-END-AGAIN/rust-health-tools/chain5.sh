#!/bin/sh
W=$1; P=$2; D=$(dirname "$0")
while kill -0 "$W" 2>/dev/null; do sleep 15; done
sh "$D/run.sh" "$P-fwlock.log" update --manifest-path "🧰️framework/Cargo.toml" --workspace --offline
sh "$D/run.sh" "$P-fw2.log" check --manifest-path "🧰️framework/Cargo.toml" --workspace --exclude semio-framework-async --all-targets --keep-going --message-format=short
sh "$D/run.sh" "$P-hublock.log" update --manifest-path "🌎️hub/Cargo.toml" --workspace --offline
sh "$D/run.sh" "$P-hub.log" check --manifest-path "🌎️hub/Cargo.toml" -p semio-hub-demonstrator-component --all-targets --keep-going --message-format=short
