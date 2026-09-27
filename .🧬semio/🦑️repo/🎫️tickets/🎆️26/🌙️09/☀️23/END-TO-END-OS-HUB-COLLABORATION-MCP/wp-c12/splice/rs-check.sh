#!/bin/zsh
# 🧪️ C12: runs the standalone ui-scene text_splice twin laws (window-3 proof) through the native lane.
# usage: zsh splice/rs-check.sh > generated/splice-law-rs-<n>.txt 2>&1
cd /Users/ueli/Documents/semio/.tmp-ticket/wp-c12/splice/rs-check || exit 1
export CARGO_INCREMENTAL=0 CARGO_BUILD_BUILD_DIR=/Users/ueli/Documents/semio/.tmp-ticket/wp-c12/target/build-standalone CARGO_TARGET_DIR=/Users/ueli/Documents/semio/.tmp-ticket/wp-c12/target
export TEXT_SPLICE_FIXTURE=/Users/ueli/Documents/semio/.tmp-ticket/wp-c12/splice/fixture.json
echo "START $(date '+%F %T')"
zsh "/Users/ueli/Documents/semio/.tmp-ticket/📜️fleet-mutex.sh" native c12 -- nice -n 15 cargo test --manifest-path Cargo.toml --no-fail-fast -- --nocapture
echo "EXIT rc=$? $(date '+%F %T')"
