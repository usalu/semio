#!/bin/zsh
# 🧾️ LB2 p5 window-3 landing (ONE native-lane hold): apply `lb2-p5-docx-xlsx-opc-reds.py --write` → check docx+xlsx
# `--lib --tests` (auto-revert on red) → regenerate the demo assets through the artifacts' own writers → both crates' lib
# tests. Capture: wp-lb2/generated/<name>.txt. wasm32 proof of the stdio plugin = the window-3 wasm lane / chain fast-check.
cd /Users/ueli/Documents/semio || exit 2
capture=".tmp-ticket/wp-lb2/generated/$1.txt"
export CARGO_INCREMENTAL=0 CARGO_BUILD_BUILD_DIR="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/cargo/build-fleet-b" CARGO_TARGET_DIR="/Users/ueli/Documents/semio/.tmp-ticket/wp-lb2/target"
{ echo "QUEUED $(date '+%H:%M:%S')"; zsh .tmp-ticket/📜️fleet-mutex.sh native lb2 -- zsh -c '
echo "START $(date "+%H:%M:%S") apply"
python3 .tmp-ticket/wp-lb2/lb2-p5-docx-xlsx-opc-reds.py --write || exit 3
nice -n 15 cargo check --message-format short --keep-going -p semio-s-artifact-stdio-docx -p semio-s-artifact-stdio-xlsx --lib --tests; rc=$?
echo "CHECK rc=$rc $(date "+%H:%M:%S")"
if [ $rc -ne 0 ]; then python3 .tmp-ticket/wp-lb2/lb2-p5-docx-xlsx-opc-reds.py --revert; echo "REVERTED"; exit $rc; fi
nice -n 15 cargo test -p semio-s-artifact-stdio-xlsx --lib -- --ignored zzz_write_demo_fixtures; echo "GENERATE xlsx rc=$? $(date "+%H:%M:%S")"
nice -n 15 cargo test -p semio-s-artifact-stdio-docx --lib -- --ignored zzz_write_native_docx_fixture; echo "GENERATE docx rc=$? $(date "+%H:%M:%S")"
nice -n 15 cargo test --no-fail-fast -p semio-s-artifact-stdio-docx -p semio-s-artifact-stdio-xlsx --lib; echo "TEST rc=$? $(date "+%H:%M:%S")"
'; } > "$capture" 2>&1
