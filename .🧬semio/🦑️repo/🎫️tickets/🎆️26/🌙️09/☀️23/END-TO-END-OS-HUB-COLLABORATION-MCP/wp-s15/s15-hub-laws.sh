#!/bin/zsh
# 🧩️ S15: hub laws for the trusted plugin module bundle (lib trusted_catalog incl. plugin_module, bin native stdio + check-in + creation routes).
cd /Users/ueli/Documents/semio || exit 1
export CARGO_INCREMENTAL=0 CARGO_TARGET_DIR=/Users/ueli/Documents/semio/.tmp-ticket/wp-s15/target
echo "[s15] lib trusted_catalog $(date '+%T')"
cargo test -p semio-hub --lib --no-fail-fast -- trusted_catalog 2>&1 | tail -40
echo "[s15] bin native/check-in/creation serial $(date '+%T')"
cargo test -p semio-hub --bin os-hub --features integration-fixtures --no-fail-fast -- --test-threads=1 native_openable_stdio_provider_is_the_only_atomic_readiness_transition space_artifact_creation_routes_are_author_owned_idempotent_and_genesis_backed check_in 2>&1 | tail -60
echo "[s15] done $(date '+%T')"
