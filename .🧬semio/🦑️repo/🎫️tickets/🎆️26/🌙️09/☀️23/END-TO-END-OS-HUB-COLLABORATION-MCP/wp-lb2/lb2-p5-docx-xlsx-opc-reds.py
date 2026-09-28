#!/usr/bin/env python3
"""🧾️ LB2 prepared patch p5 — the non-test half of the docx/xlsx reds the peer's OPC `xml_parts` migration left behind.

Measured 17:01 / 17:29 (`generated/c1-test-docx-xlsx.txt`, `generated/c3-probe-3.txt`, native lane). After the test-only half
landed (rule 22: strict/transitional analyzer/construction/composition tests on the XML authority lane, docx pStyle oracle),
11 reds remain and every one is non-test:

* xlsx `demo_xlsx_snapshot` put `xl/styles.xml` into the BINARY lane (`opc.set_part`), which `validate_authority` refuses
  ("binary authority carries an XML content part") → `fixture_honesty_law`, `grammar_conformance_law`, `protocol_walk_law`.
  Fix: the part joins the XML authority lane (namespaced `styleSheet`, content-type override), the encode/decode
  "normalization" round trip and its in-body comment go (the probe proved the new demo a codec fixed point).
* The mutations/diff `.grammar.semio` facets of both artifacts still describe the pre-migration print forms: `set-snapshot`
  (and xlsx `set-cell`/`remove-cell` addresses) now print the hex of RFC 8259 JSON, xlsx cell values gained `R[..]`, and
  `print_diff` prints the diff's JSON (`{}` when empty) → `ops_grammar_conformance_law`, `diff_grammar_conformance_law`.
  The binary diff frame is `OP_BINARY_FORMAT` + that JSON (the old `format`/`flags` header walked it only by accident).
* xlsx Strict/Transitional `worksheet_content_type_gaps` filtered on `content_type.contains("worksheet")`, so a worksheet
  typed `application/xml` — the very violation — was never reported. The worksheet ROLE is the workbook's worksheet
  relationship (`XlsxSnapshot::worksheet_part_paths`), the check reads the package-declared type of each target. The Strict
  VML check scans both lanes (a `.xml`-named VML drawing lives in the XML lane). The main-workbook lookup is one
  `XlsxSnapshot::workbook_part_path`; dead imports/consts of both subsets go.
* docx: no code change — `📜️example.docx` / `🗣️.dsl.semio` / `🎒️.pack.semio` predate the migration (relationships
  differ); the demo itself is a codec fixed point (probe) → regenerate through the artifact's own writer.

Every payload grammar/protocol was recognized/walked against the real demo cases by the temporary `lb2_probe` tests
(`generated/c3-probe-1.txt`): docx ops 18/18, xlsx ops 9/9, typed diff grammars 4/4 each, utf8 diff protocols 4/4 each.
Landing (window 3, one native hold): `lb2-p5-land.sh` = this script `--write` → check → the two fixture writers → lib tests.

Usage: lb2-p5-docx-xlsx-opc-reds.py [--dry-run | --write | --revert] [--root <repo-or-overlay root>]"""
import hashlib
import shutil
import sys
from pathlib import Path

ROOT = Path(sys.argv[sys.argv.index("--root") + 1]) if "--root" in sys.argv else Path("/Users/ueli/Documents/semio")
WRITE = "--write" in sys.argv
REVERT = "--revert" in sys.argv
HERE = Path(__file__).resolve().parent
PAYLOAD = HERE / "payload" / "p5"
BACKUP = HERE / "generated" / "p5-backup"
ART = "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/"
XB = ART + "📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/"
DB = ART + "📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/"
XS = ART + "📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/🔒️strict/🧬️schema/🦀️.rs"
XT = ART + "📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/🌉️transitional/🧬️schema/🦀️.rs"
XS_TEST = ART + "📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/🔒️strict/🧬️schema/🧪️tests/🔬️derived-analysis-unit/🦀️.rs"

