#!/bin/sh
# usage: oracle-test.sh : the package-boundary-classification law (builds the Rust syn oracle in-process through the issued launcher) inside the repo exec envelope
cd /Users/ueli/Documents/semio || exit 1
export SEMIO_TEST_ARTIFACT_DIR="/Users/ueli/.rust-g-art/files"; mkdir -p "$SEMIO_TEST_ARTIFACT_DIR"
unset SEMIO_SCRIPT_STALL_MS; export SEMIO_BUILD_GRAPH_POLICY=current
F="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🧪️tests/📦️package-boundary-classification/🟦️.ts"
nice -n 10 python3 "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/rust-g-tools/measure.py" oracle-test "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/🗑️generated/rust-g/runs.jsonl" -- bun repo --tool=semio exec --projects=@semio-tech/repo-lib -- bun test "$F"
echo "rc=$?"
