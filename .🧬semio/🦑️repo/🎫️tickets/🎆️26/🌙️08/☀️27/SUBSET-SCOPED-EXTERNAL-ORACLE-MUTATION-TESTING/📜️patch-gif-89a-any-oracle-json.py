#!/usr/bin/env python3
import json
import sys

PATH = sys.argv[1]

with open(PATH, "r", encoding="utf-8") as f:
    data = json.load(f)

# ── 1. New reader oracle ────────────────────────────────────────────────────
reader_oracle = {
    "id": "gif-89a-any-mutate-reader",
    "kind": "third-party-library",
    "ecosystem": "rust",
    "package": "gif",
    "version": "0.13",
    "engine": {"family": "gif", "implementation": "gif reader (Decoder's own public API only)", "version": "0.13.3"},
    "capabilities": ["gif-89a-mutate"],
    "license": "MIT OR Apache-2.0",
    "testOnly": True,
    "productionReachable": False,
    "networkDuringExecution": False,
    "platforms": ["darwin-arm64", "darwin-x64", "linux-x64", "linux-arm64", "win32-x64"],
    "rationale": (
        "📖️ A READER, and that is the whole point of registering it separately.\n\n"
        "This subset also carries a `🧪️oracle/🦀️component.rs` that COMPUTES what each mutation "
        "should produce, using an owned model that mirrors the `gif` crate's own surface plus "
        "hand-rolled fixed-grammar block walking to fill two gaps the crate's high-level API "
        "leaves (comment/application extensions; the background-colour-index/pixel-aspect-ratio "
        "header scalars). That entry is registered `cross-semio-implementation` and does not "
        "discharge anything, because both halves of such a comparison descend from one reading of "
        "the specification.\n\n"
        "This entry is a different mechanism. The expected state is not computed at all — it is "
        "COMMITTED, as the `after` half of a byte-reproducible fixture built directly with "
        "`gif::Encoder`. `gif` 0.13 then decodes BOTH sides through `gif::Decoder`'s own PUBLIC "
        "API — `width`/`height`/`global_palette`/`bg_color`/`repeat`, and per frame "
        "`next_frame_info`+`read_into_buffer` — and the comparison is over what it recovered. The "
        "probes say so in their own header: \"Everything here MARSHALS and READS; nothing here "
        "applies a mutation or predicts what one should.\"\n\n"
        "This reader deliberately does NOT reach for the raw-block-scan technique "
        "`component.rs` uses: doing so would just be a second hand-rolled GIF parser wearing a "
        "reader's name. Where the crate's public surface genuinely cannot answer — the "
        "pixel-aspect-ratio byte, comment extension text, application extension payloads — the "
        "corresponding kind is registered `<capability>-uncarried` in this file's own "
        "`mutationManifests`, not routed around. One real, source-verified finding this reader "
        "recovers that `component.rs`'s own broader claim overstates: the per-frame interlace "
        "flag IS publicly readable, via `Decoder::next_frame_info` before pixel decoding — see "
        "`🏭️generator/🦀️engine/src/bin/reader.rs`'s own header docstring.\n\n"
        "So the judge is a third-party implementation of the format, and nothing in this "
        "repository predicts the answer it is judging. That is what makes it qualifying where the "
        "sibling entry is not."
    ),
}
oracle_ids = {o["id"] for o in data["oracles"]}
if reader_oracle["id"] not in oracle_ids:
    data["oracles"].append(reader_oracle)

# ── 2. comparisonProfiles (local, mirrors avi's semantic-avi-v1) ───────────
comparison_profile = {
    "id": "semantic-gif-89a-reader-v1",
    "description": (
        "GIF89a documents compared through gif::Decoder's PUBLIC API only: logical screen "
        "width/height, background colour index, NETSCAPE2.0 loop count, global colour table "
        "(size + digest of its raw RGB bytes), and every frame IN ORDER (frame index is semantic "
        "identity — there is no other stable key). Each frame projects left/top/width/height, the "
        "interlace flag (recovered via Decoder::next_frame_info before pixel decode — see "
        "reader.rs), local palette (size + digest), delay, disposal, transparent index, "
        "needs-user-input, and a size+digest of its decoded (always natural-row-order) palette "
        "index buffer rather than raw bytes. Deliberately OMITTED, because gif::Decoder's public "
        "surface cannot recover them at all: pixel-aspect-ratio, comment extension text, "
        "application extension payloads — the mutations that only move those are registered "
        "`gif-89a-mutate-uncarried` instead of being forced through this profile.",
    ),
    "arrays": "ordered",
    "ignoreKeys": [],
    "pipeline": "gif-89a-reader-compare-v1",
}
# description was accidentally a 1-tuple above; fix to plain string
comparison_profile["description"] = comparison_profile["description"][0]

