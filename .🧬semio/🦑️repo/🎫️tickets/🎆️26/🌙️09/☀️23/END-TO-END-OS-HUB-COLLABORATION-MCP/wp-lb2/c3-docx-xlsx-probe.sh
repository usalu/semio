#!/bin/zsh
# 🧪️ LB2 session 14c: ONE native-lane hold — check docx+xlsx `--lib --tests`, then both crates' lib tests with the
# temporary `lb2_probe` tests armed (LB2_PROBE_DIR = payload/p5 candidates). Capture: wp-lb2/generated/<name>.txt
cd /Users/ueli/Documents/semio || exit 2
capture=".tmp-ticket/wp-lb2/generated/$1.txt"
export CARGO_INCREMENTAL=0 CARGO_BUILD_BUILD_DIR="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/cargo/build-fleet-b" CARGO_TARGET_DIR="/Users/ueli/Documents/semio/.tmp-ticket/wp-lb2/target"
export LB2_PROBE_DIR="/Users/ueli/Documents/semio/.tmp-ticket/wp-lb2/payload/p5"
{ echo "QUEUED $(date '+%H:%M:%S')"; zsh .tmp-ticket/📜️fleet-mutex.sh native lb2 -- zsh -c '
echo "START $(date "+%H:%M:%S") check"
nice -n 15 cargo check --message-format short --keep-going -p semio-s-artifact-stdio-docx -p semio-s-artifact-stdio-xlsx --lib --tests 2>&1 | /usr/bin/grep -E "^error|: error|-->|^warning: unused" ; rc=${pipestatus[1]}
echo "CHECK rc=$rc $(date "+%H:%M:%S")"
[ $rc -eq 0 ] || exit $rc
nice -n 15 cargo test --no-fail-fast -p semio-s-artifact-stdio-docx -p semio-s-artifact-stdio-xlsx --lib -- --nocapture --test-threads 1; echo "TEST rc=$? $(date "+%H:%M:%S")"
'; } > "$capture" 2>&1
