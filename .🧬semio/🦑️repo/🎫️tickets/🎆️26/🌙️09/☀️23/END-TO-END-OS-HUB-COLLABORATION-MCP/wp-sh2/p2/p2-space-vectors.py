#!/usr/bin/env python3
"""🧫️ SH2 P2: authors the Space index transient lane's committed `(before, mutation, after, diff, outcome)` vectors for
`📥️apply-directory-events` and `👥️set-artifact-presence` (applied / no-op / rejected each) plus the no-oracle decision, into the
P2 stage. The after-snapshots come from an independent Python reading of the documented directory fold (space.created,
document.announced, a human's checkpoint publication) and presence rule — never from the Rust implementation they judge."""
import copy, hashlib, json, os

REPO = "/Users/ueli/Documents/semio"
STAGE = os.path.join(REPO, ".🧬semio/🌐hub/s14-sh2-p2-stage")
SURFACE = "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🪐️space/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🫧️transient"
FIXTURES = os.path.join(REPO, "🧰️framework/🛍️products/💻️os/🧫️fixtures/📇️directory")
SPACE, DOCUMENT, T0 = "space-1", "artifact-1", 1_800_000_000_000

authority = json.load(open(os.path.join(FIXTURES, "🛡️artifact-authority.json"), encoding="utf-8"))
descriptor = copy.deepcopy(json.load(open(os.path.join(FIXTURES, "🪪️document-descriptor.json"), encoding="utf-8"))["valid"])
descriptor["spaceId"], descriptor["documentId"] = SPACE, DOCUMENT


def at(seq):
    return T0 + seq * 60_000


def event(seq, actor, body):
    kind = "user" if actor.startswith("user:") else "system"
    return {"seq": seq, "id": f"evt-{seq}", "hlc": {"physicalMs": at(seq), "logical": 0}, "actor": {"kind": kind, "id": actor}, "spaceId": SPACE, "body": body, "recordedAtMs": at(seq)}


def checkpoint(seq):
    body = copy.deepcopy(authority["checkpoint"])
    body["scope"] = {"spaceId": SPACE, "documentId": DOCUMENT}
    body["baselineFrontier"].update({"documentId": DOCUMENT, "headEditOrdinal": seq, "headEditId": f"edit:{seq}", "lastCommitSeq": seq})
    body["publishedAtMs"] = seq
    for blob in ("pack", "spr"):
        body[blob].pop("storageKey", None)
    return {"kind": "artifact.checkpoint-published", "checkpoint": body}


ORIGIN = [
    event(1, "user:u-1#s1", {"kind": "space.created", "spaceId": SPACE, "name": "Werkstatt", "spaceKind": "studio", "visibility": "private", "ownerUserId": "u-1"}),
    event(2, "system:hub", {"kind": "document.announced", "descriptor": descriptor}),
    event(3, "user:u-2#s1", checkpoint(3)),
]
RACED = [event(4, "user:u-2#s1", checkpoint(4))]


def compact(value):
    return json.dumps(value, ensure_ascii=False, separators=(",", ":"))


def fold(events):
    """The documented fold, independently: a created space, its announced documents, a human's latest publication."""
    spaces, cursor = {}, 0
    for row in events:
        if row["seq"] <= cursor:
            continue
        cursor, body = row["seq"], row["body"]
        if body["kind"] == "space.created":
            spaces[body["spaceId"]] = {
                "view": {"id": body["spaceId"], "name": body["name"], "kind": body["spaceKind"], "visibility": body["visibility"], "ownerUserId": body["ownerUserId"], "memberCount": 0, "documentCount": 0, "activeConnections": 0, "createdAtMs": row["recordedAtMs"], "updatedAtMs": row["recordedAtMs"]},
                "members": [], "documents": [], "indexedDocuments": [], "documentActivity": [],
            }
        elif body["kind"] == "document.announced":
            space = spaces[body["descriptor"]["spaceId"]]
            space["documents"].append(body["descriptor"])
            space["view"]["documentCount"] = len(space["documents"])
            space["view"]["updatedAtMs"] = row["recordedAtMs"]
        elif body["kind"] == "artifact.checkpoint-published" and row["actor"]["kind"] == "user":
            scope = body["checkpoint"]["scope"]
            space = spaces.get(scope["spaceId"])
            if space and any(d["documentId"] == scope["documentId"] for d in space["documents"]):
                activity = {"documentId": scope["documentId"], "updatedAtMs": row["recordedAtMs"], "updatedBy": row["actor"]["id"][len("user:"):].split("#")[0]}
                space["documentActivity"] = [a for a in space["documentActivity"] if a["documentId"] != scope["documentId"]] + [activity]
    return {"spaces": spaces, "cursor": cursor, "users": {}}


