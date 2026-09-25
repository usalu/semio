#!/usr/bin/env bash
# [DEBUG] M: end-to-end drive of the hub semio MCP with the official MCP Inspector CLI; transcript → mcp-e2e.log
set -u
HUB="${HUB:-http://127.0.0.1:8091}"
DIR="$(cd "$(dirname "$0")" && pwd)"
LOG="$DIR/mcp-e2e.log"
PY="$DIR/../../../../../../semio/client/bin/engine/.venv/bin/python"
: > "$LOG"
say() { printf '\n### %s\n' "$*" | tee -a "$LOG"; }
run() { printf '$ %s\n' "$*" | sed -E 's/Bearer [0-9a-f]{64}/Bearer <agent-token>/' >> "$LOG"; "$@" 2>&1 | tee -a "$LOG"; }
EMAIL="alice-$(date +%s)@semio.test"
say "register a person and create an agent token"
LOGIN=$(curl -s -X POST "$HUB/auth/register" -H 'content-type: application/json' -d "{\"name\":\"Alice\",\"email\":\"$EMAIL\",\"password\":\"correct horse battery\"}")
echo "$LOGIN" | sed 's/"token":"[^"]*"/"token":"<redacted>"/' | tee -a "$LOG"
TOKEN=$(echo "$LOGIN" | "$PY" -c 'import json,sys; print(json.load(sys.stdin)["token"])')
AGENT=$(curl -s -X POST "$HUB/auth/tokens" -H "authorization: Bearer $TOKEN" -H 'content-type: application/json' -d '{"label":"claude"}')
echo "$AGENT" | sed 's/"token":"[^"]*"/"token":"<redacted>"/' | tee -a "$LOG"
AGENT_TOKEN=$(echo "$AGENT" | "$PY" -c 'import json,sys; print(json.load(sys.stdin)["token"])')
say "create a session from the full metabolism kit"
"$PY" - "$DIR/../../../../../../semio/fixtures/stores/metabolism/wip/initialKit" > "$DIR/m-metabolism-session.json" <<'EOF'
import json, pathlib, sys
root = pathlib.Path(sys.argv[1])
kit = json.loads((root / "kit.semio.json").read_text())
index = json.loads((root / "index.semio.json").read_text())
files = {e["id"]: json.loads((root / e["file"]).read_text()) for key in ("types", "designs") for e in index[key]}
for typology in kit["typologies"]["items"]:
    for key in ("types", "designs"):
        block = typology.get(key) or {"items": []}
        block["items"] = [files.get(item["id"], item) for item in block["items"]]
print(json.dumps({"name": "Metabolism (MCP E2E)", "kit": kit}))
EOF
SESSION=$(curl -s -X POST "$HUB/sessions" -H "authorization: Bearer $TOKEN" -H 'content-type: application/json' --data-binary "@$DIR/m-metabolism-session.json")
echo "$SESSION" | tee -a "$LOG"
SID=$(echo "$SESSION" | "$PY" -c 'import json,sys; print(json.load(sys.stdin)["id"])')
say "a human collaborator listens on the session websocket"
"$PY" - "$HUB" "$SID" "$TOKEN" > "$DIR/m-mcp-e2e-ws.log" 2>&1 <<'EOF' &
import asyncio, json, sys, websockets
async def main():
    url = sys.argv[1].replace("http", "ws", 1) + f"/sessions/{sys.argv[2]}/ws?token={sys.argv[3]}&clientId=alice-browser&client=sketchpad"
    async with websockets.connect(url, max_size=None) as ws:
        async for text in ws:
            m = json.loads(text)
            if m.get("type") == "operation":
                m["query"] = m["query"][:160] + "…"
            print("[DEBUG] ws ←", json.dumps(m), flush=True)
