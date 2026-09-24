#!/bin/zsh
# 🧩️ S15: hub lib laws of the trusted catalog (incl. plugin_module and the plugin module index) on a private target dir.
cd /Users/ueli/Documents/semio || exit 1
export CARGO_INCREMENTAL=0 CARGO_TARGET_DIR=/Users/ueli/Documents/semio/.tmp-ticket/wp-s15/generated/target
echo "[s15] lib trusted_catalog $(date '+%T')"
cargo test -p semio-hub --lib --no-fail-fast -- trusted_catalog 2>&1 | tail -40
echo "[s15] done $(date '+%T')"
