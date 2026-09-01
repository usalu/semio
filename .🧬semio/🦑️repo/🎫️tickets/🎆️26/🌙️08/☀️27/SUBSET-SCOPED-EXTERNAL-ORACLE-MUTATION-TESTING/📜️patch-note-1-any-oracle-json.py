import json
import collections

ORACLE_PATH = "✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️oracle/🔣️.json"
MANIFESTS_PATH = "/tmp/note-manifests.json"
PROBE_SCRIPT = "✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/🔬️probes/📜️script.ts"

with open(ORACLE_PATH, "r", encoding="utf-8") as f:
    doc = json.load(f, object_pairs_hook=collections.OrderedDict)

with open(MANIFESTS_PATH, "r", encoding="utf-8") as f:
    fixture_manifests = json.load(f)

def probe(id_, carrier, command_subcommand, package, version, capability, engine_family, engine_impl, rationale, qualification):
    return collections.OrderedDict([
        ("id", id_),
        ("kind", "external-process"),
        ("ecosystem", "rust"),
        ("package", package),
        ("version", version),
        ("engine", collections.OrderedDict([("family", engine_family), ("implementation", engine_impl), ("version", version)])),
        ("capabilities", [capability]),
        ("outputSchema", "semio.repository-test.probe-report/v2"),
        ("deterministic", True),
        ("license", "MIT"),
        ("testOnly", True),
        ("productionReachable", False),
        ("networkDuringExecution", False),
        ("command", ["bun", PROBE_SCRIPT, command_subcommand]),
        ("rationale", rationale),
        ("qualification", qualification),
    ])

def qual(evidence, criteria):
    return collections.OrderedDict([
        ("status", "qualified"),
        ("evidence", evidence),
        ("criteria", criteria),
        ("checkedAt", "2026-08-28"),
    ])

