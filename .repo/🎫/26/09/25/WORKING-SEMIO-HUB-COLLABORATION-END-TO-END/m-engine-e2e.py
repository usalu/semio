"""[DEBUG] M: drives the local engine MCP (stdio) and the hub MCP (streamable HTTP) with the official Python MCP SDK against a running hub; transcript → engine-mcp-e2e.log."""

import asyncio
import json
import os
import pathlib
import sys
import time
import urllib.request

import httpx2
from mcp import Client, StdioServerParameters
from mcp.client.streamable_http import streamable_http_client

HUB = sys.argv[1] if len(sys.argv) > 1 else "http://127.0.0.1:8091"
TICKET = pathlib.Path(__file__).resolve().parent
ENGINE = TICKET.parents[5] / "semio" / "client" / "bin" / "engine"
LOG = open(TICKET / "engine-mcp-e2e.log", "w", encoding="utf-8")


def log(*parts):
    line = " ".join(str(p) for p in parts)
    print(line, flush=True)
    LOG.write(line + "\n")
    LOG.flush()


def hub(method, path, body=None, token=None):
    request = urllib.request.Request(f"{HUB}{path}", method=method, data=json.dumps(body).encode() if body is not None else None, headers={"content-type": "application/json", **({"authorization": f"Bearer {token}"} if token else {})})
    with urllib.request.urlopen(request) as response:
        text = response.read()
        return json.loads(text) if text else None


def structured(result):
    return result.structured_content if result.structured_content is not None else json.loads(result.content[0].text)


async def main():
    email, password = f"engine-{int(time.time())}@semio.test", "correct horse battery"
    person = hub("POST", "/auth/register", {"name": "Engine User", "email": email, "password": password})
    token = person["token"]
    metabolism = json.loads((TICKET / "m-metabolism-session.json").read_text())
    session = hub("POST", "/sessions", {"name": "Engine MCP E2E", "kit": metabolism["kit"]}, token)
    sid = session["id"]
    log("### hub session", json.dumps({k: session[k] for k in ("id", "name", "version", "hash")}))
    agent = hub("POST", "/auth/tokens", {"label": "sdk"}, token)["token"]

    log("\n### hub MCP over streamable HTTP (python mcp SDK)")
    async with httpx2.AsyncClient(headers={"authorization": f"Bearer {agent}"}, timeout=300) as http:
        async with Client(streamable_http_client(f"{HUB}/mcp", http_client=http)) as client:
            tools = await client.list_tools()
            log("tools:", [t.name for t in tools.tools])
            log("create_design:", json.dumps(structured(await client.call_tool("create_design", {"sessionId": sid, "name": "SDK Design"}))))
            log("read_kit designs:", json.dumps(structured(await client.call_tool("read_kit", {"sessionId": sid}))["designs"]))
            guide = await client.read_resource("semio://guide")
            log("guide:", guide.contents[0].text.splitlines()[0])

    log("\n### engine MCP over stdio (python mcp SDK) editing the hub session")
    env = {**os.environ, "UV_PROJECT_ENVIRONMENT": str(ENGINE / ".venv")}
    server = StdioServerParameters(command="uv", args=["--directory", str(ENGINE), "run", "--python", "3.14", "main.py", "--mcp-stdio"], env=env)
    async with Client(server) as engine:
        names = [t.name for t in (await engine.list_tools()).tools]
        log("engine tools:", len(names), [n for n in names if "hub" in n or "remote" in n])
        log("login_to_hub:", json.dumps(structured(await engine.call_tool("login_to_hub", {"serverUrl": HUB, "email": email, "password": password}))))
        log("list_hub_sessions:", json.dumps([s["name"] for s in structured(await engine.call_tool("list_hub_sessions", {"serverUrl": HUB}))["sessions"]]))
        opened = await engine.call_tool("start_working_in_remote_kit", {"serverUrl": HUB, "sessionId": sid})
        log("start_working_in_remote_kit:", json.dumps({k: v for k, v in structured(opened).items() if k in ("mode", "surface")}), "isError:", opened.is_error)
        current = structured(await engine.call_tool("read_current_kit", {}))
        design = next(d for d in current["designs"] if d["name"] == "SDK Design")
        log("design from hub:", design["id"])
        await engine.call_tool("start_working_in_design", {"id": design["id"]})
        log("start_new_design:", json.dumps(structured(await engine.call_tool("start_new_design", {"id": "019a0000-0000-7000-8000-00000000e2e1", "name": "Engine Design", "description": "pushed from the engine", "unit": "m", "icon": "", "image": "", "created_at": "", "updated_at": ""}))))
        types = {t["name"]: t for t in current["types"]}
        base, tambour = types["Base"], types["First Storey Tambour"]
        connector = lambda kind, name: next(c["id"] for c in kind["connectors"] if c["name"] == name)
        log("add_current_design_piece_with_plane:", json.dumps(structured(await engine.call_tool("add_current_design_piece_with_plane", {"id": "019a0000-0000-7000-8000-00000000e2e2", "name": "root", "kind_id": base["id"], "center_u": 0, "center_v": 0, "origin_x": 0, "origin_y": 0, "origin_z": 0, "x_axis_x": 1, "x_axis_y": 0, "x_axis_z": 0, "y_axis_x": 0, "y_axis_y": 1, "y_axis_z": 0}))))
        log("add_current_design_piece:", json.dumps(structured(await engine.call_tool("add_current_design_piece", {"id": "019a0000-0000-7000-8000-00000000e2e3", "name": "storey", "kind_id": tambour["id"]}))))
        log("add_current_design_connection:", json.dumps(structured(await engine.call_tool("add_current_design_connection", {"id": "019a0000-0000-7000-8000-00000000e2e4", "parent_piece_id": "019a0000-0000-7000-8000-00000000e2e2", "parent_connector_id": connector(base, "c0"), "child_piece_id": "019a0000-0000-7000-8000-00000000e2e3", "child_connector_id": connector(tambour, "b"), "rotation": 90, "u": 0, "v": 1, "shift": 0}))))
        finished = structured(await engine.call_tool("finish_working_in_kit", {}))
        log("finish_working_in_kit:", json.dumps(finished))
        log("logout_from_hub:", json.dumps(structured(await engine.call_tool("logout_from_hub", {"serverUrl": HUB}))))

    state = hub("GET", f"/sessions/{sid}/kit", token=token)
    designs = [d for t in state["kit"]["typologies"]["items"] for d in t.get("designs", {}).get("items", [])]
    pushed = next(d for d in designs if d["name"] == "Engine Design")
    log("\n### hub kit after the engine push", json.dumps({"version": state["version"], "hash": state["hash"], "designCount": len(designs), "Engine Design": {"pieces": pushed["pieces"]["items"], "connections": pushed["connections"]["items"]}}))
    history = hub("GET", f"/sessions/{sid}/operations?after=0", token=token)
    log("history:", json.dumps([{"version": r["version"], "participantKind": r["participantKind"], "clientId": r["clientId"], "field": r["query"].split("kit { ")[1].split("(")[0]} for r in history]))


asyncio.run(main())
