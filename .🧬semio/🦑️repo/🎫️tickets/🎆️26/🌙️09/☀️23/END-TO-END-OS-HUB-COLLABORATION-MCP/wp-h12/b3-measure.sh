#!/bin/zsh
# 📏️ H12 after-measurements on W3's published B3 catalog; launch detached (rule 28):
#   python3 .tmp-ticket/wp-w2/w2-detach.py .🧬semio/🌐hub/s13-h12-logs/b3-measure.txt zsh .tmp-ticket/wp-h12/b3-measure.sh
# 1 waits for `b3-publish.rc` → owned-probe chain (the BEFORE rows' binary) on puzzle / note / draw / wfc → after fuel
# 2 hub binary = a copy of the one 7800 runs on B3 (`s13-w3-bin/s13-w3-hub-7800-b3/os-hub`)
# 3 boot-watch: cold + 1 warm on a copy of B3 (port 8162)
# 4 hub 8161 on a B3 clone, 64 MiB guest residency → readiness → mint ×8 → residency-watch --rounds 3 → observability → stop
# Hubs run at NI 0 (measurements, comparable with H10); one hub at a time (rule 22).
setopt no_bg_nice
set -u
R=/Users/ueli/Documents/semio
H="$R/.🧬semio/🌐hub"; W="$R/.tmp-ticket/wp-h12"; G="$W/generated"; L="$H/s13-w3-logs"
TS="$R/🌎️hub/📦️packages/🟦️typescript"
CAT=${1:-$H/s13-w3-catalog-b3}
log() { echo "[h12-b3] $* $(date '+%F %T')"; }
log "waiting for the B3 publication"
until [ -e "$L/b3-publish.rc" ]; do sleep 30; done
[ "$(cat "$L/b3-publish.rc")" = 0 ] || { log "publication rc=$(cat "$L/b3-publish.rc") — stop"; exit 1; }
GEN=$(python3 -c "import json,sys;print(json.load(open(sys.argv[1]))['generationId'])" "$CAT/trusted-catalog/current.json")
P="$CAT/trusted-catalog/generations/$GEN/packages"
log "B3 published generation=$GEN packages=$(ls "$P" | tr '\n' ' ')"
PROBE="$W/target/release/h12-owned-probe"
nice -n 10 "$PROBE" chain "$P/puzzle/component.wasm" puzzle.2d.fixture puzzle.3d > "$G/probe-b3-puzzle.txt" 2>&1
for pair in note:note.document draw:drawing.document wfc:s.wfc.wfc2d; do nice -n 10 "$PROBE" chain "$P/${pair%%:*}/component.wasm" "${pair#*:}"; done > "$G/probe-b3-others.txt" 2>&1
log "probe done: $(/usr/bin/grep -c '^chain op' "$G/probe-b3-puzzle.txt" "$G/probe-b3-others.txt" | tr '\n' ' ')"
SRC="$H/s13-w3-bin/s13-w3-hub-7800-b3/os-hub"
rm -f "$H/s13-h12-bin/os-hub-b3"; cp "$SRC" "$H/s13-h12-bin/os-hub-b3" && codesign --force -s - "$H/s13-h12-bin/os-hub-b3" || exit 1
BIN="$H/s13-h12-bin/os-hub-b3"
log "hub binary $BIN ($(stat -f '%Sm' "$BIN"))"
(cd "$TS" && OS_HUB_BINARY="$BIN" bun ./📜️script.ts boot-watch --catalog-root "$CAT" --port 8162 --restarts 1 --interval-ms 500 > "$G/boot-watch-b3.txt" 2>&1)
log "boot-watch rc=$? $(/usr/bin/grep -E '^\[boot-watch\] boot [0-9]' "$G/boot-watch-b3.txt" | cut -c1-400 | tr '\n' ' ')"
NAME=s13-h12-hub-8161-b3
zsh "$W/h12-hub.sh" prepare $NAME "$CAT" "$BIN" || exit 1
zsh "$W/h12-hub.sh" start $NAME 8161 "$BIN" 67108864
started=$(date +%s)
until [ "$(curl -s -o /dev/null -w '%{http_code}' -m 5 http://127.0.0.1:8161/readyz)" = 200 ] || [ $(( $(date +%s) - started )) -gt 3600 ]; do sleep 1; done
log "8161 ready after $(( $(date +%s) - started )) s"
curl -s -m 10 http://127.0.0.1:8161/readyz > "$G/readyz-8161-b3.json"
bun "$R/.tmp-ticket/wp-h10/h10-probe.ts" mint http://127.0.0.1:8161 8 > "$G/mint-8161-b3.txt" 2>&1
log "mint $(tail -1 "$G/mint-8161-b3.txt")"
(cd "$TS" && bun ./📜️script.ts residency-watch --hub http://127.0.0.1:8161 --rounds 3 --settle-ms 60000 > "$G/residency-8161-b3.txt" 2>&1)
log "residency-watch rc=$?"
bun "$W/obs-read.ts" http://127.0.0.1:8161 "$G/observability-8161-b3.json" > "$G/observability-8161-b3.txt" 2>&1
log "observability $(head -1 "$G/observability-8161-b3.txt")"
zsh "$W/h12-hub.sh" stop $NAME
log "done"
