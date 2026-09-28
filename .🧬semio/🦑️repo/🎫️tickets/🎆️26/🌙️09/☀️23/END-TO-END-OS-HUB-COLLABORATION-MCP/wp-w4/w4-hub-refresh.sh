#!/bin/zsh
# 🔄️ W4 (14c): HUB-ONLY refresh of canonical hub 7800 — a new os-hub (+ os-mcp) from the current tree onto the SAME data root
# (`s14-w4-hub-7800-p24`: documents, users, spaces preserved) and the SAME p24 trusted catalog. Run after L1 reports T1 green.
#   check  the tree still speaks the wire the p24 guests were published against (channel-version pin + plugin actor world
#          byte-identical to the 20:45 publish); any drift → STOP (exit 2), tell the coordinator.
#   build  ONE native-lane ticket (priority stamp, build-fleet-b): `os-hub:build-dev` + `framework-os-mcp-rs:build`; the staged
#          os-hub goes into a NEW bin dir `s14-w4-bin/s14-w4-hub-7800-p24-t1/` (fresh inode + ad-hoc codesign — never an
#          in-place overwrite of a binary that is running; refuses when the dir exists or the binary is unchanged).
#   prove  no downtime: the new binary boots on an APFS clone of the live root on port 8001 (hold on its own state dir),
#          readiness + open-plan on three kinds, then the clone is stopped and removed (it is this script's own copy).
#   swap   the downtime step: stop the 7800 hold, start it with the new binary on the same root, wait for `HOLD` in the
#          hold's append-only hold.txt (boot seconds recorded), /readyz, hub-freshness; a hold that never reaches `HOLD`
#          rolls back to the previous binary (`w4-hub-resume.sh <root> s14-w4-hub-7800-p24`).
# usage: zsh w4-hub-refresh.sh [--dry-run] check|build|prove|swap      (--dry-run prints every mutating command, runs read-only checks)
setopt no_bg_nice
set -u
DRY=0; [ "${1:-}" = --dry-run ] && { DRY=1; shift; }
STEP="${1:?usage: zsh w4-hub-refresh.sh [--dry-run] check|build|prove|swap}"
R=/Users/ueli/Documents/semio
W=$R/.tmp-ticket/wp-w3
W4=$R/.tmp-ticket/wp-w4
MUTEX=$R/.tmp-ticket/📜️fleet-mutex.sh
H="$R/.🧬semio/🌐hub"
NAME=s14-w4-hub-7800-p24
ROOT="$H/$NAME"
STATE="$H/s13-w3-state-7800"
OLD_BIN="$H/s14-w4-bin/$NAME/os-hub"
NEW_NAME=$NAME-t1
NEW_DIR="$H/s14-w4-bin/$NEW_NAME"
NEW_BIN="$NEW_DIR/os-hub"
STAGED="$R/🌎️hub/📦️packages/🦀️rust/dist/build-dev"
LOGS="$H/s14-w4-logs"
CHANNEL="$R/🧰️framework/🛍️products/💻️os/🧫️fixtures/📡️channel/🔖️channel-version.json"
WORLD="$R/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧬️schema/📜️.wit"
CHANNEL_SHA=8b395a5f97899ae90191fc1b49f8737a86caa45f7b8e8f7e3e9cc38a6bc7e582
WORLD_SHA=10e8aada3abf1c741c8a5806569ee2793e64a64ede89125b52807c1aa55e9ee3
PROVE_PORT=8001
PROVE_ROOT="$H/s14-w4-hub-8001-prove"
PROVE_STATE="$H/s14-w4-state-8001"
PROVE_KINDS=(s.note.note 2d.block 2d.drawing)
export CARGO_INCREMENTAL=0 NX_DAEMON=false
log() { echo "[w4-hub-refresh] $* $(date '+%F %T')"; }
run() { echo "+ $*"; [ $DRY = 1 ] || "$@"; }
sha() { shasum -a 256 "$1" | cut -c1-64; }

check() {
  local channel=$(sha "$CHANNEL") world=$(sha "$WORLD")
  log "channel-version $channel ($([ $channel = $CHANNEL_SHA ] && echo unchanged || echo CHANGED)); plugin world $world ($([ $world = $WORLD_SHA ] && echo unchanged || echo CHANGED))"
  [ $channel = $CHANNEL_SHA ] && [ $world = $WORLD_SHA ] || { log "STOP: the hub ↔ p24 guest wire changed since the publish — no refresh"; exit 2; }
  test -f "$ROOT/trusted-catalog/current.json" && test -x "$OLD_BIN" || { log "STOP: root or current binary missing"; exit 1; }
  log "catalog $(python3 -c "import json,sys;print(json.load(open(sys.argv[1]))['generationId'][:16])" "$ROOT/trusted-catalog/current.json") hold $(sed -n 's/^hold=//p' "$STATE/pids.txt") hub $(sed -n 's/^hub=//p' "$STATE/pids.txt")"
}

