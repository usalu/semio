#!/bin/sh
W=$1; P=$2; D=$(dirname "$0")
while ps -p "$W" >/dev/null 2>&1; do sleep 20; done
sh "$D/run.sh" "$P-teachtest.log" test --manifest-path "🎓️teaching/Cargo.toml" --workspace --no-fail-fast --message-format=short
sh "$D/run.sh" "$P.log" check --workspace --all-targets --keep-going --message-format=short
