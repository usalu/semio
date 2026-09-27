#!/bin/zsh
# ✅️ WG9 settlement landing checks: kernel wasm32 (browser `sync` + wasip2) through the wasm mutex BESIDE the kernel native filtered
# law run (compiles lib + tests) through the `native` lane, both on build-fleet-b. Lines prefixed [wg9-settle].
# usage: python3 ../wp-w2/w2-detach.py <log> zsh wg9-settlement-checks.sh
set -u
R=/Users/ueli/Documents/semio
W=$R/.tmp-ticket/wp-wg9
cd $R || exit 1
export CARGO_INCREMENTAL=0 CARGO_BUILD_BUILD_DIR="$R/.🧬semio/🦑️repo/⚡️cache/cargo/build-fleet-b"
M=($R/.tmp-ticket/*fleet-mutex.sh)
echo "[wg9-settle] wasm queued $(date '+%F %T')"
zsh $M[1] wasm wg9 -- zsh -c '
echo "[wg9-settle] wasm HELD $(date "+%F %T")"
nice -n 10 cargo check -p semio-framework-os-kernel --lib --features sync --target wasm32-unknown-unknown --message-format short 2>&1 | /usr/bin/grep -E "^error|: error|Finished|could not|🔄️sync/🦀️.rs"
echo "[wg9-settle] kernel-browser rc=${pipestatus[1]} $(date "+%F %T")"
nice -n 10 cargo check -p semio-framework-os-kernel --lib --target wasm32-wasip2 --message-format short 2>&1 | /usr/bin/grep -E "^error|: error|Finished|could not|🔄️sync/🦀️.rs"
echo "[wg9-settle] kernel-wasip2 rc=${pipestatus[1]} $(date "+%F %T")"
' &
wasm_pid=$!
echo "[wg9-settle] native queued $(date '+%F %T')"
zsh $M[1] native wg9 -- env CARGO_TARGET_DIR=$W/target nice -n 15 cargo test -p semio-framework-os-kernel --lib --features sync,ureq --no-fail-fast -- document_echo_suppression_tests document_link_shortage_tests backbone_parity rollback_envelope 2>&1 | /usr/bin/grep -E "^error|: error|^test .* (ok|FAILED)$|^test result|panicked|Finished|could not"
echo "[wg9-settle] kernel-laws rc=${pipestatus[1]} $(date '+%F %T')"
wait $wasm_pid
echo "[wg9-settle] END $(date '+%F %T')"
