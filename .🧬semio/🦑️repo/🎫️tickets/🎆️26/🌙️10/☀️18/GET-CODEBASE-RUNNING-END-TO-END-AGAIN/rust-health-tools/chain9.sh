#!/bin/sh
P=$1; D=$(dirname "$0")
sh "$D/run.sh" "$P-lock.log" update --workspace --offline
sh "$D/run.sh" "$P.log" check --workspace --all-targets --keep-going --message-format=short
sh "$D/run.sh" "$P-fwlock.log" update --manifest-path "🧰️framework/Cargo.toml" --workspace --offline
sh "$D/run.sh" "$P-fw.log" check --manifest-path "🧰️framework/Cargo.toml" --workspace --exclude semio-framework-async --all-targets --keep-going --message-format=short
sh "$D/run.sh" "$P-hublock.log" update --manifest-path "🌎️hub/Cargo.toml" --workspace --offline
sh "$D/run.sh" "$P-hub.log" check --manifest-path "🌎️hub/Cargo.toml" -p semio-hub-demonstrator-component --all-targets --keep-going --message-format=short
