#!/bin/zsh
cd /Users/ueli/Documents/semio
export CARGO_INCREMENTAL=0 CARGO_TARGET_DIR=/Users/ueli/Documents/semio/.tmp-ticket/wp-p5/target
cargo nextest run -p semio-framework-os-kernel --lib --no-fail-fast > .tmp-ticket/wp-p5/generated/kernel-lib-nextest.txt 2>&1
echo "EXIT $?" >> .tmp-ticket/wp-p5/generated/kernel-lib-nextest.txt
cargo nextest run -p semio-framework-os-kernel-db --lib --no-fail-fast > .tmp-ticket/wp-p5/generated/db-lib-nextest.txt 2>&1
echo "EXIT $?" >> .tmp-ticket/wp-p5/generated/db-lib-nextest.txt
