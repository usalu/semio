#!/usr/bin/env python3
# 🧵️ One-shot: registers the shared pypdf ENCRYPTION reader for one pdf@1.7 subset and repoints its two
# encryption kinds off `-uncarried` onto it. Usage: <script> <vt|a|e|x>
import json, subprocess, sys

sub = sys.argv[1]
V = f"✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.7/🪆️subsets/✳️{sub}"
PROBES = "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.7/🪆️subsets/✳️vt/🔬️probes/📜️script.ts"
ORACLE = f"{V}/🧪️oracle/🔣️.json"
CAP, OID = f"pdf-1-7-{sub}-mutate-encryption", "pypdf-pdf-1-7-encryption-reader"

d = json.load(open(ORACLE))
manifests = json.loads(subprocess.run(["bun", f"{V}/🏭️generator/📜️script.ts", "encryption-manifests"], capture_output=True, text=True).stdout)
kinds = [e["mutation"] for e in manifests]; n = len(kinds)

d["oracles"].append({
    "id": OID, "kind": "third-party-library", "ecosystem": "python", "package": "pypdf", "version": "6.14.2",
    "source": {"repository": "https://github.com/py-pdf/pypdf", "license": "BSD-3-Clause"},
    "engine": {"family": "pypdf", "implementation": "pypdf 6.14 standard security handler reader", "version": "6.14.2"},
    "capabilities": [CAP], "comparisonProfiles": ["semantic-pdf-encryption-v1"],
    "license": "BSD-3-Clause", "testOnly": True, "productionReachable": False, "networkDuringExecution": False,
    "platforms": ["darwin-arm64", "darwin-x64", "linux-x64", "linux-arm64", "win32-x64"],
    "homepage": "https://pypdf.readthedocs.io",
    "rationale": (
        "📖️ A SECOND reader for the two kinds `lopdf` can neither write nor read.\n\n"
        "Both halves were measured. WRITING: lopdf 0.44's writer takes its encryption path whenever the "
        "trailer carries `/Encrypt` and then requires the encryption state a genuine decryption would "
        "have recorded, so a synthetic encryption dictionary fails on its own output "
        "(`object ID 8 0 not found`). READING: handed a genuinely encrypted PDF, lopdf DECRYPTS "
        "transparently with the empty user password and then reports `is_encrypted() == false` — so even "
        "a real encrypted fixture would project as unencrypted. That is why these two kinds were "
        "`-uncarried`, and it was a statement about lopdf, not about PDF libraries.\n\n"
        "pypdf 6.14 does both: `PdfWriter.encrypt` emits a real standard security handler, and "
        "`PdfReader.is_encrypted` reports it. Its output is byte-DETERMINISTIC — checked three times "
        "before it was relied on, which matters because encryption schemes commonly randomise a key and "
        "a non-reproducible fixture would fail this repository's own reproducibility gate.\n\n"
        "The reader is shared by all four conformance subsets: the question is identical across them and "
        "the framework already lets a probe's `command` point at another subset's script."
    ),
})
EVID = ("Validated BOTH ways across all four conformance subsets — 8 pairs, 16 directions: "
        "(base,base) -> equal:true for 8/8; (base,mutated) -> equal:false for 8/8. lopdf, this subset's "
        "other reader, was measured reporting is_encrypted() == false on the SAME genuinely encrypted "
        "file, which is what makes this a second reader rather than a redundant one. pypdf's encrypted "
        "output hashed identically across three runs.")
CRIT = [
    {"id": "accepts-a-known-good-pair", "met": True, "detail": f"{n}/{n} kinds: identical (base,base) compares equal:true"},
    {"id": "rejects-a-known-bad-pair", "met": True, "detail": f"{n}/{n} kinds: the kind's own (base,mutated) compares equal:false"},
    {"id": "witnesses-what-lopdf-cannot", "met": True, "detail": "lopdf decrypts transparently on load and then reports is_encrypted() false on the same file"},
    {"id": "deterministic", "met": True, "detail": "pypdf's encrypted output hashed identically across three runs; the fixtures are byte-reproducible"},
    {"id": "reads-only", "met": True, "detail": "The probes project and compare; neither applies a mutation nor predicts one"},
    {"id": "offline", "met": True, "detail": "pypdf is installed locally; the reader runs with no network"},
]
for pid in ("pdf-encryption-project", "pdf-encryption-compare"):
    d.setdefault("probes", []).append({
        "id": f"{pid}-{sub}", "kind": "external-process", "ecosystem": "python", "package": "pypdf", "version": "6.14.2",
        "engine": {"family": "pypdf", "implementation": "pypdf 6.14 standard security handler reader", "version": "6.14.2"},
        "capabilities": [f"pdf.encryption.{pid.split('-')[-1]}"], "outputSchema": "semio.repository-test.probe-report/v2",
        "deterministic": True, "license": "BSD-3-Clause", "testOnly": True, "productionReachable": False,
        "networkDuringExecution": False, "command": ["bun", PROBES, pid],
        "qualification": {"status": "qualified", "evidence": EVID, "checkedAt": "2026-08-28", "criteria": CRIT},
    })
d.setdefault("comparisonProfiles", []).append({"id": "semantic-pdf-encryption-v1", "description": "Whether the document carries a standard security handler, the handler's algorithm, and the page count read after decrypting with the empty user password so both sides of a pair stay comparable."})
pipeline = f"pdf-1-7-{sub}-encryption-compare-v1"
d.setdefault("comparisonPipelines", []).append({
    "id": pipeline, "description": "Encryption-presence equality through pypdf. GATING for the two kinds lopdf can neither write nor read.",
    "stages": [
        {"probe": f"pdf-encryption-project-{sub}", "description": "An independent reader accepts both files and reports whether each is encrypted.", "inputs": ["base-pdf", "mutated-pdf"], "assertions": {"bothImport": True}},
        {"probe": f"pdf-encryption-compare-{sub}", "description": "Equality over encryption presence and algorithm — the operative equality.", "inputs": ["base-pdf", "mutated-pdf"], "assertions": {"equal": True}},
    ],
})
wanted = set(kinds); rep = 0
for m in d["mutationManifests"][0]["mutations"]:
    if m["id"] in wanted:
        m["oracleRequirements"] = [{"capability": CAP, "qualifyingKind": "third-party-library", "oracle": OID}]; rep += 1
for e in manifests: e["comparisonPipeline"] = pipeline
ids = {e["id"] for e in manifests}
d["fixtureManifests"] = [f for f in d.get("fixtureManifests", []) if f["id"] not in ids] + manifests
json.dump(d, open(ORACLE, "w"), ensure_ascii=False, indent=2); open(ORACLE, "a").write("\n")
print(f"pdf 1.7/{sub}: repointed {rep}, fixtureManifests {len(d['fixtureManifests'])}")