FACETS = [
    (XB + "🧬️schema/🧬️mutations/📝️text/📖️.grammar.semio", "558648c96c219052a7f574415fa05ae1d929748817cc0c6e880f4070f77eaa5a", "xlsx-mutations.grammar.semio"),
    (XB + "🧬️schema/🔺️diff/📝️text/📖️.grammar.semio", "f6cd48e7801d8cf94870062cc4959b6976b79fd48af15b5d14fa55f09312b627", "xlsx-diff.grammar.semio"),
    (XB + "🧬️schema/🔺️diff/💾️binary/📡️.protocol.semio", "6c5e898e46e201558dfc7635999dffc94bda9627e27fba8176b3decc5efef69c", "xlsx-diff.protocol.semio"),
    (DB + "🧬️schema/🧬️mutations/📝️text/📖️.grammar.semio", "e3f22173ae03f270c73e376fc57c253cc5d6289c3f7e2c308892107836522e1e", "docx-mutations.grammar.semio"),
    (DB + "🧬️schema/🔺️diff/📝️text/📖️.grammar.semio", "abfad006108e8ebb978be1f47894c4487d38c593bfb3db78ff0ec15ed1ed0cb0", "docx-diff.grammar.semio"),
    (DB + "🧬️schema/🔺️diff/💾️binary/📡️.protocol.semio", "70e0a676f3779c72472e1952c6cfe319d753510eed2e5a279b22c6afabbbc03d", "docx-diff.protocol.semio"),
]

DEMO_START = "/// 📄️ FG-wave: the demo `stdio.xlsx` document — a genuinely non-trivial `XlsxSnapshot` exercising\n"
DEMO_END = '    let bytes = encode_xlsx(&snap).expect("encode demo xlsx for part-order normalization");\n    decode_xlsx(&bytes).expect("decode demo xlsx for part-order normalization")\n}\n'
DEMO_NEW = '''/// 📄️ The demo `stdio.xlsx` document — a genuinely non-trivial `XlsxSnapshot` with `SharedString`, `Number`, `Boolean`,
/// `Formula` (with a cached value) and `InlineString` cells on two sheets, plus one unmodeled XML part (`xl/styles.xml`)
/// carried verbatim in the XML authority lane. Stated in the package
/// normal form (XML parts path-ascending), so it is a fixed point of `encode_xlsx`/`decode_xlsx`. The single source of truth
/// for `📚️examples/🎬️demo/🖼️assets/🗣️.dsl.semio`/`🎒️.pack.semio` (literally this snapshot's `print_dsl`/`encode_pack`
/// output, asserted by `fixture_honesty_law`).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn demo_xlsx_snapshot() -> XlsxSnapshot {
    use crate::schema::snapshot::{XlsxCell, XlsxCellValue, XlsxSheet};
    use crate::standards::v_ecma_376::subsets::base::io::export::serializers::build_minimal_xlsx;
    use semio_s_artifact_stdio_xml::schema::snapshot::xml_document_from_text;
    const STYLES_PART: &str = "xl/styles.xml";
    const STYLES_CONTENT_TYPE: &str = "application/vnd.openxmlformats-officedocument.spreadsheetml.styles+xml";
    let workbook = XlsxWorkbook {
        sheets: vec![
            XlsxSheet {
                name: "Sheet1".into(),
                cells: vec![
                    XlsxCell { row: 1, col: 0, value: XlsxCellValue::SharedString(0) },
                    XlsxCell { row: 1, col: 1, value: XlsxCellValue::SharedString(1) },
                    XlsxCell { row: 2, col: 0, value: XlsxCellValue::SharedString(2) },
                    XlsxCell { row: 2, col: 1, value: XlsxCellValue::Number(95.5) },
                    XlsxCell { row: 3, col: 0, value: XlsxCellValue::Boolean(true) },
                    XlsxCell { row: 3, col: 1, value: XlsxCellValue::Formula { expr: "SUM(B2:B2)".into(), cached: Some(Box::new(XlsxCellValue::Number(95.5))) } },
                ],
            },
            XlsxSheet { name: "Totals".into(), cells: vec![XlsxCell { row: 1, col: 0, value: XlsxCellValue::InlineString("Total Score".into()) }] },
        ],
        shared_strings: vec!["Name".into(), "Score".into(), "Alice".into()],
    };
    let mut snap = build_minimal_xlsx(workbook);
    snap.opc.content_types.set_override(STYLES_PART, STYLES_CONTENT_TYPE);
    snap.xml_parts.push(XlsxXmlPart {
        path: STYLES_PART.into(),
        content_type: STYLES_CONTENT_TYPE.into(),
        document: xml_document_from_text("<styleSheet xmlns=\\"http://schemas.openxmlformats.org/spreadsheetml/2006/main\\"/>").expect("valid demo styles XML"),
    });
    snap.xml_parts.sort_by(|left, right| left.path.cmp(&right.path));
    snap
}
'''

