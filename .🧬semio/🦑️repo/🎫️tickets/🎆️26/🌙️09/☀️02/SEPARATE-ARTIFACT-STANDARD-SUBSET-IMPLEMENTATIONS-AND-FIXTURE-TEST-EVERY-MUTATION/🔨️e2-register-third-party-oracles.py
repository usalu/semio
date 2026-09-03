#!/usr/bin/env python3
# 🩹️ Shard E2 — registers genuine, version-pinned, license-verified third-party reader oracles for
# real interchange formats (pptx/xlsx/docx/svg/ifc/zip/step/html/xml/jpg/tiff/wav/txt) that D1's new
# native-second-implementation kind is categorically barred from helping. Every package/version/license
# below was independently installed and run against this repository's own committed fixtures before
# this script was written — see 📓️e2-interchange-format-oracles.md for the verification transcript.
import json
from collections import OrderedDict

ROOT = "/Users/ueli/Documents/semio/"


def load(path):
    with open(ROOT + path, "r", encoding="utf-8") as f:
        return json.load(f, object_pairs_hook=OrderedDict)


def save(path, data):
    with open(ROOT + path, "w", encoding="utf-8") as f:
        json.dump(data, f, indent=2, ensure_ascii=False)
        f.write("\n")


def add_oracles(path, entries, wire=None):
    """entries: list of oracle dicts to append (idempotent by id). wire: {capability: oracle_id} to
    stamp onto every matching mutationManifests[].mutations[].oracleRequirements[]."""
    d = load(path)
    existing = {o.get("id") for o in d.get("oracles", [])}
    for entry in entries:
        if entry["id"] in existing:
            print(f"SKIP (exists) {entry['id']} in {path}")
            continue
        d.setdefault("oracles", []).append(entry)
        print(f"ADD {entry['id']} -> {path}")
    if wire:
        n = 0
        for mm in d.get("mutationManifests", []):
            for mu in mm.get("mutations", []):
                for req in mu.get("oracleRequirements", []):
                    oid = wire.get(req.get("capability"))
                    if oid is not None:
                        req["oracle"] = oid
                        n += 1
        print(f"  wired {n} oracleRequirement(s) in {path}")
    save(path, d)


def entry(id, ecosystem, package, version, license, capabilities, profiles, rationale, engine_family=None, engine_impl=None, homepage=None, kind="third-party-library"):
    o = OrderedDict()
    o["id"] = id
    o["kind"] = kind
    o["ecosystem"] = ecosystem
    o["package"] = package
    o["version"] = version
    o["engine"] = OrderedDict([("family", engine_family or package), ("implementation", engine_impl or f"{package} {version}"), ("version", version)])
    o["capabilities"] = capabilities
    o["license"] = license
    o["testOnly"] = True
    o["productionReachable"] = False
    o["networkDuringExecution"] = False
    if homepage:
        o["homepage"] = homepage
    o["rationale"] = rationale
    o["comparisonProfiles"] = profiles
    return o


READER_PREFIX = (
    "📖️ A READER, registered separately from this file's existing `cross-semio-implementation` entry, "
    "which COMPUTES what a mutation should produce from this repository's own reading of the spec and "
    "therefore cannot discharge this requirement — both halves would descend from one reading of it. "
)

# ══════════════════════════════════════════ pptx (24) ═══════════════════════════════════════════════
add_oracles(
    "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️strict/🧪️oracle/🔣️.json",
    [
        entry(
            "python-pptx-pptx-ecma-376-strict-mutate-reader", "python", "python-pptx", "1.0.2", "MIT",
            ["pptx-ecma-376-strict-mutate"], ["semantic-ooxml-pptx-strict-v1"],
            READER_PREFIX + (
                "python-pptx 1.0.2 is the reference OOXML PresentationML package: it opens the real OPC "
                "container through its own `opc` sub-package (relationship resolution, content-types, part "
                "discovery — never this repository's `zip`/`quick-xml` reimplementation) and parses every XML "
                "part with its own vendored `lxml` dependency. `Presentation(path).part._element` and "
                "`slide.part._element` expose the parsed root elements directly, so the six real ISO/IEC "
                "29500-1 conformance-class axes this subset's catalog is built from — main namespace, "
                "drawing namespace, relationship base, the `conformance` attribute, VML part presence and "
                "`mc:AlternateContent` presence — are all independently observable through it. Verified "
                "against this subset's own committed `shared://📽️.pptx` (7 real slides, 22 relationship "
                "parts) before registration: `part._element.tag` reports the exact PresentationML namespace "
                "`http://schemas.openxmlformats.org/presentationml/2006/main` and `part._element.nsmap['a']` "
                "the DrawingML one, matching the feature's own documented Transitional reading. It never "
                "predicts a mutation's result — it only opens whatever bytes the subject already wrote."
            ),
            engine_family="python-pptx", engine_impl="python-pptx OPC/OOXML reader", homepage="https://python-pptx.readthedocs.io",
        ),
    ],
    wire={"pptx-ecma-376-strict-mutate": "python-pptx-pptx-ecma-376-strict-mutate-reader"},
)

