#!/bin/bash
# g8: run one cargo command in the slice's private target dir; $1 = capture name, rest = cargo args.
cd /Users/ueli/Documents/semio
name="$1"; shift
CARGO_INCREMENTAL=0 CARGO_TARGET_DIR=/Users/ueli/Documents/semio/.tmp-ticket/wp-g8/target cargo "$@" > "/Users/ueli/Documents/semio/.tmp-ticket/wp-g8/generated/$name.txt" 2>&1
echo "G8-CARGO-EXIT $?" >> "/Users/ueli/Documents/semio/.tmp-ticket/wp-g8/generated/$name.txt"
