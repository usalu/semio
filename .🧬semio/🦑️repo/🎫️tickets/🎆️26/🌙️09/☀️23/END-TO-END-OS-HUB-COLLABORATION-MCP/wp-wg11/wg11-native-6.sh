#!/bin/zsh
# 🧪️ WG11 s14b native hold 6 (fleet-b, `native` lane): the hub bin-unit echo-suppression law with WG11's pins (item 1c) on today's tree.
# usage: setopt no_bg_nice; nohup zsh wg11-native-6.sh > <capture> 2>&1 & disown        Lines prefixed [wg11-n6].
set -u
R=/Users/ueli/Documents/semio
W=$R/.tmp-ticket/wp-wg11
cd $R || exit 1
export CARGO_INCREMENTAL=0 CARGO_BUILD_BUILD_DIR="$R/.🧬semio/🦑️repo/⚡️cache/cargo/build-fleet-b"
M=($R/.tmp-ticket/*fleet-mutex.sh)
echo "[wg11-n6] queued $(date '+%F %T') pid=$$"
zsh $M[1] native wg11 -- zsh -c "
echo \"[wg11-n6] HELD \$(date '+%F %T')\"
CARGO_TARGET_DIR=$W/target nice -n 15 cargo test -p semio-hub --bin os-hub --no-fail-fast -- socket_grant_document_route_is_exact_replay_safe_actor_bound_and_revoke_live 2>&1 | /usr/bin/grep -E '^error|: error|^test .* (ok|FAILED)\$|^test result|panicked|Finished|could not|warning: .*generated'
echo \"[wg11-n6] hub-law rc=\${pipestatus[1]} \$(date '+%F %T')\"
"
echo "[wg11-n6] END $(date '+%F %T')"
