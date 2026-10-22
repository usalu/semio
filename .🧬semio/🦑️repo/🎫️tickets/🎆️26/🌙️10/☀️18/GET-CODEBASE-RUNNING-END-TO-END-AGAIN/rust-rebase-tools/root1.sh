#!/bin/sh
cd /Users/ueli/Documents/semio || exit 2
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
L="$C/🗑️generated/rust-rebase-logs"
args=""
for c in os-history os-store-read os-store-engine os-store-space-history os-store-space-schema os-store-backbone os-store-canonical-json os-store-codec-registry os-store-initialization-replay os-store-ledger os-store-member-open os-store-one-item-publication os-store-retirement os-store-snapshot-clone os-store-one-item-batch; do args="$args -p semio-framework-$c"; done
exec sh "$C/rust-rebase-tools/run.sh" Cargo.toml "$L/${1:-root1}.log" $args
