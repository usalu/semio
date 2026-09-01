#!/usr/bin/env python3
# 🧵️ One-shot patch script for the PNG 1.2/any oracle registration — adds the new reader oracle,
# comparisonProfiles/comparisonPipelines/probes, patches each mutation's oracleRequirements per the
# witnessable/uncarried split, and appends the 15 fixtureManifests entries. Run once; not idempotent
# if run twice (it would duplicate the oracle/profile/probe entries), so this is a ticket-temp
# scratch script, not a permanent one — CLAUDE.md's "no migration scripts" rule.
import json
import collections

PNG_ORACLE = "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/🧪️oracle/🔣️.json"
FIXTURES = ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️27/SUBSET-SCOPED-EXTERNAL-ORACLE-MUTATION-TESTING/🗑️temp/png-fixture-manifests.json"

with open(PNG_ORACLE, encoding="utf8") as f:
    doc = json.load(f, object_pairs_hook=collections.OrderedDict)

with open(FIXTURES, encoding="utf8") as f:
    fixtures = json.load(f)

WITNESSABLE = {
    "change-header", "replace-palette", "change-transparency", "change-gamma",
    "change-chromaticities", "change-srgb-intent", "change-physical-dims",
    "change-background", "insert-text-chunk", "remove-text-chunk",
    "replace-text-chunk", "replace-pixels",
}
UNCARRIED = {"change-timestamp", "insert-unknown-chunk", "remove-unknown-chunk"}
all_kinds = set(doc["mutationCatalogs"][0]["kinds"])
assert WITNESSABLE | UNCARRIED == all_kinds, (WITNESSABLE | UNCARRIED) ^ all_kinds
assert len(all_kinds) == 15

READER_ORACLE_ID = "png-png-1-2-mutate-reader"

# --- 1. New oracle entry -----------------------------------------------------------------------
reader_oracle = collections.OrderedDict([
    ("id", READER_ORACLE_ID),
    ("kind", "third-party-library"),
    ("ecosystem", "rust"),
    ("package", "png"),
    ("version", "0.18.1"),
    ("engine", collections.OrderedDict([("family", "png"), ("implementation", "png reader"), ("version", "0.18.1")])),
    ("capabilities", ["png-1-2-mutate"]),
    ("license", "MIT OR Apache-2.0"),
    ("testOnly", True),
    ("productionReachable", True),
    ("networkDuringExecution", False),
    ("platforms", ["darwin-arm64", "darwin-x64", "linux-x64", "linux-arm64", "win32-x64"]),
    ("rationale", (
        "📖️ A READER, and that is the whole point of registering it separately.\n\n"
        "This subset also carries a `🧪️oracle/🦀️component.rs` that COMPUTES what each mutation should produce. That entry is registered `cross-semio-implementation` and does not discharge anything, because both halves of such a comparison descend from one reading of the specification.\n\n"
        "This entry is a different mechanism. The expected state is not computed at all — it is COMMITTED, as the `after` half of a byte-reproducible fixture written directly by `png` 0.18.1's own `Encoder`/`Writer`. `png` then decodes both sides with its own `Decoder`, and the comparison is over what it recovered. The probes say so in their own header: \"Everything here MARSHALS and READS; nothing here applies a mutation or predicts what one should.\"\n\n"
        "So the judge is a third-party implementation of the format, and nothing in this repository predicts the answer it is judging. That is what makes it qualifying where the sibling `png-png-1-2-mutate` entry is not.\n\n"
        "PRODUCTION REACHABLE, measured rather than copied from the sibling entry's own claim: `🧰️framework/🛍️products/💻️os/🖥️host/🦀️component.rs` uses `png::{BitDepth, ColorType, Encoder}` directly to rasterize SVG to PNG for a real, registered OS media-export handler (`register_os_media_export_handler_kind(artifact_kind, \"png\", ...)`); `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🖼️images/🦀️component.rs` uses `png::Encoder`/`png::Decoder` directly in its own (non-test) image-editing engine. All three of `os/host`, `remodel` and `lowpoly`'s own `Cargo.toml` declare `png = \"0.17.16\"` as a real, non-optional, non-dev dependency. `🔒️dependencies.json`'s repo-root entry for `png` (`productionReachable: false`, version `0.17.16`) is STALE against this source evidence and against this subset's own pre-existing `png-png-1-2-mutate` registration, which already records `productionReachable: true` with a `productionDebt` naming the same OS host path — not corrected here because touching a shared, generated, repo-root ledger is out of this ticket's PNG-only scope, but the discrepancy is recorded so it is not mistaken for agreement.\n\n"
        "12 of this subset's 15 declared kinds are witnessable through `png::Info`'s public fields (`width`, `height`, `bit_depth`, `color_type`, `interlaced`, `palette`, `trns`, `gama_chunk`, `chrm_chunk`, `srgb`, `pixel_dims`, `bkgd`, `uncompressed_latin1_text`) plus the decoded pixel sample buffer for `replace-pixels`. Three are not: `png::Info` 0.18.1 has no `tIME` field at all, and the decoder skips unrecognised ancillary chunks entirely rather than surfacing them (`src/decoder/stream.rs`'s own `SkippedAncillaryChunk`, with no public accessor) — `change-timestamp`, `insert-unknown-chunk` and `remove-unknown-chunk` are registered `png-1-2-mutate-uncarried` rather than discharged. Full research: `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️27/SUBSET-SCOPED-EXTERNAL-ORACLE-MUTATION-TESTING/📓️png-reader-witnessability.md`, independently re-verified against the vendored crate source (`~/.cargo/registry/src/*/png-0.18.1/`) while building this retrofit."
    )),
])
doc["oracles"].append(reader_oracle)

