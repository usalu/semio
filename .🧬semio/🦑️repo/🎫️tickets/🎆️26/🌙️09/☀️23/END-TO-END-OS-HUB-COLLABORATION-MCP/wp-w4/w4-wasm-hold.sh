#!/bin/zsh
# 🔒️ W4 (session 14): W3's `final` wasm hold (wp-w3/w3-wasm-hold.sh) on session-14 names — logs `s14-w4-logs`, catalog `s14-w4-catalog-all`;
#   everything the phase compiles for wasm32 runs inside ONE fleet `wasm` hold (w4-chain.sh starts it through the mutex). rebuild-all and the
#   publication retry until green (w4-green.zsh: a peer's transient red waits for its crates' wasm32 check, then resumes `--from` the failed step).
#   final forward warm lane (every release in the `--packages all` order, starts once the hub prewarm is done) ‖ rebuild-all
#        --to flow-core-bindings → catalog preflight → `--packages all` bootstrap ‖ reverse lane (renderer wasm-release first, never
#        cut; then releases from the end) → <OUT>/final-publish.rc → cut the reverse lane (the renderer always finishes) → the shared
#        RELEASE plugin-module root re-materialized from the current tree (browser support = shard worker, preview2 vendor, fonts; every
#        component's `materialize-release`) → `w3-release-root-check.ts release` (shard worker == `shardWorkerSource()` with the codec case,
#        every module present with the current host shim).
# usage: zsh w4-wasm-hold.sh final
setopt no_bg_nice
set -u
PHASE="$1"
cd /Users/ueli/Documents/semio || exit 1
H="${W3_HUB_ROOT:-/Users/ueli/Documents/semio/.🧬semio/🌐hub}"
OUT="$H/s14-w4-logs"
W="${W3_DIR:-/Users/ueli/Documents/semio/.tmp-ticket/wp-w3}"
ALL_ORDER=(stdio gis animate architect block cad dag demonstrator draw energy fem flow forms imperative layout lowpoly mathematical norm note playbook procedural process puzzle raster reasoning remodel sequence shooting sourcing space trinity vcs wfc writer)
log() { echo "[w4-$PHASE-wasm] $* $(date '+%F %T')"; }
step() { local name="$1"; shift; log "START $name"; local s=$(date +%s); "$@" > "$OUT/$PHASE-$name.txt" 2>&1; local rc=$?; log "END $name rc=$rc wall=$(( $(date +%s) - s ))s"; return $rc; }
source "${W4_DIR:-/Users/ueli/Documents/semio/.tmp-ticket/wp-w4}/w4-green.zsh"
cut_lane() {
  touch "$1"
  local pid=$(cat "$1.pid" 2>/dev/null)
  [ -n "$pid" ] && ps -o command= -p "$pid" 2>/dev/null | /usr/bin/grep -q 'component-release' && kill -TERM "$pid"
}
bootstrap() { OS_HUB_DATA="$1" bun nx run os-hub:trusted-catalog-bootstrap --packages "$2" --outputStyle=stream; }
rebuild_resume() {
  local from=$(/usr/bin/grep -oE 'rebuild-all [0-9]+/[0-9]+ [a-z-]+ \([a-z]+\) started' "$1" | tail -1 | awk '{print $3}')
  echo bun nx run @semio-tech/plugin-registry:rebuild-all ${from:+--from $from} --to flow-core-bindings --outputStyle=stream
}
publish_all() {
  local catalog="$H/s14-w4-catalog-all"
  [ -d "$catalog" ] && [ -n "$(ls -A "$catalog")" ] && { echo "[w4] moving the failed attempt's catalog aside"; mv "$catalog" "$catalog-failed-$(date +%H%M%S)"; }
  mkdir -p "$catalog"; chmod 700 "$catalog"
  bootstrap "$catalog" all
}
w4_on_failure() { [ "$1" = rebuild-all ] && failure "$2"; [ "$1" = publish-all ] && /usr/bin/grep -E 'error|Error' "$2" | tail -5 | sed 's/^/  | /'; return 0; }
failure() {
  log "FAILED in $(/usr/bin/grep -oE 'rebuild-all [0-9]+/[0-9]+ [a-z-]+ \([a-z]+\) started' "$1" | tail -1 | sed 's/ started$//')"
  log "every crate: $(/usr/bin/grep -oE 'could not compile `[^`]+`' "$1" | sort -u | tr '\n' ' ')"
  log "first crate: $(/usr/bin/grep -m1 -E 'could not compile `' "$1")"
  /usr/bin/grep -m1 -A14 -E '(^|: )error(\[E[0-9]+\])?: ' "$1" | sed 's/^/  | /'
}

case "$PHASE" in
  final)
    rm -f "$OUT/final-warm-fwd.stop" "$OUT/final-lane-rev.stop" "$OUT/final-publish.rc"
    ( until [ -e "$OUT/final-hub-prewarm.done" ] || [ -e "$OUT/final-warm-fwd.stop" ]; do sleep 20; done
      zsh "$W/w3-lane.sh" "$OUT/final-warm-fwd" "$OUT/final-warm-fwd.stop" $ALL_ORDER ) > "$OUT/final-warm-fwd.txt" 2>&1 &
    fwd=$!
    w4_retry rebuild-all wasm blind rebuild_resume -- bun nx run @semio-tech/plugin-registry:rebuild-all --to flow-core-bindings --outputStyle=stream || { cut_lane "$OUT/final-warm-fwd.stop"; wait $fwd; exit 1; }
    step preflight bun nx run os-hub:trusted-catalog-preflight --packages all || { /usr/bin/grep -A8 'refused' "$OUT/final-preflight.txt" | sed 's/^/  | /'; cut_lane "$OUT/final-warm-fwd.stop"; wait $fwd; exit 1; }
    touch "$OUT/final-warm-fwd.stop"
    ( while kill -0 $fwd 2>/dev/null; do sleep 10; done
      zsh "$W/w3-lane.sh" "$OUT/final-lane-rev" "$OUT/final-lane-rev.stop" renderer ${(Oa)ALL_ORDER} ) > "$OUT/final-lane-rev.txt" 2>&1 &
    rev=$!
    w4_retry publish-all wasm strict - -- publish_all; rc=$?
    echo $rc > "$OUT/final-publish.rc"
    cut_lane "$OUT/final-lane-rev.stop"; wait $fwd $rev
    log "renderer $(/usr/bin/grep 'END renderer' "$OUT/final-lane-rev.txt" | tail -1)"
    [ $rc = 0 ] || exit $rc
    step release-support bun nx run @semio-tech/framework-plugin-web:support-release --skip-nx-cache --outputStyle=stream
    step release-modules bun nx run-many -t materialize-release --skip-nx-cache --parallel=2 --outputStyle=stream
    step release-root-check bun "$W/w3-release-root-check.ts" release; check=$?
    log "release root $(/usr/bin/grep '\[w3-release-root-check\]' "$OUT/final-release-root-check.txt" | tail -1)"
    exit $check
    ;;
  *) log "unknown phase"; exit 1 ;;
esac
log "DONE"
