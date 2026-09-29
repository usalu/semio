#!/usr/bin/env python3
"""🧫️ SH2 14c P1: authors the Home transient lane's committed `(before, mutation, after, diff, outcome)` vectors for
`📬️apply-directory-page` (applied / no-op / rejected) from the directory wire grammar itself — canonical pages sealed with
their SHA-256 receipt exactly as the hub seals them — plus the no-oracle decision file with every fixture's digest, into the
SH2 stage (`.🧬semio/🌐hub/s14-sh2-stage`). Independent of the Rust implementation it judges."""
import hashlib, json, os

REPO = "/Users/ueli/Documents/semio"
STAGE = os.path.join(REPO, ".🧬semio/🌐hub/s14-sh2-stage")
SURFACE = "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient"
BINDING = "a" * 64


def compact(value):
    return json.dumps(value, ensure_ascii=False, separators=(",", ":"))


def event(seq, body, space_id=None):
    row = {"seq": seq, "id": f"event-{seq}", "hlc": {"physicalMs": seq, "logical": 0}, "actor": {"kind": "user", "id": "user:u1#s1"}}
    if space_id is not None:
        row["spaceId"] = space_id
    row["body"] = body
    row["recordedAtMs"] = seq
    return row


def sealed(after, through, events):
    unsigned = {"schema": "semio.directory.event-page.v1", "sessionBindingSha256": BINDING, "authorizationGeneration": 3, "afterSeqExclusive": after, "throughSeqInclusive": through, "hasMore": False, "events": events}
    receipt = hashlib.sha256(compact(unsigned).encode()).hexdigest()
    return compact({**unsigned, "receiptSha256": receipt}), receipt


FIRST, FIRST_RECEIPT = sealed(0, 3, [
    event(1, {"kind": "user.created", "userId": "u1", "email": "ada@semio.dev", "displayName": "Ada"}),
    event(2, {"kind": "space.created", "spaceId": "space-a", "name": "Werkstatt", "spaceKind": "studio", "visibility": "private", "ownerUserId": "u1"}, "space-a"),
    event(3, {"kind": "member.upserted", "spaceId": "space-a", "userId": "u1", "role": "author"}, "space-a"),
])
RACED, _ = sealed(2, 4, [event(4, {"kind": "space.renamed", "spaceId": "space-a", "name": "Atelier"}, "space-a")])

EMPTY = {"sessionBindingSha256": "", "authorizationGeneration": 0, "receiptSha256": "", "directory": {"spaces": {}, "cursor": 0, "users": {}}}
LISTED = {
    "sessionBindingSha256": BINDING,
    "authorizationGeneration": 3,
    "receiptSha256": FIRST_RECEIPT,
    "directory": {
        "spaces": {
            "space-a": {
                "view": {"id": "space-a", "name": "Werkstatt", "kind": "studio", "visibility": "private", "ownerUserId": "u1", "memberCount": 1, "documentCount": 0, "activeConnections": 0, "createdAtMs": 2, "updatedAtMs": 3},
                "members": [{"userId": "u1", "email": "ada@semio.dev", "displayName": "Ada", "role": "author"}],
                "documents": [],
                "indexedDocuments": [],
            }
        },
        "cursor": 3,
        "users": {"u1": {"id": "u1", "email": "ada@semio.dev", "displayName": "Ada", "createdAtMs": 1}},
    },
}

VECTORS = [
    ("✅️apply-directory-page-applied", "applied", EMPTY, FIRST, LISTED, [], "The origin page of a signed-in human's directory lists the one space it creates, joined with its author's member row and the user it names."),
    ("🟰️apply-directory-page-no-op", "no-op", LISTED, FIRST, LISTED, [{"level": "warning", "code": "mutation.no-op"}], "Replaying the page whose frontier the projection already holds folds nothing and answers the same receipt."),
    ("🚫️apply-directory-page-rejected", "rejected", LISTED, RACED, LISTED, [{"level": "error", "code": "s.home.directory-event-page-frontier-race"}], "A same-authority page that does not continue the held frontier is refused by name and leaves the projection untouched."),
]


def write(relative, value):
    path = os.path.join(STAGE, SURFACE, relative)
    os.makedirs(os.path.dirname(path), exist_ok=True)
    text = json.dumps(value, ensure_ascii=False, indent=2) + "\n"
    with open(path, "w", encoding="utf-8") as handle:
        handle.write(text)
    return {"path": f"../{relative}", "mediaType": "application/json", "sha256": "sha256:" + hashlib.sha256(text.encode()).hexdigest(), "bytes": len(text.encode())}