# --- 2. noOracleDecisions: document the 3 uncarried kinds --------------------------------------
doc["noOracleDecisions"] = [collections.OrderedDict([
    ("id", "png-tIME-and-unknown-ancillary-chunks-uncarried"),
    ("capabilities", ["png-1-2-mutate-uncarried"]),
    ("rationale", (
        "The `png` 0.18.1 reader cannot witness three of this subset's fifteen kinds, for a crate-API reason rather than a carrier-export reason (contrast `sequence`'s `csv` case, where the export itself never writes the field). `change-timestamp`: `png::Info` has no `tIME` field in this version — confirmed absent from `src/common.rs`. `insert-unknown-chunk`/`remove-unknown-chunk`: the decoder's own `SkippedAncillaryChunk` path discards any chunk type it does not recognise before it ever reaches `Info`, and no public accessor exposes what was skipped. All three fixtures are still built — `png-codec`'s `Writer::write_chunk` raw escape hatch writes real tIME/unknown-chunk bytes for both `before` and `after` — so the byte-level material exists and is committed; what is missing is a public read path in this specific crate version to compare it back. See `📓️png-reader-witnessability.md` for the line-level crate-source evidence."
    )),
    ("substitutes", ["specification-vectors"]),
])]

# --- 3. comparisonProfiles ------------------------------------------------------------------
comparison_profile = collections.OrderedDict([
    ("id", "semantic-png-1-2-v1"),
    ("description", (
        "PNG 1.2 documents compared structurally through `png` 0.18.1's own decode: the four normative IHDR fields (width, height, bitDepth, colorType) plus interlaced; the palette as an ordered list of RGB triples (palette index is pixel-sample identity for an Indexed image); tRNS, gAMA (as its exact scaled integer, never a lossy float), cHRM (all eight scaled-integer coordinates), the sRGB rendering intent, pHYs, bKGD and every tEXt chunk as an ordered list of (keyword, text) pairs — all kept as real typed values so a diff names the exact field, per this fleet's own opaque-payload precedent. The decoded pixel sample buffer projects as a size+digest pair rather than raw bytes, the same treatment `semantic-avi-v1` gives a movi chunk payload. Distinct from the repository-wide `semantic-raster-v1` profile (defined in `✏️s/🔌️plugins/🗄️stdio/🧪️oracle/🔣️.json`), which explicitly IGNORES gamma and ancillary chunks — this profile exists because this subset's mutation vocabulary is made almost entirely of the fields that one deliberately canonicalizes away."
    )),
    ("arrays", "ordered"),
    ("ignoreKeys", []),
    ("pipeline", "png-1-2-crate-compare-v1"),
])
doc["comparisonProfiles"] = [comparison_profile]

