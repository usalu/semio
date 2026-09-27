#!/bin/zsh
# 🔒️ W3: everything one chain phase compiles for wasm32, run inside ONE fleet `wasm` hold (w3-chain.sh starts it through the mutex).
#   b3   forward warm lane (B releases in the bootstrap's order, starts once the hub prewarm is done so at most two cargos compile)
#        ‖ rebuild-all --to flow-core-bindings → catalog preflight → B3 bootstrap ‖ reverse lane (renderer wasm-release first, then B
#        releases from the end) → <OUT>/b3-publish.rc → cut the reverse lane (the renderer always finishes) → rest-warm lane over the 25
#        other packages in the `--packages all` order until the `all` phase touches <OUT>/rest-warm.stop.
#   all  catalog preflight → `--packages all` bootstrap ‖ reverse lane over the 25 → <OUT>/all-publish.rc → cut the lane.
# usage: zsh w3-wasm-hold.sh b3|all
setopt no_bg_nice
set -u
PHASE="$1"
cd /Users/ueli/Documents/semio || exit 1
H="${W3_HUB_ROOT:-/Users/ueli/Documents/semio/.🧬semio/🌐hub}"
OUT="$H/s13-w3-logs"
W=/Users/ueli/Documents/semio/.tmp-ticket/wp-w3
B_ORDER=(stdio gis note animate block writer draw puzzle wfc)
REST_ORDER=(architect cad dag demonstrator energy fem flow forms imperative layout lowpoly mathematical norm playbook procedural process raster reasoning remodel sequence shooting sourcing space trinity vcs)
log() { echo "[w3-$PHASE-wasm] $* $(date '+%F %T')"; }
step() { local name="$1"; shift; log "START $name"; local s=$(date +%s); "$@" > "$OUT/$PHASE-$name.txt" 2>&1; local rc=$?; log "END $name rc=$rc wall=$(( $(date +%s) - s ))s"; return $rc; }
cut_lane() {
  touch "$1"
  local pid=$(cat "$1.pid" 2>/dev/null)
  [ -n "$pid" ] && ps -o command= -p "$pid" 2>/dev/null | /usr/bin/grep -q 'component-release' && kill -TERM "$pid"
}
bootstrap() { OS_HUB_DATA="$1" bun nx run os-hub:trusted-catalog-bootstrap --packages "$2" --outputStyle=stream; }
failure() {
  log "FAILED in $(/usr/bin/grep -oE 'rebuild-all [0-9]+/[0-9]+ [a-z-]+ \([a-z]+\) started' "$1" | tail -1 | sed 's/ started$//')"
  log "every crate: $(/usr/bin/grep -oE 'could not compile `[^`]+`' "$1" | sort -u | tr '\n' ' ')"
  log "first crate: $(/usr/bin/grep -m1 -E 'could not compile `' "$1")"
  /usr/bin/grep -m1 -A14 -E '(^|: )error(\[E[0-9]+\])?: ' "$1" | sed 's/^/  | /'
}

case "$PHASE" in
  b3)
    rm -f "$OUT/b3-warm-fwd.stop" "$OUT/b3-lane-rev.stop" "$OUT/b3-publish.rc" "$OUT/rest-warm.stop"
    ( until [ -e "$OUT/b3-hub-prewarm.done" ] || [ -e "$OUT/b3-warm-fwd.stop" ]; do sleep 20; done
      zsh "$W/w3-lane.sh" "$OUT/b3-warm-fwd" "$OUT/b3-warm-fwd.stop" $B_ORDER ) > "$OUT/b3-warm-fwd.txt" 2>&1 &
    fwd=$!
    step rebuild-all bun nx run @semio-tech/plugin-registry:rebuild-all --to flow-core-bindings --outputStyle=stream || { failure "$OUT/b3-rebuild-all.txt"; cut_lane "$OUT/b3-warm-fwd.stop"; wait $fwd; exit 1; }
    step preflight bun nx run os-hub:trusted-catalog-preflight --packages all || { /usr/bin/grep -A8 'refused' "$OUT/b3-preflight.txt" | sed 's/^/  | /'; cut_lane "$OUT/b3-warm-fwd.stop"; wait $fwd; exit 1; }
    touch "$OUT/b3-warm-fwd.stop"
    ( while kill -0 $fwd 2>/dev/null; do sleep 10; done
      zsh "$W/w3-lane.sh" "$OUT/b3-lane-rev" "$OUT/b3-lane-rev.stop" renderer ${(Oa)B_ORDER} ) > "$OUT/b3-lane-rev.txt" 2>&1 &
    rev=$!
    mkdir -p "$H/s13-w3-catalog-b3"; chmod 700 "$H/s13-w3-catalog-b3"
    step publish-b3 bootstrap "$H/s13-w3-catalog-b3" "${(j:,:)B_ORDER}"; rc=$?
    echo $rc > "$OUT/b3-publish.rc"
    cut_lane "$OUT/b3-lane-rev.stop"; wait $fwd $rev
    log "renderer $(/usr/bin/grep 'END renderer' "$OUT/b3-lane-rev.txt" | tail -1)"
    [ $rc = 0 ] || exit $rc
    [ -e "$OUT/rest-warm.stop" ] || step rest-warm zsh "$W/w3-lane.sh" "$OUT/rest-warm" "$OUT/rest-warm.stop" $REST_ORDER
    ;;
  all)
    rm -f "$OUT/all-lane-rev.stop" "$OUT/all-publish.rc"
    step preflight bun nx run os-hub:trusted-catalog-preflight --packages all || exit 1
    zsh "$W/w3-lane.sh" "$OUT/all-lane-rev" "$OUT/all-lane-rev.stop" ${(Oa)REST_ORDER} > "$OUT/all-lane-rev.txt" 2>&1 &
    rev=$!
    mkdir -p "$H/s13-w3-catalog-all"; chmod 700 "$H/s13-w3-catalog-all"
    step publish-all bootstrap "$H/s13-w3-catalog-all" all; rc=$?
    echo $rc > "$OUT/all-publish.rc"
    cut_lane "$OUT/all-lane-rev.stop"; wait $rev
    exit $rc
    ;;
  *) log "unknown phase"; exit 1 ;;
esac
log "DONE"
