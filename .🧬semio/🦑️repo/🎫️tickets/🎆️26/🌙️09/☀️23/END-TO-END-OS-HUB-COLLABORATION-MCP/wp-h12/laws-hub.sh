#!/bin/zsh
# ⚖️ H12: the hub laws of items 1, 2, 4, 5 and 6 (trusted catalog, observability, creation genesis, readiness, hostile
# input) through the native mutex in build-fleet-b (rule 30); capture → <capture>.
OUT=$1
cd /Users/ueli/Documents/semio
export CARGO_INCREMENTAL=0 CARGO_BUILD_BUILD_DIR="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/cargo/build-fleet-b" CARGO_TARGET_DIR=/Users/ueli/Documents/semio/.tmp-ticket/wp-h12/target
echo "=== start $(date +%T)" > "$OUT"
zsh /Users/ueli/Documents/semio/.tmp-ticket/📜️fleet-mutex.sh native h12 -- zsh -c 'nice -n 15 cargo test --no-fail-fast -p semio-hub --lib -- trusted_catalog observability genesis_materialization --test-threads 4; echo "=== LIB EXIT $? $(date +%T)"; nice -n 15 cargo test --no-fail-fast -p semio-hub --bin os-hub -- every_served_readiness_body_is_the_declared_readiness_schema a_booting_hub_answers_readiness observability hostile --test-threads 4; echo "=== BIN EXIT $? $(date +%T)"' >> "$OUT" 2>&1
echo "=== EXIT $? $(date +%T)" >> "$OUT"