asyncio.run(main())
EOF
WS_PID=$!
sleep 2
INSPECT=(npx --yes @modelcontextprotocol/inspector --cli "$HUB/mcp" --transport http --header "Authorization: Bearer $AGENT_TOKEN")
say "inspector: tools/list (strict schema portability check)"
run "${INSPECT[@]}" --method tools/list --strict
say "inspector: resources/list + resources/templates/list + guide"
run "${INSPECT[@]}" --method resources/list
run "${INSPECT[@]}" --method resources/templates/list
run "${INSPECT[@]}" --method resources/read --uri semio://guide
say "inspector: prompts"
run "${INSPECT[@]}" --method prompts/list
run "${INSPECT[@]}" --method prompts/get --prompt-name design_assistant --prompt-args "sessionId=$SID" "goal=Stack capsules on a base"
say "inspector: read tools"
run "${INSPECT[@]}" --method tools/call --tool-name list_sessions
run "${INSPECT[@]}" --method tools/call --tool-name read_kit --tool-arg "sessionId=$SID"
TYPES=$("${INSPECT[@]}" --method tools/call --tool-name list_types --tool-arg "sessionId=$SID" 2>/dev/null)
echo "$TYPES" | head -c 1500 >> "$LOG"
BASE=$(echo "$TYPES" | "$PY" -c 'import json,sys; print(next(t["id"] for t in json.load(sys.stdin)["structuredContent"]["types"] if t["name"]=="Base"))')
TAMBOUR=$(echo "$TYPES" | "$PY" -c 'import json,sys; print(next(t["id"] for t in json.load(sys.stdin)["structuredContent"]["types"] if t["name"]=="First Storey Tambour"))')
say "inspector: write tools (each one hub operation, broadcast live)"
DESIGN=$("${INSPECT[@]}" --method tools/call --tool-name create_design --tool-arg "sessionId=$SID" "name=Agent Tower" "description=Built by an agent over MCP" "unit=m" 2>&1 | tee -a "$LOG" | "$PY" -c 'import json,sys; print(json.load(sys.stdin)["structuredContent"]["designId"])')
ROOT=$("${INSPECT[@]}" --method tools/call --tool-name add_piece --tool-arg "sessionId=$SID" "designId=$DESIGN" "typeId=$BASE" "name=root" 'position={"plane":{"origin":{"x":0,"y":0,"z":0},"xAxis":{"x":1,"y":0,"z":0},"yAxis":{"x":0,"y":1,"z":0}},"center":{"u":0,"v":0}}' 2>&1 | tee -a "$LOG" | "$PY" -c 'import json,sys; print(json.load(sys.stdin)["structuredContent"]["pieceId"])')
STOREY=$("${INSPECT[@]}" --method tools/call --tool-name add_piece --tool-arg "sessionId=$SID" "designId=$DESIGN" "typeId=$TAMBOUR" "name=storey 1" "parent={\"pieceId\":\"$ROOT\",\"connector\":\"c0\",\"childConnector\":\"b\",\"rotation\":90}" 2>&1 | tee -a "$LOG" | "$PY" -c 'import json,sys; print(json.load(sys.stdin)["structuredContent"]["pieceId"])')
LOOSE=$("${INSPECT[@]}" --method tools/call --tool-name add_piece --tool-arg "sessionId=$SID" "designId=$DESIGN" "typeId=$TAMBOUR" "name=storey 2" "parent={\"pieceId\":\"$STOREY\",\"connector\":\"t\",\"childConnector\":\"b\"}" 2>&1 | tee -a "$LOG" | "$PY" -c 'import json,sys; print(json.load(sys.stdin)["structuredContent"]["pieceId"])')
run "${INSPECT[@]}" --method tools/call --tool-name connect_pieces --tool-arg "sessionId=$SID" "designId=$DESIGN" "parentPieceId=$ROOT" "parentConnector=c1" "childPieceId=$LOOSE" "childConnector=t" "rotation=180"
run "${INSPECT[@]}" --method tools/call --tool-name update_piece --tool-arg "sessionId=$SID" "designId=$DESIGN" "pieceId=$STOREY" "name=first storey" "description=renamed by the agent"
run "${INSPECT[@]}" --method tools/call --tool-name read_design --tool-arg "sessionId=$SID" "designId=$DESIGN"
run "${INSPECT[@]}" --method tools/call --tool-name list_participants --tool-arg "sessionId=$SID"
run "${INSPECT[@]}" --method tools/call --tool-name get_history --tool-arg "sessionId=$SID" "limit=2"
run "${INSPECT[@]}" --method tools/call --tool-name run_query --tool-arg "sessionId=$SID" 'query={ session { stores { edges { node { wip { theKit { kit { name hash } } } } } } } }'
say "the human sees the same kit (GET /sessions/{id}/kit)"
curl -s "$HUB/sessions/$SID/kit" -H "authorization: Bearer $TOKEN" | "$PY" -c 'import json,sys; s=json.load(sys.stdin); print(json.dumps({"version": s["version"], "hash": s["hash"], "designs": [d["name"] for t in s["kit"]["typologies"]["items"] for d in t.get("designs",{}).get("items",[])]}))' | tee -a "$LOG"
sleep 2
kill "$WS_PID" 2>/dev/null
say "websocket transcript of the human collaborator"
cat "$DIR/m-mcp-e2e-ws.log" | tee -a "$LOG"
