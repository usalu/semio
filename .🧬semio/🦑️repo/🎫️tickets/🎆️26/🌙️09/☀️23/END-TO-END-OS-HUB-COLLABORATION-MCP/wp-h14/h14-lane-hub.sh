#!/bin/zsh
# ⚖️ H14: hub compile proof + laws through the native lane in build-fleet-b (preamble 14 rule 3): `cargo check -p semio-hub
# --lib --bins --tests`, then the lib laws (trusted catalog incl. residency, observability, lag-rebootstrap pair content) and
# the bin observability/readiness laws; capture → <capture> [extra lib filters…].
OUT=$1
shift
export H14_FILTERS="$*"
cd /Users/ueli/Documents/semio
export CARGO_INCREMENTAL=0 CARGO_BUILD_BUILD_DIR="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/cargo/build-fleet-b" CARGO_TARGET_DIR=/Users/ueli/Documents/semio/.tmp-ticket/wp-h14/target
echo "=== queued $(date +%T)" > "$OUT"
zsh /Users/ueli/Documents/semio/.tmp-ticket/📜️fleet-mutex.sh native h14 -- zsh -c 'echo "=== start $(date +%T)"; nice -n 15 cargo check -p semio-hub --lib --bins --tests; echo "=== CHECK EXIT $? $(date +%T)"; nice -n 15 cargo test --no-fail-fast -p semio-hub --lib -- trusted_catalog observability lag_rebootstrap ${=H14_FILTERS} --test-threads 4; echo "=== LIB EXIT $? $(date +%T)"; nice -n 15 cargo test --no-fail-fast -p semio-hub --bin os-hub -- every_served_readiness_body_is_the_declared_readiness_schema observability --test-threads 4; echo "=== BIN EXIT $? $(date +%T)"' >> "$OUT" 2>&1
echo "=== EXIT $? $(date +%T)" >> "$OUT"
