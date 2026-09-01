#!/usr/bin/env python3
# 🩹 Additive patch for ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖊️dxf/🏅️standards/🔖️r12/🪆️subsets/✳️any/🧪️oracle/🔣️.json
#
# Registers the new READER oracle (dxf-crate-r12-mutate-reader, kind: third-party-library),
# leaves the existing cross-semio-implementation oracle (dxf-crate-r12-mutate) and
# 🧪️oracle/🦀️component.rs byte-for-byte untouched, adds probes + one GATING comparisonPipeline,
# retargets every mutation's oracleRequirements[].oracle to the new reader id, and merges in the
# 37 new fixtureManifests entries produced by 🏭️generator/📜️script.ts (drafting-plate's existing
# entry is never touched).
#
# Run once: python3 📜️patch-dxf-r12-any-oracle-json.py
import json
from pathlib import Path

REPO = Path("/Users/ueli/Documents/semio")
ORACLE_JSON = REPO / "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖊️dxf/🏅️standards/🔖️r12/🪆️subsets/✳️any/🧪️oracle/🔣️.json"
NEW_MANIFESTS = REPO / ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️27/SUBSET-SCOPED-EXTERNAL-ORACLE-MUTATION-TESTING/🗑️temp/dxf-r12-any/all-manifests.json"

READER_ORACLE_ID = "dxf-crate-r12-mutate-reader"
CROSS_SEMIO_ORACLE_ID = "dxf-crate-r12-mutate"

READER_ORACLE = {
    "id": READER_ORACLE_ID,
    "kind": "third-party-library",
    "ecosystem": "rust",
    "package": "dxf",
    "version": "0.6",
    "engine": {"family": "dxf-rs", "implementation": "dxf reader", "version": "0.6"},
    "capabilities": ["dxf-r12-mutate"],
    "license": "MIT",
    "testOnly": True,
    "productionReachable": False,
    "networkDuringExecution": False,
    "platforms": ["darwin-arm64", "darwin-x64", "linux-x64", "linux-arm64", "win32-x64"],
    "rationale": (
        "📖️ A READER, and that is the whole point of registering it separately.\n\n"
        "This subset also carries a 🧪️oracle/🦀️component.rs that COMPUTES what each mutation should "
        "produce. That entry is registered cross-semio-implementation and does not discharge "
        "anything, because both halves of such a comparison descend from one reading of the DXF R12 "
        "specification.\n\n"
        "This entry is a different mechanism. The expected state is not computed at all — it is "
        "COMMITTED, as the after half of a byte-reproducible fixture (../🏭️generator/📜️script.ts + "
        "🦀️engine, one recipe per witnessable (mutation, outcome) coordinate). dxf 0.6 then parses "
        "the real DXF group-code stream on BOTH sides and the comparison is over what it recovered. "
        "The probes say so in their own header: \"Everything here MARSHALS and READS; nothing here "
        "applies a mutation or predicts what one should.\"\n\n"
        "So the judge is a third-party implementation of the format, and nothing in this repository "
        "predicts the answer it is judging. That is what makes it qualifying where the sibling entry "
        "is not.\n\n"
        "One honest narrowing, carried from the earlier research on this subset and re-verified here: "
        "set-header-var/remove-header-var are witnessed only for $INSBASE — dxf::Header is a fixed "
        "generated struct with no arbitrary $VAR slot, and no other header variable this subset could "
        "target survives an R12 save/reload through this reference library at all. A second, NEW "
        "finding from this retrofit: set-header-var's real production dispatch "
        "(🧬️schema/🔺️diff/🦀️component.rs's validate_named_targets, read directly) has no reachable "
        "rejection against any well-formed base document — both its modify-branch (target already "
        "present) and its add-branch (target absent) always validate — so this oracle's fixture "
        "corpus carries set-header-var-applied only, no set-header-var-rejected-*; see the ticket-root "
        "report for the full reachability argument. A third finding, also new: dxf 0.6's own LOADER "
        "(not its writer) resynthesizes a removed-but-still-referenced LAYER/LTYPE row with default "
        "values on read (ensure_layer_is_present / ensure_line_type_is_present, called from add_entity "
        "/ add_layer during parsing) — verified not to weaken the gate (expected and actual are read "
        "by the same loader, so a genuine regression still differs from the resynthesized default) "
        "but recorded because the projection shows a default-valued residual row, not a clean absence, "
        "for remove-layer-applied and remove-linetype-applied specifically."
    ),
}