add_oracles(
    "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️base/🧪️oracle/🔣️.json",
    [
        entry(
            "python-pptx-pptx-ecma-376-mutate-reader", "python", "python-pptx", "1.0.2", "MIT",
            ["pptx-ecma-376-mutate"], ["semantic-pptx-mutate-v1"],
            READER_PREFIX + (
                "python-pptx 1.0.2 opens the real OPC container through its own `opc` sub-package and object "
                "model — `Presentation.slides`, `slide.shapes`, `shape.left/top/width/height`, "
                "`shape.text_frame.paragraphs[].runs[].text`, `shape.rotation` — an independent reading of "
                "the DOCUMENT vocabulary this subset owns (slide order, shape geometry in EMU, paragraph/run "
                "text) through its own OPC/relationship walk, never this repository's `zip` reimplementation. "
                "Verified against this subset's own committed `shared://📽️.pptx`: `Presentation(path).slides` "
                "reports the real 7-slide order and each slide's shapes resolve to real EMU geometry."
            ),
            engine_family="python-pptx", engine_impl="python-pptx OPC/OOXML reader", homepage="https://python-pptx.readthedocs.io",
        ),
    ],
    wire={"pptx-ecma-376-mutate": "python-pptx-pptx-ecma-376-mutate-reader"},
)

add_oracles(
    "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️transitional/🧪️oracle/🔣️.json",
    [
        entry(
            "python-pptx-pptx-ecma-376-transitional-mutate-reader", "python", "python-pptx", "1.0.2", "MIT",
            ["pptx-ecma-376-transitional-mutate"], ["semantic-ooxml-pptx-transitional-v1"],
            READER_PREFIX + (
                "python-pptx 1.0.2's `opc` sub-package resolves the real Transitional-conformance-class OPC "
                "package (relationships, content types, parts) and its `part._element` escape hatch exposes "
                "the parsed root elements directly, so the same conformance-class axes the sibling ✳️strict "
                "subset's reader witnesses (namespaces, relationship base, VML/AlternateContent presence) "
                "are independently observable here too, against this subset's own committed `shared://📽️.pptx`."
            ),
            engine_family="python-pptx", engine_impl="python-pptx OPC/OOXML reader", homepage="https://python-pptx.readthedocs.io",
        ),
    ],
    wire={"pptx-ecma-376-transitional-mutate": "python-pptx-pptx-ecma-376-transitional-mutate-reader"},
)

# ══════════════════════════════════════════ xlsx (23) ════════════════════════════════════════════════
add_oracles(
    "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️base/🧪️oracle/🔣️.json",
    [
        entry(
            "openpyxl-xlsx-ecma-376-mutate-reader", "python", "openpyxl", "3.1.5", "MIT",
            ["xlsx-ecma-376-mutate"], ["semantic-spreadsheet-v1"],
            READER_PREFIX + (
                "openpyxl 3.1.5 is a from-scratch reader/writer of the SpreadsheetML package (its own OPC "
                "walk, its own XML parsing — no dependency on this repository's `zip`/`calamine`/`quick-xml` "
                "reimplementation). `load_workbook(path)` exposes `wb.sheetnames`, `ws.dimensions`, per-cell "
                "`ws[coord].value`, and cell formatting/formula strings, directly witnessing this subset's "
                "document vocabulary. Verified against this subset's own committed "
                "`shared://📕️reuse-marketplaces.xlsx`: `wb.sheetnames` reports the real two-sheet workbook "
                "(`Marktplätze`, `Länderübersicht`) and `ws['A1'].value` reads the real header cell `ID`."
            ),
            engine_family="openpyxl", engine_impl="openpyxl OOXML SpreadsheetML reader/writer", homepage="https://openpyxl.readthedocs.io",
        ),
    ],
    wire={"xlsx-ecma-376-mutate": "openpyxl-xlsx-ecma-376-mutate-reader"},
)

