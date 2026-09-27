#!/bin/zsh
# 🔎️ H12: `cargo check -p semio-hub --bins --tests` through the native mutex in build-fleet-b (rule 30), then the hub bin laws;
# capture → <capture> [extra bin-law filters…].
OUT=$1
shift
export H12_FILTERS="$*"
cd /Users/ueli/Documents/semio
export CARGO_INCREMENTAL=0 CARGO_BUILD_BUILD_DIR="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/cargo/build-fleet-b" CARGO_TARGET_DIR=/Users/ueli/Documents/semio/.tmp-ticket/wp-h12/target
echo "=== start $(date +%T)" > "$OUT"
zsh /Users/ueli/Documents/semio/.tmp-ticket/📜️fleet-mutex.sh native h12 -- zsh -c 'nice -n 15 cargo check -p semio-hub --bins --tests; echo "=== CHECK EXIT $? $(date +%T)"; nice -n 15 cargo test --no-fail-fast -p semio-hub --bin os-hub -- every_served_readiness_body_is_the_declared_readiness_schema a_booting_hub_answers_readiness observability hostile ${=H12_FILTERS} --test-threads 4; echo "=== BIN EXIT $? $(date +%T)"' >> "$OUT" 2>&1
echo "=== EXIT $? $(date +%T)" >> "$OUT"
