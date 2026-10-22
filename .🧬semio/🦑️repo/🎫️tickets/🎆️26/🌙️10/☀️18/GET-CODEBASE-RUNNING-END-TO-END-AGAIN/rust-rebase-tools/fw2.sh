#!/bin/sh
cd /Users/ueli/Documents/semio || exit 2
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
L="$C/🗑️generated/rust-rebase-logs"
args=""
for c in job job-publication pack pack-record-encode-cursor dsl-record replication-mutation-retained replication-mutation-retained-field replication tool-run value-derive async-tests; do args="$args -p semio-framework-$c"; done
exec sh "$C/rust-rebase-tools/run.sh" "🧰️framework/Cargo.toml" "$L/${1:-fw2}.log" $args
