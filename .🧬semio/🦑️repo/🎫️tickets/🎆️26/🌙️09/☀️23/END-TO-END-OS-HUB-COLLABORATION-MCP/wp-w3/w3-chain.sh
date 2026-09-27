#!/bin/zsh
# 🔗️ W3 (session 13): the ONE consolidated chain, launched by the coordinator (python3 .tmp-ticket/wp-w2/w2-detach.py <log> zsh w3-chain.sh <phase>).
#   b3   os-hub build-dev prewarm (hub mutex) ‖ ONE wasm hold (w3-wasm-hold.sh b3: B release warm lane ‖ rebuild-all = cargo provenance →
#        mutation-authority freshness → describe+materialize-dev ×60 → generate → check → activate-s → verify-s → flow-core bindings;
#        catalog preflight; B3 bootstrap ‖ lane: renderer wasm-release, B releases from the end; then rest-warm of the other 25) →
#        once B3 is published: os-hub build-dev + os-mcp build → 7800 onto B3 on a fresh root → readiness, /readyz, hub-freshness,
#        footprint, open-plan probe over every creatable kind, footprint → waits for the wasm hold (renderer, rest-warm) to end.
#   final os-hub build-dev prewarm ‖ ONE wasm hold (w3-wasm-hold.sh final: every release warming ‖ rebuild-all; catalog preflight;
#        `--packages all` bootstrap ‖ lane: renderer wasm-release, releases from the end) → once published: os-hub build-dev + os-mcp
#        build → 7800 onto the all catalog on a fresh root (the B3 root and binary stay for rollback: stop the hold, then
#        `zsh w3-hub-resume.sh s13-w3-hub-7800-b3`) → readiness, /readyz, hub-freshness, footprint, open-plan probe over every kind;
#        meanwhile the hold re-materializes the release plugin-module root and checks it (w3-wasm-hold.sh final).
# Logs, catalogs, data roots and binaries live under .🧬semio/🌐hub/s13-w3-*; the chain uses the DEFAULT build-dir (warm wasm units).
setopt no_bg_nice
set -u
PHASE="${1:?usage: w3-chain.sh b3|final}"
cd /Users/ueli/Documents/semio || exit 1
H="${W3_HUB_ROOT:-/Users/ueli/Documents/semio/.🧬semio/🌐hub}"
OUT="$H/s13-w3-logs"
W="${W3_DIR:-/Users/ueli/Documents/semio/.tmp-ticket/wp-w3}"
MUTEX=(${W3_MUTEX:-/Users/ueli/Documents/semio/.tmp-ticket/📜️fleet-mutex.sh})
HUB_DEV=(/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/dist/build-dev)
unset CARGO_TARGET_DIR CARGO_BUILD_TARGET_DIR CARGO_BUILD_BUILD_DIR
export CARGO_INCREMENTAL=0 NX_DAEMON=false
mkdir -p "$OUT"
log() { echo "[w3-$PHASE] $* $(date '+%F %T')"; }
step() { local name="$1"; shift; log "START $name"; local s=$(date +%s); "$@" > "$OUT/$PHASE-$name.txt" 2>&1; local rc=$?; log "END $name rc=$rc wall=$(( $(date +%s) - s ))s"; return $rc; }
hub_build() { zsh "$MUTEX[1]" hub w3 -- bun nx run os-hub:build-dev --skip-nx-cache --outputStyle=stream; }

wasm_hold() {
  zsh "$MUTEX[1]" wasm w3 -- zsh "$W/w3-wasm-hold.sh" "$PHASE" > "$OUT/$PHASE-wasm-hold.txt" 2>&1 &
  hold=$!
  until [ -e "$OUT/$PHASE-publish.rc" ] || ! kill -0 $hold 2>/dev/null; do sleep 20; done
  cat "$OUT/$PHASE-wasm-hold.txt"
  [ -e "$OUT/$PHASE-publish.rc" ] || { wait $hold; log "wasm hold ended without a publication rc=$?"; return 1; }
  local rc=$(cat "$OUT/$PHASE-publish.rc")
  log "publication rc=$rc"
  [ "$rc" = 0 ]
}

