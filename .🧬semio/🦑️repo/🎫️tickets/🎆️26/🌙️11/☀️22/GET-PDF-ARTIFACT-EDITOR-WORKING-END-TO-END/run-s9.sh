#!/bin/sh
# usage: run-s9.sh <log name> <test filters..>  (slice 9: one gated, watched cargo test run over the replace-page unit crate (mut-family-11), which holds the retained-graph preparation tests; saved real-world files in 🗑️generated/real-world; log in 🗑️generated)
cd "$(dirname "$0")"
name=$1; shift
sh ./run-s4.sh "$name" test -p semio-s-artifact-stdio-pdf-mut-family-11 --lib -- "$@" --test-threads=1 --nocapture