COMPARISON_PIPELINE = {
    "id": "dxf-r12-reader-compare-v1",
    "description": (
        "Reads the subject's produced DXF and the fixture's own expected DXF with an independent dxf "
        "0.6 projection (../🏭️generator/🦀️engine's `project` subcommand, independently re-derived "
        "from ../🧪️oracle/🦀️component.rs's project_dxf_r12 shape, never imported from it), then "
        "compares their projections under semantic-dxf-r12-v1 (LAYER/STYLE/LTYPE name-keyed; BLOCKS "
        "and ENTITIES, including a block's own nested entity list, order-significant). GATING."
    ),
    "stages": [
        {
            "probe": "dxf-import",
            "description": "An independent reader accepts both files.",
            "inputs": ["expected-dxf", "actual-dxf"],
            "assertions": {"bothImport": True},
        },
        {
            "probe": "dxf-compare",
            "description": "Structural equality under semantic-dxf-r12-v1 — the operative equality.",
            "inputs": ["expected-dxf", "actual-dxf"],
            "assertions": {"equal": True},
        },
    ],
}

PROBE_COMMAND_PREFIX = ["bun", "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖊️dxf/🏅️standards/🔖️r12/🪆️subsets/✳️any/🔬️probes/📜️script.ts"]
PROBE_ENGINE = {"family": "dxf-rs", "implementation": "engine (dxf 0.6 + this artifact's own semantic-dxf-r12-v1 projection)", "version": "dxf@0.6.1"}

PROBES = [
    {
        "id": "dxf-import",
        "kind": "external-process",
        "ecosystem": "rust",
        "package": "dxf",
        "version": "0.6.1",
        "engine": PROBE_ENGINE,
        "capabilities": ["dxf.reader.import"],
        "outputSchema": "semio.repository-test.probe-report/v2",
        "deterministic": True,
        "license": "MIT",
        "testOnly": True,
        "productionReachable": False,
        "networkDuringExecution": False,
        "command": PROBE_COMMAND_PREFIX + ["dxf-import"],
        "rationale": "An INDEPENDENT reader (dxf 0.6's own group-code parser, via the standalone engine binary) accepts both files at all. Nothing downstream means anything if one of them does not parse.",
        "qualification": {
            "status": "qualified",
            "evidence": "Run against every generated fixture this session (38 recipes, 58 files) — bothImport true in every case; confirmed directly for insert-entity-applied/before.dxf + after.dxf.",
            "checkedAt": "2026-08-28",
            "criteria": [
                {"id": "reads-a-real-dxf", "met": True, "detail": "decodes HEADER, TABLES (LAYER/STYLE/LTYPE), BLOCKS, ENTITIES via dxf::Drawing::load"},
                {"id": "offline", "met": True, "detail": "engine depends only on dxf 0.6, resolved from the local cargo registry cache; no network during execution"},
            ],
        },
    },
    {
        "id": "dxf-project",
        "kind": "external-process",
        "ecosystem": "rust",
        "package": "dxf",
        "version": "0.6.1",
        "engine": PROBE_ENGINE,
        "capabilities": ["dxf.reader.project"],
        "outputSchema": "semio.repository-test.probe-report/v2",
        "deterministic": True,
        "license": "MIT",
        "testOnly": True,
        "productionReachable": False,
        "networkDuringExecution": False,
        "command": PROBE_COMMAND_PREFIX + ["dxf-project"],
        "rationale": "The typed projection semantic-dxf-r12-v1 is measured against — acadVersion, insertionBase, LAYER/STYLE/LTYPE rows, BLOCKS (name, base point, nested entity list), top-level ENTITIES, all with the exact field set ../🧪️oracle/🦀️component.rs's own project_dxf_r12 uses.",
        "qualification": {
            "status": "qualified",
            "evidence": "Run against insert-entity-applied/after.dxf: reports entityCount 8 (base 7 + 1 inserted circle at index 2), matching the real production InsertEntity dispatch's own index-addressed insert.",
            "checkedAt": "2026-08-28",
            "criteria": [
                {"id": "name-keyed-tables-ordered-entities", "met": True, "detail": "layers/styles/linetypes compare name-keyed (validate_named_targets is the real production semantics); blocks/entities compare positionally (validate_indexed_targets)"},
                {"id": "no-opaque-payloads-needed", "met": True, "detail": "unlike AVI's movi chunks, this subset's document has no large opaque binary payloads to hash — the typed projection is already the full comparable shape"},
            ],
        },
    },
    {
        "id": "dxf-compare",
        "kind": "external-process",
        "ecosystem": "rust",
        "package": "dxf",
        "version": "0.6.1",
        "engine": PROBE_ENGINE,
        "capabilities": ["dxf.reader.compare"],
        "outputSchema": "semio.repository-test.probe-report/v2",
        "deterministic": True,
        "license": "MIT",
        "testOnly": True,
        "productionReachable": False,
        "networkDuringExecution": False,
        "command": PROBE_COMMAND_PREFIX + ["dxf-compare"],
        "rationale": "Name-keyed-for-tables / ordered-for-blocks-and-entities structural equality over two independently-decoded projections, at semantic-dxf-r12-v1's own 1e-4 tolerance — the GATING comparison. Computes no mutation semantics, only structural equality of two already-existing byte blobs.",
        "qualification": {
            "status": "qualified",
            "evidence": (
                "Validated BOTH ways this session with real measured numbers: no-mutation-no-op's before.dxf vs "
                "after.dxf (byte-identical pair) -> {equal:true, diffCount:0}; set-style-applied's after.dxf vs "
                "before.dxf (one field, NOTES' font, deliberately different) -> {equal:false, diffCount:1, "
                "diffs:[\"$.styles[name=NOTES].font: \\\"arial.ttf\\\" \\u2260 \\\"romans.shx\\\"\"]}. Tolerance "
                "independently confirmed discriminating at the declared 1e-4: a circle radius perturbed by 1e-5 "
                "compares equal (0 diffs), perturbed by 1e-2 does not (1 diff, delta 0.009999999999990905)."
            ),
            "checkedAt": "2026-08-28",
            "criteria": [
                {"id": "accepts-a-known-good-pair", "met": True, "detail": "identical before/after bytes compare equal:true, diffCount:0"},
                {"id": "rejects-a-known-bad-pair", "met": True, "detail": "a single deliberately-wrong field compares equal:false, diffCount:1, and the diff names the exact field path"},
                {"id": "tolerance-discriminates", "met": True, "detail": "1e-5 radius perturbation accepted, 1e-2 rejected, at the profile's own declared 1e-4 threshold"},
            ],
        },
    },
]


