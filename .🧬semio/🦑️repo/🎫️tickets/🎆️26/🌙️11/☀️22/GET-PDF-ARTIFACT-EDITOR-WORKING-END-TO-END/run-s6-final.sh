#!/bin/sh
# usage: run-s6-final.sh  (slice 6: PDF lib, export, page editor, testkit and snapshot crates' unit tests, then the layout IO check; one gated cargo at a time; logs in 🗑️generated)
cd "$(dirname "$0")"
P=semio-s-artifact-stdio-pdf
sh ./run-s4.sh s6-final test --no-fail-fast -p $P -p $P-io-export -p $P-page-editor-tests -p $P-testkit -p $P-snapshot-2 --lib -- --test-threads=3
NATIVE_CARGO_DIR="/Users/ueli/Documents/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout" sh ./run-s4.sh s6-layout check -p semio-s-artifact-layout-layout-io --lib --message-format=short
