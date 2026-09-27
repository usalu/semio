#!/bin/zsh
# 🔐️ WG9: renderer native `--lib --tests` check + hub-workspace/relay/board-presence laws through the `native` lane (fleet-b).
# usage: python3 ../wp-w2/w2-detach.py <log> zsh wg9-renderer-native.sh
set -u
R=/Users/ueli/Documents/semio
W=$R/.tmp-ticket/wp-wg9
cd $R || exit 1
export CARGO_INCREMENTAL=0 CARGO_BUILD_BUILD_DIR="$R/.🧬semio/🦑️repo/⚡️cache/cargo/build-fleet-b"
M=($R/.tmp-ticket/*fleet-mutex.sh)
echo "[wg9-rn] queued $(date '+%F %T')"
zsh $M[1] native wg9 -- zsh -c "
nice -n 15 cargo check -p semio-framework-os-renderer-wgpu --lib --tests --message-format short 2>&1 | /usr/bin/grep -E '^error|: error|Finished|could not'
echo \"[wg9-rn] renderer-check rc=\${pipestatus[1]} \$(date '+%F %T')\"
[ \${pipestatus[1]} -eq 0 ] || exit 1
CARGO_TARGET_DIR=$W/target nice -n 15 cargo test -p semio-framework-os-renderer-wgpu --lib --no-fail-fast -- hub_projection_workspace_tests document_relay_tests board_presence 2>&1 | /usr/bin/grep -E '^error|: error|^test .* (ok|FAILED)$|^test result|panicked|could not'
echo \"[wg9-rn] renderer-laws rc=\${pipestatus[1]} \$(date '+%F %T')\"
"
echo "[wg9-rn] END $(date '+%F %T')"
