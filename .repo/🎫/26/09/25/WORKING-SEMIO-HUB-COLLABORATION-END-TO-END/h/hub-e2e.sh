#!/usr/bin/env bash
# 🌐 Hub Protocol v1 end-to-end run against a live semio-hub (scratch, ticket folder). Usage: `bash hub-e2e.sh [base]`.
set -euo pipefail
BASE=${1:-http://127.0.0.1:8080}
HERE=$(cd "$(dirname "$0")" && pwd)
ROOT=$(git -C "$HERE" rev-parse --show-toplevel)
FIXTURE="$ROOT/semio/fixtures/stores/metabolism/wip/initialKit/kit.semio.json"
STAMP=$(date +%s)
BODY=$(mktemp)

call() {
  local method=$1 path=$2 token=${3:-} data=${4:-}
  local args=(-sS -o "$BODY" -w '%{http_code}' -X "$method" "$BASE$path")
  [[ -n $token ]] && args+=(-H "authorization: Bearer $token")
  [[ -n $data ]] && args+=(-H 'content-type: application/json' --data-binary "$data")
  STATUS=$(curl "${args[@]}")
  echo "\$ curl -X $method $path${token:+ (bearer ${token:0:8}…)} -> $STATUS $(head -c 360 "$BODY")"
}

json() { jq -r "$1" "$BODY"; }

call GET /health
call POST /auth/register "" "{\"name\":\"Alice\",\"email\":\"alice-$STAMP@semio.test\",\"password\":\"correct horse battery\"}"
ALICE=$(json .token)
call POST /auth/register "" "{\"name\":\"Bob\",\"email\":\"bob-$STAMP@semio.test\",\"password\":\"correct horse battery\"}"
BOB=$(json .token)
call POST /auth/login "" "{\"email\":\"alice-$STAMP@semio.test\",\"password\":\"wrong password\"}"
call POST /auth/login "" "{\"email\":\"ALICE-$STAMP@semio.test\",\"password\":\"correct horse battery\"}"
call GET /auth/me "$ALICE"
call POST /auth/tokens "$ALICE" '{"label":"claude-code"}'

CREATE=$(mktemp)
jq -c -n --slurpfile kit "$FIXTURE" '{name: "Metabolism E2E", kit: $kit[0]}' > "$CREATE"
call POST /sessions "$ALICE" "@$CREATE"
SESSION=$(json .id)
HASH0=$(json .hash)
call GET "/sessions/$SESSION/kit" "$BOB"
call GET "/sessions/$SESSION/kit" "$ALICE"
echo "[e2e] kit: version $(json .version), hash $(json .hash), name $(json .kit.name), typologies $(json '.kit.typologies.items | length'), designs $(json '[.kit.typologies.items[].designs.items[]] | length')"
call POST "/sessions/$SESSION/shares" "$ALICE" '{"role":"editor","label":"studio"}'
SHARE=$(json .token)
call POST "/shares/$SHARE/join" "$BOB" '{}'
call GET "/sessions/$SESSION/members" "$BOB"
call GET /sessions "$BOB"

echo "[e2e] websockets"
bun "$HERE/hub-e2e-ws.ts" "$BASE" "$SESSION" "$ALICE" "$BOB"

call GET "/sessions/$SESSION/operations?after=0" "$BOB"
echo "[e2e] history: $(jq -c '[.[] | {version, participantKind, clientId, hash}]' "$BODY")"
HASH2=$(jq -r '.[-1].hash' "$BODY")
call GET "/sessions/$SESSION/kit/at/0" "$BOB"
echo "[e2e] kit at 0 hash $(json .hash) (created $HASH0)"
call GET "/sessions/$SESSION/kit/at/2" "$BOB"
echo "[e2e] kit at 2 hash $(json .hash) (last operation $HASH2)"
call POST "/sessions/$SESSION/graphql" "$BOB" '{"query":"query { session { stores { edges { node { wip { theKit { kit { name hash } } } } } } } }"}'
call POST "/sessions/$SESSION/graphql" "$BOB" '{"query":"mutation($storeId: ID!, $changeId: ID!) { session { store(id: $storeId) { theKit { unsavedChange(id: $changeId) { kit { rename(newName: \"x\") { ok } } } } } } }"}'
call GET "/sessions/$SESSION/presence" "$BOB"
call DELETE "/sessions/$SESSION" "$BOB"
call DELETE "/sessions/$SESSION" "$ALICE"
call GET "/sessions/$SESSION" "$ALICE"
call POST /auth/logout "$ALICE"
call GET /auth/me "$ALICE"
rm -f "$BODY" "$CREATE"
