#!/bin/zsh
# ⚖️ H14 hold 9 (native lane): apply h14-hello-deadline-armed.py (restore-on-red), kernel-db check default + no vcs, db_sync laws,
# db_engine vcs_integration laws alone (single thread: they share process-global vcs admission), the engine history law alone,
# semio-hub check (links the new kernel-db).
OUT=$1
W=/Users/ueli/Documents/semio/.tmp-ticket/wp-h14
BK="/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-h14-backup/hold9"
cd /Users/ueli/Documents/semio
export CARGO_INCREMENTAL=0 CARGO_BUILD_BUILD_DIR="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/cargo/build-fleet-b" CARGO_TARGET_DIR="/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-h14-target" W BK NX_DAEMON=false
echo "=== queued $(date +%T)" > "$OUT"
zsh /Users/ueli/Documents/semio/.tmp-ticket/📜️fleet-mutex.sh native h14 -- zsh -c '
echo "=== start $(date +%T)"
L="/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-h14-logs"
mkdir -p "$BK"
S="/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🔄️sync/🦀️.rs"
cp "$S" "$BK/sync.orig"; python3 $W/h14-hello-deadline-armed.py; shasum "$S" | cut -c1-40 > "$BK/sync.applied"
nice -n 15 cargo check -p semio-framework-os-kernel-db --lib --tests --message-format short 2>&1 | /usr/bin/grep -E -A5 "^error|: error|Finished|sync/🦀️.rs.*warning" | head -40; db=${pipestatus[1]}; echo "=== DB CHECK EXIT $db $(date +%T)"
if [ "$db" = 0 ]; then nice -n 15 cargo check -p semio-framework-os-kernel-db --no-default-features --features fs,deflate --lib --message-format short 2>&1 | /usr/bin/grep -E "^error|: error|Finished"; nv=${pipestatus[1]}; else nv=skipped; fi; echo "=== DB NO-VCS CHECK EXIT $nv $(date +%T)"
if [ "$db" != 0 ] || [ "$nv" != 0 ]; then if [ "$(shasum "$S" | cut -c1-40)" = "$(cat "$BK/sync.applied")" ]; then cp "$BK/sync.orig" "$S"; echo "=== REVERTED deadline-armed"; fi; fi
nice -n 15 cargo test -p semio-framework-os-kernel-db --lib --no-fail-fast -- db_sync:: --test-threads 4 > "$L/hold9-sync-laws.txt" 2>&1; echo "=== SYNC LAWS EXIT $? $(date +%T)"
/usr/bin/grep -E "^test result|FAILED|panicked" "$L/hold9-sync-laws.txt" | head -20
nice -n 15 cargo test -p semio-framework-os-kernel-db --lib --no-fail-fast -- db_engine::vcs_integration --test-threads 1 > "$L/hold9-vcs-laws.txt" 2>&1; echo "=== VCS LAWS (1 thread) EXIT $? $(date +%T)"
/usr/bin/grep -E "^test result|FAILED|panicked" "$L/hold9-vcs-laws.txt" | head -20
nice -n 15 cargo test -p semio-framework-os-kernel-db --lib --no-fail-fast -- artifact_history_empty_and_two_batch_replay_are_deterministic --test-threads 1 2>&1 | /usr/bin/grep -E "^test |panicked|^test result|witness"; echo "=== HISTORY LAW EXIT ${pipestatus[1]} $(date +%T)"
nice -n 15 cargo check -p semio-hub --lib --bins --tests --message-format short 2>&1 | /usr/bin/grep -E -A5 "^error|: error|Finished" | head -30; echo "=== HUB CHECK EXIT ${pipestatus[1]} $(date +%T)"
' >> "$OUT" 2>&1
echo "=== EXIT $? $(date +%T)" >> "$OUT"
