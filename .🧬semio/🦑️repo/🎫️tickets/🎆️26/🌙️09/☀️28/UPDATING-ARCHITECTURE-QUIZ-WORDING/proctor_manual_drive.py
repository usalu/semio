"""Drive a running dev proctor through its HTTP API the way the browser client does (design §9a).

Usage: python proctor_manual_drive.py <base-url> <tenant> <out-dir>
Writes every request/response pair as JSON into <out-dir> and prints a compact transcript.
"""
import json
import os
import sys
import urllib.error
import urllib.request

BASE, TENANT, OUT = sys.argv[1], sys.argv[2], sys.argv[3]
os.makedirs(OUT, exist_ok=True)
STEP = [0]


def ident(seed):
    return f"{seed:032x}"


def as_bytes(value):
    return list(json.dumps(value, separators=(",", ":")).encode("utf-8"))


def from_bytes(values):
    return json.loads(bytes(values).decode("utf-8"))


def call(method, path, body=None, headers=None):
    data = None if body is None else json.dumps(body).encode("utf-8")
    request = urllib.request.Request(BASE + path, data=data, method=method, headers={"content-type": "application/json", **(headers or {})})
    try:
        with urllib.request.urlopen(request) as response:
            return response.status, dict(response.headers), response.read().decode("utf-8")
    except urllib.error.HTTPError as error:
        return error.code, dict(error.headers), error.read().decode("utf-8")


def record(name, request, status, answer):
    STEP[0] += 1
    with open(os.path.join(OUT, f"{STEP[0]:02d}-{name}.json"), "w", encoding="utf-8") as handle:
        json.dump({"request": request, "status": status, "response": answer}, handle, indent=2, ensure_ascii=False)


def command(name, quiz_command):
    kind = quiz_command["type"]
    learner = quiz_command["learner"]
    envelope = {
        "commandId": quiz_command["id"],
        "kind": f"quiz.{kind}",
        "version": 1,
        "target": {"tenant": TENANT, "kind": "quiz-roster", "id": "roster"} if kind == "identify-learner" else {"tenant": TENANT, "kind": "quiz-learner", "id": learner},
        "scope": TENANT,
        "principal": {"kind": "anonymous"} if kind == "identify-learner" else {"kind": "user", "id": learner},
        "session": None,
        "device": None,
        "payload": as_bytes(quiz_command),
        "causalFrontier": None,
        "clientHlc": {"millis": 1727500000000, "counter": 0},
        "expectedRevision": None,
        "idempotencyKey": quiz_command["id"],
        "capabilityProof": None,
        "trace": {"trace_id": "manual", "span_id": "manual"},
    }
    status, _, text = call("POST", "/commands", envelope)
    outcome = json.loads(text)
    events = [from_bytes(record_["payload"]) for record_ in outcome.get("events", [])]
    record(name, {"envelope": envelope, "payloadDecoded": quiz_command}, status, {"outcome": outcome, "eventsDecoded": events})
    detail = outcome.get("reason", {}).get("detail")
    print(f"[DEBUG] {name}: {status} {outcome.get('status')} {[event['type'] for event in events]}{' ' + detail if detail else ''}")
    return outcome, events


def query(name, quiz_query):
    envelope = {"queryId": ident(STEP[0] + 9000), "kind": f"quiz.{quiz_query['type']}", "version": 1, "scope": TENANT, "principal": {"kind": "anonymous"}, "arguments": as_bytes(quiz_query), "consistency": {"kind": "authority"}, "cursor": None}
    status, _, text = call("POST", "/queries", envelope)
    result = json.loads(text)
    view = from_bytes(result["value"]) if status == 200 else result
    record(name, {"envelope": envelope, "argumentsDecoded": quiz_query}, status, {"result": {**result, "value": "<bytes>"} if status == 200 else result, "viewDecoded": view})
    print(f"[DEBUG] {name}: {status}")
    return view


status, _, text = call("GET", "/instance")
print(f"[DEBUG] instance: {status} {json.loads(text)['id']}")
catalog = query("query-catalog", {"type": "catalog"})
ada, run = ident(0xADA), ident(0x5EED)
command("identify-pseudonym", {"type": "identify-learner", "id": ident(1), "learner": ada, "identity": {"kind": "pseudonym", "handle": "  Ada   Lovelace "}})
command("identify-recall", {"type": "identify-learner", "id": ident(2), "learner": ident(0xB0B), "identity": {"kind": "name", "handle": "ada lovelace"}})
command("identify-anonymous", {"type": "identify-learner", "id": ident(3), "learner": ident(0xA11), "identity": {"kind": "anonymous"}})
quiz_id = catalog["quizzes"][0]["id"]
command("start-run", {"type": "start-run", "id": ident(4), "learner": ada, "run": run, "quiz": quiz_id})
sheet = query("query-run-open", {"type": "run", "run": run})["sheet"]
first = sheet["tasks"][0]
if first["kind"] == "sorting":
    answer = {"kind": "sorting", "order": [item["id"] for item in first["items"]]}
elif first["kind"] == "classification":
    answer = {"kind": "classification", "assignments": {item["id"]: first["categories"][0]["id"] for item in first["items"]}}
else:
    answer = {"kind": "matching", "assignments": {dimension["id"]: {item["id"]: index for index, item in enumerate(first["items"])} for dimension in first["dimensions"]}}
recorded = {"type": "record-answer", "id": ident(5), "learner": ada, "run": run, "task": first["id"], "answer": answer}
command("record-answer", recorded)
command("record-answer-retry", recorded)
command("submit-incomplete", {"type": "submit-run", "id": ident(6), "learner": ada, "run": run})
query("query-learner", {"type": "learner", "learner": ada})
query("query-leaderboard", {"type": "leaderboard"})
status, headers, body = call("GET", "/")
print(f"[DEBUG] site /: {status} {headers.get('Content-Type')} {headers.get('Cache-Control')}")
status, headers, body = call("GET", "/quiz/deep/link")
print(f"[DEBUG] site /quiz/deep/link: {status} {headers.get('Cache-Control')}")
