#!/bin/zsh
# 🚀️ W2 (session 12): the ONE post-publish sequence, launched by the coordinator once `--packages all` reports rc=0:
# 1. build the current-tree os-hub (hub mutex), 2. restart hub 7800 onto `.🧬semio/🌐hub/w2-catalog-all` on a fresh data root with the
# catalog's guest-codec verification memory, 3. wait for readiness, capture `/readyz` + idle footprint, 4. open-plan probe over every
# creatable kind, footprint after the creations, 5. one summary line. Every step logs to `.🧬semio/🌐hub/s12-w2-logs/after-all.txt`.
# usage (detached): python3 w2-detach.py <log> zsh w2-after-all.sh
setopt no_bg_nice
set -u
cd /Users/ueli/Documents/semio || exit 1
H="/Users/ueli/Documents/semio/.🧬semio/🌐hub"
OUT="$H/s12-w2-logs"
W=/Users/ueli/Documents/semio/.tmp-ticket/wp-w2
MUTEX=(/Users/ueli/Documents/semio/.tmp-ticket/*fleet-mutex.sh)
HUB_RUST=(/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust)
CATALOG="$H/w2-catalog-all"
NAME=s12-w2-hub-7800-all
STATE="$H/s12-w2-state-7800"
unset CARGO_TARGET_DIR CARGO_BUILD_TARGET_DIR
export CARGO_INCREMENTAL=0 NX_DAEMON=false
log() { echo "[w2-after] $* $(date '+%F %T')"; }
/usr/bin/grep -q 'END publish w2-catalog-all rc=0' "$OUT"/publish-all-3-chain.txt "$OUT"/publish-all.txt 2>/dev/null || { log "REFUSED: no rc=0 publish of w2-catalog-all recorded"; exit 1; }
test -f "$CATALOG/trusted-catalog/current.json" || { log "REFUSED: $CATALOG has no current generation"; exit 1; }
test -e "$H/$NAME" && { log "REFUSED: $H/$NAME already exists"; exit 1; }

log "START hub-build"
zsh "$MUTEX[1]" hub w2 -- zsh -c "cd '$HUB_RUST[1]' && cargo build --manifest-path Cargo.toml --bin os-hub" > "$OUT/after-all-hub-build.txt" 2>&1 || { log "hub build FAILED"; tail -20 "$OUT/after-all-hub-build.txt"; exit 1; }
BIN_SRC="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/cargo/target/debug/os-hub"
log "END hub-build sha256=$(shasum -a 256 "$BIN_SRC" | cut -c1-64)"

OLD=$(sed -n 's/^hold=//p' "$STATE/pids.txt" 2>/dev/null)
log "START restart old-hold=${OLD:--}"
zsh "$W/w2-restart-7800.sh" "$CATALOG" "$BIN_SRC" "$NAME" "${OLD:--}" > "$OUT/after-all-restart.txt" 2>&1 || { log "restart FAILED"; tail -20 "$OUT/after-all-restart.txt"; exit 1; }
log "END restart $(tail -1 "$OUT/after-all-restart.txt")"

started=$(date +%s)
until /usr/bin/grep -qE 'HOLD |HOLD_END|WAIT_FAIL' "$STATE/status.txt" 2>/dev/null || [ $(( $(date +%s) - started )) -gt 7200 ]; do sleep 15; done
/usr/bin/grep -q 'HOLD ' "$STATE/hold.txt" || { log "7800 NOT READY after $(( $(date +%s) - started ))s: $(cat "$STATE/status.txt")"; exit 1; }
log "READY boot=$(( $(date +%s) - started ))s $(/usr/bin/grep 'HOLD ' "$STATE/hold.txt" | tail -1)"
curl -s -m 20 http://127.0.0.1:7800/readyz > "$OUT/readyz-7800-all.json"
HUB=$(sed -n 's/^hub=//p' "$STATE/pids.txt")
footprint "$HUB" > "$OUT/footprint-7800-all-idle.txt" 2>&1
log "footprint idle $(head -2 "$OUT/footprint-7800-all-idle.txt" | tail -1 | /usr/bin/grep -o 'Footprint: [0-9.]* [KMG]B')"

log "START open-plan probe"
bun "$W/w2-open-plan-probe.ts" http://127.0.0.1:7800 all > "$OUT/open-plan-probe-7800-all.txt" 2>&1
rc=$?
footprint "$HUB" > "$OUT/footprint-7800-all-after-creations.txt" 2>&1
log "END open-plan probe rc=$rc $(tail -1 "$OUT/open-plan-probe-7800-all.txt") footprint $(head -2 "$OUT/footprint-7800-all-after-creations.txt" | tail -1 | /usr/bin/grep -o 'Footprint: [0-9.]* [KMG]B')"
log "DONE"
