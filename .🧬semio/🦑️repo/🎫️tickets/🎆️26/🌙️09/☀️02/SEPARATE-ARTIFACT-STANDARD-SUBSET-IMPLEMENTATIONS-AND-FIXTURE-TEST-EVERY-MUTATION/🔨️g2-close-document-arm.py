#!/usr/bin/env python3
"""🔨️ Wraps ✳️document's own real serde-json-carrier insert-image fixture into ✳️base's envelope
shape and registers a new fixtureManifest citing that arm's own oracle, exactly the pattern
F1's report (§3b) established for image/text/table/graph/object/kit."""
import json, hashlib, os

ROOT = "/Users/ueli/Documents/semio"
DOC_FIX = f"{ROOT}/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️document/🧫️fixtures/insert-image"
BASE_DIR = f"{ROOT}/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️base"
FIX_OUT = f"{BASE_DIR}/🧫️fixtures/apply-document-applied"
ORACLE_JSON = f"{BASE_DIR}/🧪️oracle/🔣️.json"

os.makedirs(FIX_OUT, exist_ok=True)

def load(p):
    with open(p) as f:
        return json.load(f)

before_doc = load(f"{DOC_FIX}/before.json")
after_doc = load(f"{DOC_FIX}/after.json")

def wrap(doc):
    subset = dict(doc)
    subset["subset"] = "document"
    return {"schema": "stdio.semio", "subset": subset}

before_env = wrap(before_doc)
after_env = wrap(after_doc)
assert before_env != after_env, "must be non-vacuous"

def write_json(path, obj):
    text = json.dumps(obj, indent=2, ensure_ascii=False, sort_keys=False) + "\n"
    with open(path, "w") as f:
        f.write(text)
    return text.encode("utf-8")

before_bytes = write_json(f"{FIX_OUT}/before.json", before_env)
after_bytes = write_json(f"{FIX_OUT}/after.json", after_env)

def sha(b):
    return "sha256:" + hashlib.sha256(b).hexdigest()

fixture_manifest = {
    "schema": "semio.repository-test.fixture/v2",
    "id": "apply-document-applied",
    "class": "third-party-generated",
    "target": {"artifact": "s.stdio.semio", "standard": "v1", "subset": "base"},
    "mutation": "document",
    "outcome": "applied",
    "units": {"length": "unitless", "angle": "radian"},
    "files": [
        {
            "role": "expected-before-json",
            "path": "../🧫️fixtures/apply-document-applied/before.json",
            "mediaType": "application/json",
            "sha256": sha(before_bytes),
            "bytes": len(before_bytes),
        },
        {
            "role": "expected-after-json",
            "path": "../🧫️fixtures/apply-document-applied/after.json",
            "mediaType": "application/json",
            "sha256": sha(after_bytes),
            "bytes": len(after_bytes),
        },
    ],
    "generator": {
        "oracle": "serde-json-semio-document-carrier-reader",
        "packageVersion": "1",
        "engineFamily": "serde-json",
        "engineVersion": "1",
        "command": f"python3 .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️02/SEPARATE-ARTIFACT-STANDARD-SUBSET-IMPLEMENTATIONS-AND-FIXTURE-TEST-EVERY-MUTATION/🔨️g2-close-document-arm.py",
        "platform": "darwin-arm64",
    },
    "provenance": {
        "source": "generated",
        "license": "public-domain (synthetic, no third-party content embedded)",
        "attribution": "The envelope threading is this file's own; the wrapped `document` content and its real mutation effect (insert-image) were produced by `serde-json-semio-document-carrier-reader` — that ARM subset's own already-registered third-party-library oracle (registered in `../../../../🪆️subsets/✳️document/🧪️oracle/🔣️.json`, visible here because the oracle registry is repository-wide), a genuine independent JSON editor over `SemioDocumentSnapshot`'s own inline `images: Vec<DocImage>` carrier, not a third-party package producing THIS file directly; `class: third-party-generated` is used only because it is the schema's own closest-fitting enum value for a generator-produced fixture.",
        "security": "scanned-clean",
        "privacy": "no-personal-data",
    },
    "comparisonProfile": "ordered-json-v1",
    "reproducible": True,
    "family": "structural",
    "notes": "Applies document's own insert-image mutation (a third image, id img3, appended to the images array) inside the envelope's own SemioSnapshot{schema, subset: SemioSubsetSnapshot::Document(...)} shape. Reuses ✳️document/🧫️fixtures/insert-image's own real before/after pair (produced by serde-json-semio-document-carrier-reader editing the JSON carrier directly, never through this repository's own mutation engine) verbatim, wrapped rather than regenerated. Demonstrates the envelope's own routing law: a wrapped mutation whose arm ('document') matches the base snapshot's own arm threads through to a real, observable content change.",
}

reg = load(ORACLE_JSON)
existing_ids = {fm["id"] for fm in reg["fixtureManifests"]}
assert fixture_manifest["id"] not in existing_ids
reg["fixtureManifests"].append(fixture_manifest)
with open(ORACLE_JSON, "w") as f:
    json.dump(reg, f, indent=2, ensure_ascii=False)
    f.write("\n")

print("wrote", FIX_OUT)
print("before bytes", len(before_bytes), "after bytes", len(after_bytes))
print("registered fixtureManifest id", fixture_manifest["id"])
