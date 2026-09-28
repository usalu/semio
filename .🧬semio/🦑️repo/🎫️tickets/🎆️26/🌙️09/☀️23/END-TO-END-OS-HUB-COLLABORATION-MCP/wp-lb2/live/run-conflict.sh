#!/bin/zsh
# ⚔️ LB2 item 3: LD's same-field conflict probe on an own vigilant hub with the committed-DOMAIN-op criterion.
# up:    boots hub 8130 (fresh root, 7800's current binary + B3 catalog clone, OS_HUB_MERGE_POLICY=vigilant), link proxy 8131→8130 (control 8132),
#        serves 6630 (A → 8130) and 6631 (B → 8131), all detached; waits for the hub's ready.json.
# probe <tag> <locale> [policy]: runs probe-s12-conflict.mjs (committed-DOMAIN-op census on a third document socket).
# down:  stops every pid this script started (recorded in .🧬semio/🌐hub/s14-lb2-conflict-pids.txt).
set -u
W=/Users/ueli/Documents/semio/.tmp-ticket/wp-lb2/live
H="/Users/ueli/Documents/semio/.🧬semio/🌐hub"
NAME=conflict-vigilant
STATE="$H/s14-lb2-$NAME-state"
DATA="$H/s14-lb2-$NAME"
PIDS="$H/s14-lb2-conflict-pids.txt"
LOGS="$H/s14-lb2-logs"
mkdir -p "$LOGS"
case "$1" in
  up)
    OS_HUB_MERGE_POLICY=vigilant zsh "$W/c10-hub.sh" 8130 "$H/s13-w3-catalog-b3" "$H/s13-w3-bin/s13-w3-hub-7800-b3/os-hub" "$NAME" || exit 1
    echo "hold $(cat "$STATE/detach.txt")" >> "$PIDS"
    echo "proxy $(python3 "$W/c10-detach.py" "$LOGS/proxy-8131.txt" bun "$W/c10-link-proxy.ts" 8131 8130 8132)" >> "$PIDS"
    echo "serveA $(python3 "$W/c10-detach.py" "$LOGS/serve-6630.txt" zsh "$W/serve.sh" s 6630 http://127.0.0.1:8130 dev)" >> "$PIDS"
    echo "serveB $(python3 "$W/c10-detach.py" "$LOGS/serve-6631.txt" zsh "$W/serve.sh" s 6631 http://127.0.0.1:8131 dev)" >> "$PIDS"
    until [ -f "$STATE/ready.json" ]; do sleep 5; done
    echo "hub ready $(date '+%H:%M:%S')"; cat "$PIDS"
    ;;
  probe)
    TAG="$2"; LOCALE="$3"
    cd "$W" && S_CONFLICT_LOCALE="$LOCALE" S_CONFLICT_POLICY=vigilant S_CONFLICT_CONTROL=http://127.0.0.1:8132 S_MATRIX_HUB=http://127.0.0.1:8130 \
      S_MATRIX_ADMIN_FILE="$STATE/admin-capability.json" \
      bun probe-s12-conflict.mjs "$TAG" http://127.0.0.1:6630 http://127.0.0.1:6631 -
    ;;
  down)
    while read -r label pid; do
      for child in $(pgrep -P "$pid" 2>/dev/null); do for grandchild in $(pgrep -P "$child" 2>/dev/null); do kill -TERM "$grandchild" 2>/dev/null; done; kill -TERM "$child" 2>/dev/null; done
      kill -TERM "$pid" 2>/dev/null && echo "stopped $label $pid"
    done < "$PIDS"
    touch "$STATE/stop" 2>/dev/null
    ;;
esac
