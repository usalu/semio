#!/bin/zsh
# ⚖️ H14 hold 12 (native lane): h14-ack-law-permits.py (test-only, restore-on-red), semio-hub --tests check (warnings of the law's
# lines must be gone), the Ack law + the plan idle law.
OUT=$1
W=/Users/ueli/Documents/semio/.tmp-ticket/wp-h14
BK="/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-h14-backup/hold12"
cd /Users/ueli/Documents/semio
export CARGO_INCREMENTAL=0 CARGO_BUILD_BUILD_DIR="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/cargo/build-fleet-b" CARGO_TARGET_DIR="/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-h14-target" W BK NX_DAEMON=false
echo "=== queued $(date +%T)" > "$OUT"
zsh /Users/ueli/Documents/semio/.tmp-ticket/📜️fleet-mutex.sh native h14 -- zsh -c '
echo "=== start $(date +%T)"
L="/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-h14-logs"
T="/Users/ueli/Documents/semio/🌎️hub/🧪️tests/🔬️bin-unit/🦀️.rs"
mkdir -p "$BK"; cp "$T" "$BK/laws.orig"
python3 $W/h14-ack-law-permits.py; shasum "$T" | cut -c1-40 > "$BK/applied"
nice -n 15 cargo test -p semio-hub --bin os-hub --no-fail-fast -- a_batch_committed_past_the_frame_deadline a_plan_socket_outlives socket_grant_revoke_before_command_admission > "$L/hold12-laws.txt" 2>&1; rc=$?; echo "=== LAWS EXIT $rc $(date +%T)"
/usr/bin/grep -E "^test |^test result|panicked" "$L/hold12-laws.txt" | head
/usr/bin/grep -c "unused .SemaphorePermit" "$L/hold12-laws.txt"
if [ "$rc" != 0 ] && [ "$(shasum "$T" | cut -c1-40)" = "$(cat "$BK/applied")" ]; then cp "$BK/laws.orig" "$T"; echo "=== REVERTED"; fi
' >> "$OUT" 2>&1
echo "=== EXIT $? $(date +%T)" >> "$OUT"