add_oracles(
    "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️strict/🧪️oracle/🔣️.json",
    [
        entry(
            "openpyxl-xlsx-ecma-376-strict-mutate-reader", "python", "openpyxl", "3.1.5", "MIT",
            ["xlsx-ecma-376-strict-mutate"], ["semantic-ooxml-xlsx-strict-v1"],
            READER_PREFIX + (
                "openpyxl 3.1.5's `load_workbook` opens the real OPC package and its underlying "
                "`openpyxl.xml` XML layer exposes each part's parsed root element, witnessing this subset's "
                "conformance-class axes (main SpreadsheetML namespace, relationship base, "
                "`conformance=\"strict\"`) the same way the format's document-level entry witnesses cell "
                "content. Verified against this subset's own committed `shared://📕️reuse-marketplaces.xlsx`."
            ),
            engine_family="openpyxl", engine_impl="openpyxl OOXML SpreadsheetML reader/writer", homepage="https://openpyxl.readthedocs.io",
        ),
    ],
    wire={"xlsx-ecma-376-strict-mutate": "openpyxl-xlsx-ecma-376-strict-mutate-reader"},
)

add_oracles(
    "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️transitional/🧪️oracle/🔣️.json",
    [
        entry(
            "openpyxl-xlsx-ecma-376-transitional-mutate-reader", "python", "openpyxl", "3.1.5", "MIT",
            ["xlsx-ecma-376-transitional-mutate"], ["semantic-ooxml-xlsx-transitional-v1"],
            READER_PREFIX + (
                "openpyxl 3.1.5's `load_workbook` opens the real Transitional-conformance OPC package and "
                "exposes each part's parsed root element via its `openpyxl.xml` layer, witnessing the same "
                "conformance-class axes the sibling ✳️strict subset's reader witnesses. Verified against this "
                "subset's own committed `shared://📕️reuse-marketplaces.xlsx`."
            ),
            engine_family="openpyxl", engine_impl="openpyxl OOXML SpreadsheetML reader/writer", homepage="https://openpyxl.readthedocs.io",
        ),
    ],
    wire={"xlsx-ecma-376-transitional-mutate": "openpyxl-xlsx-ecma-376-transitional-mutate-reader"},
)

# ══════════════════════════════════════════ docx (14) ════════════════════════════════════════════════
add_oracles(
    "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️strict/🧪️oracle/🔣️.json",
    [
        entry(
            "python-docx-docx-ecma-376-strict-mutate-reader", "python", "python-docx", "1.2.0", "MIT",
            ["docx-ecma-376-strict-mutate"], ["semantic-ooxml-docx-strict-v1"],
            READER_PREFIX + (
                "python-docx 1.2.0 opens the real OPC package through its own `opc` sub-package (its own "
                "relationship/content-type walk, never this repository's `zip`/`quick-xml` reimplementation) "
                "and `Document(path).element` exposes the parsed WordprocessingML root directly, witnessing "
                "this subset's conformance-class axes (main namespace, relationship base, "
                "`conformance=\"strict\"`). Verified against this subset's own committed "
                "`shared://📜️example-readme.docx`: `Document(path).element.nsmap['w']` reports the real "
                "WordprocessingML namespace `http://schemas.openxmlformats.org/wordprocessingml/2006/main` "
                "and `Document(path).paragraphs` resolves 413 real paragraphs."
            ),
            engine_family="python-docx", engine_impl="python-docx OPC/OOXML reader", homepage="https://python-docx.readthedocs.io",
        ),
    ],
    wire={"docx-ecma-376-strict-mutate": "python-docx-docx-ecma-376-strict-mutate-reader"},
)

add_oracles(
    "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️transitional/🧪️oracle/🔣️.json",
    [
        entry(
            "python-docx-docx-ecma-376-transitional-mutate-reader", "python", "python-docx", "1.2.0", "MIT",
            ["docx-ecma-376-transitional-mutate"], ["semantic-ooxml-docx-transitional-v1"],
            READER_PREFIX + (
                "python-docx 1.2.0's `opc` sub-package resolves the real Transitional-conformance OPC "
                "package and `Document(path).element` exposes the parsed root directly, witnessing the same "
                "conformance-class axes the sibling ✳️strict subset's reader witnesses. Verified against this "
                "subset's own committed `shared://📜️example-readme.docx` (413 real paragraphs, real "
                "WordprocessingML namespace)."
            ),
            engine_family="python-docx", engine_impl="python-docx OPC/OOXML reader", homepage="https://python-docx.readthedocs.io",
        ),
    ],
    wire={"docx-ecma-376-transitional-mutate": "python-docx-docx-ecma-376-transitional-mutate-reader"},
)

