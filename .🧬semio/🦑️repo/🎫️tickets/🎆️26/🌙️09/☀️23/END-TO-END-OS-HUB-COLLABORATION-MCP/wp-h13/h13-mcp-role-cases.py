"""🧢️ H13 one-off: adds the principal-role cases to the os-mcp authenticated hub descriptor fixture (idempotent).
An agent session binds at its hub-capped role below its account's member row; a human session whose page role differs
from its member row, and any principal whose page role is above its member row, are refused. Receipts are
sha256(canonical page without `receiptSha256`), the rule `DirectorySpaceAdministrationPageV1::receipt_matches` checks.
usage: python3 h13-mcp-role-cases.py [--dry-run]"""
import hashlib, json, sys

PATH = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🔗️remote/🧫️fixtures/🔣️authenticated-hub-descriptor-index.json"
BINDING = "4b" * 32
READY_URIS = ["semio://workspace", "semio://workspace/artifacts", "semio://workspace/scopes/space-a/shared-doc/descriptor"]


def dumps(value):
    return json.dumps(value, separators=(",", ":"), ensure_ascii=False)


def session(kind):
    return {"status": 200, "canonicalBody": dumps({"schema": "semio.directory.session-authority.v1", "sessionBindingSha256": BINDING, "authorizationGeneration": 7, "userId": "user-a", "email": "a@example.invalid", "displayName": "A", "expiresAt": 900000, "sessionKind": kind})}


def page(template, access, space_role, row_role):
    unsigned = {"access": access}
    for key in ["schema", "sessionBindingSha256", "authorizationGeneration", "spaceId", "space", "members", "documents"]:
        unsigned[key] = json.loads(dumps(template[key]))
    unsigned["space"]["role"] = space_role
    unsigned["members"]["rows"][0]["role"] = row_role
    if access == "author":
        unsigned["invites"] = template["invites"]
        unsigned["capabilities"] = template["capabilities"]
    receipt = hashlib.sha256(dumps(unsigned).encode()).hexdigest()
    return {"status": 200, "canonicalBody": dumps({**unsigned, "receiptSha256": receipt})}


def refused():
    return {"state": "revoked", "errorCode": "PERMISSION_DENIED", "cacheAction": "invalidate", "resourceUris": [], "noTokenLeak": True}


fixture = json.load(open(PATH))
template = json.loads(fixture["cases"]["memberReady"]["responses"][1]["canonicalBody"])
assert hashlib.sha256(dumps({k: v for k, v in template.items() if k != "receiptSha256"}).encode()).hexdigest() == template["receiptSha256"], "receipt rule drifted"
cases = {
    "agentBelowMembership": {"tokenPresent": True, "responses": [session("agent"), page(template, "member", "spectator", "author")], "expected": {"state": "ready", "errorCode": None, "cacheAction": "publish", "resourceUris": READY_URIS, "noTokenLeak": True}},
    "humanBelowMembership": {"tokenPresent": True, "responses": [session("external"), page(template, "member", "spectator", "author")], "expected": refused()},
    "principalAboveMembership": {"tokenPresent": True, "responses": [session("agent"), page(template, "author", "author", "spectator")], "expected": refused()},
}
changed = any(fixture["cases"].get(name) != case for name, case in cases.items())
fixture["cases"].update(cases)
print(f"{'would change' if changed else 'unchanged'}: {', '.join(cases)}")
if changed and "--dry-run" not in sys.argv:
    open(PATH, "w").write(json.dumps(fixture, indent=2, ensure_ascii=False) + "\n")
    print("written")
