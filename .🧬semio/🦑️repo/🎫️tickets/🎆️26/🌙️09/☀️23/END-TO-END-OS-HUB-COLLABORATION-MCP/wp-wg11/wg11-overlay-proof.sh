#!/bin/zsh
# 🧪️ WG11 s14 overlay proof of the window-3 patches (preamble 14 rule 2/3, session-13 rule 37): ONE `overlay` lane hold, private
# build-dir + target inside the overlay (never the shared build-dir). At hold time: re-sync the overlay from the tree, apply every
# patch listed in wg11-overlay-patches.txt (dry run first, then --apply, ROOT rewritten to the overlay), then native checks + laws
# of every touched crate and the TS twins. Lines prefixed [wg11-ov].
# usage: setopt no_bg_nice; nohup zsh wg11-overlay-proof.sh > <capture> 2>&1 & disown
set -u
R=/Users/ueli/Documents/semio
W=$R/.tmp-ticket/wp-wg11
O="$R/.🧬semio/🌐hub/s14-wg11-overlay"
M=($R/.tmp-ticket/*fleet-mutex.sh)
echo "[wg11-ov] queued $(date '+%F %T') pid=$$"
zsh $M[1] overlay wg11 -- zsh -c "
set -u
echo \"[wg11-ov] HELD \$(date '+%F %T')\"
python3 $W/wg11-overlay-sync.py || exit 1
for patch in \$(/usr/bin/grep -v '^#' $W/wg11-overlay-patches.txt); do
  python3 $W/wg11-overlay-apply.py \"$O\" \"$W/\$patch\" || { echo \"[wg11-ov] patch \$patch FAILED\"; exit 1; }
done
cd \"$O\" || exit 1
export CARGO_INCREMENTAL=0 CARGO_BUILD_BUILD_DIR=\"$O/.cargo-build\" CARGO_TARGET_DIR=\"$O/target\"
for crate in semio-framework-ui-contract 'semio-framework-os-kernel --features sync,ureq' semio-framework-os-renderer-wgpu; do
  nice -n 15 cargo check -p \${=crate} --lib --tests --message-format short 2>&1 | /usr/bin/grep -E '^error|: error|warning: unused|Finished|could not'
  echo \"[wg11-ov] check \$crate rc=\${pipestatus[1]} \$(date '+%F %T')\"
done
nice -n 15 cargo test -p semio-framework-ui-contract --lib --no-fail-fast -- accessibility 2>&1 | /usr/bin/grep -E '^test .* (ok|FAILED)\$|^test result|panicked'
echo \"[wg11-ov] ui-contract laws rc=\${pipestatus[1]} \$(date '+%F %T')\"
nice -n 15 cargo test -p semio-framework-os-kernel --lib --features sync,ureq --no-fail-fast -- canonical_checkpoint_pair rebootstrap document_echo_suppression_tests backbone_parity 2>&1 | /usr/bin/grep -E '^test .* (ok|FAILED)\$|^test result|panicked'
echo \"[wg11-ov] kernel laws rc=\${pipestatus[1]} \$(date '+%F %T')\"
nice -n 15 cargo test -p semio-framework-os-renderer-wgpu --lib --no-fail-fast -- --test-threads=1 board_presence accessibility_projection hub_projection_workspace_tests::a_ 2>&1 | /usr/bin/grep -E '^test .* (ok|FAILED)\$|^test result|panicked'
echo \"[wg11-ov] renderer laws rc=\${pipestatus[1]} \$(date '+%F %T')\"
"
echo "[wg11-ov] END $(date '+%F %T')"
