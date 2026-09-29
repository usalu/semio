#!/usr/bin/env python3
"""🕒️ SH2 P2: authors the language-neutral document-activity cases of the directory read model
(`🧫️fixtures/📇️directory/🕒️document-activity-v1.json`) into the P2 stage. The expected activity is computed HERE, by an
independent Python reading of the documented rule (a human's checkpoint publication of an announced document of a folded
space records `recordedAtMs` + the plain user id; admin/system publications, unannounced documents and unknown spaces change
nothing; replay is idempotent), never by the Rust or TypeScript fold it judges."""
import copy, json, os, re

REPO = "/Users/ueli/Documents/semio"
STAGE = os.path.join(REPO, ".🧬semio/🌐hub/s14-sh2-p2-stage")
FIXTURES = os.path.join(REPO, "🧰️framework/🛍️products/💻️os/🧫️fixtures/📇️directory")
TARGET = os.path.join(STAGE, "🧰️framework/🛍️products/💻️os/🧫️fixtures/📇️directory/🕒️document-activity-v1.json")

authority = json.load(open(os.path.join(FIXTURES, "🛡️artifact-authority.json"), encoding="utf-8"))
descriptor_fixture = json.load(open(os.path.join(FIXTURES, "🪪️document-descriptor.json"), encoding="utf-8"))["valid"]
SPACE = "space-a"


def event(seq, actor, body, space_id=None, user_id=None):
    row = {"seq": seq, "id": f"event-{seq}", "hlc": {"physicalMs": 1_800_000_000_000 + seq * 60_000, "logical": 0}, "actor": actor}
    if space_id is not None:
        row["spaceId"] = space_id
    if user_id is not None:
        row["userId"] = user_id
    row["body"] = body
    row["recordedAtMs"] = 1_800_000_000_000 + seq * 60_000
    return row


def user(user_id, session):
    return {"kind": "user", "id": f"user:{user_id}#{session}"}


def checkpoint(space_id, document_id, ordinal):
    body = copy.deepcopy(authority["checkpoint"])
    body["scope"] = {"spaceId": space_id, "documentId": document_id}
    body["baselineFrontier"]["documentId"] = document_id
    body["baselineFrontier"]["headEditOrdinal"] = ordinal
    body["baselineFrontier"]["headEditId"] = f"edit:{ordinal}"
    body["baselineFrontier"]["lastCommitSeq"] = ordinal
    body["publishedAtMs"] = ordinal
    for blob in ("pack", "spr"):
        body[blob].pop("storageKey", None)
    return {"kind": "artifact.checkpoint-published", "checkpoint": body}


def announced(document_id):
    descriptor = copy.deepcopy(descriptor_fixture)
    descriptor["spaceId"], descriptor["documentId"] = SPACE, document_id
    return {"kind": "document.announced", "descriptor": descriptor}


SYSTEM, ADMIN = {"kind": "system", "id": "system"}, {"kind": "admin", "id": "admin:ops"}
EVENTS = [
    event(1, SYSTEM, {"kind": "user.created", "userId": "u-ada", "email": "ada@semio.dev", "displayName": "Ada"}, user_id="u-ada"),
    event(2, SYSTEM, {"kind": "user.created", "userId": "u-ben", "email": "ben@semio.dev", "displayName": "Ben"}, user_id="u-ben"),
    event(3, user("u-ada", "s1"), {"kind": "space.created", "spaceId": SPACE, "name": "Werkstatt", "spaceKind": "studio", "visibility": "private", "ownerUserId": "u-ada"}, SPACE),
    event(4, user("u-ada", "s1"), {"kind": "member.upserted", "spaceId": SPACE, "userId": "u-ada", "role": "author"}, SPACE, "u-ada"),
    event(5, user("u-ada", "s1"), {"kind": "member.upserted", "spaceId": SPACE, "userId": "u-ben", "role": "author"}, SPACE, "u-ben"),
    event(6, user("u-ada", "s1"), announced("doc-plan"), SPACE),
    event(7, user("u-ada", "s1"), announced("doc-note"), SPACE),
    event(8, SYSTEM, checkpoint(SPACE, "doc-plan", 1), SPACE),
    event(9, user("u-ada", "s1"), checkpoint(SPACE, "doc-plan", 2), SPACE),
    event(10, user("u-ben", "s2"), checkpoint(SPACE, "doc-unannounced", 1), SPACE),
    event(11, user("u-ben", "s7"), checkpoint(SPACE, "doc-note", 1), SPACE),
    event(12, user("u-ben", "s2"), checkpoint(SPACE, "doc-plan", 3), SPACE),
    event(13, user("u-ben", "s2"), checkpoint("space-unknown", "doc-plan", 1), "space-unknown"),
    event(14, ADMIN, checkpoint(SPACE, "doc-plan", 4), SPACE),
    event(15, {"kind": "user", "id": "user:#s9"}, checkpoint(SPACE, "doc-note", 2), SPACE),
]


def expected_activity(events):
    spaces, cursor = {}, 0
    for row in events:
        if row["seq"] <= cursor:
            continue
        cursor = row["seq"]
        body = row["body"]
        if body["kind"] == "space.created":
            spaces[body["spaceId"]] = {"documents": [], "activity": []}
        elif body["kind"] == "document.announced" and body["descriptor"]["spaceId"] in spaces:
            spaces[body["descriptor"]["spaceId"]]["documents"].append(body["descriptor"]["documentId"])
        elif body["kind"] == "artifact.checkpoint-published":
            scope, actor = body["checkpoint"]["scope"], row["actor"]
            author = actor["id"][len("user:"):].split("#")[0] if actor["kind"] == "user" and actor["id"].startswith("user:") else ""
            space = spaces.get(scope["spaceId"])
            if not author or space is None or scope["documentId"] not in space["documents"]:
                continue
            activity = {"documentId": scope["documentId"], "updatedAtMs": row["recordedAtMs"], "updatedBy": author}
            rows = space["activity"]
            index = next((i for i, existing in enumerate(rows) if existing["documentId"] == scope["documentId"]), None)
            if index is None:
                rows.append(activity)
            else:
                rows[index] = activity
    return cursor, {space_id: space["activity"] for space_id, space in spaces.items()}


cursor, activity = expected_activity(EVENTS)
fixture = {
    "schema": "semio.os.directory-document-activity-fixture/v1",
    "description": "Document activity of the directory read model: a checkpoint a HUMAN publishes for an ANNOUNCED document of a folded space records the event's recordedAtMs and the publisher's plain user id (user:{userId}#{session}); a system or admin publication, an unannounced document, an unknown space and a malformed user actor change nothing; the latest publication wins in place; replaying the same events is a no-op. The expected rows are computed by an independent Python reading of this rule (wp-sh2/p2/p2-activity-fixture.py), replayed by the Rust fold and its TypeScript twin.",
    "events": EVENTS,
    "expected": {"cursor": cursor, "documentActivity": activity},
}
os.makedirs(os.path.dirname(TARGET), exist_ok=True)
text = json.dumps(fixture, ensure_ascii=False, indent=2)
text = re.sub(r"\[\s*((?:-?\d+,\s*)*-?\d+)\s*\]", lambda match: "[" + ", ".join(part.strip() for part in match.group(1).split(",")) + "]", text)
open(TARGET, "w", encoding="utf-8").write(text + "\n")
print(json.dumps(fixture["expected"], ensure_ascii=False))
