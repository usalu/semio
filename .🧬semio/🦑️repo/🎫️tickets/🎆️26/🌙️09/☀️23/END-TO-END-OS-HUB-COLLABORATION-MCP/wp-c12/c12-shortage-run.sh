#!/bin/zsh
# 🔌️ C12: the standalone link-shortage probe against one hub: two link proxies (A 8023/8025, B 8024/8026), two serves from S18's
# `ensureDevServe` (6525 → proxy A, 6526 → proxy B), `probe-c12-shortage.mjs`, then everything it started is stopped.
# usage: zsh c12-shortage-run.sh <tag> <hubPort> <spaceId> [cuts=5000,15000,60000]
TAG="$1"; HUB_PORT="$2"; SPACE="$3"; CUTS="${4:-5000,15000,60000}"
cd /Users/ueli/Documents/semio/.tmp-ticket/wp-c12 || exit 1
source ./env.sh
export NX_DAEMON=false C12_TYPISTS="${C12_TYPISTS:-A}" S_MATRIX_HUB="http://127.0.0.1:$HUB_PORT"
bun c12-link-proxy.ts 8023 "$HUB_PORT" 8025 > generated/$TAG-proxy-a.txt 2>&1 &
PA=$!
bun c12-link-proxy.ts 8024 "$HUB_PORT" 8026 > generated/$TAG-proxy-b.txt 2>&1 &
PB=$!
echo "START $(date +%T) tag=$TAG hub=$HUB_PORT proxies=$PA,$PB typists=$C12_TYPISTS"
bun c12-with-serve.ts 6525 http://127.0.0.1:8023 -- bun c12-with-serve.ts 6526 http://127.0.0.1:8024 -- bun probe-c12-shortage.mjs "$TAG" http://127.0.0.1:6525 http://127.0.0.1:6526 http://127.0.0.1:8025 http://127.0.0.1:8026 "$SPACE" "$CUTS"
RC=$?
kill $PA $PB 2>/dev/null
echo "EXIT rc=$RC $(date +%T)"
