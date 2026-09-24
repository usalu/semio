#!/bin/zsh
# 🏗️ G10: os-hub build into my target dir, under the hub fleet mutex.
cd /Users/ueli/Documents/semio || exit 1
export CARGO_INCREMENTAL=0 CARGO_TARGET_DIR=/Users/ueli/Documents/semio/.tmp-ticket/wp-g10/target
date
cargo build -p semio-hub --bin os-hub
echo "BUILD_EXIT=$?"; date
