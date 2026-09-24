#!/bin/bash
# g6: headless native reference — the wfc genesis solve (`solve_with_job`, debug) timed by libtest; $1 = capture name.
cd /Users/ueli/Documents/semio
CARGO_INCREMENTAL=0 CARGO_TARGET_DIR=/Users/ueli/Documents/semio/.tmp-ticket/wp-g6/target cargo test -p semio-s-artifact-wfc-bitmap --lib the_artifact_bound_genesis_document_resolves_and_solves -- -Z unstable-options --report-time > "/Users/ueli/Documents/semio/.tmp-ticket/wp-g6/generated/$1.txt" 2>&1
echo "G6-NATIVE-EXIT $?" >> "/Users/ueli/Documents/semio/.tmp-ticket/wp-g6/generated/$1.txt"