# --- 4. probes ----------------------------------------------------------------------------------
def probe_command(name):
    return ["bun", "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/🔬️probes/📜️script.ts", name]

probes = [
    collections.OrderedDict([
        ("id", "png-import"),
        ("kind", "external-process"),
        ("ecosystem", "rust"),
        ("package", "png"),
        ("version", "0.18.1"),
        ("engine", collections.OrderedDict([("family", "png"), ("implementation", "png-codec (png 0.18.1)"), ("version", "png@0.18.1")])),
        ("capabilities", ["png.crate.import"]),
        ("outputSchema", "semio.repository-test.probe-report/v2"),
        ("deterministic", True),
        ("license", "MIT OR Apache-2.0"),
        ("testOnly", True),
        ("productionReachable", False),
        ("networkDuringExecution", False),
        ("command", probe_command("png-import")),
        ("rationale", "An INDEPENDENT reader (png-codec's own use of `png::Decoder`) accepts both files at all. Nothing downstream means anything if one of them does not parse."),
        ("qualification", collections.OrderedDict([
            ("status", "qualified"),
            ("evidence", "Run against every generated fixture this session (15 recipes, 30 files) — bothImport true in every case; confirmed directly for change-gamma-applied/before.png and change-gamma-applied/after.png."),
            ("checkedAt", "2026-08-28"),
            ("criteria", [
                collections.OrderedDict([("id", "reads-a-real-png"), ("met", True), ("detail", "decodes the PNG signature, IHDR, and every ancillary chunk this crate version surfaces through png::Decoder::read_info")]),
                collections.OrderedDict([("id", "offline"), ("met", True), ("detail", "png-codec depends only on png 0.18.1, resolved from the local cargo registry cache via --offline; no network during execution")]),
            ]),
        ])),
    ]),
    collections.OrderedDict([
        ("id", "png-project"),
        ("kind", "external-process"),
        ("ecosystem", "rust"),
        ("package", "png"),
        ("version", "0.18.1"),
        ("engine", collections.OrderedDict([("family", "png"), ("implementation", "png-codec (png 0.18.1)"), ("version", "png@0.18.1")])),
        ("capabilities", ["png.crate.project"]),
        ("outputSchema", "semio.repository-test.probe-report/v2"),
        ("deterministic", True),
        ("license", "MIT OR Apache-2.0"),
        ("testOnly", True),
        ("productionReachable", False),
        ("networkDuringExecution", False),
        ("command", probe_command("png-project")),
        ("rationale", "The typed projection semantic-png-1-2-v1 is measured against — header fields, palette, trns, gamma (exact scaled integer), chromaticities, srgb intent, physical dims, background, every tEXt chunk in order, and the decoded pixel buffer as size+digest."),
        ("qualification", collections.OrderedDict([
            ("status", "qualified"),
            ("evidence", "Run against replace-palette-applied/after.png: reports width 4, height 2, colorType \"indexed\", hasPalette true, with the projection's palette array holding the four replaced RGB triples ([255,255,0], [0,255,255], [255,0,255], [32,32,32]) verbatim."),
            ("checkedAt", "2026-08-28"),
            ("criteria", [
                collections.OrderedDict([("id", "small-fields-stay-typed"), ("met", True), ("detail", "palette entries, gamma, chromaticities, physical dims, header fields and tEXt keyword/text pairs project as their real values, matching semantic-png-1-2-v1's own field-level treatment")]),
                collections.OrderedDict([("id", "opaque-pixel-payload"), ("met", True), ("detail", "the decoded pixel sample buffer projects as size+digest, never raw bytes")]),
            ]),
        ])),
    ]),
    collections.OrderedDict([
        ("id", "png-compare"),
        ("kind", "external-process"),
        ("ecosystem", "rust"),
        ("package", "png"),
        ("version", "0.18.1"),
        ("engine", collections.OrderedDict([("family", "png"), ("implementation", "png-codec (png 0.18.1)"), ("version", "png@0.18.1")])),
        ("capabilities", ["png.crate.compare"]),
        ("outputSchema", "semio.repository-test.probe-report/v2"),
        ("deterministic", True),
        ("license", "MIT OR Apache-2.0"),
        ("testOnly", True),
        ("productionReachable", False),
        ("networkDuringExecution", False),
        ("command", probe_command("png-compare")),
        ("rationale", "Structural equality over two independently-decoded projections — the GATING comparison. Computes no mutation semantics, only structural equality of two already-existing byte blobs."),
        ("qualification", collections.OrderedDict([
            ("status", "qualified"),
            ("evidence", "Validated BOTH ways this session with real measured numbers: change-gamma-applied's before.png vs itself (byte-identical pair) -> {equal:true, diffCount:0}; change-gamma-applied's before.png vs after.png (gAMA deliberately differs, 45455 vs 100000) -> {equal:false, diffCount:1, diffs:[\"$.gamma: 45455 \\u2260 100000\"]}."),
            ("checkedAt", "2026-08-28"),
            ("criteria", [
                collections.OrderedDict([("id", "accepts-a-known-good-pair"), ("met", True), ("detail", "identical before/before bytes compare equal:true, diffCount:0")]),
                collections.OrderedDict([("id", "rejects-a-known-bad-pair"), ("met", True), ("detail", "a single deliberately-wrong field (gAMA) compares equal:false, diffCount:1, and the diff names the exact field path")]),
            ]),
        ])),
    ]),
]
doc["probes"] = probes

