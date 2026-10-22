#!/bin/sh
# usage: test.sh LOGPREFIX cargo-test-args...   (RUST_MIN_STACK 32 MiB, retried by run.sh on infra refusals)
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
L="$C/🗑️generated/rust-port-logs"
P=$1; shift
export RUST_MIN_STACK=33554432
sh "$C/rust-health-tools/run.sh" "$L/$P-update.log" update --workspace --offline
sh "$C/rust-health-tools/run.sh" "$L/$P.log" test "$@"