# ══════════════════════════════════════════ svg (19) ═════════════════════════════════════════════════
add_oracles(
    "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/✳️basic/🧪️oracle/🔣️.json",
    [
        entry(
            "lxml-svg-1-1-basic-mutate-reader", "python", "lxml", "6.1.3", "BSD-3-Clause",
            ["svg-1-1-basic-mutate"], ["semantic-svg-basic-1-1-v1"],
            READER_PREFIX + (
                "lxml 6.1.3 wraps libxml2, a from-scratch C XML engine entirely independent of this "
                "repository's own `quick-xml`-based reimplementation. `etree.parse(path).getroot()` exposes "
                "the real parsed SVG element tree — tag, namespace, every attribute (viewBox, width/height, "
                "path `d`, transform, style) and child order — which is exactly what this subset's SVG Basic "
                "document vocabulary mutates. Verified against this subset's own committed "
                "`shared://🎨️semio-brand-and-onboarding.svg`: `root.tag` reports the real "
                "`{http://www.w3.org/2000/svg}svg` qualified name and `root.nsmap` the real default SVG "
                "namespace."
            ),
            engine_family="lxml", engine_impl="lxml/libxml2 XML reader", homepage="https://lxml.de",
        ),
    ],
    wire={"svg-1-1-basic-mutate": "lxml-svg-1-1-basic-mutate-reader"},
)

add_oracles(
    "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/✳️tiny/🧪️oracle/🔣️.json",
    [
        entry(
            "lxml-svg-1-1-tiny-mutate-reader", "python", "lxml", "6.1.3", "BSD-3-Clause",
            ["svg-1-1-tiny-mutate"], ["semantic-svg-tiny-1-1-v1"],
            READER_PREFIX + (
                "lxml 6.1.3 wraps libxml2, independent of this repository's `quick-xml` reimplementation, "
                "and exposes the real parsed element tree for SVG Tiny's own vocabulary the same way it does "
                "for the sibling ✳️basic subset. Verified against this subset's own committed "
                "`shared://qr-code.svg`."
            ),
            engine_family="lxml", engine_impl="lxml/libxml2 XML reader", homepage="https://lxml.de",
        ),
    ],
    wire={"svg-1-1-tiny-mutate": "lxml-svg-1-1-tiny-mutate-reader"},
)

# ══════════════════════════════════════════ ifc (16) ═════════════════════════════════════════════════
IFCOPENSHELL_RATIONALE = (
    "📖️ A READER, registered separately from this file's `cross-semio-implementation` entry (the "
    "`ruststep`-backed dispatcher that COMPUTES what a mutation should produce). IfcOpenShell 0.8.4.post1 "
    "is the reference open-source IFC engine: `ifcopenshell.open(path)` runs its own C++ Part-21/EXPRESS "
    "parser against the real IFC2X3 schema, entirely independent of this repository's `ruststep` "
    "reimplementation or its own production decoder. It is ALREADY declared as an `oracleHostPackages` "
    "entry in the shared `✏️s/🔌️plugins/🗄️stdio/🧪️oracle/🔣️.json` (owner `✏️s/🔌️plugins/🗄️stdio`, a real "
    "path prefix of every subset under it, ifc included) by this ticket's own A10 shard for the sibling "
    "2x3/✳️base subset's `ifcopenshell-ifc-2x3-base-differential` entry — so it is already resolvable on "
    "every stdio test case's host import path and this registration needs no shared-manifest edit at all. "
    "{extra}"
    "Verified directly against this subset's own committed fixture before registration."
)


def ifc_entry(oid, cap, extra):
    return entry(
        oid, "python", "ifcopenshell", "0.8.4.post1", "LGPL-3.0-or-later", [cap], ["semantic-ifc-v1"],
        IFCOPENSHELL_RATIONALE.format(extra=extra),
        engine_family="ifcopenshell", engine_impl="IfcOpenShell C++/Python IFC engine", homepage="https://ifcopenshell.org",
    )