if data.get("comparisonProfiles") in (None, []):
    data["comparisonProfiles"] = [comparison_profile]
else:
    existing_ids = {p["id"] for p in data["comparisonProfiles"]}
    if comparison_profile["id"] not in existing_ids:
        data["comparisonProfiles"].append(comparison_profile)

# ── 3. probes ────────────────────────────────────────────────────────────
probe_command_prefix = ["bun", "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/🏅️standards/🔖️89a/🪆️subsets/✳️any/🔬️probes/📜️script.ts"]

probes = [
    {
        "id": "gif-import",
        "kind": "external-process",
        "ecosystem": "rust",
        "package": "gif",
        "version": "0.13.3",
        "engine": {"family": "gif", "implementation": "gif-89a-reader (gif 0.13 + Decoder's own public API only)", "version": "gif@0.13.3"},
        "capabilities": ["gif.reader.import"],
        "outputSchema": "semio.repository-test.probe-report/v2",
        "deterministic": True,
        "license": "MIT OR Apache-2.0",
        "testOnly": True,
        "productionReachable": False,
        "networkDuringExecution": False,
        "command": probe_command_prefix + ["gif-import"],
        "rationale": "An INDEPENDENT reader (gif::Decoder's own public API, via the standalone reader binary) accepts both files at all. Nothing downstream means anything if one of them does not parse.",
        "qualification": {
            "status": "qualified",
            "evidence": "Run against every generated recipe this session (17 recipes, 34 files) — bothImport true in every case; confirmed directly for set-frame-interlace-applied/before.gif.",
            "checkedAt": "2026-08-28",
            "criteria": [
                {"id": "reads-a-real-gif", "met": True, "detail": "decodes GIF magic, logical screen descriptor, global colour table, every frame's image descriptor + pixel data, graphic control extension, NETSCAPE2.0 loop extension"},
                {"id": "offline", "met": True, "detail": "the reader binary depends only on gif 0.13, resolved from the local cargo registry cache; no network during execution"},
            ],
        },
    },
    {
        "id": "gif-project",
        "kind": "external-process",
        "ecosystem": "rust",
        "package": "gif",
        "version": "0.13.3",
        "engine": {"family": "gif", "implementation": "gif-89a-reader (gif 0.13 + Decoder's own public API only)", "version": "gif@0.13.3"},
        "capabilities": ["gif.reader.project"],
        "outputSchema": "semio.repository-test.probe-report/v2",
        "deterministic": True,
        "license": "MIT OR Apache-2.0",
        "testOnly": True,
        "productionReachable": False,
        "networkDuringExecution": False,
        "command": probe_command_prefix + ["gif-project"],
        "rationale": "The typed projection semantic-gif-89a-reader-v1 is measured against — width/height/backgroundColorIndex/loopCount/global palette, every frame in order with full geometry/interlace/palette/delay/disposal/transparency/userInput, opaque payloads as size+digest.",
        "qualification": {
            "status": "qualified",
            "evidence": "Run against set-frame-interlace-applied/after.gif: reports frameCount 3, backgroundColorIndex 2, loopCount 3, with frames[0].interlaced true and frames[0].indicesDigest identical to the pre-mutation projection (only the flag moved) — the reader's own header docstring documents why this is source-verified rather than assumed.",
            "checkedAt": "2026-08-28",
            "criteria": [
                {"id": "positional-not-keyed", "met": True, "detail": "frames project as an ordered array, matching semantic-gif-89a-reader-v1's own arrays:\"ordered\" rule — frame index is this format's semantic identity"},
                {"id": "opaque-payloads", "met": True, "detail": "global/local palette bytes and per-frame decoded pixel-index bytes project as size+digest, never raw bytes"},
                {"id": "witnessability-boundary-honoured", "met": True, "detail": "aspectRatio/comments/appExtensions are absent from the projection entirely, matching the 5 kinds registered `gif-89a-mutate-uncarried` rather than silently reporting an empty ok"},
            ],
        },
    },
    {
        "id": "gif-compare",
        "kind": "external-process",
        "ecosystem": "rust",
        "package": "gif",
        "version": "0.13.3",
        "engine": {"family": "gif", "implementation": "gif-89a-reader (gif 0.13 + Decoder's own public API only)", "version": "gif@0.13.3"},
        "capabilities": ["gif.reader.compare"],
        "outputSchema": "semio.repository-test.probe-report/v2",
        "deterministic": True,
        "license": "MIT OR Apache-2.0",
        "testOnly": True,
        "productionReachable": False,
        "networkDuringExecution": False,
        "command": probe_command_prefix + ["gif-compare"],
        "rationale": "Ordered structural equality over two independently-decoded projections — the GATING comparison. Computes no mutation semantics, only structural equality of two already-existing byte blobs.",
        "qualification": {
            "status": "qualified",
            "evidence": "Validated BOTH ways this session with real measured numbers: no-mutation-no-op's before.gif vs after.gif (byte-identical pair) -> {equal:true, diffCount:0}; set-background-color-index-applied's before.gif vs after.gif (the one field the recipe deliberately changes) -> {equal:false, diffCount:1, diffs:[\"$.backgroundColorIndex: 2 \\u2260 5\"]}; set-frame-delay-applied's before.gif vs after.gif -> {equal:false, diffCount:1, diffs:[\"$.frames[0].delayCs: 10 \\u2260 99\"]}.",
            "checkedAt": "2026-08-28",
            "criteria": [
                {"id": "accepts-a-known-good-pair", "met": True, "detail": "identical before/after bytes compare equal:true, diffCount:0"},
                {"id": "rejects-a-known-bad-pair", "met": True, "detail": "a single deliberately-wrong field compares equal:false, diffCount:1, and the diff names the exact field path"},
            ],
        },
    },
]
if data.get("probes") in (None, []):
    data["probes"] = probes