wait_hold() {
  local state="$1" cap="$2" started=$(date +%s)
  until /usr/bin/grep -qE 'HOLD |HOLD_END|WAIT_FAIL' "$state/hold.txt" 2>/dev/null || [ $(( $(date +%s) - started )) -gt $cap ]; do sleep 5; done
  /usr/bin/grep -q 'HOLD ' "$state/hold.txt" 2>/dev/null && { echo $(( $(date +%s) - started )); return 0; }
  tail -3 "$state/hold.txt" 2>/dev/null | sed 's/^/  | /' >&2
  return 1
}

stop_hold() {
  local state="$1" pattern="$2" hold=$(sed -n 's/^hold=//p' "$1/pids.txt" 2>/dev/null) hub=$(sed -n 's/^hub=//p' "$1/pids.txt" 2>/dev/null)
  alive() { ps -o command= -p "$1" 2>/dev/null | /usr/bin/grep -q "$2"; }
  [ -n "$hold" ] && alive "$hold" "$pattern" || { log "no live hold in $state"; return 0; }
  run touch "$state/stop"
  [ $DRY = 1 ] && return 0
  for i in {1..60}; do alive "$hold" "$pattern" || break; sleep 1; done
  alive "$hold" "$pattern" && { log "hold $hold ignored stop; TERM"; kill "$hold"; sleep 3; }
  [ -n "$hub" ] && alive "$hub" os-hub && { log "hub $hub outlived its hold; TERM"; kill "$hub"; for i in {1..20}; do alive "$hub" os-hub || break; sleep 1; done; }
  log "stopped hold $hold (hub $hub)"
}

build() {
  test ! -e "$NEW_DIR" || { log "REFUSED: $NEW_DIR exists"; exit 1; }
  local started=$(date +%s) old=$(sha "$OLD_BIN")
  log "build: native lane (stamp 20260928120104, build-fleet-b) → $LOGS/refresh-build.txt"
  run env FLEET_TICKET_STAMP=20260928120104 zsh "$MUTEX" native w4 -- env CARGO_BUILD_BUILD_DIR="$R/.🧬semio/🦑️repo/⚡️cache/cargo/build-fleet-b" nice -n 5 zsh -c \
    'cd /Users/ueli/Documents/semio && echo "START $(date +%T)" && bun nx run os-hub:build-dev --skip-nx-cache --outputStyle=stream && bun nx run @semio-tech/framework-os-mcp-rs:build --skip-nx-cache --outputStyle=stream; rc=$?; echo "END rc=$rc $(date +%T)"; exit $rc' \
    > "$LOGS/refresh-build.txt" 2>&1
  [ $DRY = 1 ] || { local rc=$?; log "build rc=$rc wall=$(( $(date +%s) - started ))s"; [ $rc = 0 ] || { tail -5 "$LOGS/refresh-build.txt"; exit 1; }; }
  [ $DRY = 1 ] || { [ "$(sha "$STAGED/os-hub")" != "$old" ] || { log "REFUSED: the staged os-hub is the running binary (nothing to refresh)"; exit 1; }; }
  run mkdir -p "$NEW_DIR"
  run cp "$STAGED/os-hub" "$NEW_BIN"
  run codesign -s - -f "$NEW_BIN"
  run cp -p "$STAGED/os-hub.sources.json" "$NEW_DIR/os-hub.sources.json"
  [ $DRY = 1 ] || log "installed $NEW_BIN sha256 $(sha "$NEW_BIN" | cut -c1-16) (was $(echo $old | cut -c1-16))"
}

