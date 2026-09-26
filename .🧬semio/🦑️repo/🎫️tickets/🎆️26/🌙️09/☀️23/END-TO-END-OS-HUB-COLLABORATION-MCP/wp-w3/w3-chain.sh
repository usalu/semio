#!/bin/zsh
# 🔗️ W3 (session 13): the ONE consolidated chain, launched by the coordinator (python3 .tmp-ticket/wp-w2/w2-detach.py <log> zsh w3-chain.sh <phase>).
#   b3   rebuild-all (cargo provenance → mutation-authority freshness → describe+materialize-dev ×60 → generate → check → activate-s →
#        verify-s → flow-core bindings) with os-hub build-dev prewarmed beside it → catalog preflight (seconds) → B3 publish (9 packages,
#        one warm-behind release lane walking the list from the end) → os-hub build-dev + os-mcp build → 7800 onto B3 on a fresh root →
#        readiness, /readyz, hub-freshness, footprint, open-plan probe over every creatable kind, footprint.
#   all  catalog preflight → `--packages all` publish (warm-behind lane over the remaining 25) → os-hub build-dev → 7800 onto it on a
#        fresh root → the same verification.
# Every wasm32 span runs inside the fleet `wasm` mutex; logs, catalogs, data roots and binaries live under .🧬semio/🌐hub/s13-w3-*.
setopt no_bg_nice
set -u
PHASE="${1:?usage: w3-chain.sh b3|all}"
cd /Users/ueli/Documents/semio || exit 1
H="/Users/ueli/Documents/semio/.🧬semio/🌐hub"
OUT="$H/s13-w3-logs"
W=/Users/ueli/Documents/semio/.tmp-ticket/wp-w3
MUTEX=(/Users/ueli/Documents/semio/.tmp-ticket/*fleet-mutex.sh)
HUB_DEV=(/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/dist/build-dev)
B_PACKAGES="stdio,gis,note,animate,block,writer,draw,puzzle,wfc"
ALL_ORDER="stdio gis animate architect block cad dag demonstrator draw energy fem flow forms imperative layout lowpoly mathematical norm note playbook procedural process puzzle raster reasoning remodel sequence shooting sourcing space trinity vcs wfc writer"
unset CARGO_TARGET_DIR CARGO_BUILD_TARGET_DIR CARGO_BUILD_BUILD_DIR
export CARGO_INCREMENTAL=0 NX_DAEMON=false
mkdir -p "$OUT"
log() { echo "[w3-$PHASE] $* $(date '+%F %T')"; }
step() { local name="$1"; shift; log "START $name"; local s=$(date +%s); "$@" > "$OUT/$PHASE-$name.txt" 2>&1; local rc=$?; log "END $name rc=$rc wall=$(( $(date +%s) - s ))s"; return $rc; }

publish() {
  local catalog="$1" packages="$2" behind="$3" inner="$OUT/$PHASE-publish-inner.sh"
  mkdir -p "$catalog"; chmod 700 "$catalog"; rm -f "$OUT/$PHASE-lane.stop" "$OUT/$PHASE-lane.pid"
  cat > "$inner" <<INNER
setopt no_bg_nice
lane() {
  for p in ${behind}; do
    [ -e "$OUT/$PHASE-lane.stop" ] && break
    echo "[lane] START \$p \$(date +%T)"
    bun nx run @semio-tech/\$p-plugin:component-release --outputStyle=stream > "$OUT/$PHASE-lane-\$p.txt" 2>&1 &
    echo \$! > "$OUT/$PHASE-lane.pid"; wait \$!
    echo "[lane] END \$p rc=\$? \$(date +%T)"
  done
}
if [ "\${W3_LANE:-1}" = 1 ]; then lane > "$OUT/$PHASE-lane.txt" 2>&1 & lanepid=\$!; fi
OS_HUB_DATA="$catalog" bun nx run os-hub:trusted-catalog-bootstrap --packages $packages --outputStyle=stream; rc=\$?
touch "$OUT/$PHASE-lane.stop"
if [ -n "\${lanepid:-}" ]; then nxpid=\$(cat "$OUT/$PHASE-lane.pid" 2>/dev/null); [ -n "\$nxpid" ] && kill -TERM "\$nxpid" 2>/dev/null; wait \$lanepid 2>/dev/null; fi
exit \$rc
INNER
  zsh "$MUTEX[1]" wasm w3 -- zsh "$inner"
}

move_7800() {
  local catalog="$1" name="$2" oldstate="$3"
  local old=$(sed -n 's/^hold=//p' "$oldstate/pids.txt" 2>/dev/null)
  OLDSTATE="$oldstate" zsh "$W/w3-restart-7800.sh" "$catalog" "$HUB_DEV[1]" "$name" "${old:--}" || return 1
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
    log "START hub-prewarm (beside rebuild-all)"
    ( zsh "$MUTEX[1]" hub w3 -- bun nx run os-hub:build-dev --skip-nx-cache --outputStyle=stream > "$OUT/b3-hub-prewarm.txt" 2>&1; echo "[w3-b3] END hub-prewarm rc=$? $(date '+%F %T')" >> "$OUT/b3-hub-prewarm.txt" ) &
    prewarm=$!
    step rebuild-all zsh "$MUTEX[1]" wasm w3 -- bun nx run @semio-tech/plugin-registry:rebuild-all --to flow-core-bindings --outputStyle=stream || { wait $prewarm; exit 1; }
    /usr/bin/grep -h 'components=' "$OUT/b3-rebuild-all.txt" | tail -1
    wait $prewarm; log "hub-prewarm $(tail -1 "$OUT/b3-hub-prewarm.txt")"
    step preflight bun nx run os-hub:trusted-catalog-preflight --packages all || exit 1
    step publish-b3 publish "$H/s13-w3-catalog-b3" "$B_PACKAGES" "wfc puzzle draw writer block animate note" || exit 1
    step hub-build zsh "$MUTEX[1]" hub w3 -- bun nx run os-hub:build-dev --skip-nx-cache --outputStyle=stream || exit 1
    step mcp-build bun nx run @semio-tech/framework-os-mcp-rs:build --outputStyle=stream
    move_7800 "$H/s13-w3-catalog-b3" s13-w3-hub-7800-b3 "$H/s12-w2-state-7800" || exit 1
    ;;
  all)
    test -e "$H/s13-w3-hub-7800-all" && { log "REFUSED: $H/s13-w3-hub-7800-all exists"; exit 1; }
    step preflight bun nx run os-hub:trusted-catalog-preflight --packages all || exit 1
    rest=$(for p in ${=ALL_ORDER}; do case ",$B_PACKAGES," in *",$p,"*) ;; *) echo $p ;; esac; done | tail -r | tr '\n' ' ')
    step publish-all publish "$H/s13-w3-catalog-all" all "$rest" || exit 1
    step hub-build zsh "$MUTEX[1]" hub w3 -- bun nx run os-hub:build-dev --skip-nx-cache --outputStyle=stream || exit 1
    move_7800 "$H/s13-w3-catalog-all" s13-w3-hub-7800-all "$H/s13-w3-state-7800" || exit 1
    ;;
  *) log "unknown phase"; exit 1 ;;
esac
log "DONE"
