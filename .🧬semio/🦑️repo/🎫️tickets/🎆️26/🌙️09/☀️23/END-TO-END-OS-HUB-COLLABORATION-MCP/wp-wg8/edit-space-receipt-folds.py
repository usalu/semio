#!/usr/bin/env python3
"""WG8 s12: schema + fixture cases `receiptFolds` for the space list's read-your-writes fold (both renderers)."""
import json
import pathlib

ROOT = pathlib.Path("/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🏘️spaces")
GOLDEN = pathlib.Path("/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧫️fixtures/📇️directory/⚡️events.json")


def events(user, *bodies, start=1, at=1790000000000):
    return [
        {"seq": start + i, "id": f"018f6b1a-6e40-7c31-9d21-{start + i:012d}", "hlc": {"physicalMs": at + i, "logical": 0}, "actor": {"kind": "user", "id": f"user:{user}#shell-1"}, **({"spaceId": body["spaceId"]} if "spaceId" in body else {}), "body": body, "recordedAtMs": at + i}
        for i, body in enumerate(bodies)
    ]


def row(id, name, access, role, member_count=2, document_count=1, active=0, updated=1700000000000, kind="studio", visibility="private"):
    return {"id": id, "name": name, "kind": kind, "visibility": visibility, "access": access, "role": role, "memberCount": member_count, "documentCount": document_count, "activeConnections": active, "updatedAtMs": updated}


OLD = row("sp-old", "Old Studio", "author", "author")
PUBLIC = row("sp-public", "Open Studio", "public", None, member_count=4, visibility="public")
AT = 1790000000000
cases = [
    {
        "id": "a-space-i-create-is-listed-at-once-with-my-membership",
        "userId": "u-amara",
        "rows": [OLD],
        "events": events("u-amara", {"kind": "space.created", "spaceId": "sp-new", "name": "New Studio", "spaceKind": "studio", "visibility": "private", "ownerUserId": "u-amara"}, {"kind": "member.upserted", "spaceId": "sp-new", "userId": "u-amara", "role": "author"}),
        "expected": [row("sp-new", "New Studio", "author", "author", member_count=1, document_count=0, updated=AT + 1), OLD],
    },
    {
        "id": "an-archive-i-create-lists-me-as-a-spectator-member",
        "userId": "u-amara",
        "rows": [],
        "events": events("u-amara", {"kind": "space.created", "spaceId": "sp-arch", "name": "Shelf", "spaceKind": "archive", "visibility": "private", "ownerUserId": "u-amara"}, {"kind": "member.upserted", "spaceId": "sp-arch", "userId": "u-amara", "role": "spectator"}),
        "expected": [row("sp-arch", "Shelf", "member", "spectator", member_count=1, document_count=0, updated=AT + 1, kind="archive")],
    },
    {
        "id": "a-space-someone-else-creates-is-not-mine-to-list",
        "userId": "u-amara",
        "rows": [OLD],
        "events": events("u-noor", {"kind": "space.created", "spaceId": "sp-noor", "name": "Noor", "spaceKind": "atelier", "visibility": "private", "ownerUserId": "u-noor"}, {"kind": "member.upserted", "spaceId": "sp-noor", "userId": "u-noor", "role": "author"}),
        "expected": [OLD],
    },
    {
        "id": "rename-visibility-and-archive-restate-a-listed-space",
        "userId": "u-amara",
        "rows": [OLD],
        "events": events("u-amara", {"kind": "space.renamed", "spaceId": "sp-old", "name": "Renamed Studio"}, {"kind": "space.visibility-changed", "spaceId": "sp-old", "visibility": "public"}, {"kind": "space.archived", "spaceId": "sp-old"}),
        "expected": [row("sp-old", "Renamed Studio", "member", "spectator", kind="archive", visibility="public", updated=AT + 2)],
    },
    {
        "id": "leaving-a-private-space-drops-its-row",
        "userId": "u-amara",
        "rows": [OLD],
        "events": events("u-amara", {"kind": "member.removed", "spaceId": "sp-old", "userId": "u-amara"}),
        "expected": [],
    },
    {
        "id": "leaving-a-public-space-keeps-it-listed-as-public",
        "userId": "u-amara",
        "rows": [row("sp-open", "Open", "author", "author", member_count=3, active=2, visibility="public")],
        "events": events("u-amara", {"kind": "member.removed", "spaceId": "sp-open", "userId": "u-amara"}),
        "expected": [row("sp-open", "Open", "public", None, member_count=2, active=0, visibility="public", updated=AT)],
    },
    {
        "id": "joining-a-public-space-makes-me-a-member-and-counts-me",
        "userId": "u-amara",
        "rows": [PUBLIC],
        "events": events("u-amara", {"kind": "member.upserted", "spaceId": "sp-public", "userId": "u-amara", "role": "author"}),
        "expected": [row("sp-public", "Open Studio", "author", "author", member_count=5, visibility="public", updated=AT)],
    },
    {
        "id": "a-deleted-space-leaves-the-list",
        "userId": "u-amara",
        "rows": [OLD, PUBLIC],
        "events": events("u-amara", {"kind": "space.deleted", "spaceId": "sp-old"}),
        "expected": [PUBLIC],
    },
    {
        "id": "a-redeemed-invite-counts-a-new-member-an-upsert-of-another-user-does-not",
        "userId": "u-amara",
        "rows": [OLD],
        "events": events("u-amara", {"kind": "invite.redeemed", "spaceId": "sp-old", "userId": "u-devon", "inviteId": "inv-1", "role": "author"}, {"kind": "member.upserted", "spaceId": "sp-old", "userId": "u-noor", "role": "spectator"}, {"kind": "member.removed", "spaceId": "sp-old", "userId": "u-devon"}),
        "expected": [row("sp-old", "Old Studio", "author", "author", member_count=2, updated=AT + 2)],
    },
    {
        "id": "an-announced-document-counts-in-its-space",
        "userId": "u-amara",
        "rows": [OLD],
        "events": events("u-amara", {"kind": "document.announced", "descriptor": {**json.loads((GOLDEN.parent / "🪪️document-descriptor.json").read_text())["valid"], "spaceId": "sp-old", "documentId": "artifact-1"}}),
        "expected": [row("sp-old", "Old Studio", "author", "author", document_count=2, updated=AT)],
    },
]
golden = json.loads(GOLDEN.read_text())["events"]
cases.append(
    {
        "id": "the-golden-directory-log-folds-to-the-read-models-view-of-its-owner",
        "userId": "u-amara",
        "rows": [],
        "events": golden,
        "expected": [row("sp-studio-fabrication", "Fabrication Studio", "member", "spectator", member_count=2, document_count=0, kind="archive", visibility="public", updated=golden[-1]["recordedAtMs"])],
    }
)

