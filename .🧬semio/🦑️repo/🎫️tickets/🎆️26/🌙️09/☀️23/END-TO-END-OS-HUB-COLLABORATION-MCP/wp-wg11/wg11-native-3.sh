#!/bin/zsh
# 🧪️ WG11 s14 native hold 3 (fleet-b, `native` lane): builds the renderer lib test binary of the CURRENT tree (live laws: gate,
# cross-shell) and keeps a durable copy (`.🧬semio/🌐hub/s14-wg11-bin/renderer-tests-<stamp>`) for item 3 + live runs. [wg11-n3]
set -u
R=/Users/ueli/Documents/semio
W=$R/.tmp-ticket/wp-wg11
cd $R || exit 1
export CARGO_INCREMENTAL=0 CARGO_BUILD_BUILD_DIR="$R/.🧬semio/🦑️repo/⚡️cache/cargo/build-fleet-b"
M=($R/.tmp-ticket/*fleet-mutex.sh)
echo "[wg11-n3] queued $(date '+%F %T') pid=$$"
zsh $M[1] native wg11 -- zsh -c "
echo \"[wg11-n3] HELD \$(date '+%F %T')\"
CARGO_TARGET_DIR=$W/target nice -n 15 cargo test -p semio-framework-os-renderer-wgpu --lib --no-run --message-format short 2>&1 | tee $W/generated/n3-build.txt | /usr/bin/grep -E '^error|: error|Finished|could not|Executable'
echo \"[wg11-n3] renderer-tests rc=\${pipestatus[1]} \$(date '+%F %T')\"
"
BIN=$(/usr/bin/grep -o 'Executable unittests[^(]*([^)]*)' $W/generated/n3-build.txt | tail -1 | sed 's/.*(\(.*\))/\1/')
case "$BIN" in /*) ;; *) BIN="$R/$BIN" ;; esac
if [ -x "$BIN" ]; then mkdir -p "$R/.🧬semio/🌐hub/s14-wg11-bin"; STAMP=$(date '+%H%M'); cp "$BIN" "$R/.🧬semio/🌐hub/s14-wg11-bin/renderer-tests-$STAMP"; echo "[wg11-n3] durable renderer-tests-$STAMP"; fi
echo "[wg11-n3] END $(date '+%F %T')"