EMPTY = {"spaceId": "", "directory": {"spaces": {}, "cursor": 0, "users": {}}, "presence": []}
LISTED = {"spaceId": SPACE, "directory": fold(ORIGIN), "presence": []}
PRESENT = {**EMPTY, "presence": [{"artifactId": DOCUMENT, "actorsCsv": "user:u-1#s1,user:u-2#s1"}]}


def events_verb(after, events):
    return {"ApplyDirectoryEvents": {"spaceId": SPACE, "afterSeqExclusive": after, "eventsJson": compact(events)}}


def presence_verb(artifact_id, actors):
    return {"SetArtifactPresence": {"artifactId": artifact_id, "actorsCsv": actors}}


VECTORS = [
    ("📥️apply-directory-events", "apply-directory-events", "✅️apply-directory-events-applied", "applied", EMPTY, events_verb(0, ORIGIN), LISTED, [], "The origin batch of a space lists its announced document, and the human's checkpoint publication it carries records the document's activity."),
    ("📥️apply-directory-events", "apply-directory-events", "🟰️apply-directory-events-no-op", "no-op", LISTED, events_verb(0, ORIGIN), LISTED, [{"level": "warning", "code": "mutation.no-op"}], "Replaying from the origin exactly the history the projection holds folds nothing."),
    ("📥️apply-directory-events", "apply-directory-events", "🚫️apply-directory-events-rejected", "rejected", LISTED, events_verb(1, RACED), LISTED, [{"level": "error", "code": "s.space.directory-events-frontier-race"}], "A batch that does not continue the held frontier is refused by name and leaves the projection untouched."),
    ("👥️set-artifact-presence", "set-artifact-presence", "✅️set-artifact-presence-applied", "applied", EMPTY, presence_verb(DOCUMENT, "user:u-1#s1,user:u-2#s1"), PRESENT, [], "A presence row lists the live actors on the artifact's documents."),
    ("👥️set-artifact-presence", "set-artifact-presence", "🟰️set-artifact-presence-no-op", "no-op", PRESENT, presence_verb(DOCUMENT, "user:u-1#s1,user:u-2#s1"), PRESENT, [{"level": "warning", "code": "mutation.no-op"}], "The same row again changes nothing."),
    ("👥️set-artifact-presence", "set-artifact-presence", "🚫️set-artifact-presence-rejected", "rejected", PRESENT, presence_verb("", "user:u-1#s1"), PRESENT, [{"level": "error", "code": "s.space.artifact-presence-invalid"}], "A row naming no artifact is refused by name and leaves the projection untouched."),
]


def write(relative, value):
    path = os.path.join(STAGE, SURFACE, relative)
    os.makedirs(os.path.dirname(path), exist_ok=True)
    text = json.dumps(value, ensure_ascii=False, indent=2) + "\n"
    open(path, "w", encoding="utf-8").write(text)
    return {"path": f"../{relative}", "mediaType": "application/json", "sha256": "sha256:" + hashlib.sha256(text.encode()).hexdigest(), "bytes": len(text.encode())}


