"""🩺️ Drives a locally served proctor the way the browser client does, for checking the operator verbs by hand.

    python domain_operator_probe.py <origin> <catalog id> register <learner id> <handle>
    python domain_operator_probe.py <origin> <catalog id> handle <handle>
    python domain_operator_probe.py <origin> <catalog id> leaderboard [learner id]

`register` signs a learner up under a pseudonym (the command targets the handle actor, whose id is the lowercase hex
of the normalized handle key — here the handle must already be typed normalized), `handle` recalls who holds a
handle, `leaderboard` reads the board. Prints the status and the JSON answer.
"""

import json
import sys
import urllib.error
import urllib.request
import uuid


def post(origin: str, path: str, body: dict) -> tuple[int, dict]:
    request = urllib.request.Request(origin + path, data=json.dumps(body).encode(), headers={"content-type": "application/json"}, method="POST")
    try:
        with urllib.request.urlopen(request, timeout=30) as answer:
            return answer.status, json.loads(answer.read() or b"null")
    except urllib.error.HTTPError as error:
        return error.code, json.loads(error.read() or b"null")


def query(origin: str, tenant: str, arguments: dict) -> tuple[int, object]:
    envelope = {"queryId": uuid.uuid4().hex, "kind": f"quiz.{arguments['type']}", "version": 1, "scope": tenant, "principal": {"kind": "anonymous"}, "arguments": list(json.dumps(arguments).encode()), "consistency": {"kind": "authority"}, "cursor": None}
    status, result = post(origin, "/queries", envelope)
    if status != 200:
        return status, result
    return status, json.loads(bytes(result["value"]).decode())


def main() -> None:
    origin, tenant, verb, *rest = sys.argv[1:]
    if verb == "register":
        learner, handle = rest
        command = {"type": "identify-learner", "id": uuid.uuid4().hex, "learner": learner, "identity": {"kind": "pseudonym", "handle": handle}}
        envelope = {
            "commandId": command["id"],
            "kind": "quiz.identify-learner",
            "version": 1,
            "target": {"tenant": tenant, "kind": "quiz-handle", "id": handle.lower().encode().hex()},
            "scope": tenant,
            "principal": {"kind": "anonymous"},
            "session": None,
            "device": None,
            "payload": list(json.dumps(command).encode()),
            "causalFrontier": None,
            "clientHlc": {"millis": 1_790_000_000_000, "counter": 0},
            "expectedRevision": None,
            "idempotencyKey": command["id"],
            "capabilityProof": None,
            "trace": {"trace_id": "probe", "span_id": "probe"},
        }
        status, outcome = post(origin, "/commands", envelope)
        print(status, json.dumps({key: outcome.get(key) for key in ("status", "reason")}, ensure_ascii=False))
    elif verb == "handle":
        print(*query(origin, tenant, {"type": "handle", "handle": rest[0]}))
    elif verb == "leaderboard":
        print(*query(origin, tenant, {"type": "leaderboard", **({"learner": rest[0]} if rest else {})}))
    else:
        raise SystemExit(__doc__)


if __name__ == "__main__":
    main()
