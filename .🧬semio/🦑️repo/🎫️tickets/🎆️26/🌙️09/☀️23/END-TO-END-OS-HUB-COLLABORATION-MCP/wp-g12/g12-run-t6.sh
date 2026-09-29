#!/bin/zsh
# 🧭️ G12 t6 wave (session 15): (1) 7800 battery — participant, quartet, security, refused-relay, untrusted-content, hub coverage;
# (2) own hub 8031 on a clone of the t6 catalog with fresh users + ONE serve 6530 joined to it → user path en + de → serve and hub
# stopped, root deleted; (3) durability on its own self-booting hub 8032 (t6 binary + the 8031 root clone of the catalog).
# usage: zsh g12-run-t6.sh <tag>
# env: SEMIO_OS_MCP_BIN (default s14-g12-bin/semio-os-mcp-s15c), G12_T6_CATALOG (s14-w4-catalog-t6),
#      G12_T6_HUB_BIN (s14-w4-bin/s14-w4-hub-7800-t6/os-hub), G12_STEPS ("7800 user durability").
set -u
TAG="${1:?tag}"
H="/Users/ueli/Documents/semio/.🧬semio/🌐hub"; W="/Users/ueli/Documents/semio/.tmp-ticket/wp-g12"
export SEMIO_OS_MCP_BIN="${SEMIO_OS_MCP_BIN:-$H/s14-g12-bin/semio-os-mcp-s15c}"
CAT="${G12_T6_CATALOG:-s14-w4-catalog-t6}"; HUBBIN="${G12_T6_HUB_BIN:-$H/s14-w4-bin/s14-w4-hub-7800-t6/os-hub}"
STEPS=" ${G12_STEPS:-7800 user durability} "
LOG="$H/s14-g12-logs/run-t6-$TAG.txt"
say() { echo "$(date +%H:%M:%S) $*" | tee -a "$LOG"; }
say "run-t6 $TAG gateway=$SEMIO_OS_MCP_BIN catalog=$CAT hub-bin=$HUBBIN steps=$STEPS"
test -x "$SEMIO_OS_MCP_BIN" || { say "no gateway binary"; exit 1; }
if [[ "$STEPS" == *" 7800 "* ]]; then
  G12_ONLY="participant quartet security refused untrusted coverage" zsh "$W/g12-battery.sh" http://127.0.0.1:7800 "$H/s13-w3-state-7800" "$TAG"
  say "7800 battery rc=$? (summary: s14-g12-logs/battery-$TAG/summary.txt)"
fi
if [[ "$STEPS" == *" user "* ]]; then
  test -x "$HUBBIN" && test -d "$H/$CAT" || { say "no t6 hub binary or catalog"; exit 1; }
  ROOT="s14-g12-hub-8031-$TAG"
  { G12_CATALOG="$CAT" zsh "$W/g12-hub.sh" prepare "$ROOT" "$HUBBIN" && zsh "$W/g12-hub.sh" start "$ROOT" 8031 "$HUBBIN"; } >> "$LOG" 2>&1 || { say "hub 8031 did not start"; exit 1; }
  for i in $(seq 1 120); do curl -sf -o /dev/null http://127.0.0.1:8031/readyz && break; sleep 5; done
  curl -sf -o /dev/null http://127.0.0.1:8031/readyz || { say "hub 8031 not ready after 600 s"; zsh "$W/g12-hub.sh" stop "$ROOT" >> "$LOG" 2>&1; exit 1; }
  say "hub 8031 ready"
  bun "$W/g12-serve.ts" 6530 http://127.0.0.1:8031 >> "$LOG" 2>&1; say "serve 6530 rc=$?"
  mkdir -p "$H/s14-g12-hub-8031-state"
  G12_ONLY="user-path" zsh "$W/g12-battery.sh" http://127.0.0.1:8031 "$H/s14-g12-hub-8031-state" "$TAG-8031" http://127.0.0.1:6530
  say "user path rc=$? (summary: s14-g12-logs/battery-$TAG-8031/summary.txt)"
  SP=$(lsof -nP -tiTCP:6530 -sTCP:LISTEN 2>/dev/null | head -1)
  if [ -n "$SP" ]; then
    PG=$(ps -o pgid= -p "$SP" | tr -d ' ')
    [ -n "$PG" ] && [ "$PG" != "$(ps -o pgid= -p $$ | tr -d ' ')" ] && kill -TERM -- "-$PG" 2>/dev/null
    say "serve 6530 stopped (pid $SP, group $PG)"
  fi
  zsh "$W/g12-hub.sh" stop "$ROOT" >> "$LOG" 2>&1; say "hub 8031 stopped"
fi
if [[ "$STEPS" == *" durability "* ]]; then
  ROOT="s14-g12-hub-8031-$TAG"
  test -d "$H/$ROOT" || { G12_CATALOG="$CAT" zsh "$W/g12-hub.sh" prepare "$ROOT" "$HUBBIN" >> "$LOG" 2>&1 || { say "durability root not prepared"; exit 1; }; }
  G12_ONLY="durability" G12_HUB_BINARY="$HUBBIN" G12_HUB_ROOT="$H/$ROOT" zsh "$W/g12-battery.sh" http://127.0.0.1:7800 "$H/s13-w3-state-7800" "$TAG-durability"
  say "durability rc=$? (summary: s14-g12-logs/battery-$TAG-durability/summary.txt)"
fi
rm -rf "$H/s14-g12-hub-8031-$TAG" "$H/s14-g12-hub-8031-$TAG.pid"; say "run-t6 $TAG done (8031 root deleted)"