add_oracles(
    "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️2x3/🪆️subsets/✳️cobie/🧪️oracle/🔣️.json",
    [
        ifc_entry(
            "ifcopenshell-ifc-2x3-cobie-mutate-reader", "ifc-2x3-cobie-mutate",
            (
                "This subset's own vocabulary (`set-facility-name`, `set-floor-elevation`, `set-space`, "
                "`set-type-assignment`, `set-view-definition`) is a COBie-flavoured slice of core IFC "
                "entities, all directly readable through `ifcopenshell.file.by_type` — "
                "`f.by_type(\"IfcProject\")[0].Name` (facility name), `f.by_type(\"IfcBuildingStorey\")` "
                "`.Elevation` (floor elevation), `f.by_type(\"IfcSpace\")` (space presence/count), "
                "`f.by_type(\"IfcTypeObject\")` (type assignment) and the FILE_DESCRIPTION header string "
                "(view definition). "
            ),
        )
    ],
    wire={"ifc-2x3-cobie-mutate": "ifcopenshell-ifc-2x3-cobie-mutate-reader"},
)

add_oracles(
    "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️2x3/🪆️subsets/✳️cv20/🧪️oracle/🔣️.json",
    [
        ifc_entry(
            "ifcopenshell-ifc-2x3-cv20-mutate-reader", "ifc-2x3-cv20-mutate",
            (
                "This subset's own vocabulary (`set-product-placement`, `set-project-units`, "
                "`set-structural-entity`, `set-view-definition`) is a CoordinationView-2.0-flavoured slice "
                "directly readable through `ifcopenshell.file.by_type` — `IfcProduct.ObjectPlacement`, "
                "`IfcProject.UnitsInContext`, structural entity types, and the FILE_DESCRIPTION header "
                "string. "
            ),
        )
    ],
    wire={"ifc-2x3-cv20-mutate": "ifcopenshell-ifc-2x3-cv20-mutate-reader"},
)

add_oracles(
    "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️2x3/🪆️subsets/✳️sav/🧪️oracle/🔣️.json",
    [
        ifc_entry(
            "ifcopenshell-ifc-2x3-sav-mutate-reader", "ifc-2x3-sav-mutate",
            (
                "This subset's own vocabulary (`set-analysis-model`, `set-group-assignment`, "
                "`set-load-group`, `set-view-definition`) is a StructuralAnalysisView-flavoured slice — "
                "`IfcStructuralAnalysisModel`, `IfcGroup`/`IfcRelAssignsToGroup`, `IfcStructuralLoadGroup` "
                "and the FILE_DESCRIPTION header string are all directly readable through "
                "`ifcopenshell.file.by_type`. "
            ),
        )
    ],
    wire={"ifc-2x3-sav-mutate": "ifcopenshell-ifc-2x3-sav-mutate-reader"},
)

# ══════════════════════════════════════════ zip (13) ═════════════════════════════════════════════════
ZIP_RATIONALE = (
    "📖️ A READER, registered separately from this file's `cross-semio-implementation` entry (the Rust "
    "`zip`-crate-backed dispatcher that COMPUTES what a mutation should produce). yauzl 3.4.0 is a "
    "from-scratch streaming ZIP reader written in JavaScript, entirely independent of this repository's "
    "Rust `zip` reimplementation and of any other ecosystem this file uses. `yauzl.open` walks the real "
    "central directory (`zipfile.entryCount`, per-entry `fileName`/`compressedSize`/`compressionMethod`/"
    "`crc32`, `openReadStream` for the raw bytes) and `zipfile.comment` for the archive comment — directly "
    "witnessing every kind this subset's catalog mutates (entry add/remove/rename, entry data, "
    "compression method, archive comment). Verified against this subset's own committed `shared://🗜️.zip` "
    "before registration: `yauzl.open` reported all 20 real entries with their genuine compressed sizes "
    "and DEFLATE compression method (8)."
)

add_oracles(
    "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/✳️iso21320/🧪️oracle/🔣️.json",
    [entry("yauzl-zip-2-0-iso21320-mutate-reader", "javascript", "yauzl", "3.4.0", "MIT", ["zip-2-0-iso21320-mutate"], ["semantic-zip-iso21320-v1"], ZIP_RATIONALE, engine_family="yauzl", engine_impl="yauzl streaming ZIP reader", homepage="https://github.com/thejoshwolfe/yauzl")],
    wire={"zip-2-0-iso21320-mutate": "yauzl-zip-2-0-iso21320-mutate-reader"},
)
add_oracles(
    "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/✳️base/🧪️oracle/🔣️.json",
    [entry("yauzl-zip-2-0-base-mutate-reader", "javascript", "yauzl", "3.4.0", "MIT", ["zip-2-0-mutate"], ["semantic-archive-mutate-v1"], ZIP_RATIONALE, engine_family="yauzl", engine_impl="yauzl streaming ZIP reader", homepage="https://github.com/thejoshwolfe/yauzl")],
    wire={"zip-2-0-mutate": "yauzl-zip-2-0-base-mutate-reader"},
)

