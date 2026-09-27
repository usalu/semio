#!/bin/zsh
# 🧪️ WG11 s14 native hold 1 (fleet-b, `native` lane): (a) hub `os-hub --tests` check + the echo-suppression pinned law
# (item 1c); (b) the kernel lost-Ack settlement laws on the current tree (item 2). Lines prefixed [wg11-n1].
# usage: setopt no_bg_nice; nohup zsh wg11-native-1.sh > <capture> 2>&1 & disown
set -u
R=/Users/ueli/Documents/semio
W=$R/.tmp-ticket/wp-wg11
cd $R || exit 1
export CARGO_INCREMENTAL=0 CARGO_BUILD_BUILD_DIR="$R/.🧬semio/🦑️repo/⚡️cache/cargo/build-fleet-b"
M=($R/.tmp-ticket/*fleet-mutex.sh)
echo "[wg11-n1] queued $(date '+%F %T') pid=$$"
zsh $M[1] native wg11 -- zsh -c "
echo \"[wg11-n1] HELD \$(date '+%F %T')\"
nice -n 15 cargo check -p semio-hub --bin os-hub --tests --message-format short 2>&1 | /usr/bin/grep -E '^error|: error|warning: unused|Finished|could not|bin-unit'
echo \"[wg11-n1] hub-check rc=\${pipestatus[1]} \$(date '+%F %T')\"
CARGO_TARGET_DIR=$W/target nice -n 15 cargo test -p semio-hub --bin os-hub --no-fail-fast -- socket_grant_document_route_is_exact_replay_safe_actor_bound_and_revoke_live 2>&1 | /usr/bin/grep -E '^error|: error|^test .* (ok|FAILED)$|^test result|panicked|Finished|could not'
echo \"[wg11-n1] hub-law rc=\${pipestatus[1]} \$(date '+%F %T')\"
CARGO_TARGET_DIR=$W/target nice -n 15 cargo test -p semio-framework-os-kernel --lib --features sync,ureq --no-fail-fast -- document_echo_suppression_tests document_link_shortage_tests backbone_parity rollback_envelope 2>&1 | /usr/bin/grep -E '^error|: error|^test .* (ok|FAILED)$|^test result|panicked|Finished|could not'
echo \"[wg11-n1] kernel-laws rc=\${pipestatus[1]} \$(date '+%F %T')\"
"
echo "[wg11-n1] END $(date '+%F %T')"
