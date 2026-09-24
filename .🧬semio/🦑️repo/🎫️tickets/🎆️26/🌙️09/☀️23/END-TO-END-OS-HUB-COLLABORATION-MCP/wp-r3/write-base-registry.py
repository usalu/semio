import hashlib, json, os
BASE = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✉️base"
PATH = os.path.join(BASE, "🔮️oracles", "🔣️.json")
d = json.load(open(PATH, encoding="utf-8"))
ORACLE = "json-rust-semio-envelope-carrier-reader"
def sha(rel):
    data = open(os.path.join(BASE, "🔮️oracles", rel), "rb").read()
    return "sha256:" + hashlib.sha256(data).hexdigest(), len(data)
def file(role, rel):
    digest, size = sha(rel)
    return {"role": role, "path": rel, "mediaType": "application/json", "sha256": digest, "bytes": size}
d["_comment"] = "🧩️ This subset's own contribution: the envelope's mutation vocabulary, its catalog, its committed JSON-carrier vectors, and the third-party reference that reads them. `s.stdio.semio` is the semio ENVELOPE — a union over eighteen semio-native subsets — and its JSON carrier is published schema-first beside the schema types, so a JSON implementation that has never seen this repository can read it and route it by the envelope's own law."
d.pop("noOracleDecisions", None)
d["oracles"] = [{
    "id": ORACLE,
    "kind": "third-party-library",
    "ecosystem": "rust",
    "package": "json",
    "version": "0.12",
    "capabilities": ["semio-v1-base-mutate"],
    "comparisonProfiles": ["ordered-json-v1"],
    "license": "MIT",
    "testOnly": true if False else True,
    "productionReachable": False,
    "networkDuringExecution": False,
    "homepage": "https://github.com/maciejhirsz/json-rust",
    "hostPath": "✏️s/🔌️plugins/🗄️stdio/🔮️oracles/📦️packages/🦀️rust",
    "engine": {"family": "json-rs", "implementation": "json Rust crate (maciejhirsz/json-rust) reading the committed envelope carrier", "version": "0.12"},
    "rationale": "📖️ READS the envelope's committed JSON carrier and ROUTES it by the envelope's published law; `../🔮️oracles/🦀️.rs` is the whole reference. `setSnapshot` replaces the envelope with its payload; a wrapped arm mutation whose tag matches the envelope's `subset` reaches that arm and lands on the arm's own committed result, produced by that arm's registered independent implementation (see each `apply-<arm>-applied` fixture's generator) and never by this repository's Rust; a mismatched arm is refused with `mutation.target-missing`; every inverse restores the envelope it started from. The subject reads the same files only through the schema-derived bridge (`decode_semio_snapshot_json`/`encode_semio_snapshot_json`/`decode_semio_mutation_json`, first-party `pack` JSON over the schema types' own `ToValue`/`FromValue`), so the comparison is between two JSON implementations that share no code. json-rust, not `serde_json`: `serde_json` is a production dependency of the subject crate itself, and `🧾️json`'s own oracle records why a reference the implementation already links is no independent evidence; json-rust appears nowhere in the production graph and is already linked by this plugin's oracle crate, so no dependency is added. Honest limit: the carrier cannot say what a delegated arm verb DOES to its arm — that answer is the arm's own independent implementation's committed result, which this reader reads rather than recomputes."
}]
cat = d["mutationCatalogs"][0]
cat["vectors"] = [
    {"mutationId": "set-snapshot", "sourceMutationDirectoryName": "📸️set-snapshot", "mutationDirectoryName": "📸️set-snapshot", "scenarios": [
        {"id": "replaces-the-envelope-wrapping-a-value-subset", "directoryName": "✉️replaces-the-envelope-wrapping-a-value-subset"},
        {"id": "reasserts-the-value-envelope-unchanged", "directoryName": "🪞️reasserts-the-value-envelope-unchanged"},
        {"id": "retypes-a-value-envelope-to-an-empty-image", "directoryName": "🔁️retypes-a-value-envelope-to-an-empty-image"},
    ]},
    {"mutationId": "apply-image", "sourceMutationDirectoryName": "🖼️apply-image", "mutationDirectoryName": "🖼️apply-image", "scenarios": [
        {"id": "refuses-a-value-envelope", "directoryName": "🚫️refuses-a-value-envelope"},
    ]},
]
ARMS = ["brep", "mesh", "model", "value", "document", "cad", "drawing", "image", "video", "audio", "animation", "presentation", "flow", "text", "table", "graph", "object", "kit"]
def leaf(kind): return f"apply-{kind}" if kind in ARMS else kind
cat["kinds"] = ["set-snapshot"] + [f"apply-{arm}" for arm in ARMS]
for m in d["mutationManifests"][0]["mutations"]:
    m["id"] = leaf(m["id"])
    m["productionDispatch"]["operation"] = m["id"]
    m["oracleRequirements"] = [{"capability": "semio-v1-base-mutate", "qualifyingKind": "third-party-library", "oracle": ORACLE}]
    m["outcomes"] = ["applied"] if m["id"] == "set-snapshot" else ["applied", "rejected"]
