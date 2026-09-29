"""🔗️ C12: builds `🧵️canonical-edit/🧫️fixtures/🔗️edit-digest-chains.json` — the language-neutral vectors of the revision digest
rule (single-operation canonical JSON, grown-to-two and grown-to-ten-thousand chained), with digests from this THIRD
implementation (Python hashlib), replayed by the Rust store and the TS oracle. usage: python3 make-edit-digest-chains.py <out>"""
import hashlib
import json
import sys

PREFIX = b"semio.artifact.cursor.v2"


def u64(value):
    return value.to_bytes(8, "big")


def hash_record(domain, parts):
    digest = hashlib.sha256()
    digest.update(PREFIX)
    digest.update(u64(len(domain)))
    digest.update(domain)
    for part in parts:
        digest.update(u64(len(part)))
        digest.update(part)
    return digest.digest()


def canonical(value):
    return json.dumps(value, ensure_ascii=False, separators=(",", ":")).encode("utf-8")


def chain(domain, items):
    state = bytes(32)
    for item in items:
        state = hash_record(domain, [state, canonical(item)])
    return state


def digest(edit):
    if len(edit["forwards"]) <= 1:
        return hash_record(b"edit", [edit["id"].encode(), canonical(edit)]).hex()
    tag = lambda key: bytes([1 if key in edit else 0])
    text = lambda key: edit.get(key, "").encode()
    meta = edit.get("mutationMeta", [])
    return hash_record(b"edit-chained", [
        edit["id"].encode(),
        tag("actor"), text("actor"),
        tag("description"), text("description"),
        tag("coalesceKey"), text("coalesceKey"),
        edit["sequenceNumber"].to_bytes(4, "big", signed=True),
        edit["startedAt"].encode(),
        tag("finishedAt"), text("finishedAt"),
        u64(len(edit["forwards"])), chain(b"edit-forward", edit["forwards"]),
        u64(len(edit["inverse"])), chain(b"edit-inverse", edit["inverse"]),
        u64(len(meta)), chain(b"edit-meta", meta),
    ]).hex()


def meta(index):
    return {"mutation_id": f"edit-typing#{index}", "dependencies": ["prior-1"], "base_version": index, "author_id": "actor-1", "timestamp": {"actor": 1, "physical_ms": 42 + index, "logical": 0}, "undo_policy": "ExactBaseOnly", "label": "Edit text", "group_id": "group-typing"}


def edit(count, with_meta):
    record = {"id": "edit-typing", "actor": "actor-1", "forwards": [{"SetN": {"n": index + 1}} for index in range(count)], "inverse": [{"SetN": {"n": index}} for index in range(count)]}
    if with_meta:
        record["mutationMeta"] = [meta(index) for index in range(count)]
    record.update({"description": "typing 🧵", "coalesceKey": "typing", "sequenceNumber": 7, "startedAt": "2026-09-29T11:00:00Z", "finishedAt": "2026-09-29T11:00:09Z"})
    return record


single, two, large = edit(1, True), edit(2, True), edit(10_000, False)
header = {key: value for key, value in large.items() if key not in ("forwards", "inverse")}
fixture = {
    "schema": "semio.store.edit-digest-chains.v1",
    "cases": [
        {"name": "single-operation", "edit": single, "expectedDigest": digest(single)},
        {"name": "grown-to-two", "edit": two, "expectedDigest": digest(two)},
        {"name": "grown-to-ten-thousand", "header": header, "generatedOperations": 10_000, "expectedDigest": digest(large)},
    ],
}
open(sys.argv[1], "w", encoding="utf-8").write(json.dumps(fixture, indent=2, ensure_ascii=False) + "\n")
print({case["name"]: case["expectedDigest"][:16] for case in fixture["cases"]})
