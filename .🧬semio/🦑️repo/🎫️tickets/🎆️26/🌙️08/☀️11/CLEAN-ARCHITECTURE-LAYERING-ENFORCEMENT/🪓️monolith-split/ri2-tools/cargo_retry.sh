#!/bin/bash
# usage: cargo_retry.sh <tag> <extra args...> -- <crates...> : retries cargo_batch.sh every 10 minutes while the launcher refuses for headroom (max 12 tries)
HERE="$(cd "$(dirname "$0")" && pwd)"
G="$HERE/../🗑️generated"
TAG="$1"
for i in $(seq 1 12); do
  "$HERE/cargo_batch.sh" "$@"
  if ! grep -q "Insufficient physical compiler initial headroom" "$G/ri2-cargo-$TAG.log"; then break; fi
  cp "$G/ri2-cargo-$TAG.log" "$G/ri2-cargo-$TAG.refused-$i.log"
  sleep 600
done
