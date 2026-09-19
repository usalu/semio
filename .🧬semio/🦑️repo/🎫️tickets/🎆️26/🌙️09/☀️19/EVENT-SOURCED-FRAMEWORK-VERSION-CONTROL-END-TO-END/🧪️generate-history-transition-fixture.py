"""🧫️ Independent (non-Rust) encoder for `semio.history.transition` payloads: writes the language-agnostic
fixture the Rust codec and the TS/Ajv schema test both check against."""
import json, sys

def varint(n):
    out = bytearray()
    while True:
        b = n & 0x7F
        n >>= 7
        if n:
            out.append(b | 0x80)
        else:
            out.append(b)
            return bytes(out)

def s(v):
    b = v.encode("utf-8")
    return varint(len(b)) + b

def opt(v):
    return b"\x00" if v is None else b"\x01" + s(v)

def ids(v):
    return varint(len(v)) + b"".join(s(i) for i in v)

def encode(t):
    k = t["kind"]
    if k == "revert":
        return varint(0) + ids(t["mutationIds"])
    if k == "reinstate":
        return varint(1) + ids(t["mutationIds"])
    if k == "commit":
        out = varint(2) + s(t["checkpointId"]) + opt(t["parentId"]) + s(t["changeId"]) + ids(t["mutationIds"]) + opt(t["description"]) + s(t["savedAt"])
        out += varint(len(t["authors"])) + b"".join(s(a["id"]) + s(a["name"]) + opt(a["avatar"]) for a in t["authors"])
        return out + opt(t["message"]) + s(t["timestamp"])
    if k == "branch":
        return varint(3) + s(t["alternativeId"]) + s(t["name"]) + s(t["checkpointId"])
    if k == "checkout":
        return varint(4) + s(t["checkpointId"]) + opt(t["alternativeId"])
    if k == "repin":
        return varint(5) + s(t["checkpointId"]) + s(t["pinnedCheckpointId"]) + varint(len(t["pins"])) + b"".join(s(p["childUri"]) + s(p["checkpointId"]) for p in t["pins"])
    raise ValueError(k)

long_id = "op-" + "x" * 140
accepted = [
    ("revert-one", {"kind": "revert", "mutationIds": ["op-a-1"]}),
    ("revert-many", {"kind": "revert", "mutationIds": ["op-a-1", "op-a-2", "op-a-3"]}),
    ("reinstate-long-id", {"kind": "reinstate", "mutationIds": [long_id]}),
    ("commit-full", {"kind": "commit", "checkpointId": "checkpoint-2", "parentId": "checkpoint-1", "changeId": "change-2", "mutationIds": ["op-a-1", "op-b-1"], "description": "Tiles and source", "savedAt": "2026-09-19T12:00:00Z", "authors": [{"id": "actor-a", "name": "Ada", "avatar": "https://example.test/ada.png"}, {"id": "actor-b", "name": "Bé 🧪", "avatar": None}], "message": "merge both replicas", "timestamp": "1789819200000"}),
    ("commit-minimal", {"kind": "commit", "checkpointId": "checkpoint-1", "parentId": None, "changeId": "change-1", "mutationIds": [], "description": None, "savedAt": "", "authors": [], "message": None, "timestamp": ""}),
    ("branch", {"kind": "branch", "alternativeId": "alternative-2", "name": "Variant B", "checkpointId": "checkpoint-1"}),
    ("checkout-alternative", {"kind": "checkout", "checkpointId": "checkpoint-1", "alternativeId": "alternative-2"}),
    ("checkout-plain", {"kind": "checkout", "checkpointId": "checkpoint-2", "alternativeId": None}),
    ("repin", {"kind": "repin", "checkpointId": "checkpoint-2", "pinnedCheckpointId": "checkpoint-2-pinned", "pins": [{"childUri": "semio://child/a", "checkpointId": "child-checkpoint-4"}, {"childUri": "semio://child/b", "checkpointId": "child-checkpoint-1"}]}),
]
revert = encode(accepted[0][1])
malformed = [
    ("empty", b"", "truncated"),
    ("unknown-tag", varint(6), "unknown transition tag 6"),
    ("trailing-bytes", revert + b"\x00", "trailing bytes"),
    ("truncated-id", revert[:-1], "truncated"),
    ("invalid-option-tag", varint(4) + s("checkpoint-1") + b"\x02", "invalid option tag 2"),
    ("id-count-exceeds-payload", varint(0) + varint(1000), "id count exceeds payload"),
]
fixture = {
    "schema": "semio.history.transition.v1",
    "diffSchema": "semio.history.transition",
    "cases": [{"id": i, "payloadHex": encode(t).hex(), "expect": {"outcome": "accepted", "transition": t}} for i, t in accepted]
    + [{"id": i, "payloadHex": b.hex(), "expect": {"outcome": "malformed", "detail": d}} for i, b, d in malformed],
}
json.dump(fixture, sys.stdout, indent=2, ensure_ascii=False)
sys.stdout.write("\n")
