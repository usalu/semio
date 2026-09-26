#!/bin/zsh
# 🗄️ H11 item 2: restart on the same data root + backup/restore live drill (README procedure) on a running hub.
# usage: h11-restart-backup-drill.sh <root-name> <port> <binary>
set -u
cd /Users/ueli/Documents/semio
NAME=$1; PORT=$2; BIN=$3
H="/Users/ueli/Documents/semio/.🧬semio/🌐hub"; LOGS="$H/s13-h11-logs"; ROOT="$H/$NAME"
DRILL=/Users/ueli/Documents/semio/.tmp-ticket/wp-h10/h10-backup-drill.ts
hub() { zsh /Users/ueli/Documents/semio/.tmp-ticket/wp-h11/h11-hub.sh "$@"; }
now() { python3 -c 'import time;print(int(time.time()*1000))'; }
echo "=== restart leg $(date +%T)"
bun $DRILL seed http://127.0.0.1:$PORT "$LOGS/drill-$NAME-restart.json" s.note.note 20 | tail -2 | cut -c1-400 || exit 1
hub stop $NAME || exit 1
t=$(now); hub start $NAME $PORT $BIN | tail -1
hub wait-ready $PORT $t 900 || exit 1
bun $DRILL verify http://127.0.0.1:$PORT "$LOGS/drill-$NAME-restart.json" | tail -1 | cut -c1-600
echo "restart-verify-exit $?"
echo "=== backup leg $(date +%T)"
bun $DRILL seed http://127.0.0.1:$PORT "$LOGS/drill-$NAME-backup.json" s.note.note 20 | tail -2 | cut -c1-400 || exit 1
hub stop $NAME || exit 1
ARCHIVE="$H/s13-h11-backups/$NAME-$(date -u +%Y%m%dT%H%M%SZ).tar.gz"; mkdir -p "$H/s13-h11-backups"
t=$(now); tar -C "$H" -czf "$ARCHIVE" "$NAME" || exit 1
echo "archived $(du -sh "$ROOT" | cut -f1) → $(du -h "$ARCHIVE" | cut -f1) in $(( $(now) - t )) ms"
mv "$ROOT" "$ROOT-before-restore"
t=$(now); tar -C "$H" -xzf "$ARCHIVE" || exit 1
echo "restored in $(( $(now) - t )) ms; diff: $(diff -rq "$ROOT" "$ROOT-before-restore" | wc -l | tr -d ' ') differing paths"
t=$(now); hub start $NAME $PORT $BIN | tail -1
hub wait-ready $PORT $t 900 || exit 1
bun $DRILL verify http://127.0.0.1:$PORT "$LOGS/drill-$NAME-backup.json" | tail -1 | cut -c1-600
echo "backup-verify-exit $?"
echo "=== done $(date +%T)"