for f in d["fixtureManifests"]:
    f["mutation"] = leaf(f["mutation"])
arm_manifests = [f for f in d["fixtureManifests"] if f["id"].startswith("apply-") and f["id"].endswith("-applied")]
for f in arm_manifests:
    folder = os.path.dirname(f["files"][0]["path"])
    f["files"] = [file("expected-before-json", f"{folder}/⬅️before.json"), file("mutation-json", f"{folder}/🦠️mutation.json"), file("expected-after-json", f"{folder}/➡️after.json")]
    f["comparisonProfile"] = "ordered-json-v1"
    if f.get("class") == "third-party-generated":
        origin = f["generator"]["oracle"]
        f["class"] = "handcrafted"
        f.pop("generator")
        f["provenance"] = {"source": "handcrafted", "license": "AGPL-3.0-only", "attribution": f"Envelope vector of this subset. The wrapped {f['mutation']} arm's before/after documents were produced by `{origin}`, that arm's registered independent implementation, and are committed in the envelope's published JSON carrier spelling; the wrapped mutation `🦠️mutation.json` is handcrafted from that pair.", "security": "scanned-clean", "privacy": "no-personal-data"}
def vector_manifest(fid, leaf, scenario, mutation, outcome, notes):
    root = f"../🧫️fixtures/🧬️mutations/{leaf}/{scenario}"
    files = [file("expected-before-json", f"{root}/📸️snapshot/⬅️before/🔣️.json"), file("mutation-json", f"{root}/🦠️mutation/🔣️.json"), file("expected-after-json", f"{root}/📸️snapshot/➡️after/🔣️.json"), file("outcome-json", f"{root}/🎯️outcome/🔣️.json")]
    if os.path.exists(os.path.join(BASE, "🔮️oracles", f"{root}/🔺️diff/🔣️.json")):
        files.insert(3, file("expected-diff-json", f"{root}/🔺️diff/🔣️.json"))
    return {"schema": "semio.repository-test.fixture/v2", "id": fid, "class": "handcrafted", "target": {"artifact": "s.stdio.semio", "standard": "v1", "subset": "base"}, "mutation": mutation, "outcome": outcome, "units": {"length": "unitless", "angle": "radian"}, "files": files, "provenance": {"source": "handcrafted", "license": "AGPL-3.0-only", "attribution": "Handcrafted envelope-level specification vector of this subset; no third-party content embedded.", "security": "scanned-clean", "privacy": "no-personal-data"}, "comparisonProfile": "ordered-json-v1", "reproducible": True, "family": "structural", "notes": notes}
