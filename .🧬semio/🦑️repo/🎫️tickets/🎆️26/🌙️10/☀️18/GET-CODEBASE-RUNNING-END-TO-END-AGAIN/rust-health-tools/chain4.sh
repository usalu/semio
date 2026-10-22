#!/bin/sh
W=$1; P=$2; D=$(dirname "$0")
while kill -0 "$W" 2>/dev/null; do sleep 15; done
sh "$D/run.sh" "$P-lock.log" update --workspace --offline
sh "$D/run.sh" "$P.log" check --workspace --all-targets --keep-going --message-format=short
sh "$D/run.sh" "$P-fw.log" check --manifest-path "🧰️framework/Cargo.toml" --workspace --exclude semio-framework-async --all-targets --keep-going --message-format=short
