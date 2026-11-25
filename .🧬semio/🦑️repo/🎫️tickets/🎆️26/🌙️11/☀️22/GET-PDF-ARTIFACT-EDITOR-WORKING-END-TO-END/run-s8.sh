#!/bin/sh
# usage: run-s8.sh <log name> <test filters..>  (slice 8: one gated, watched cargo test run over the snapshot-value crate's unit tests only; saved real-world files in 🗑️generated/real-world; log in 🗑️generated)
cd "$(dirname "$0")"
name=$1; shift
sh ./run-s4.sh "$name" test -p semio-s-artifact-stdio-pdf-pdf-snapshot-value-tests --lib -- "$@" --test-threads=1 --nocapture