SNAP_ANCHOR = "    /// 📘️ Projects the spreadsheet view without creating persisted semantic authority.\n"
SNAP_NEW = '''    /// 📗️ Resolves the main workbook part from the package root's officeDocument relationship (Transitional or Strict
    /// relationship type).
    pub fn workbook_part_path(&self) -> Option<String> {
        use crate::standards::v_ecma_376::subsets::base::io::REL_TYPE_OFFICE_DOCUMENT_STRICT;
        self.opc.resolve_relationship("", REL_TYPE_OFFICE_DOCUMENT).or_else(|| self.opc.resolve_relationship("", REL_TYPE_OFFICE_DOCUMENT_STRICT))
    }

    /// 📑️ Resolves every worksheet part by its ROLE — the targets of the main workbook's worksheet relationships
    /// (Transitional or Strict type), independent of the part's path or its declared content type.
    pub fn worksheet_part_paths(&self) -> Vec<String> {
        use crate::standards::v_ecma_376::subsets::base::io::{REL_TYPE_WORKSHEET, REL_TYPE_WORKSHEET_STRICT};
        let Some(workbook) = self.workbook_part_path() else { return Vec::new() };
        self.opc
            .relationships_for(&workbook)
            .iter()
            .filter(|relationship| relationship.rel_type == REL_TYPE_WORKSHEET || relationship.rel_type == REL_TYPE_WORKSHEET_STRICT)
            .map(|relationship| resolve_relationship_target(&workbook, &relationship.target))
            .collect()
    }

'''

IO_ANCHOR = 'pub const REL_TYPE_SHARED_STRINGS_STRICT: &str = "http://purl.oclc.org/ooxml/officeDocument/relationships/sharedStrings";\n'
IO_NEW = IO_ANCHOR + '''/// 🏅️ Strict's `worksheet` relationship TYPE — the workbook-owned pointer that gives a part its worksheet role in a Strict
/// package, same rationale as the two Strict relationship types above.
pub const REL_TYPE_WORKSHEET_STRICT: &str = "http://purl.oclc.org/ooxml/officeDocument/relationships/worksheet";
'''

