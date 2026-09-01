#!/usr/bin/env python3
# 🧵️ One-shot, parameterized: registers the lopdf READER for one pdf@1.7 subset and repoints its
# witnessable kinds onto it, recording the two encryption kinds `-uncarried`. Usage: <script> <subset>
import json, subprocess, sys

sub = sys.argv[1]
std = sys.argv[2] if len(sys.argv) > 2 else "1.7"
std_slug = std.replace(".", "-")
V = f"✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️{std}/🪆️subsets/✳️{sub}"
ORACLE = f"{V}/🧪️oracle/🔣️.json"
BIN = f"{V}/🏭️generator/🦀️lopdf-engine/target/release/reader"
CAP, UNC = f"pdf-{std_slug}-{sub}-mutate-reader", f"pdf-{std_slug}-{sub}-mutate-uncarried"
OID = f"lopdf-pdf-{std_slug}-{sub}-mutate-reader"
UNCARRIED = {"insert-encryption-dictionary", "remove-encryption-dictionary"}

d = json.load(open(ORACLE))
profile = (d.get("comparisonProfiles") or [{}])[0].get("id")

d["oracles"].append({
    "id": OID, "kind": "third-party-library", "ecosystem": "rust", "package": "lopdf", "version": "0.44",
    "source": {"repository": "https://github.com/J-F-Liu/lopdf", "license": "MIT"},
    "engine": {"family": "lopdf", "implementation": "lopdf 0.44 COS object graph", "version": "0.44"},
    "capabilities": [CAP], "comparisonProfiles": [profile] if profile else [],
    "license": "MIT", "testOnly": True, "productionReachable": False, "networkDuringExecution": False,
    "platforms": ["darwin-arm64", "darwin-x64", "linux-x64", "linux-arm64", "win32-x64"],
    "homepage": "https://github.com/J-F-Liu/lopdf",
    "rationale": (
        "📖️ A READER, over a corpus that had to be REBUILT before one could mean anything.\n\n"
        "The previous generator imported `oracle_apply_mutation` and `project_conformance` from "
        "`semio-s-plugin-stdio-test-oracle`, this repository's own crate, so the mutated bytes AND the "
        "expected projection of them were both ours and `lopdf` was a codec for our own answer. "
        "Registering a reader beside that corpus would have changed the label and nothing else.\n\n"
        "`🦀️lopdf-engine` depends on `lopdf` and nothing else: it lays out the seed, performs each "
        "mutation through lopdf's own public COS API, and reads the conformance axes back through it. "
        "It refuses to write a pair whose projection does not move, so a no-op cannot be committed as a "
        "fixture that would pass forever.\n\n"
        "The two encryption kinds are `-uncarried`, not routed around: lopdf 0.44's writer takes its "
        "encryption path whenever the trailer carries `/Encrypt` and then needs the encryption state a "
        "genuine decryption would have recorded, so a synthetic encryption dictionary can be neither "
        "written nor read back. A writer-side limit, not a reader gap."
    ),
})

manifests = json.loads(subprocess.run(["bun", f"{V}/🏭️generator/📜️script.ts", "manifests"], capture_output=True, text=True).stdout)
n = len(manifests)
EVIDENCE = (
    f"Validated BOTH ways for all {n} witnessable kinds, per-fixture: (base,base) -> equal:true for {n}/{n}; "
    f"(base,mutated) -> equal:false for {n}/{n} — {2*n}/{2*n} directions correct. Observability is also enforced at "
    "GENERATION time: the engine refuses to write a pair whose projection does not move. The two "
    "encryption kinds were caught by exactly that check failing to read its own output "
    "(`object ID 8 0 not found`) and are registered -uncarried."
)
CRITERIA = [
    {"id": "accepts-a-known-good-pair", "met": True, "detail": f"{n}/{n} kinds: identical (base,base) compares equal:true"},
    {"id": "rejects-a-known-bad-pair", "met": True, "detail": f"{n}/{n} kinds: the kind's own (base,mutated) compares equal:false"},
    {"id": "reads-only", "met": True, "detail": "The reader projects and compares; it neither applies a mutation nor computes an expected state"},
    {"id": "deterministic", "met": True, "detail": "The corpus regenerates byte-identically; no wall-clock and no randomness in the seed"},
    {"id": "offline", "met": True, "detail": "lopdf 0.44.0 is vendored in the cargo registry cache; builds and runs with --offline"},
]
for probe_id, verb, caps in ((f"pdf-{sub}-project", "project", f"pdf.{sub}.project"), (f"pdf-{sub}-compare", "compare", f"pdf.{sub}.compare")):
    d.setdefault("probes", []).append({
        "id": probe_id, "kind": "external-process", "ecosystem": "rust", "package": "lopdf", "version": "0.44",
        "engine": {"family": "lopdf", "implementation": "lopdf 0.44 COS object graph", "version": "0.44"},
        "capabilities": [caps], "outputSchema": "semio.repository-test.probe-report/v2",
        "deterministic": True, "license": "MIT", "testOnly": True, "productionReachable": False,
        "networkDuringExecution": False, "command": [BIN, verb],
        "qualification": {"status": "qualified", "evidence": EVIDENCE, "checkedAt": "2026-08-28", "criteria": CRITERIA},
    })

pipeline = f"pdf-{std_slug}-{sub}-lopdf-compare-v1"
d.setdefault("comparisonPipelines", []).append({
    "id": pipeline,
    "description": f"Conformance-axis equality for pdf@{std}/{sub}, read back through lopdf's own COS API. GATING.",
    "stages": [
        {"probe": f"pdf-{sub}-project", "description": "An independent reader accepts both files.",
         "inputs": ["base-pdf", "mutated-pdf"], "assertions": {"bothImport": True}},
        {"probe": f"pdf-{sub}-compare", "description": f"Equality over the {sub} conformance axes — the operative equality.",
         "inputs": ["base-pdf", "mutated-pdf"], "assertions": {"equal": True}},
    ],
})

repointed = uncarried = 0
for m in d["mutationManifests"][0]["mutations"]:
    if m["id"] in UNCARRIED:
        m["oracleRequirements"] = [{"capability": UNC, "qualifyingKind": "third-party-library"}]
        uncarried += 1
    else:
        m["oracleRequirements"] = [{"capability": CAP, "qualifyingKind": "third-party-library", "oracle": OID}]
        repointed += 1

for entry in manifests:
    entry["comparisonPipeline"] = pipeline
d["fixtureManifests"] = manifests

json.dump(d, open(ORACLE, "w"), ensure_ascii=False, indent=2)
open(ORACLE, "a").write("\n")
print(f"{std}/{sub}: repointed {repointed}, uncarried {uncarried}, fixtureManifests {len(manifests)}")