# --- 5. comparisonPipelines -----------------------------------------------------------------
doc["comparisonPipelines"] = [collections.OrderedDict([
    ("id", "png-1-2-crate-compare-v1"),
    ("description", "Reads the subject's produced PNG and the fixture's own expected PNG with an independent png-codec projection, then compares their ordered projections. GATING."),
    ("stages", [
        collections.OrderedDict([
            ("probe", "png-import"),
            ("description", "An independent reader accepts both files."),
            ("inputs", ["expected-png", "actual-png"]),
            ("assertions", {"bothImport": True}),
        ]),
        collections.OrderedDict([
            ("probe", "png-compare"),
            ("description", "Structural equality — the operative equality."),
            ("inputs", ["expected-png", "actual-png"]),
            ("assertions", {"equal": True}),
        ]),
    ]),
])]

# --- 6. Patch mutationManifests oracleRequirements -----------------------------------------
patched = {"witnessable": [], "uncarried": []}
for manifest in doc["mutationManifests"]:
    for mutation in manifest["mutations"]:
        mid = mutation["id"]
        reqs = mutation["oracleRequirements"]
        assert len(reqs) == 1, f"{mid} unexpected oracleRequirements shape"
        req = reqs[0]
        assert dict(req) == {"capability": "png-1-2-mutate", "qualifyingKind": "third-party-library"}, f"{mid} unexpected requirement {dict(req)}"
        if mid in WITNESSABLE:
            new_req = collections.OrderedDict([("capability", "png-1-2-mutate"), ("qualifyingKind", "third-party-library"), ("oracle", READER_ORACLE_ID)])
            patched["witnessable"].append(mid)
        elif mid in UNCARRIED:
            new_req = collections.OrderedDict([("capability", "png-1-2-mutate-uncarried"), ("qualifyingKind", "third-party-library")])
            patched["uncarried"].append(mid)
        else:
            raise AssertionError(f"{mid} not classified")
        mutation["oracleRequirements"] = [new_req]

print("witnessable:", sorted(patched["witnessable"]))
print("uncarried:", sorted(patched["uncarried"]))
assert len(patched["witnessable"]) == 12
assert len(patched["uncarried"]) == 3

# --- 7. fixtureManifests --------------------------------------------------------------------
doc["fixtureManifests"] = fixtures

# --- Reorder top-level keys to mirror the avi reference's own ordering ----------------------
ORDER = ["$schema", "schemaVersion", "_comment", "oracles", "noOracleDecisions", "comparisonProfiles", "mutationCatalogs", "probes", "comparisonPipelines", "mutationManifests", "fixtureManifests"]
assert set(ORDER) == set(doc.keys()), (set(ORDER) ^ set(doc.keys()))
reordered = collections.OrderedDict((k, doc[k]) for k in ORDER)

with open(PNG_ORACLE, "w", encoding="utf8") as f:
    json.dump(reordered, f, indent=2, ensure_ascii=False)
    f.write("\n")

print("patched", PNG_ORACLE)
