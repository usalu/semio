"""🔎️ C10: signs a human in over HTTP and prints the directory event page they see (kind, space, seq) — the hub-side
truth a Home listing must match. usage: python3 c10-hub-events.py <hubUrl> <email> <password>"""
import json, sys, urllib.request, secrets
hub, email, password = sys.argv[1:4]
def call(method, path, body=None, token=None):
    req = urllib.request.Request(hub + path, method=method, data=None if body is None else json.dumps(body).encode(), headers={"content-type": "application/json", **({"authorization": f"Bearer {token}"} if token else {})})
    with urllib.request.urlopen(req, timeout=30) as r:
        return json.loads(r.read() or b"null")
session = call("POST", "/auth/sessions", {"schema": "semio.hub.auth.credential-sign-in/v1", "email": email, "password": password, "deviceInstanceId": secrets.token_hex(16), "clientClass": "browser"})
token = session.get("capability") or session.get("token") or session.get("session", {}).get("capability")
page = call("GET", "/directory/event-page/v1?after=0", token=token)
events = page.get("events", [])
print("through", page.get("through"), "hasMore", page.get("hasMore"), "events", len(events))
for e in events:
    body = e.get("event", e)
    print(e.get("seq"), body.get("kind") or body.get("type"), (body.get("spaceId") or body.get("space_id") or "")[:36], json.dumps(body)[:160])
spaces = call("GET", "/directory/spaces", token=token)
print("spaces", json.dumps(spaces)[:600])