MAIN_OLD = '''snapshot.opc.resolve_relationship("", semio_s_artifact_stdio_zip::opc::REL_TYPE_OFFICE_DOCUMENT).or_else(|| snapshot.opc.resolve_relationship("", crate::standards::v_ecma_376::subsets::base::io::REL_TYPE_OFFICE_DOCUMENT_STRICT))'''
MAIN_NEW = "snapshot.workbook_part_path()"
CONSTS_OLD = '''    use semio_s_artifact_stdio_xml::schema::snapshot::{xml_document_from_text, xml_document_to_text, XmlAttr, XmlNode};

    const WORKBOOK_PART: &str = "xl/workbook.xml";
    const WORKBOOK_CONTENT_TYPE: &str = "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet.main+xml";
'''
CONSTS_NEW = "    use semio_s_artifact_stdio_xml::schema::snapshot::{XmlAttr, XmlNode};\n"
ANALYSIS_USE_OLD = "    use semio_s_artifact_stdio_xml::schema::snapshot::{xml_document_from_text, XmlNode};\n"
ANALYSIS_USE_NEW = "    use semio_s_artifact_stdio_xml::schema::snapshot::XmlNode;\n"
GAPS_BODY_OLD = '''    fn worksheet_content_type_gaps(snapshot: &XlsxSnapshot) -> Vec<Diagnostic> {
        snapshot
            .xml_parts
            .iter()
            .filter(|p| p.content_type.contains("worksheet") && p.content_type != WORKSHEET_CONTENT_TYPE)
            .map(|p| soft(CODE_WORKSHEET_CONTENT_TYPE, format!("worksheet part {} resolves content type {:?}, expected {WORKSHEET_CONTENT_TYPE:?} (ECMA-376 Part 1 §12.3.24)", p.path, p.content_type)))
            .collect()
    }
'''
GAPS_BODY_NEW = '''    fn worksheet_content_type_gaps(snapshot: &XlsxSnapshot) -> Vec<Diagnostic> {
        snapshot
            .worksheet_part_paths()
            .into_iter()
            .filter_map(|path| {
                let content_type = snapshot.opc.content_types.resolve(&path).map(str::to_string);
                (content_type.as_deref() != Some(WORKSHEET_CONTENT_TYPE))
                    .then(|| soft(CODE_WORKSHEET_CONTENT_TYPE, format!("worksheet part {path} resolves content type {content_type:?}, expected {WORKSHEET_CONTENT_TYPE:?} (ECMA-376 Part 1 §12.3.24)")))
            })
            .collect()
    }
'''
STRICT_EDITS = [
    ("strict construction imports + dead consts", CONSTS_OLD, CONSTS_NEW),
    ("strict stamp docstring", "    /// 🖋️ Real-rewrites `snapshot.opc`'s `xl/workbook.xml` root attrs to Strict shape. A no-op on the\n", "    /// 🖋️ Real-rewrites the main workbook XML part's root attrs to Strict shape. A no-op on the\n"),
    ("strict stamp main path", "        let main_path = " + MAIN_OLD + ";\n", "        let main_path = " + MAIN_NEW + ";\n"),
    ("strict analysis imports", ANALYSIS_USE_OLD, ANALYSIS_USE_NEW),
    ("strict root attrs path", "        let path = " + MAIN_OLD + "?;\n", "        let path = " + MAIN_NEW + "?;\n"),
    (
        "strict gaps docstring",
        "    /// 🩺️ Real worksheet content-type scan. Small enough (and CODE_* consts stay subset-namespaced\n    /// per the pattern doc) that duplicating beats a cross-subset dependency on 🌉️transitional's own\n    /// copy for one five-line check.\n",
        "    /// 🩺️ Real worksheet content-type scan over every part the workbook's worksheet relationships target (its role),\n    /// reading the package-declared type. Small enough (and CODE_* consts stay subset-namespaced) that duplicating beats a\n    /// cross-subset dependency on 🌉️transitional's own copy.\n",
    ),
    ("strict gaps body", GAPS_BODY_OLD, GAPS_BODY_NEW),
    (
        "strict VML scan",
        '''        for part in &snapshot.opc.parts {
            if part.content_type == VML_CONTENT_TYPE {
                out.push(hard(CODE_VML_FORBIDDEN, format!("part {} declares legacy VML drawing content type {VML_CONTENT_TYPE:?} -- ISO/IEC 29500-1 Strict removes VML support entirely", part.path)));
            }
        }
''',
        '''        let lanes = snapshot.opc.parts.iter().map(|part| (&part.path, &part.content_type)).chain(snapshot.xml_parts.iter().map(|part| (&part.path, &part.content_type)));
        for (path, _) in lanes.filter(|(_, content_type)| content_type.as_str() == VML_CONTENT_TYPE) {
            out.push(hard(CODE_VML_FORBIDDEN, format!("part {path} declares legacy VML drawing content type {VML_CONTENT_TYPE:?} -- ISO/IEC 29500-1 Strict removes VML support entirely")));
        }
''',
    ),
]
TRANSITIONAL_EDITS = [
    ("transitional construction imports + dead consts", CONSTS_OLD, CONSTS_NEW),
    ("transitional stamp docstring", "    /// 🖋️ Real-rewrites `snapshot.opc`'s `xl/workbook.xml` root attrs to explicit Transitional shape.\n", "    /// 🖋️ Real-rewrites the main workbook XML part's root attrs to explicit Transitional shape.\n"),
    ("transitional stamp main path", "        let main_path = " + MAIN_OLD + ";\n", "        let main_path = " + MAIN_NEW + ";\n"),
    ("transitional analysis imports", ANALYSIS_USE_OLD, ANALYSIS_USE_NEW),
    ("transitional root attrs path", "        let path = " + MAIN_OLD + "?;\n", "        let path = " + MAIN_NEW + "?;\n"),
    (
        "transitional gaps docstring",
        "    /// 🩺️ Real worksheet content-type scan -- same check as 🔒️strict's own copy, duplicated (small\n    /// enough, CODE_* consts stay subset-namespaced) rather than a cross-subset dependency.\n",
        "    /// 🩺️ Real worksheet content-type scan over every part the workbook's worksheet relationships target (its role),\n    /// reading the package-declared type -- same check as 🔒️strict's own copy, duplicated (small enough, CODE_* consts\n    /// stay subset-namespaced) rather than a cross-subset dependency.\n",
    ),
    ("transitional gaps body", GAPS_BODY_OLD, GAPS_BODY_NEW),
]
VML_TEST_ANCHOR = "    /// 🏷️ The worksheet role comes from the workbook's worksheet relationship, not from the part's path or its\n"
VML_TEST_NEW = '''    /// 🗂️ A VML drawing named `.xml` lives in the XML authority lane — the Strict ban reads both lanes.
    #[semio_framework_async_macros::async_test]
    async fn vml_drawing_in_the_xml_lane_is_hard() {
        use crate::standards::v_ecma_376::subsets::base::schema::snapshot::XlsxXmlPart;
        use semio_s_artifact_stdio_xml::schema::snapshot::xml_document_from_text;
        let mut snapshot = snapshot_with_workbook(STRICT_SML_NS, STRICT_R_NS, Some("strict"));
        snapshot.opc.content_types.set_override("xl/drawings/vmlDrawing1.xml", VML_CONTENT_TYPE);
        snapshot.xml_parts.push(XlsxXmlPart { path: "xl/drawings/vmlDrawing1.xml".into(), content_type: VML_CONTENT_TYPE.into(), document: xml_document_from_text("<xml/>").expect("valid probe XML") });
        snapshot.validate_authority().expect("a `.xml`-named VML drawing is an XML-lane part");
        let diagnostics = check_strict_conformance(&snapshot);
        assert!(diagnostics.iter().any(|d| d.code.0 == CODE_VML_FORBIDDEN && d.severity == Severity::Error), "got {diagnostics:?}");
    }

''' + VML_TEST_ANCHOR

