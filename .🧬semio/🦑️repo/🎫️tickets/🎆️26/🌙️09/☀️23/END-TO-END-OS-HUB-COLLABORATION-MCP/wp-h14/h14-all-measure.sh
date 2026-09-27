#!/bin/zsh
# 📏️ H14 item 3 on the all-package catalog; launch detached:
#   python3 .tmp-ticket/wp-w2/w2-detach.py .🧬semio/🌐hub/s14-h14-logs/all-measure.txt zsh .tmp-ticket/wp-h14/h14-all-measure.sh <catalog-root> <binary> [residency-bytes]
# 1 boot-watch: cold + 2 warm on a copy of the catalog (port 8162) → /readyz timings with all packages
# 2 hub 8161 on a clone (residency default or <residency-bytes>) → readiness → residency-watch --rounds 2 (every creatable kind:
#   creation latency per package, compiles/admitted/released per round, footprint, RSS by pid) → observability → stop
# One hub at a time (preamble 14 rule 6), NI 0 for comparable timings.
setopt no_bg_nice
set -u
R=/Users/ueli/Documents/semio
H="$R/.🧬semio/🌐hub"; W="$R/.tmp-ticket/wp-h14"; G="$W/generated"; D="$H/s14-h14-logs"; mkdir -p "$D"
TS="$R/🌎️hub/📦️packages/🟦️typescript"
CAT=$1; BIN=$2; BYTES=${3:-}
log() { echo "[h14-all] $* $(date '+%F %T')"; }
GEN=$(python3 -c "import json,sys;print(json.load(open(sys.argv[1]))['generationId'])" "$CAT/trusted-catalog/current.json")
log "catalog $CAT generation $GEN packages $(ls "$CAT/trusted-catalog/generations/$GEN/packages" | wc -l | tr -d ' ') binary $BIN ($(stat -f '%Sm' "$BIN"))"
(cd "$TS" && OS_HUB_BINARY="$BIN" bun ./📜️script.ts boot-watch --catalog-root "$CAT" --port 8162 --restarts 2 --interval-ms 500 > "$D/boot-watch-all.txt" 2>&1)
log "boot-watch rc=$? $(/usr/bin/grep -E '^\[boot-watch\] boot [0-9]' "$D/boot-watch-all.txt" | cut -c1-400 | tr '\n' ' ')"
NAME=s14-h14-hub-8161-all
zsh "$W/h14-hub.sh" prepare $NAME "$CAT" "$BIN" || exit 1
zsh "$W/h14-hub.sh" start $NAME 8161 "$BIN" $BYTES
PID=$(cat "$H/$NAME.pid")
started=$(date +%s)
until [ "$(curl -s -o /dev/null -w '%{http_code}' -m 5 http://127.0.0.1:8161/readyz)" = 200 ] || [ $(( $(date +%s) - started )) -gt 3600 ]; do sleep 1; done
log "8161 ready after $(( $(date +%s) - started )) s (pid $PID)"
curl -s -m 10 http://127.0.0.1:8161/readyz > "$D/readyz-8161-all.json"
(cd "$TS" && bun ./📜️script.ts residency-watch --hub http://127.0.0.1:8161 --pid $PID --rounds 2 --settle-ms 60000 > "$D/residency-8161-all.txt" 2>&1)
log "residency-watch rc=$? $(/usr/bin/grep '^\[acceptance\]' "$D/residency-8161-all.txt" | head -1 | cut -c1-300)"
OS_HUB_PROBE_EMAIL=user1@semio.dev OS_HUB_PROBE_PASSWORD="$(sed -n 's/.*user1@semio.dev|User One|\([^"|]*\).*/\1/p' "$R/.tmp-ticket/wp-w3/w3-restart-7800.sh" | head -1)" bun "$W/h14-obs-read.ts" http://127.0.0.1:8161 "$D/observability-8161-all.json" > "$D/observability-8161-all.txt" 2>&1
log "observability $(head -1 "$D/observability-8161-all.txt")"
zsh "$W/h14-hub.sh" stop $NAME
log "done"
