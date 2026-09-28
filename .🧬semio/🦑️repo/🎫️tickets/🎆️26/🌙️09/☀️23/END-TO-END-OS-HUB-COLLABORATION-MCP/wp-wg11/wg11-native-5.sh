#!/bin/zsh
# 🧪️ WG11 s14b native hold 5 (fleet-b, `native` lane): ONE compile of the renderer lib test target of the CURRENT tree with
# `-Zprint-type-sizes` (item 3: the async state machines whose debug poll temporaries overflow libtest's 2 MiB thread) — keeps every
# type ≥ 16 KiB (`generated/s14b/n5-type-sizes.txt`) and a durable copy of the produced test binary
# (`.🧬semio/🌐hub/s14-wg11-bin/renderer-tests-<stamp>`) for the live laws. Lines prefixed [wg11-n5].
set -u
R=/Users/ueli/Documents/semio
W=$R/.tmp-ticket/wp-wg11
cd $R || exit 1
export CARGO_INCREMENTAL=0 CARGO_BUILD_BUILD_DIR="$R/.🧬semio/🦑️repo/⚡️cache/cargo/build-fleet-b"
M=($R/.tmp-ticket/*fleet-mutex.sh)
echo "[wg11-n5] queued $(date '+%F %T') pid=$$"
zsh $M[1] native wg11 -- zsh -c "
echo \"[wg11-n5] HELD \$(date '+%F %T')\"
CARGO_TARGET_DIR=$W/target nice -n 15 cargo rustc -p semio-framework-os-renderer-wgpu --lib --profile test --message-format short -- -Zprint-type-sizes 2>$W/generated/s14b/n5-build.txt | awk '/^print-type-size type: / { if (match(\$0, /\`: [0-9]+ bytes/)) { n = substr(\$0, RSTART + 3, RLENGTH - 9); if (n + 0 >= 16384) print } }' > $W/generated/s14b/n5-type-sizes.txt
echo \"[wg11-n5] renderer-test-build rc=\${pipestatus[1]} \$(date '+%F %T')\"
/usr/bin/grep -E '^error|: error|could not|Finished' $W/generated/s14b/n5-build.txt | head -20
"
BIN=$(ls -t "$CARGO_BUILD_BUILD_DIR"/debug/build/semio-framework-os-renderer-wgpu/*/out/semio_framework_os_renderer_wgpu-* 2>/dev/null | /usr/bin/grep -v '\.d$' | head -1)
if [ -n "$BIN" ] && [ -x "$BIN" ]; then mkdir -p "$R/.🧬semio/🌐hub/s14-wg11-bin"; STAMP=$(date '+%H%M'); cp "$BIN" "$R/.🧬semio/🌐hub/s14-wg11-bin/renderer-tests-$STAMP"; echo "[wg11-n5] durable renderer-tests-$STAMP from $BIN"; fi
echo "[wg11-n5] END $(date '+%F %T')"
