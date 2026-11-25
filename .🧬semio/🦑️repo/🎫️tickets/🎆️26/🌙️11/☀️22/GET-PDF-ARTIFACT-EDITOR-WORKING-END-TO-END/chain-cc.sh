#!/bin/sh
cd "$(dirname "$0")"
export RUST_BACKTRACE=1 NATIVE_CARGO_DIR=/Users/ueli/Documents/semio/🧰️framework
sh run-s4.sh cc-pack test -p semio-framework-pack-record-encode-cursor --lib -- --test-threads=1
sh run-s4.sh cc-clone test -p semio-framework-replication-causal --lib -- --test-threads=1 original_causal_envelope_clone