fixture_path = ROOT / "🔣️.json"
fixture = json.loads(fixture_path.read_text())
fixture["receiptFolds"] = cases
fixture_path.write_text(json.dumps(fixture, indent=2, ensure_ascii=False) + "\n")

schema_path = ROOT / "🧬️.schema.json"
schema = json.loads(schema_path.read_text())
if "receiptFolds" not in schema["required"]:
    schema["required"].append("receiptFolds")
row_schema = {
    "type": "object",
    "additionalProperties": False,
    "required": ["id", "name", "kind", "visibility", "access", "role", "memberCount", "documentCount", "activeConnections", "updatedAtMs"],
    "properties": {
        "id": {"type": "string", "minLength": 1},
        "name": {"type": "string", "minLength": 1},
        "kind": {"enum": ["atelier", "studio", "archive"]},
        "visibility": {"enum": ["private", "public"]},
        "access": {"enum": ["author", "member", "public"]},
        "role": {"enum": ["author", "spectator", None]},
        "memberCount": {"type": "integer", "minimum": 0},
        "documentCount": {"type": "integer", "minimum": 0},
        "activeConnections": {"type": "integer", "minimum": 0},
        "updatedAtMs": {"type": "integer"},
    },
}
schema["properties"]["receiptFolds"] = {
    "description": "🧾️ Read-your-writes: the caller's rows after one command receipt's directory events, in receipt order (`spaceRowsAfterEventsV1` / `space_rows_after_events`). `expected` is sorted like `spaceRowsV1`.",
    "type": "array",
    "minItems": 11,
    "items": {
        "type": "object",
        "additionalProperties": False,
        "required": ["id", "userId", "rows", "events", "expected"],
        "properties": {
            "id": {"type": "string", "minLength": 1},
            "userId": {"type": "string", "minLength": 1},
            "rows": {"type": "array", "items": {"$ref": "#/definitions/spaceRow"}},
            "events": {"type": "array", "items": {"type": "object", "required": ["seq", "id", "hlc", "actor", "body", "recordedAtMs"]}},
            "expected": {"type": "array", "items": {"$ref": "#/definitions/spaceRow"}},
        },
    },
}
schema.setdefault("definitions", {})["spaceRow"] = row_schema
schema_path.write_text(json.dumps(schema, indent=2, ensure_ascii=False) + "\n")
print("cases", len(cases))
