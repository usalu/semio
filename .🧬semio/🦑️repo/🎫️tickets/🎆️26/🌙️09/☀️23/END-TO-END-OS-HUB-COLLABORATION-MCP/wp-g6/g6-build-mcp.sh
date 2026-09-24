#!/bin/bash
# g6: debug build of semio-os-mcp into the slice's private target dir (shared build-dir); $1 = capture name.
cd /Users/ueli/Documents/semio
CARGO_INCREMENTAL=0 CARGO_TARGET_DIR=/Users/ueli/Documents/semio/.tmp-ticket/wp-g6/target cargo build -p semio-framework-os-mcp --bin semio-os-mcp > "/Users/ueli/Documents/semio/.tmp-ticket/wp-g6/generated/$1.txt" 2>&1
echo "G6-BUILD-EXIT $?" >> "/Users/ueli/Documents/semio/.tmp-ticket/wp-g6/generated/$1.txt"
