#!/usr/bin/env bash
# ⏱️ Hub latency benchmark (scratch, ticket folder): creates a session from a kit body, applies operations over HTTP and reads the kit back. Usage: `bash hub-perf.sh <base> <body.json> [ops]`.
set -euo pipefail
BASE=$1
BODY_FILE=$2
OPS=${3:-10}
OUT=$(mktemp)
TOKEN=$(curl -s -X POST "$BASE/auth/register" -H 'content-type: application/json' -d "{\"name\":\"Perf\",\"email\":\"perf-$(date +%s%N)@semio.test\",\"password\":\"correct horse battery\"}" | jq -r .token)
curl -s -o "$OUT" -w "create %{http_code} %{time_total}s (body $(stat -c %s "$BODY_FILE") B)\n" -X POST "$BASE/sessions" -H "authorization: Bearer $TOKEN" -H 'content-type: application/json' --data-binary @"$BODY_FILE"
SESSION=$(jq -r .id "$OUT")
RENAME='mutation($storeId: ID!, $changeId: ID!, $name: String!) { session { store(id: $storeId) { theKit { unsavedChange(id: $changeId) { kit { rename(newName: $name) { ok errors { message } } } } } } } }'
DESIGN='mutation($storeId: ID!, $changeId: ID!, $id: ID!, $name: String!) { session { store(id: $storeId) { theKit { unsavedChange(id: $changeId) { kit { createDesign(id: $id, name: $name) { ok errors { message } } } } } } } }'
total=0
for i in $(seq 1 "$OPS"); do
  if (( i % 2 )); then
    payload=$(jq -cn --arg q "$RENAME" --arg n "Perf $i" --arg id "perf-$SESSION-$i" '{operationId:$id, clientId:"perf", query:$q, variables:{name:$n}}')
  else
    payload=$(jq -cn --arg q "$DESIGN" --arg n "Perf Design $i" --arg id "perf-$SESSION-$i" --arg d "$(python3 -c 'import uuid;print(uuid.uuid4())')" '{operationId:$id, clientId:"perf", query:$q, variables:{id:$d, name:$n}}')
  fi
  t=$(curl -s -o "$OUT" -w "%{time_total}" -X POST "$BASE/sessions/$SESSION/operations" -H "authorization: Bearer $TOKEN" -H 'content-type: application/json' -d "$payload")
  echo "op $i $(jq -r '.version // .error' "$OUT") ${t}s"
  total=$(python3 -c "print($total+$t)")
done
echo "ops mean $(python3 -c "print(round($total/$OPS,4))")s"
curl -s -o "$OUT" -w "GET kit %{http_code} %{time_total}s %{size_download}B\n" "$BASE/sessions/$SESSION/kit" -H "authorization: Bearer $TOKEN"
curl -s -o /dev/null -w "delete %{http_code}\n" -X DELETE "$BASE/sessions/$SESSION" -H "authorization: Bearer $TOKEN"
rm -f "$OUT"
