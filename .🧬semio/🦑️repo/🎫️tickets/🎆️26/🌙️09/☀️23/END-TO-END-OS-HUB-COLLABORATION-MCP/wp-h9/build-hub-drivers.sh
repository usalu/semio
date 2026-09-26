#!/bin/zsh
# os-hub with every storage driver linked, from this tree, into the H9 private target.
cd /Users/ueli/Documents/semio
export CARGO_INCREMENTAL=0 CARGO_TARGET_DIR=/Users/ueli/Documents/semio/.tmp-ticket/wp-h9/target RUST_MIN_STACK=268435456
echo "=== start $(date +%T)"
nice -n 15 cargo build -p semio-hub --bin os-hub --no-default-features --features sqlite,postgres,neo4j,native-artifact-execution 2>&1 | /usr/bin/grep -E "^error|Finished|warning: unused" | tail -20
echo "EXIT ${pipestatus[1]}"
ls -la $CARGO_TARGET_DIR/debug/os-hub
echo "=== done $(date +%T)"
