#!/bin/zsh
# 🔐️ WG9 follow-up landing (after the item 1+2 chain ends): WG7's renderer-only `s12-hub-sign-in-off-interaction.py` — dry run on the
# tree of that moment (aborts cleanly when an anchor moved), apply, renderer native `--lib --tests` check, the hub-workspace + relay
# laws, then the renderer wasm32 `--lib` check through the wasm mutex. Every step logs `[wg9-signin] <step> rc=<n>`; stops at the
# first red step. usage: python3 ../wp-w2/w2-detach.py <log> zsh wg9-signin-chain.sh <pid to wait for>
set -u
R=/Users/ueli/Documents/semio
T=$R/.tmp-ticket
W=$T/wp-wg9
cd $R || exit 1
export CARGO_INCREMENTAL=0 CARGO_BUILD_BUILD_DIR="$R/.🧬semio/🦑️repo/⚡️cache/cargo/build-fleet-b"
while kill -0 "$1" 2>/dev/null; do sleep 15; done
step() {
  local name="$1"; shift
  until [ "$(ps -axo command | /usr/bin/grep -c '^[^ ]*rustc ')" -le 14 ]; do sleep 30; done
  echo "[wg9-signin] $name start $(date '+%F %T')"
  nice -n 10 "$@" 2>&1 | /usr/bin/grep -E "^error|: error|^test result|FAILED|panicked|Finished|could not|DRY RUN|APPLIED|anchor occurs|🐚️Shell/.*(error|warning)|hub-projection-workspace/.*(error|warning)"
  local rc=$pipestatus[1]
  echo "[wg9-signin] $name rc=$rc $(date '+%F %T')"
  [ $rc -eq 0 ] || exit $rc
}
step dry-run python3 $T/wp-wg7/s12-hub-sign-in-off-interaction.py
step apply python3 $T/wp-wg7/s12-hub-sign-in-off-interaction.py --apply
step renderer-check cargo check -p semio-framework-os-renderer-wgpu --lib --tests --message-format short
step renderer-laws env CARGO_TARGET_DIR=$W/target cargo test -p semio-framework-os-renderer-wgpu --lib --no-fail-fast -- hub_projection_workspace_tests document_relay_tests
echo "[wg9-signin] wasm queued $(date '+%F %T')"
M=($T/*fleet-mutex.sh)
zsh $M[1] wasm wg9 -- zsh -c '
echo "[wg9-signin] wasm HELD $(date "+%F %T")"
nice -n 10 cargo check -p semio-framework-os-renderer-wgpu --lib --target wasm32-unknown-unknown --message-format short 2>&1 | /usr/bin/grep -E "^error|: error|Finished|could not|🐚️Shell/.*(error|warning)"
echo "[wg9-signin] renderer-wasm32 rc=${pipestatus[1]} $(date "+%F %T")"
'
echo "[wg9-signin] END $(date '+%F %T')"
