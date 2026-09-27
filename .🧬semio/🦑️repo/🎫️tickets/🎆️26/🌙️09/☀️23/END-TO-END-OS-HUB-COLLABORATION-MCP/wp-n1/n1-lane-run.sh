#!/bin/zsh
# N1: one native-lane hold — the two repaired norm test reds, then the ticket-local fixture emitter build.
export CARGO_INCREMENTAL=0 CARGO_BUILD_BUILD_DIR="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/cargo/build-fleet-b" CARGO_TARGET_DIR=/Users/ueli/Documents/semio/.tmp-ticket/wp-n1/target
cd /Users/ueli/Documents/semio || exit 1
echo "[n1] tests start $(date +%T)"
nice -n 15 cargo test -p semio-s-plugin-norm --lib --test compliance_gate --no-fail-fast -- surface_laws_hold set_active_example compliance_gate_all_families
echo "[n1] tests exit=$? $(date +%T)"
cd .tmp-ticket/wp-n1/emitter && nice -n 15 cargo build --offline
echo "[n1] emitter exit=$? $(date +%T)"
