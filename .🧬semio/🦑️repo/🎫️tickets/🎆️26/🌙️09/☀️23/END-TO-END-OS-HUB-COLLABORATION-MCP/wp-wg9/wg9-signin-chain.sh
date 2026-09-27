#!/bin/zsh
# 🔐️ WG9 landing (window 2): WG7's renderer-only `s12-hub-sign-in-off-interaction.py` — dry run on the tree of that moment (aborts
# cleanly when an anchor moved), apply, then the renderer wasm32 `--lib` check through the wasm mutex BESIDE the native `--lib --tests`
# check and the hub-workspace + relay laws through the `native` lane (preamble 13 rules 29/32). Every step logs
# `[wg9-signin] <step> rc=<n>`; the native steps stop at the first red one.
# usage: python3 ../wp-w2/w2-detach.py <log> zsh wg9-signin-chain.sh
set -u
R=/Users/ueli/Documents/semio
T=$R/.tmp-ticket
W=$T/wp-wg9
cd $R || exit 1
export CARGO_INCREMENTAL=0 CARGO_BUILD_BUILD_DIR="$R/.🧬semio/🦑️repo/⚡️cache/cargo/build-fleet-b"
M=($T/*fleet-mutex.sh)
step() {
  local name="$1"; shift
  echo "[wg9-signin] $name start $(date '+%F %T')"
  "$@" 2>&1 | /usr/bin/grep -E "^error|: error|^test result|FAILED|panicked|Finished|could not|DRY RUN|APPLIED|anchor occurs|🐚️Shell/.*(error|warning)|hub-projection-workspace/.*(error|warning)"
  local rc=$pipestatus[1]
  echo "[wg9-signin] $name rc=$rc $(date '+%F %T')"
  [ $rc -eq 0 ] || exit $rc
}
step dry-run python3 $T/wp-wg7/s12-hub-sign-in-off-interaction.py
step apply python3 $T/wp-wg7/s12-hub-sign-in-off-interaction.py --apply
echo "[wg9-signin] wasm queued $(date '+%F %T')"
zsh $M[1] wasm wg9 -- zsh -c '
echo "[wg9-signin] wasm HELD $(date "+%F %T")"
nice -n 10 cargo check -p semio-framework-os-renderer-wgpu --lib --target wasm32-unknown-unknown --message-format short 2>&1 | /usr/bin/grep -E "^error|: error|Finished|could not|🐚️Shell/.*(error|warning)"
echo "[wg9-signin] renderer-wasm32 rc=${pipestatus[1]} $(date "+%F %T")"
' &
wasm_pid=$!
step renderer-check zsh $M[1] native wg9 -- nice -n 15 cargo check -p semio-framework-os-renderer-wgpu --lib --tests --message-format short
step renderer-laws zsh $M[1] native wg9 -- env CARGO_TARGET_DIR=$W/target nice -n 15 cargo test -p semio-framework-os-renderer-wgpu --lib --no-fail-fast -- hub_projection_workspace_tests document_relay_tests
wait $wasm_pid
echo "[wg9-signin] END $(date '+%F %T')"