probes = [
    probe(
        "note-dxf-project", "dxf", "dxf-project", "dxf", "0.6", "note.carrier.dxf", "dxf-rs", "dxf crate ascii reader/writer",
        "Reads DXF R12 bytes with the dxf crate and reports every LINE entity's (start,end) pair it recovers — nothing downstream means anything if the carrier does not parse at all. Shells to ../🏭️generator/🦀️note-oracle-codec, the SAME standalone crate that writes this subset's dxf fixtures.",
        qual(
            "📓️note-1-any-fixture-corpus.md — dxf-project run against creates-an-ink-block/after.dxf recovered lineCount 6 (3 Ink blocks × 2 segments each), matching the recipe's own point counts exactly.",
            [{"id": "reads-the-committed-corpus", "met": True, "detail": "dxf-project on creates-an-ink-block/after.dxf returns lineCount:6 with every (start,end) pair matching the recipe's authored points"}],
        ),
    ),
    probe(
        "note-svg-project", "svg", "svg-project", "quick-xml", "0.42", "note.carrier.svg", "quick-xml", "quick-xml pull-parser/writer",
        "Reads the SVG XML with quick-xml and reports every visible block's <g transform> + kind-specific content it recovers. Shells to ../🏭️generator/🦀️note-oracle-codec, the SAME standalone crate that writes this subset's svg fixtures.",
        qual(
            "📓️note-1-any-fixture-corpus.md — svg-project run against creates-an-ink-block/after.svg recovered blockCount 9 (8 base blocks + 1 new Ink block), each with its own decomposed transform matrix.",
            [{"id": "reads-the-committed-corpus", "met": True, "detail": "svg-project on creates-an-ink-block/after.svg returns blockCount:9, transforms decomposed to [a,b,c,d,e,f] per block, text/path/image/rect content per kind"}],
        ),
    ),
    probe(
        "note-pdf-project", "pdf", "pdf-project", "lopdf", "0.44", "note.carrier.pdf", "lopdf", "lopdf content-stream reader/writer",
        "Reads the PDF with lopdf and reports the Tj text operands it recovers from the single page's content stream. Shells to ../🏭️generator/🦀️note-oracle-codec, the SAME standalone crate that writes this subset's pdf fixtures.",
        qual(
            "📓️note-1-any-fixture-corpus.md — pdf-project run against retitles-the-document/after.pdf recovered text:[\"Project Kickoff Notes Welcome to the note.\"], title concatenated with the Text block's own content exactly as NoteIntoPdf's body specifies.",
            [{"id": "reads-the-committed-corpus", "met": True, "detail": "pdf-project on retitles-the-document/after.pdf returns pageCount:1, text:[\"Project Kickoff Notes Welcome to the note.\"]"}],
        ),
    ),
    probe(
        "note-dxf-compare", "dxf", "dxf-compare", "dxf", "0.6", "note.carrier.dxf", "dxf-rs", "dxf crate ascii reader/writer",
        "Reads EXPECTED and ACTUAL DXF bytes and reports whether the SET of (start,end) LINE pairs agree, naming every unmatched line — the semantic-note-dxf-ink-v1 comparison profile's own 'arrays: set' rule. Shells to ../🏭️generator/🦀️note-oracle-codec.",
        qual(
            "📓️note-1-any-fixture-corpus.md — validated BOTH ways: redraws-the-sketch-polyline/after.dxf vs itself -> agree:true, differenceCount:0; before.dxf vs after.dxf (a real point edit) -> agree:false, differenceCount:4, naming the exact unmatched start/end pairs on both sides.",
            [
                {"id": "accepts-a-known-good-pair", "met": True, "detail": "after.dxf vs after.dxf -> agree:true, differenceCount:0"},
                {"id": "rejects-a-known-bad-pair-by-name", "met": True, "detail": "before.dxf vs after.dxf -> agree:false, differenceCount:4, differences name the exact (start,end) pairs that do not match on either side"},
            ],
        ),
    ),
    probe(
        "note-svg-compare", "svg", "svg-compare", "quick-xml", "0.42", "note.carrier.svg", "quick-xml", "quick-xml pull-parser/writer",
        "Reads EXPECTED and ACTUAL SVG bytes and reports whether every block's transform + kind-specific content agree, in document order (semantic-note-svg-drawing-v1 declares no set-arrays, so order IS significant). Shells to ../🏭️generator/🦀️note-oracle-codec.",
        qual(
            "📓️note-1-any-fixture-corpus.md — validated BOTH ways: thickens-the-sketch-stroke/before.svg vs itself -> agree:true, differenceCount:0; before.svg vs after.svg (stroke-width 2->5) -> agree:false, differenceCount:1, naming block[1].strokeWidth differs: Some(2.0) vs Some(5.0).",
            [
                {"id": "accepts-a-known-good-pair", "met": True, "detail": "before.svg vs before.svg -> agree:true, differenceCount:0"},
                {"id": "rejects-a-known-bad-pair-by-name", "met": True, "detail": "before.svg vs after.svg -> agree:false, differenceCount:1, differences: ['block[1].strokeWidth differs: Some(2.0) vs Some(5.0)']"},
            ],
        ),
    ),
    probe(
        "note-pdf-compare", "pdf", "pdf-compare", "lopdf", "0.44", "note.carrier.pdf", "lopdf", "lopdf content-stream reader/writer",
        "Reads EXPECTED and ACTUAL PDF bytes and reports whether the extracted page text agrees exactly. Shells to ../🏭️generator/🦀️note-oracle-codec.",
        qual(
            "📓️note-1-any-fixture-corpus.md — validated BOTH ways: retitles-the-document/before.pdf vs itself -> agree:true, differenceCount:0; before.pdf vs after.pdf (title swap) -> agree:false, differenceCount:1, naming the exact expected/actual text strings.",
            [
                {"id": "accepts-a-known-good-pair", "met": True, "detail": "before.pdf vs before.pdf -> agree:true, differenceCount:0"},
                {"id": "rejects-a-known-bad-pair-by-name", "met": True, "detail": "before.pdf vs after.pdf -> agree:false, differenceCount:1, differences: ['page text differs: expected [\"Untitled Note Welcome to the note.\"] actual [\"Project Kickoff Notes Welcome to the note.\"]']"},
            ],
        ),
    ),
]

new_doc = collections.OrderedDict()
for key, value in doc.items():
    new_doc[key] = value
    if key == "oracles":
        new_doc["probes"] = probes
new_doc["fixtureManifests"] = fixture_manifests

with open(ORACLE_PATH, "w", encoding="utf-8") as f:
    json.dump(new_doc, f, indent=2, ensure_ascii=False)
    f.write("\n")

print("done:", list(new_doc.keys()))
print("probes:", len(new_doc["probes"]))
print("fixtureManifests:", len(new_doc["fixtureManifests"]))