def main():
    manifests = []
    for name, status, before, page, after, messages, notes in VECTORS:
        base = f"🧫️fixtures/📬️apply-directory-page/{name}"
        files = [
            {"role": "expected-before", **write(f"{base}/📸️snapshot/⬅️before/🔣️.json", before)},
            {"role": "mutation", **write(f"{base}/🦠️mutation/🔣️.json", {"ApplyDirectoryPage": {"pageJson": page}})},
            {"role": "expected-after", **write(f"{base}/📸️snapshot/➡️after/🔣️.json", after)},
            {"role": "expected-diff", **write(f"{base}/🔺️diff/🔣️.json", after)},
            {"role": "declared-outcome", **write(f"{base}/🎯️outcome/🔣️.json", {"status": status, "messages": messages})},
        ]
        manifests.append({
            "schema": "semio.repository-test.fixture/v2",
            "id": name.split("️", 1)[1],
            "class": "handcrafted",
            "target": {"artifact": "s.space.home", "standard": "1", "subset": "any", "surface": "✏️editor/🫧️transient"},
            "mutation": "apply-directory-page",
            "outcome": status,
            "units": {"length": "unitless", "angle": "unitless"},
            "files": files,
            "provenance": {"source": "authored", "license": "public-domain (handcrafted by this repository)", "attribution": "Handcrafted before/mutation/after vector: the page is sealed with its SHA-256 receipt over the canonical unsigned page exactly as the hub seals it (`wp-sh2/sh2-p1-vectors.py`), the after-snapshot is written from the directory fold's documented semantics, not produced by the implementation.", "security": "scanned-clean", "privacy": "no-personal-data"},
            "comparisonProfile": "ordered-json-v1",
            "reproducible": True,
            "family": "transient-state-lane",
            "notes": notes,
        })
    capability = "s-home-1-any-editor-transient-mutate"
    oracles = {
        "$schema": "../../../../../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧬️schema/🔣️.json",
        "schemaVersion": 2,
        "_comment": "🎚️ The transient state lane of `s.space.home`'s editor (✏️editor/🫧️transient): its mutation vocabulary `HomeTransientMutation` over `HomeTransient`, claimed by `../../../🧪️tests/🫧️mutate-s-home-1-any-editor-transient` and backed by one committed vector per declared outcome class.",
        "oracles": [],
        "noOracleDecisions": [{
            "id": "s-home-1-any-editor-transient-state-lane-semantics",
            "capabilities": [capability],
            "rationale": "`HomeTransientMutation` is the transient state lane of `s.space.home`'s ✏️editor/🫧️transient surface: it folds authenticated, receipt-sealed hub directory pages into the Home launcher's local projection. This is this repository's own state record over its own directory wire, not a published format, so no third party implements it. What is under test is that `apply-directory-page` folds exactly the page's events onto the held frontier, that replaying the held frontier is a warned `mutation.no-op`, and that a page which does not continue the frontier is refused by name without touching the projection. Confidence comes from one committed, handcrafted (before, mutation, after, diff, outcome) vector per declared outcome class under `../🧫️fixtures/`, the page sealed by the hub's own receipt rule, exercised end to end through the production dispatch bridge.",
            "substitutes": ["specification-vectors"],
            "coversMutations": True,
            "referenceSurvey": {
                "ecosystemsSearched": ["npm", "crates.io"],
                "candidatesConsidered": [
                    {"package": "zustand", "ecosystem": "npm", "verdict": "cannot-express-the-mutation", "reason": "An in-memory state container with arbitrary setter functions; it has no directory fold, no frontier or receipt rule and no typed page vocabulary."},
                    {"package": "im", "ecosystem": "crates.io", "verdict": "cannot-express-the-mutation", "reason": "Persistent immutable collections; they share structure but define no directory events, frontier admission or refusal semantics."},
                ],
                "whyNoneQualifies": "`HomeTransient` is a projection defined by this repository over its own hub directory events; generic state containers can hold its rows but cannot adjudicate which page continues a frontier or how an event folds.",
            },
        }],
        "mutationCatalogs": [{"id": "s-home-1-any-editor-transient", "capability": capability, "standardDirectoryName": "🔖️1", "subsetDirectoryName": "✳️any", "kinds": ["apply-directory-page"], "vectors": [{"mutationId": "apply-directory-page", "sourceMutationDirectoryName": "📬️apply-directory-page", "mutationDirectoryName": "📬️apply-directory-page", "scenarios": [{"id": name.split("️", 1)[1], "directoryName": name} for name, *_ in VECTORS]}]}],
        "mutationManifests": [{
            "schema": "semio.repository-test.mutation-manifest/v2",
            "artifact": "s.space.home",
            "standard": "1",
            "subset": "any",
            "surface": "✏️editor/🫧️transient",
            "mutations": [{
                "id": "apply-directory-page",
                "capability": capability,
                "payloadSchema": "🧬️schema/🔣️.json",
                "outcomes": ["applied", "no-op", "rejected"],
                "productionDispatch": {"operation": "apply-directory-page", "bridgeVersion": 1, "variant": "ApplyDirectoryPage"},
                "oracleRequirements": [{"capability": capability, "qualifyingKind": "third-party-library"}],
            }],
        }],
        "fixtureManifests": manifests,
    }
    path = os.path.join(STAGE, SURFACE, "🔮️oracles/🔣️.json")
    os.makedirs(os.path.dirname(path), exist_ok=True)
    with open(path, "w", encoding="utf-8") as handle:
        handle.write(json.dumps(oracles, ensure_ascii=False, indent=2) + "\n")
    print(f"{len(manifests)} vectors + oracles written")


if __name__ == "__main__":
    main()