else:
    existing_probe_ids = {p["id"] for p in data["probes"]}
    for p in probes:
        if p["id"] not in existing_probe_ids:
            data["probes"].append(p)

# ── 4. comparisonPipelines ──────────────────────────────────────────────────
pipeline = {
    "id": "gif-89a-reader-compare-v1",
    "description": "Reads the subject's produced GIF and the fixture's own expected GIF with an independent gif::Decoder-public-API-only projection, then compares their ordered projections. GATING.",
    "stages": [
        {"probe": "gif-import", "description": "An independent reader accepts both files.", "inputs": ["expected-gif", "actual-gif"], "assertions": {"bothImport": True}},
        {"probe": "gif-compare", "description": "Ordered structural equality — the operative equality.", "inputs": ["expected-gif", "actual-gif"], "assertions": {"equal": True}},
    ],
}
if data.get("comparisonPipelines") in (None, []):
    data["comparisonPipelines"] = [pipeline]
else:
    existing_pipeline_ids = {p["id"] for p in data["comparisonPipelines"]}
    if pipeline["id"] not in existing_pipeline_ids:
        data["comparisonPipelines"].append(pipeline)

# ── 5. mutationManifests: fix outcomes + retarget oracleRequirements ───────
WITNESSABLE_OUTCOMES = {
    "no-mutation": ["no-op"],
    "set-snapshot": ["applied", "no-op"],
}
UNCARRIED = {"set-pixel-aspect-ratio", "insert-comment", "remove-comment", "add-app-extension", "remove-app-extension"}

mm = data["mutationManifests"]
mutations = mm[0]["mutations"] if isinstance(mm, list) else mm["mutations"]

changed_outcomes = []
changed_requirements = []
for m in mutations:
    kind = m["id"]
    # outcomes: derived from the real dispatch (see ticket report) — every kind wraps uniformly
    # in MutationOutcome::new(...), no per-kind rejection branch anywhere in this subset.
    new_outcomes = WITNESSABLE_OUTCOMES.get(kind, ["applied"])
    if m["outcomes"] != new_outcomes:
        changed_outcomes.append((kind, m["outcomes"], new_outcomes))
        m["outcomes"] = new_outcomes

    old_req = m["oracleRequirements"][0]
    if kind in UNCARRIED:
        new_req = {"capability": "gif-89a-mutate-uncarried", "qualifyingKind": "third-party-library"}
    else:
        new_req = {"capability": "gif-89a-mutate", "qualifyingKind": "third-party-library", "oracle": "gif-89a-any-mutate-reader"}
    if old_req != new_req:
        changed_requirements.append((kind, old_req, new_req))
        m["oracleRequirements"] = [new_req]

print("=== outcomes changed ===")
for kind, old, new in changed_outcomes:
    print(f"  {kind}: {old} -> {new}")
print("=== oracleRequirements changed ===")
for kind, old, new in changed_requirements:
    print(f"  {kind}: {old} -> {new}")

# ── 6. fixtureManifests: append the 17 new recipe entries (merge, never overwrite) ──
FIXTURES_DIR_REL = "../🧫️fixtures"


