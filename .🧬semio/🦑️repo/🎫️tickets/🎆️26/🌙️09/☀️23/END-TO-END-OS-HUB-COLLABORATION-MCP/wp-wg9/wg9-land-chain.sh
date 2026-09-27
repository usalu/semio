#!/bin/zsh
# 🛬️ WG9 landing chain for items 1 + 2 (link expiry, echo suppression kernel + hub), one cargo at a time on the fleet's second
# build-dir (preamble 13 rules 24–26): kernel native check → kernel filtered laws → hub check → hub catch-up laws → wasm32 kernel
# checks (wasm mutex). Every step logs `[wg9-chain] <step> rc=<n>`; the chain stops at the first red step.
# usage: python3 ../wp-w2/w2-detach.py <log> zsh wg9-land-chain.sh [step…]  (no steps = all: kernel-check kernel-laws hub-check hub-laws wasm)
set -u
R=/Users/ueli/Documents/semio
W=$R/.tmp-ticket/wp-wg9
cd $R || exit 1
export CARGO_INCREMENTAL=0 CARGO_BUILD_BUILD_DIR="$R/.🧬semio/🦑️repo/⚡️cache/cargo/build-fleet-b"
only=("$@")
step() {
  local name="$1"; shift
  (( ${#only} == 0 || ${only[(Ie)$name]} )) || return 0
  until [ "$(ps -axo command | /usr/bin/grep -c '^[^ ]*rustc ')" -le ${WG9_RUSTC_BOUND:-14} ]; do sleep 30; done
  echo "[wg9-chain] $name start $(date '+%F %T')"
  nice -n 10 "$@" 2>&1 | /usr/bin/grep -E "^error|: error|^test result|^test .*(echo|link_shortage|backbone_parity|socket_grant).* ok$|FAILED|panicked|Finished|could not|🔄️sync/🦀️.rs|🏗️bootstrap/🦀️.rs.*(error|warning)|bin-unit/🦀️.rs.*(error|warning)"
  local rc=$pipestatus[1]
  echo "[wg9-chain] $name rc=$rc $(date '+%F %T')"
  [ $rc -eq 0 ] || exit $rc
}
step kernel-check cargo check -p semio-framework-os-kernel --lib --tests --features sync,ureq --message-format short
step kernel-laws env CARGO_TARGET_DIR=$W/target cargo test -p semio-framework-os-kernel --lib --features sync,ureq --no-fail-fast -- document_echo_suppression_tests document_link_shortage_tests backbone_parity
step hub-check cargo check -p semio-hub --bin os-hub --tests --message-format short
step hub-laws env CARGO_TARGET_DIR=$W/target cargo test -p semio-hub --bin os-hub --no-fail-fast -- socket_grant_document_route_is_exact_replay_safe_actor_bound_and_revoke_live
if (( ${#only} == 0 || ${only[(Ie)wasm]} )); then
  echo "[wg9-chain] wasm queued $(date '+%F %T')"
  zsh $W/wg9-wasm-kernel.sh
fi
echo "[wg9-chain] END $(date '+%F %T')"
