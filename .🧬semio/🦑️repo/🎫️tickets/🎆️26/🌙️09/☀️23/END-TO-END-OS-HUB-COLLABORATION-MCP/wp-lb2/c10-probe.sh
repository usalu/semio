#!/bin/zsh
# 🧪️ LB2 14c probes (temporary `lb2_probe_*` tests, removed after): xml source apply + demo parse, json definition actions,
# plugin() schema registration. ONE native hold. Capture: wp-lb2/generated/<name>.txt
cd /Users/ueli/Documents/semio || exit 2
capture=".tmp-ticket/wp-lb2/generated/$1.txt"
export CARGO_INCREMENTAL=0 CARGO_BUILD_BUILD_DIR="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/cargo/build-fleet-b" CARGO_TARGET_DIR="/Users/ueli/Documents/semio/.tmp-ticket/wp-lb2/target"
{ echo "QUEUED $(date '+%H:%M:%S')"; zsh .tmp-ticket/📜️fleet-mutex.sh native lb2 -- zsh -c '
echo "START $(date "+%H:%M:%S")"
nice -n 15 cargo test --no-fail-fast -p semio-s-artifact-stdio-json -p semio-s-artifact-stdio-xml --features semio-s-artifact-stdio-json/component-app-assembly,semio-s-artifact-stdio-xml/component-app-assembly --lib -- --nocapture lb2_probe; echo "LIB rc=$? $(date "+%H:%M:%S")"
nice -n 15 cargo test --no-fail-fast -p semio-s-plugin-stdio --test shipped_fleet -- --nocapture lb2_probe; echo "FLEET rc=$? $(date "+%H:%M:%S")"
'; } > "$capture" 2>&1
