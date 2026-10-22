#!/bin/sh
# usage: chain3.sh WAITPID LOGPREFIX -> after WAITPID ends: framework ws, teaching ws, hub demonstrator check
W=$1; P=$2; D=$(dirname "$0")
while kill -0 "$W" 2>/dev/null; do sleep 15; done
sh "$D/run.sh" "$P-fw.log" check --manifest-path "🧰️framework/Cargo.toml" --workspace --all-targets --keep-going --message-format=short
sh "$D/run.sh" "$P-teaching.log" check --manifest-path "🎓️teaching/Cargo.toml" --workspace --all-targets --keep-going --message-format=short
sh "$D/run.sh" "$P-hub.log" check --manifest-path "🌎️hub/Cargo.toml" -p semio-hub-demonstrator-component --all-targets --keep-going --message-format=short
