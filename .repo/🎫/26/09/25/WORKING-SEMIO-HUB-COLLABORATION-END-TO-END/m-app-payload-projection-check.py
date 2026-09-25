"""[DEBUG] M: feeds the engine MCP App payload kit (rs projection wire) to rs `installProjection` through a running hub (`POST /sessions {kit}`) and compares what rs kept; transcript → m-app-payload-projection-check.log."""

import importlib.util
import json
import pathlib
import sys
import time
import urllib.request

HUB = sys.argv[1] if len(sys.argv) > 1 else "http://127.0.0.1:8091"
TICKET = pathlib.Path(__file__).resolve().parent
ENGINE = TICKET.parents[5] / "semio" / "client" / "bin" / "engine"
LOG = open(TICKET / "m-app-payload-projection-check.log", "w", encoding="utf-8")


def log(*parts):
    line = " ".join(str(p) for p in parts)
    print(line, flush=True)
    LOG.write(line + "\n")


def hub(method, path, body=None, token=None):
    request = urllib.request.Request(f"{HUB}{path}", method=method, data=json.dumps(body).encode() if body is not None else None, headers={"content-type": "application/json", **({"authorization": f"Bearer {token}"} if token else {})})
    with urllib.request.urlopen(request) as response:
        return json.loads(response.read())


def summary(kit):
    items = lambda value: value["items"] if isinstance(value, dict) else value or []
    typologies = items(kit["typologies"])
    designs = {d["id"]: d for t in typologies for d in items(t.get("designs"))}
    nakagin = designs["9a890dd4-0a9c-48ac-920a-9e62666465ef"]
    return {
        "typologies": len(typologies),
        "types": sum(len(items(t.get("types"))) for t in typologies),
        "designs": len(designs),
        "nakaginPieces": len(items(nakagin["pieces"])),
        "nakaginFixedPieces": sum(1 for p in items(nakagin["pieces"]) if p.get("pose")),
        "nakaginConnections": len(items(nakagin["connections"])),
        "files": len(items(kit.get("files"))),
    }


sys.path.insert(0, str(ENGINE))
spec = importlib.util.spec_from_file_location("engine_check", ENGINE / "main.py")
engine = importlib.util.module_from_spec(spec)
sys.modules["engine_check"] = engine
spec.loader.exec_module(engine)
ctx = type("Ctx", (), {"session": object()})()
engine.start_working_in_local_kit(str(TICKET.parents[5] / "semio" / "fixtures" / "stores" / "metabolism"), ctx)
engine.start_working_in_design("9a890dd4-0a9c-48ac-920a-9e62666465ef", ctx)
for show in (engine.show_diagram, engine.show_design):
    payload = show(ctx).structured_content
    log(f"### {show.__name__}: surface={payload['surface']} design={payload['design']['id']}")
    log("payload kit:", json.dumps(summary(payload["kit"])))
    token = hub("POST", "/auth/register", {"name": "Check", "email": f"check-{time.time_ns()}@semio.test", "password": "correct horse battery"})["token"]
    session = hub("POST", "/sessions", {"name": "Payload Projection", "kit": payload["kit"]}, token)
    installed = hub("GET", f"/sessions/{session['id']}/kit", token=token)
    log("after rs installProjection:", json.dumps(summary(installed["kit"])), "hash", installed["hash"])