prove() {
  test -x "$NEW_BIN" || [ $DRY = 1 ] || { log "REFUSED: $NEW_BIN missing (run build)"; exit 1; }
  test ! -e "$PROVE_ROOT" || { log "REFUSED: $PROVE_ROOT exists"; exit 1; }
  lsof -nP -iTCP:$PROVE_PORT -sTCP:LISTEN >/dev/null && { log "REFUSED: port $PROVE_PORT bound"; exit 1; }
  run cp -c -R -p "$ROOT" "$PROVE_ROOT"
  run rm -rf "$PROVE_STATE"
  run mkdir -p "$PROVE_STATE"
  run python3 "$W/w3-detach.py" "$PROVE_STATE/hold.txt" bun "$W4/w4-hub-hold.ts" $PROVE_PORT "$PROVE_ROOT" "$NEW_BIN" "$PROVE_STATE"
  [ $DRY = 1 ] && { echo "+ wait HOLD in $PROVE_STATE/hold.txt (≤ 1800 s); readyz; open-plan ${PROVE_KINDS[*]}; stop; rm -rf $PROVE_ROOT"; return 0; }
  local boot; boot=$(wait_hold "$PROVE_STATE" 1800) || { log "PROVE FAILED: the new binary never reached HOLD on the clone"; stop_hold "$PROVE_STATE" "hub-hold\.ts $PROVE_PORT "; exit 1; }
  log "prove READY on $PROVE_PORT boot=${boot}s"
  curl -s -m 20 http://127.0.0.1:$PROVE_PORT/readyz > "$LOGS/refresh-prove-readyz.json"
  bun "$W/w3-open-plan-probe.ts" http://127.0.0.1:$PROVE_PORT $PROVE_KINDS > "$LOGS/refresh-prove-open-plan.txt" 2>&1; local probe=$?
  log "prove readyz $(/usr/bin/grep -o '"status":"[a-z]*"' "$LOGS/refresh-prove-readyz.json" | head -1) open-plan rc=$probe $(/usr/bin/grep -cE '^PASS' "$LOGS/refresh-prove-open-plan.txt")/${#PROVE_KINDS} PASS"
  stop_hold "$PROVE_STATE" "hub-hold\.ts $PROVE_PORT "
  rm -rf "$PROVE_ROOT"
  log "clone removed"
  [ $probe = 0 ]
}

swap() {
  check
  test -x "$NEW_BIN" || [ $DRY = 1 ] || { log "REFUSED: $NEW_BIN missing (run build)"; exit 1; }
  stop_hold "$STATE" 'hub-hold\.ts 7800 '
  [ $DRY = 1 ] || { lsof -nP -iTCP:7800 -sTCP:LISTEN >/dev/null && { log "REFUSED: port 7800 still bound"; exit 1; }; }
  local aside="$STATE-$(date +%H%M%S)"
  run mv "$STATE" "$aside"
  run mkdir -p "$STATE"
  local data=$(cd "$ROOT" && pwd -P) started=$(date +%s)
  run python3 "$W/w3-detach.py" "$STATE/hold.txt" bun "$W4/w4-hub-hold.ts" 7800 "$data" "$NEW_BIN" "$STATE"
  [ $DRY = 1 ] && { echo "+ wait HOLD in $STATE/hold.txt (≤ 1800 s) else rollback: stop + zsh $W4/w4-hub-resume.sh $NAME $NAME; readyz; hub-freshness"; return 0; }
  local boot; boot=$(wait_hold "$STATE" 1800) || {
    log "SWAP FAILED: no HOLD after $(( $(date +%s) - started ))s — rolling back to $OLD_BIN"
    stop_hold "$STATE" 'hub-hold\.ts 7800 '
    zsh "$W4/w4-hub-resume.sh" "$NAME" "$NAME"
    exit 1
  }
  log "7800 READY on $NEW_NAME boot=${boot}s (20:57 binary: 616 s logged, 8 s real; post-reboot resume 5 s)"
  curl -s -m 20 http://127.0.0.1:7800/readyz > "$LOGS/refresh-readyz-7800.json"
  bun nx run os-hub-ts:hub-freshness --hub http://127.0.0.1:7800 > "$LOGS/refresh-hub-freshness.txt" 2>&1
  log "readyz $(/usr/bin/grep -o '"status":"[a-z]*"' "$LOGS/refresh-readyz-7800.json" | head -1) hub-freshness $(/usr/bin/grep -o '"verdict":"[a-z]*"' "$LOGS/refresh-hub-freshness.txt" | head -1) hold $(sed -n 's/^hold=//p' "$STATE/pids.txt") hub $(sed -n 's/^hub=//p' "$STATE/pids.txt")"
}

case "$STEP" in
  check) check ;;
  build) check; build ;;
  prove) prove ;;
  swap) swap ;;
  *) echo "unknown step $STEP"; exit 1 ;;
esac