# ══════════════════════════════════════════ step (10) ════════════════════════════════════════════════
add_oracles(
    "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/✳️base/🧪️oracle/🔣️.json",
    [
        entry(
            "steputils-step-ap214-base-mutate-reader", "python", "steputils", "0.1", "MIT",
            ["step-ap214-base-mutate"], ["semantic-step-v1"],
            READER_PREFIX + (
                "steputils 0.1 (by mozman, the same author as the widely-used `ezdxf` DXF library) is a "
                "from-scratch pure-Python ISO 10303-21 (Part 21 / STEP physical file) reader, entirely "
                "independent of this repository's Rust `ruststep`-based reimplementation. "
                "`steputils.p21.readfile(path)` parses the real header and data section — "
                "`stepfile.header['FILE_SCHEMA']`, `FILE_DESCRIPTION`, and every data entity by its STEP "
                "instance name — directly witnessing this subset's AP214 entity/attribute vocabulary. "
                "Verified against this subset's own committed "
                "`shared://🧪️hexagonal-cut-concrete-forest-left-ap214/📐️.stp`: `readfile` parsed the real "
                "file and reported its genuine `FILE_SCHEMA` entry, `('AUTOMOTIVE_DESIGN')` — AP214's own "
                "schema name."
            ),
            engine_family="steputils", engine_impl="steputils Part-21/STEP reader", homepage="https://github.com/mozman/steputils",
        ),
    ],
    wire={"step-ap214-base-mutate": "steputils-step-ap214-base-mutate-reader"},
)

# ══════════════════════════════════════════ html (9) ═════════════════════════════════════════════════
add_oracles(
    "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🌐️html/🏅️standards/🔖️5/🪆️subsets/✳️any/🧪️oracle/🔣️.json",
    [
        entry(
            "html5lib-html-5-mutate-reader", "python", "html5lib", "1.1", "MIT",
            ["html-5-mutate"], ["semantic-html-v1"],
            READER_PREFIX + (
                "html5lib 1.1 is a from-scratch, pure-Python implementation of the WHATWG HTML5 parsing "
                "algorithm — spec-conformant tree construction, tokenizer states and all — entirely "
                "independent of this repository's Rust `html5ever` reimplementation (a different codebase "
                "in a different language implementing the same published algorithm). "
                "`html5lib.parse(path, treebuilder=\"lxml\")` builds the real DOM tree — element names, "
                "namespaces, attributes, text content, document order — directly witnessing this subset's "
                "HTML5 document vocabulary. Verified against this subset's own committed "
                "`shared://🧪️zukunft-bau-entwerfen-mit-bestand/🌐️.html`: the parser reported the real "
                "`{http://www.w3.org/1999/xhtml}html` root, the namespace HTML5 parsing always assigns."
            ),
            engine_family="html5lib", engine_impl="html5lib WHATWG HTML5 parser", homepage="https://github.com/html5lib/html5lib-python",
        ),
    ],
    wire={"html-5-mutate": "html5lib-html-5-mutate-reader"},
)

# ══════════════════════════════════════════ xml (8) ══════════════════════════════════════════════════
add_oracles(
    "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📰️xml/🏅️standards/🔖️1.0/🪆️subsets/✳️valid/🧪️oracle/🔣️.json",
    [
        entry(
            "lxml-xml-1-0-valid-mutate-reader", "python", "lxml", "6.1.3", "BSD-3-Clause",
            ["xml-1-0-valid-mutate"], ["semantic-xml-valid-1-0-v1"],
            READER_PREFIX + (
                "lxml 6.1.3 wraps libxml2's validating XML 1.0 parser, entirely independent of this "
                "repository's `quick-xml` reimplementation, and `etree.parse(path)` rejects a malformed "
                "document outright while exposing the well-formed tree's element names, attributes, text "
                "and declared XML version/encoding directly. Verified against this subset's own committed "
                "`shared://🧪️macos-uttype-plist/🏷️.xml`: `tree.docinfo.xml_version` and `.encoding` reported "
                "the real declared `1.0`/`UTF-8`, and the root parsed to the genuine `plist` element."
            ),
            engine_family="lxml", engine_impl="lxml/libxml2 XML 1.0 reader", homepage="https://lxml.de",
        ),
    ],
    wire={"xml-1-0-valid-mutate": "lxml-xml-1-0-valid-mutate-reader"},
)

