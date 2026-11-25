#!/bin/sh
# usage: run-s7.sh <log name> <test filters..>  (slice 7: one gated, watched cargo test run over the PDF lib, testkit and snapshot-value crates' unit tests; saved real-world files in 🗑️generated/real-world; log in 🗑️generated)
cd "$(dirname "$0")"
name=$1; shift
P=semio-s-artifact-stdio-pdf
sh ./run-s4.sh "$name" test --no-fail-fast -p $P -p $P-testkit -p $P-pdf-snapshot-value-tests --lib -- "$@" --test-threads=2 --nocapture