move_7800() {
  local catalog="$1" name="$2" oldstate="$3"
  local old=$(sed -n 's/^hold=//p' "$oldstate/pids.txt" 2>/dev/null)
  step restart env OLDSTATE="$oldstate" zsh "$W/w3-restart-7800.sh" "$catalog" "$HUB_DEV[1]" "$name" "${old:--}" || return 1
  local state="$H/s13-w3-state-7800" started=$(date +%s)
  until /usr/bin/grep -qE 'HOLD |HOLD_END|WAIT_FAIL' "$state/status.txt" 2>/dev/null || [ $(( $(date +%s) - started )) -gt 7200 ]; do sleep 15; done
  /usr/bin/grep -q 'HOLD ' "$state/hold.txt" || { log "7800 NOT READY after $(( $(date +%s) - started ))s: $(cat "$state/status.txt" 2>/dev/null)"; return 1; }
  log "READY boot=$(( $(date +%s) - started ))s $(/usr/bin/grep 'HOLD ' "$state/hold.txt" | tail -1)"
  curl -s -m 20 http://127.0.0.1:7800/readyz > "$OUT/$PHASE-readyz-7800.json"
  bun nx run os-hub-ts:hub-freshness --hub http://127.0.0.1:7800 > "$OUT/$PHASE-hub-freshness.txt" 2>&1
  log "hub-freshness $(/usr/bin/grep -o '"verdict":"[a-z]*"' "$OUT/$PHASE-hub-freshness.txt" | head -1)"
  local hub=$(sed -n 's/^hub=//p' "$state/pids.txt")
  footprint "$hub" > "$OUT/$PHASE-footprint-idle.txt" 2>&1
  step open-plan-probe bun "$W/w3-open-plan-probe.ts" http://127.0.0.1:7800 all
  log "open-plan $(tail -1 "$OUT/$PHASE-open-plan-probe.txt")"
  footprint "$hub" > "$OUT/$PHASE-footprint-after-creations.txt" 2>&1
  log "footprint idle $(/usr/bin/grep -o 'Footprint: [0-9.]* [KMG]B' "$OUT/$PHASE-footprint-idle.txt" | head -1) after $(/usr/bin/grep -o 'Footprint: [0-9.]* [KMG]B' "$OUT/$PHASE-footprint-after-creations.txt" | head -1)"
}

case "$PHASE" in
  b3)
    test -e "$H/s13-w3-hub-7800-b3" && { log "REFUSED: $H/s13-w3-hub-7800-b3 exists"; exit 1; }
    rm -f "$OUT/b3-hub-prewarm.done" "$OUT/b3-publish.rc"
    ( step hub-prewarm hub_build; touch "$OUT/b3-hub-prewarm.done" ) &
    wasm_hold || { wait; exit 1; }
    step hub-build hub_build || exit 1
    step mcp-build bun nx run @semio-tech/framework-os-mcp-rs:build --skip-nx-cache --outputStyle=stream
    move_7800 "$H/s13-w3-catalog-b3" s13-w3-hub-7800-b3 "$H/s12-w2-state-7800" || exit 1
    log "B3 SERVED on 7800; waiting for the wasm hold (renderer, rest-warm)"
    wait
    ;;
  final)
    test -e "$H/s13-w3-hub-7800-all" && { log "REFUSED: $H/s13-w3-hub-7800-all exists"; exit 1; }
    test -e "$H/s13-w3-catalog-all/trusted-catalog/current.json" && { log "REFUSED: $H/s13-w3-catalog-all already carries a publication"; exit 1; }
    touch "$OUT/rest-warm.stop"
    rm -f "$OUT/final-hub-prewarm.done" "$OUT/final-publish.rc"
    ( step hub-prewarm hub_build; touch "$OUT/final-hub-prewarm.done" ) &
    wasm_hold || { wait; exit 1; }
    step hub-build hub_build || exit 1
    step mcp-build bun nx run @semio-tech/framework-os-mcp-rs:build --skip-nx-cache --outputStyle=stream
    move_7800 "$H/s13-w3-catalog-all" s13-w3-hub-7800-all "$H/s13-w3-state-7800" || exit 1
    log "ALL SERVED on 7800; waiting for the wasm hold (renderer, release plugin-module root)"
    wait
    /usr/bin/grep -E 'release-(support|modules|root-check)|release root' "$OUT/final-wasm-hold.txt" | /usr/bin/grep -v '^  |'
    ;;
  *) log "unknown phase"; exit 1 ;;
esac
log "DONE"
