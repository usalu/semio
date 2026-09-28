#!/bin/zsh
# 🧪️ LB2 item 8 (rule 22, test-only): docx/xlsx lib tests ported to the XML-parts snapshot — ONE native-lane hold: check
# `--lib --tests` of both crates; when that compiles, regenerate the xlsx set-snapshot quintet through its own generator
# (`--ignored zzz_write_committed_quintet`), then run both crates' lib tests. Capture: wp-lb2/generated/<name>.txt
cd /Users/ueli/Documents/semio || exit 2
capture=".tmp-ticket/wp-lb2/generated/$1.txt"
export CARGO_INCREMENTAL=0 CARGO_BUILD_BUILD_DIR="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/cargo/build-fleet-b" CARGO_TARGET_DIR="/Users/ueli/Documents/semio/.tmp-ticket/wp-lb2/target"
{ echo "QUEUED $(date '+%H:%M:%S')"; zsh .tmp-ticket/📜️fleet-mutex.sh native lb2 -- zsh -c '
echo "START $(date "+%H:%M:%S") check"
nice -n 15 cargo check --message-format short --keep-going -p semio-s-artifact-stdio-docx -p semio-s-artifact-stdio-xlsx --lib --tests; rc=$?
echo "CHECK rc=$rc $(date "+%H:%M:%S")"
[ $rc -eq 0 ] || exit $rc
nice -n 15 cargo test -p semio-s-artifact-stdio-xlsx --lib --no-fail-fast -- --ignored zzz_write_committed_quintet; echo "GENERATE rc=$? $(date "+%H:%M:%S")"
nice -n 15 cargo test -p semio-s-artifact-stdio-docx -p semio-s-artifact-stdio-xlsx --lib --no-fail-fast; echo "TEST rc=$? $(date "+%H:%M:%S")"
'; } > "$capture" 2>&1