def file_entry(role, recipe_id, filename, media_type="image/gif"):
    import hashlib
    from pathlib import Path

    base = Path(PATH).parent / "../🧫️fixtures" / recipe_id / filename
    base = base.resolve()
    b = base.read_bytes()
    digest = hashlib.sha256(b).hexdigest()
    return {"role": role, "path": f"{FIXTURES_DIR_REL}/{recipe_id}/{filename}", "mediaType": media_type, "sha256": f"sha256:{digest}", "bytes": len(b)}


RECIPES = [
    ("no-mutation-no-op", "no-mutation", "no-op", "Identity — before and after bytes are the same document; no-mutation's diff is unconditionally a no-op."),
    ("set-snapshot-applied", "set-snapshot", "applied", "Whole-document replace: screen size, palette, background index, loop count and frames all change together."),
    ("set-snapshot-no-op", "set-snapshot", "no-op", "Replacement snapshot is byte-identical to the current one — the dispatch's own documented no-op/warn branch."),
    ("set-screen-size-applied", "set-screen-size", "applied", "Only the logical screen width/height change."),
    ("set-global-color-table-applied", "set-global-color-table", "applied", "The global colour table is replaced with a different palette."),
    ("set-background-color-index-applied", "set-background-color-index", "applied", "Only the background colour index scalar changes — readable via Decoder::bg_color, a real public getter, even though the encoder has no setter for it."),
    ("set-loop-count-applied", "set-loop-count", "applied", "Only the NETSCAPE2.0 loop count changes."),
    ("insert-frame-applied", "insert-frame", "applied", "A fourth frame is appended."),
    ("remove-frame-applied", "remove-frame", "applied", "The middle (index 1) frame is removed."),
    ("move-frame-applied", "move-frame", "applied", "The first frame is moved to the end."),
    ("set-frame-geometry-applied", "set-frame-geometry", "applied", "Frame 0's left/top offset changes; width/height/pixels untouched."),
    ("set-frame-pixels-applied", "set-frame-pixels", "applied", "Frame 0's palette-index buffer is replaced, same geometry."),
    ("set-frame-interlace-applied", "set-frame-interlace", "applied", "Frame 0's interlace flag flips; rows are re-stored in GIF's 4-pass order. The reader recovers the flag via Decoder::next_frame_info (before pixel decode)."),
    ("set-frame-delay-applied", "set-frame-delay", "applied", "Frame 0's delay changes."),
    ("set-frame-disposal-applied", "set-frame-disposal", "applied", "Frame 0's disposal method changes."),
    ("set-frame-transparency-applied", "set-frame-transparency", "applied", "Frame 0 gains a transparent index."),
    ("set-frame-user-input-applied", "set-frame-user-input", "applied", "Frame 0's needs-user-input flag flips."),
]

fixture_manifests = data["fixtureManifests"]
existing_fixture_ids = {f["id"] for f in fixture_manifests}

platform = "darwin-arm64"
added = 0
for recipe_id, mutation, outcome, notes in RECIPES:
    if recipe_id in existing_fixture_ids:
        continue
    entry = {
        "schema": "semio.repository-test.fixture/v2",
        "id": recipe_id,
        "class": "third-party-generated",
        "target": {"artifact": "s.stdio.gif", "standard": "89a", "subset": "any"},
        "mutation": mutation,
        "outcome": outcome,
        "units": {"length": "unitless", "angle": "degree"},
        "files": [file_entry("expected-before-gif", recipe_id, "before.gif"), file_entry("expected-after-gif", recipe_id, "after.gif")],
        "generator": {
            "oracle": "gif-89a-any-mutate-reader",
            "packageVersion": "0.13.3",
            "engineFamily": "gif",
            "engineVersion": "0.13.3",
            "command": f"bun ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/🏅️standards/🔖️89a/🪆️subsets/✳️any/🏭️generator/📜️script.ts build --only {recipe_id}",
            "platform": platform,
        },
        "provenance": {"source": "generated", "license": "MIT OR Apache-2.0 (gif)", "attribution": "Generated with gif (MIT OR Apache-2.0) via the standalone reader binary in 🏭️generator/🦀️engine/src/bin/reader.rs", "security": "scanned-clean", "privacy": "no-personal-data"},
        "comparisonProfile": "semantic-gif-89a-reader-v1",
        "reproducible": True,
        "family": "structural",
        "notes": notes,
    }
    fixture_manifests.append(entry)
    added += 1

print(f"=== fixtureManifests: {added} new entries appended (pattern-strip untouched) ===")

with open(PATH, "w", encoding="utf-8") as f:
    json.dump(data, f, indent=2, ensure_ascii=False)
    f.write("\n")

print("wrote", PATH)
