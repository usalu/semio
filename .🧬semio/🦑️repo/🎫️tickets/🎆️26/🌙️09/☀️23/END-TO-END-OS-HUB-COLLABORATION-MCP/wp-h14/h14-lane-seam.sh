#!/bin/zsh
# ⚖️ H14 14b item 7: inside ONE native-lane hold — apply `h14-remove-authz-seam.py --write`, check kernel-db (lib+tests) and
# semio-hub (lib+bins+tests); a red check reverts the codemod before the hold ends (the tree never keeps a red seam removal);
# green → the db artifact/engine/security laws; capture → <capture>.
OUT=$1
cd /Users/ueli/Documents/semio
export CARGO_INCREMENTAL=0 CARGO_BUILD_BUILD_DIR="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/cargo/build-fleet-b" CARGO_TARGET_DIR=/Users/ueli/Documents/semio/.tmp-ticket/wp-h14/target
echo "=== queued $(date +%T)" > "$OUT"
zsh /Users/ueli/Documents/semio/.tmp-ticket/📜️fleet-mutex.sh native h14 -- zsh -c '
echo "=== start $(date +%T)"
python3 /Users/ueli/Documents/semio/.tmp-ticket/wp-h14/h14-remove-authz-seam.py --write || { echo "=== CODEMOD FAILED $(date +%T)"; exit 1; }
nice -n 15 cargo check -p semio-framework-os-kernel-db --lib --tests --message-format short 2>&1 | /usr/bin/grep -E "^error|: error|Finished"; db=${pipestatus[1]}; echo "=== DB CHECK EXIT $db $(date +%T)"
if [ "$db" = 0 ]; then nice -n 15 cargo check -p semio-hub --lib --bins --tests --message-format short 2>&1 | /usr/bin/grep -E "^error|: error|Finished"; hub=${pipestatus[1]}; else hub=skipped; fi; echo "=== HUB CHECK EXIT $hub $(date +%T)"
if [ "$db" != 0 ] || [ "$hub" != 0 ]; then python3 /Users/ueli/Documents/semio/.tmp-ticket/wp-h14/h14-remove-authz-seam.py --revert; echo "=== REVERTED $(date +%T)"; exit 1; fi
nice -n 15 cargo test -p semio-framework-os-kernel-db --lib --no-fail-fast -- db_artifact:: db_engine::tests db_security:: --test-threads 4 2>&1 | /usr/bin/grep -E "FAILED|panicked|^test result|^error|: error"; echo "=== DB LAWS EXIT ${pipestatus[1]} $(date +%T)"
' >> "$OUT" 2>&1
echo "=== EXIT $? $(date +%T)" >> "$OUT"