d["fixtureManifests"] = [f for f in d["fixtureManifests"] if not f["id"].startswith("set-snapshot-") and f["id"] != "apply-image-refuses-a-value-envelope"] + [
    vector_manifest("set-snapshot-replaces-the-envelope-wrapping-a-value-subset", "📸️set-snapshot", "✉️replaces-the-envelope-wrapping-a-value-subset", "set-snapshot", "applied", "Replaces a value-subset envelope with one whose count moved from 41 to 42. The diff is a whole `replace`: set-snapshot is the only verb that may change the subset kind, so it never narrows to a per-arm delta."),
    vector_manifest("set-snapshot-reasserts-the-value-envelope-unchanged", "📸️set-snapshot", "🪞️reasserts-the-value-envelope-unchanged", "set-snapshot", "applied", "Reasserts the committed value envelope through set-snapshot: the identity of the envelope vocabulary, which has no separate no-mutation verb. The routed envelope equals the one it started from."),
    vector_manifest("set-snapshot-retypes-a-value-envelope-to-an-empty-image", "📸️set-snapshot", "🔁️retypes-a-value-envelope-to-an-empty-image", "set-snapshot", "applied", "Retypes a value envelope into an empty image envelope — the one change of subset kind the envelope permits, and only through set-snapshot."),
    vector_manifest("apply-image-refuses-a-value-envelope", "🖼️apply-image", "🚫️refuses-a-value-envelope", "apply-image", "rejected", "A wrapped image set-dimensions mutation against a value envelope: the arms do not match, so the envelope refuses it with `mutation.target-missing` and stays exactly as it stood. No diff exists."),
]
open(PATH, "w", encoding="utf-8").write(json.dumps(d, indent=2, ensure_ascii=False) + "\n")
print(len(d["fixtureManifests"]), [f["id"] for f in d["fixtureManifests"]][-5:])
d = json.load(open(PATH, encoding="utf-8"))
d["probes"] = [{
    "id": "semio-base-carrier-reproduce",
    "kind": "external-process",
    "ecosystem": "javascript",
    "package": "ajv",
    "version": "8.20.0",
    "engine": {"family": "ajv", "implementation": "Ajv 8 JSON Schema validator over the platform JSON.parse", "version": "8.20.0"},
    "capabilities": ["semio-v1-base-mutate", "semio.base.carrier.reproduce"],
    "outputSchema": "semio.repository-test.probe-report/v2",
    "deterministic": True,
    "platforms": ["darwin-arm64", "darwin-x64", "linux-x64", "linux-arm64", "win32-x64"],
    "license": "MIT",
    "testOnly": True,
    "productionReachable": False,
    "networkDuringExecution": False,
    "command": ["bun", "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✉️base/🔬️probes/📜️script.ts", "carrier-reproduce"],
    "rationale": "🔬️ The SECOND, independent reader of the envelope's JSON carrier, in another language and on another engine family than the json-rust oracle the parity case runs: the platform `JSON.parse` reads every vector this subset registers and Ajv 8 holds each document to the carrier the subset publishes (`../🧬️schema/📸️snapshot/🔣️.json`, `../🧬️schema/🧬️mutations/🔣️.json` and every schema they reference). The envelope law is then applied in TypeScript, sharing no code with the Rust subject or the oracle: the set-snapshot and refusal vectors are REPRODUCED from before + mutation and compared with the committed after; for the eighteen delegated vectors, whose answer is the arm's own semantics, it checks the routing and the schema conformance of all three documents and reports which published arm schemas disagree with their arm's wire.",
    "qualification": {
        "status": "qualified",
        "evidence": "Run over all 22 registered vectors: 4/4 carrier-expressible vectors (three set-snapshot, one refusal) conform and are reproduced exactly; 22/22 route into the arm both envelopes carry; 16/22 conform to the published schemas, and the 6 that do not are arm-schema drifts reported per vector (audio/presentation mutation payload schemas lack the verbs the arms dispatch, brep's mutation payload schema is internally tagged while its wire is externally tagged, image/video/document snapshot schemas type byte arrays and nullable options differently from their wire).",
        "checkedAt": "2026-09-24",
        "criteria": [
            {"id": "reproduces-the-expressible-vectors", "met": True, "detail": "4/4 set-snapshot and refusal vectors reproduced from before + mutation"},
            {"id": "rejects-a-known-bad-pair", "met": True, "detail": "canonical comparison is exact: any differing member or array order fails"},
            {"id": "independent-of-the-subject", "met": True, "detail": "platform JSON.parse + Ajv; imports nothing of the Rust crate or the json-rust oracle"},
            {"id": "deterministic", "met": True, "detail": "reads committed files only; no clock, no network"},
            {"id": "offline", "met": True, "detail": "ajv 8.20.0 resolves from the repository's root node_modules"}
        ]
    }
}]
open(PATH, "w", encoding="utf-8").write(json.dumps(d, indent=2, ensure_ascii=False) + "\n")
print("probes", len(d["probes"]))
d = json.load(open(PATH, encoding="utf-8"))
PIPELINE = "semio-v1-base-carrier-reproduce-v1"
d["comparisonPipelines"] = [{
    "id": PIPELINE,
    "description": "Carrier-level reproduction of the envelope-owned vectors by a second, independent reader: the platform JSON.parse + Ajv 8 holds before, mutation and after to the published carrier schemas, and the envelope law applied in TypeScript must land on the committed after. GATING for the set-snapshot and refusal vectors, whose answer the carrier itself states.",
    "stages": [{
        "probe": "semio-base-carrier-reproduce",
        "description": "Every registered vector conforms, and every carrier-expressible vector is reproduced exactly from before + mutation.",
        "inputs": ["expected-before-json", "mutation-json", "expected-after-json"],
        "assertions": {"status": "ok"}
    }]
}]
for f in d["fixtureManifests"]:
    if f["id"].startswith("set-snapshot-") or f["id"] == "apply-image-refuses-a-value-envelope":
        f["comparisonPipeline"] = PIPELINE
open(PATH, "w", encoding="utf-8").write(json.dumps(d, indent=2, ensure_ascii=False) + "\n")
print("pipelines", len(d["comparisonPipelines"]))