EDITS = {
    XB + "🧬️schema/🦀️.rs": None,
    XB + "🧬️schema/📸️snapshot/🦀️.rs": [("snapshot role helpers", SNAP_ANCHOR, SNAP_NEW + SNAP_ANCHOR)],
    XB + "🚪️io/🦀️.rs": [("Strict worksheet relationship type", IO_ANCHOR, IO_NEW)],
    XS: STRICT_EDITS,
    XT: TRANSITIONAL_EDITS,
    XS_TEST: [("VML XML-lane law", VML_TEST_ANCHOR, VML_TEST_NEW)],
}


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


touched = [rel for rel, _, _ in FACETS] + list(EDITS)
if REVERT:
    for rel in touched:
        backup = BACKUP / rel
        if backup.exists():
            shutil.copyfile(backup, ROOT / rel)
            print("restored", rel)
    sys.exit(0)

problems, applied, plans = [], 0, {}
for rel, expected, payload in FACETS:
    path, source = ROOT / rel, PAYLOAD / payload
    current = sha(path)
    if current == sha(source):
        applied += 1
    elif current != expected:
        problems.append(f"{rel}: live sha256 {current[:12]} is neither the analyzed {expected[:12]} nor the payload")
    else:
        plans[rel] = source.read_text(encoding="utf-8")
demo_rel = XB + "🧬️schema/🦀️.rs"
demo_text = (ROOT / demo_rel).read_text(encoding="utf-8")
if "const STYLES_PART: &str = \"xl/styles.xml\";" in demo_text:
    applied += 1
elif demo_text.count(DEMO_START) != 1 or demo_text.count(DEMO_END) != 1:
    problems.append(f"{demo_rel}: expected 1x demo docstring head and 1x normalization tail, found {demo_text.count(DEMO_START)}/{demo_text.count(DEMO_END)}")
else:
    start, end = demo_text.index(DEMO_START), demo_text.index(DEMO_END) + len(DEMO_END)
    plans[demo_rel] = demo_text[:start] + DEMO_NEW + demo_text[end:]
for rel, edits in EDITS.items():
    if edits is None:
        continue
    text = (ROOT / rel).read_text(encoding="utf-8")
    new = text
    pending = 0
    for label, old, replacement in edits:
        if replacement in new:
            continue
        if new.count(old) != 1:
            problems.append(f"{rel}: expected 1x {label}, found {new.count(old)}")
            continue
        new = new.replace(old, replacement, 1)
        pending += 1
    if pending:
        plans[rel] = new
    else:
        applied += 1
for problem in problems:
    print("PROBLEM", problem)
print(f"root={ROOT} files={len(plans)} already-applied={applied} problems={len(problems)} write={WRITE}")
if WRITE and not problems:
    for rel, text in plans.items():
        backup = BACKUP / rel
        backup.parent.mkdir(parents=True, exist_ok=True)
        shutil.copyfile(ROOT / rel, backup)
        (ROOT / rel).write_text(text, encoding="utf-8")
        print("written", rel)
sys.exit(1 if problems else 0)