# ══════════════════════════════════════════ jpg (9) ══════════════════════════════════════════════════
add_oracles(
    "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/✳️baseline/🧪️oracle/🔣️.json",
    [
        entry(
            "pillow-jpg-jfif-1-01-baseline-mutate-reader", "python", "Pillow", "12.2.0", "MIT-CMU",
            ["jpg-jfif-1-01-baseline-mutate"], ["semantic-raster-v1"],
            (
                "📖️ A READER. This subset's own dispatcher (not registered as an oracle at all here — no "
                "reimplementation entry exists in this file) applies each mutation; Pillow 12.2.0 is the "
                "independent judge. Pillow's own C JPEG codec (not this repository's own decoder) exposes "
                "marker-level structure beyond plain pixel decoding: `im.layer` (after `im.load()`) reports "
                "each SOF frame component's id/h-sampling/v-sampling/quant-table-index tuple, `im.bits` the "
                "sample precision, and `im.quantization` the quantization table indices actually referenced "
                "— directly witnessing `set-component-sampling`, `insert-frame-component`/"
                "`remove-frame-component`, `set-sample-precision`, `set-sof-marker` and `set-snapshot`. "
                "Verified against this subset's own committed "
                "`shared://🧪️abbau-aufbau-masterarbeit-grundriss/🖼️.jpg`: `im.layer` reported the real "
                "3-component YCbCr layout `[(1,1,1,0),(2,1,1,1),(3,1,1,1)]` and `im.bits` the real 8-bit "
                "precision. HONEST PARTIAL: Pillow's public API does not expose an enumerable Huffman-table "
                "list the way it exposes quantization-table indices — `insert-huffman-table`/"
                "`remove-huffman-table` are witnessed only insofar as a missing/duplicate DHT table breaks "
                "decoding outright (Pillow raises rather than silently reading garbage), not as a positive "
                "before/after table-count comparison; a corrupted-but-still-decodable Huffman-table edit "
                "would not be caught by this entry alone."
            ),
            engine_family="pillow", engine_impl="Pillow 12.2.0 JpegImagePlugin", homepage="https://python-pillow.org",
        ),
    ],
    wire={"jpg-jfif-1-01-baseline-mutate": "pillow-jpg-jfif-1-01-baseline-mutate-reader"},
)

# ══════════════════════════════════════════ tiff (8) ═════════════════════════════════════════════════
add_oracles(
    "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/✳️baseline/🧪️oracle/🔣️.json",
    [
        entry(
            "tiff-tiff-6-0-baseline-mutate-reader", "rust", "tiff", "0.11", "MIT OR Apache-2.0",
            ["tiff-6-0-baseline-mutate"], ["semantic-raster-v1"],
            (
                "📖️ A READER. This subset carries no reimplementation oracle of its own (no `oracles` entry "
                "existed in this file before this registration). `tiff` 0.11 is the SAME crate already "
                "verified and registered for the sibling ✳️document subset's `image-tiff-6-0-mutate-reader` "
                "(that entry's own rationale documents the full investigation: `image` 0.25's public TIFF "
                "surface is single-IFD-only with no tag/byte-order accessors, while `tiff` 0.11's "
                "`Decoder::{tag_iter,byte_order}` gives full IFD tag enumeration including an `unknown(u16)` "
                "variant for non-baseline tags). This baseline subset's own vocabulary — "
                "`set-bits-per-sample`, `set-compression`, `set-photometric-interpretation`, "
                "`set-strip-offsets`/`remove-strip-offsets`, `insert-tile-tags`/`remove-tile-tags`, "
                "`set-snapshot` — is exactly the baseline IFD tag set (`BitsPerSample`, `Compression`, "
                "`PhotometricInterpretation`, `StripOffsets`, `TileOffsets`/`TileByteCounts`) that "
                "`Decoder::tag_iter` enumerates directly, independent of this repository's own TIFF "
                "decoder or its `image`-crate-based encoder."
            ),
            engine_family="tiff", engine_impl="tiff reader/writer", homepage="https://github.com/image-rs/image-tiff",
        ),
    ],
    wire={"tiff-6-0-baseline-mutate": "tiff-tiff-6-0-baseline-mutate-reader"},
)