def main():
    manifests, catalog = [], {}
    for directory, kind, name, status, before, mutation, after, messages, notes in VECTORS:
        base = f"🧫️fixtures/{directory}/{name}"
        files = [
            {"role": "expected-before", **write(f"{base}/📸️snapshot/⬅️before/🔣️.json", before)},
            {"role": "mutation", **write(f"{base}/🦠️mutation/🔣️.json", mutation)},
            {"role": "expected-after", **write(f"{base}/📸️snapshot/➡️after/🔣️.json", after)},
            {"role": "expected-diff", **write(f"{base}/🔺️diff/🔣️.json", after)},
            {"role": "declared-outcome", **write(f"{base}/🎯️outcome/🔣️.json", {"status": status, "messages": messages})},
        ]
        manifests.append({
            "schema": "semio.repository-test.fixture/v2",
            "id": name.split("️", 1)[1],
            "class": "handcrafted",
            "target": {"artifact": "s.space.space", "standard": "1", "subset": "any", "surface": "👁️viewer/🫧️transient"},
            "mutation": kind,
            "outcome": status,
            "units": {"length": "unitless", "angle": "unitless"},
            "files": files,
            "provenance": {"source": "authored", "license": "public-domain (handcrafted by this repository)", "attribution": "Handcrafted before/mutation/after vector: the after snapshot is written from the directory fold's and the presence rule's documented semantics by an independent Python reading (`wp-sh2/p2/p2-space-vectors.py`), not produced by the implementation.", "security": "scanned-clean", "privacy": "no-personal-data"},
            "comparisonProfile": "ordered-json-v1",
            "reproducible": True,
            "family": "transient-state-lane",
            "notes": notes,
        })
        catalog.setdefault((kind, directory), []).append(name)
    capability = "s-space-1-any-viewer-transient-mutate"
    oracles = {
        "$schema": "../../../../../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧬️schema/🔣️.json",
        "schemaVersion": 2,
        "_comment": "🎚️ The transient state lane of `s.space.space` (👁️viewer/🫧️transient, reused by the editor): its mutation vocabulary `SpaceIndexTransientMutation` over `SpaceIndexTransient`, claimed by `../../../🧪️tests/🫧️mutate-s-space-1-any-viewer-transient` and backed by one committed vector per declared outcome class.",
        "oracles": [],
        "noOracleDecisions": [{
            "id": "s-space-1-any-viewer-transient-state-lane-semantics",
            "capabilities": [capability],
            "rationale": "`SpaceIndexTransientMutation` is the transient state lane of `s.space.space`: it folds bounded batches of one space's directory events into the Space table's projection through the canonical kernel fold, and replaces per-artifact presence rows. This is this repository's own state record over its own directory wire, not a published format, so no third party implements it. What is under test is that `apply-directory-events` folds exactly a batch that continues the held frontier (or restarts from the origin), that replaying the held history is a warned `mutation.no-op`, that any other batch is refused by name without touching the projection, and that `set-artifact-presence` replaces exactly one artifact's row. Confidence comes from one committed, handcrafted (before, mutation, after, diff, outcome) vector per declared outcome class under `../🧫️fixtures/`, exercised end to end through the production dispatch bridge.",
            "substitutes": ["specification-vectors"],
            "coversMutations": True,
            "referenceSurvey": {
                "ecosystemsSearched": ["npm", "crates.io"],
                "candidatesConsidered": [
                    {"package": "zustand", "ecosystem": "npm", "verdict": "cannot-express-the-mutation", "reason": "An in-memory state container with arbitrary setter functions; it has no directory fold, no frontier rule and no typed batch vocabulary."},
                    {"package": "im", "ecosystem": "crates.io", "verdict": "cannot-express-the-mutation", "reason": "Persistent immutable collections; they share structure but define no directory events, frontier admission or refusal semantics."},
                ],
                "whyNoneQualifies": "`SpaceIndexTransient` is a projection defined by this repository over its own hub directory events; generic state containers can hold its rows but cannot adjudicate which batch continues a frontier or how an event folds.",
            },
        }],
        "mutationCatalogs": [{
            "id": "s-space-1-any-viewer-transient",
            "capability": capability,
            "standardDirectoryName": "🔖️1",
            "subsetDirectoryName": "✳️any",
            "kinds": [kind for kind, _ in catalog],
            "vectors": [{"mutationId": kind, "sourceMutationDirectoryName": directory, "mutationDirectoryName": directory, "scenarios": [{"id": name.split("️", 1)[1], "directoryName": name} for name in names]} for (kind, directory), names in catalog.items()],
        }],
        "mutationManifests": [{
            "schema": "semio.repository-test.mutation-manifest/v2",
            "artifact": "s.space.space",
            "standard": "1",
            "subset": "any",
            "surface": "👁️viewer/🫧️transient",
            "mutations": [{
                "id": kind,
                "capability": capability,
                "payloadSchema": "🧬️schema/🔣️.json",
                "outcomes": ["applied", "no-op", "rejected"],
                "productionDispatch": {"operation": kind, "bridgeVersion": 1, "variant": "ApplyDirectoryEvents" if kind == "apply-directory-events" else "SetArtifactPresence"},
                "oracleRequirements": [{"capability": capability, "qualifyingKind": "third-party-library"}],
            } for kind, _ in catalog],
        }],
        "fixtureManifests": manifests,
    }
    path = os.path.join(STAGE, SURFACE, "🔮️oracles/🔣️.json")
    os.makedirs(os.path.dirname(path), exist_ok=True)
    open(path, "w", encoding="utf-8").write(json.dumps(oracles, ensure_ascii=False, indent=2) + "\n")
    print(f"{len(manifests)} vectors + oracles written")


if __name__ == "__main__":
    main()