def main() -> None:
    oracle = json.loads(ORACLE_JSON.read_text(encoding="utf-8"))
    new_manifests = json.loads(NEW_MANIFESTS.read_text(encoding="utf-8"))

    # 1) Register the new reader oracle — the existing cross-semio-implementation entry is untouched.
    if not any(o["id"] == READER_ORACLE_ID for o in oracle["oracles"]):
        oracle["oracles"].append(READER_ORACLE)

    # 2) Add the `pipeline` reference to the existing comparisonProfile (every other field untouched).
    for profile in oracle["comparisonProfiles"]:
        if profile["id"] == "semantic-dxf-r12-v1":
            profile["pipeline"] = COMPARISON_PIPELINE["id"]

    # 3) probes / comparisonPipelines are new top-level keys for this subset — add them.
    oracle["probes"] = PROBES
    oracle["comparisonPipelines"] = [COMPARISON_PIPELINE]

    # 4) Retarget every WITNESSABLE mutation's oracleRequirements[].oracle. All 19 declared kinds are
    #    witnessable (verified per-kind in the prior research and re-confirmed here); none is
    #    -uncarried at the KIND level. set-header-var keeps its declared ["applied","rejected"]
    #    outcomes (a structural/protocol-level declaration) — only its FIXTURE coverage is narrower,
    #    recorded in the ticket-root report and in the new oracle's own rationale above, never by
    #    inventing a schema field here.
    retargeted = 0
    for manifest in oracle["mutationManifests"]:
        for mutation in manifest["mutations"]:
            for requirement in mutation["oracleRequirements"]:
                if requirement.get("oracle") == CROSS_SEMIO_ORACLE_ID:
                    requirement["oracle"] = READER_ORACLE_ID
                    retargeted += 1
    print(f"retargeted {retargeted} oracleRequirements -> {READER_ORACLE_ID}")

    # 5) Merge fixtureManifests — drafting-plate's existing entry is left exactly as it was; every
    #    other new-manifest id not already present is appended.
    existing_ids = {f["id"] for f in oracle["fixtureManifests"]}
    added = 0
    for manifest in new_manifests:
        if manifest["id"] == "drafting-plate":
            continue  # pre-existing, untouched
        if manifest["id"] in existing_ids:
            continue
        oracle["fixtureManifests"].append(manifest)
        added += 1
    print(f"appended {added} new fixtureManifests (total now {len(oracle['fixtureManifests'])})")

    ORACLE_JSON.write_text(json.dumps(oracle, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")
    print(f"wrote {ORACLE_JSON}")


if __name__ == "__main__":
    main()