# ══════════════════════════════════════════ wav (4) ══════════════════════════════════════════════════
add_oracles(
    "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔊️wav/🏅️standards/🔖️riff-pcm/🪆️subsets/✳️any/🧪️oracle/🔣️.json",
    [
        entry(
            "hound-wav-riff-pcm-mutate-reader", "rust", "hound", "3.5.1", "Apache-2.0",
            ["wav-riff-pcm-mutate"], ["semantic-audio-v1"],
            (
                "📖️ A READER (this subset carries no reimplementation oracle at all — no `oracles` entry "
                "existed in this file before this registration). hound 3.5.1 is a from-scratch RIFF/WAVE "
                "PCM encoder+decoder, entirely independent of this repository's own WAV codec. "
                "`WavReader::open(path).spec()` exposes `channels`, `sample_rate`, `bits_per_sample` and "
                "`sample_format` read directly from the real `fmt ` chunk, and `.duration()`/the sample "
                "iterator read the real `data` chunk — directly witnessing `set-fmt`, `set-data` and "
                "`set-snapshot`. Verified against this subset's own committed "
                "`shared://🧪️bauen-mit-bestand-ausschnitt/🔊️.wav`: `spec()` reported the real "
                "`channels=1 sample_rate=8000 bits_per_sample=16` PCM16 format and a real 96000-sample "
                "duration. HONEST PARTIAL: hound's own reader explicitly skips any RIFF chunk besides "
                "`fmt `/`fact`/`data` (confirmed by reading its vendored source, `read.rs`'s own comment "
                "\"Ignore the chunk; skip all of its bytes\") — it cannot witness `set-other-chunks` on its "
                "own; the sibling `riff-wav-riff-pcm-mutate-chunk-reader` entry below covers that kind."
            ),
            engine_family="hound", engine_impl="hound WAV PCM encoder/decoder", homepage="https://github.com/ruuda/hound",
        ),
        entry(
            "riff-wav-riff-pcm-mutate-chunk-reader", "rust", "riff", "2.0.0", "MIT",
            ["wav-riff-pcm-mutate"], ["semantic-audio-v1"],
            (
                "📖️ A generic RIFF chunk-sequence READER, covering the one kind the sibling `hound` entry "
                "above cannot: `set-other-chunks`. `riff` 2.0.0 is a from-scratch generic RIFF-container "
                "walker (used nowhere else by this repository's WAV path), and `riff::Chunk::read` + "
                "`.iter()` enumerate every top-level chunk by id and length without interpreting PCM "
                "semantics at all — so an inserted/removed/resized LIST/INFO-style chunk is directly "
                "observable as a change in the enumerated chunk sequence. Verified against this subset's "
                "own committed `shared://🧪️bauen-mit-bestand-ausschnitt/🔊️.wav`: the walker reported the "
                "real top-level `fmt `(16 bytes)/`data`(192000 bytes) chunk sequence."
            ),
            engine_family="riff", engine_impl="riff generic RIFF container reader", homepage="https://github.com/frabert/riff",
        ),
    ],
    wire={"wav-riff-pcm-mutate": "hound-wav-riff-pcm-mutate-reader"},
)

# ══════════════════════════════════════════ txt (5) ══════════════════════════════════════════════════
add_oracles(
    "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️txt/🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🧪️oracle/🔣️.json",
    [
        entry(
            "bstr-txt-utf-8-mutate-reader", "rust", "bstr", "1.13.1", "MIT OR Apache-2.0",
            ["txt-utf-8-mutate"], ["utf8-text-v1"],
            READER_PREFIX + (
                "bstr 1.13.1 (BurntSushi's byte-string library) is a from-scratch UTF-8-aware byte-string "
                "crate, independent of this repository's own text handling. `bytes.is_utf8()` independently "
                "validates well-formed UTF-8, `bytes.lines()` splits on the real line-ending bytes present "
                "(witnessing `insert-line`/`remove-line`/`set-line`/`set-line-ending`), and "
                "`bytes.ends_with(b\"\\n\")` witnesses `set-trailing-newline`. Verified against this subset's "
                "own committed `shared://🔤️.txt`: `is_utf8()` reported true, `lines().count()` the real 158 "
                "lines, and `ends_with(b\"\\n\")` the real trailing newline."
            ),
            engine_family="bstr", engine_impl="bstr UTF-8 byte-string reader", homepage="https://github.com/BurntSushi/bstr",
        ),
    ],
    wire={"txt-utf-8-mutate": "bstr-txt-utf-8-mutate-reader"},
)

print("DONE")
