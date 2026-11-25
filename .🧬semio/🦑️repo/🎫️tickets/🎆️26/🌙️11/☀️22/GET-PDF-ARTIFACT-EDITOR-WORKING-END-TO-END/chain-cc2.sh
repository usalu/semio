#!/bin/sh
cd "$(dirname "$0")"
export NATIVE_CARGO_DIR=/Users/ueli/Documents/semio/🧰️framework
sh run-s4.sh cc-causal test -p semio-framework-replication-causal --lib -- --test-threads=1
sh run-s4.sh cc-paged test -p semio-framework-value-paged --lib -- --test-threads=1
