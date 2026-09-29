#!/bin/zsh
# ⚖️ H14 hold 11 (native lane): apply h14-plan-socket-live.py (hub, restore-on-red), semio-hub check, the plan/socket/presence/
# revocation laws (red → revert), the whole os-hub bin suite (informational), the integration-fixtures Check In laws.
OUT=$1
W=/Users/ueli/Documents/semio/.tmp-ticket/wp-h14
BK="/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-h14-backup/hold11"
cd /Users/ueli/Documents/semio
export CARGO_INCREMENTAL=0 CARGO_BUILD_BUILD_DIR="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/cargo/build-fleet-b" CARGO_TARGET_DIR="/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-h14-target" W BK NX_DAEMON=false
echo "=== queued $(date +%T)" > "$OUT"
zsh /Users/ueli/Documents/semio/.tmp-ticket/📜️fleet-mutex.sh native h14 -- zsh -c '
echo "=== start $(date +%T)"
L="/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-h14-logs"
S="/Users/ueli/Documents/semio/🌎️hub/🏗️bootstrap/🦀️.rs"
T="/Users/ueli/Documents/semio/🌎️hub/🧪️tests/🔬️bin-unit/🦀️.rs"
mkdir -p "$BK"
cp "$S" "$BK/bootstrap.orig"; cp "$T" "$BK/laws.orig"
python3 $W/h14-plan-socket-live.py; shasum "$S" "$T" | cut -c1-40 > "$BK/applied"
revert() { if [ "$(shasum "$S" "$T" | cut -c1-40)" = "$(cat "$BK/applied")" ]; then cp "$BK/bootstrap.orig" "$S"; cp "$BK/laws.orig" "$T"; echo "=== REVERTED plan-socket-live ($1)"; else echo "=== NOT REVERTED: files changed since apply ($1)"; fi; }
nice -n 15 cargo check -p semio-hub --lib --bins --tests --message-format short 2>&1 | /usr/bin/grep -E -A6 "^error|: error|Finished|bootstrap/🦀️.rs:[0-9]+:[0-9]+: warning" | head -60; hub=${pipestatus[1]}; echo "=== HUB CHECK EXIT $hub $(date +%T)"
if [ "$hub" != 0 ]; then revert check; exit 1; fi
nice -n 15 cargo test -p semio-hub --bin os-hub --no-fail-fast -- a_plan_socket_outlives document_open_plan socket_grant presence admin_removal revok a_batch_committed_past_the_frame_deadline --test-threads 4 > "$L/hold11-plan-laws.txt" 2>&1; laws=$?; echo "=== PLAN/SOCKET LAWS EXIT $laws $(date +%T)"
/usr/bin/grep -E "^test result|FAILED|panicked" "$L/hold11-plan-laws.txt" | head -30
/usr/bin/grep -E "^test (quick::)?a_plan_socket_outlives|^test document_open_plan_socket_consume" "$L/hold11-plan-laws.txt"
if [ "$laws" != 0 ]; then revert laws; exit 1; fi
nice -n 15 cargo test -p semio-hub --bin os-hub --no-fail-fast -- --test-threads 4 > "$L/hold11-bin-all.txt" 2>&1; echo "=== OS-HUB BIN ALL EXIT $? $(date +%T)"
/usr/bin/grep -E "^test result|FAILED|panicked at" "$L/hold11-bin-all.txt" | head -30
nice -n 15 cargo test -p semio-hub --bin os-hub --features integration-fixtures --no-fail-fast -- check_in the_checkpoint_policy --test-threads 2 2>&1 | /usr/bin/grep -E "^test |panicked|^test result"; echo "=== HUB CHECK-IN LAWS EXIT ${pipestatus[1]} $(date +%T)"
' >> "$OUT" 2>&1
echo "=== EXIT $? $(date +%T)" >> "$OUT"
