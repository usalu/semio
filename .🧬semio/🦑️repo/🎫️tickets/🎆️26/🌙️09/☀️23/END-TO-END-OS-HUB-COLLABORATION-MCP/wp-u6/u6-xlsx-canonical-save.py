#!/usr/bin/env python3
"""📕️ U6 window-3 set C7 (T3 stdio, xlsx): the three canonical-save reds after the OPC `xml_parts` migration.

* PRODUCT — shared-string drafts: a cell's address revision (`🧭️cell-address`, the optimistic token every draft, window binding and
  agent lane carries) hashed only the worksheet XML, so editing the shared-string entry a `t="s"` cell displays left the revision
  unchanged and a draft made against the old text published over the new one (`unchanged_cell_drafts_preserve_types…`: "referenced
  text changes invalidate the draft"). The revision now also hashes the referenced `sst/si` entry (resolved through the workbook's
  sharedStrings relationship), so that change refuses the draft typed (`stdio.xlsx.cell-conflict`) and a stale address stays stale.
* ORACLE — calamine 0.36 rejects `CT_Cell/extLst` (`cells_reader.rs`: "v, f, or is"), which ECMA-376 Part 1 §18.3.1.4 allows and the
  fixture carries on purpose (lossless extension round trip). The quick-xml event check still reads every part verbatim; the calamine
  VALUE oracle reads a value projection of the same package with cell-level `extLst` removed.
* FIXTURE — the XML model carries no whitespace outside the root, so a no-op save reproduces exactly the writers' form: the OPC
  writer for `[Content_Types].xml` and relationship parts (declaration + CRLF), the XML document writer for every other XML part
  (declaration + LF, LF after each prolog node and before each epilog node). The authored canonical-save packages (parts and
  `expectedSheetXml`) are stated in that form, so a no-op save is byte-stable part by part.

Usage: u6-xlsx-canonical-save.py [--dry-run | --write | --revert] [--root <repo root>]"""
import hashlib
import json
import re
import sys
from pathlib import Path

ROOT = Path(sys.argv[sys.argv.index("--root") + 1]) if "--root" in sys.argv else Path("/Users/ueli/Documents/semio")
WRITE = "--write" in sys.argv
REVERT = "--revert" in sys.argv
BACKUP = Path("/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-u6-backup/xlsx-canonical-save")
BASE = "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/"
ADDRESS = BASE + "🧬️schema/🧬️mutations/🧭️cell-address/🦀️.rs"
TESTS = BASE + "✏️editor/🧪️tests/🔬️unit/🦀️.rs"
FIXTURE = BASE + "🧫️fixtures/🧬️canonical-xml-save/🔣️.json"

REVISION_OLD = """fn address_revision(root: &XmlNode, path: &[usize]) -> Result<String, String> {
    let mut hash = 0xcbf29ce484222325;
"""
REVISION_NEW = """/// 🔤️ The shared-string entry (`sst/si`) a `t="s"` cell displays, if it is one — hashed into the cell's revision, so a change to
/// the referenced text invalidates a draft exactly like a change to the cell itself.
fn referenced_shared_string<'a>(snapshot: &'a XlsxSnapshot, root: &XmlNode, path: &[usize]) -> Result<Option<&'a XmlNode>, String> {
    let (cell, scope) = scoped_node_at_path(root, path)?;
    if attribute_value(cell, &scope, &[""], "t")? != Some("s") {
        return Ok(None);
    }
    let XmlNode::Element { children, .. } = cell else { return Ok(None) };
    let mut index = None;
    for child in children {
        if element_matches(child, &namespace_scope(&scope, child), &SPREADSHEETML_NAMESPACES, "v")? {
            let XmlNode::Element { children: value, .. } = child else { continue };
            let text: String = value.iter().filter_map(|node| if let XmlNode::Text { text } = node { Some(text.as_str()) } else { None }).collect();
            index = text.trim().parse::<usize>().ok();
        }
    }
    let Some(index) = index else { return Ok(None) };
    let workbook = workbook_path(snapshot)?;
    let Some(relationship) = snapshot.opc.relationships_for(&workbook).iter().find(|relationship| relationship.rel_type.ends_with("/sharedStrings")) else { return Ok(None) };
    let Some(sst) = snapshot.xml_part(&resolve_relationship_target(&workbook, &relationship.target)).and_then(|part| part.document.root.as_ref()) else { return Ok(None) };
    let sst_scope = namespace_scope(&[], sst);
    let XmlNode::Element { children, .. } = sst else { return Ok(None) };
    let mut seen = 0;
    for child in children {
        if element_matches(child, &namespace_scope(&sst_scope, child), &SPREADSHEETML_NAMESPACES, "si")? {
            if seen == index {
                return Ok(Some(child));
            }
            seen += 1;
        }
    }
    Ok(None)
}

fn address_revision(snapshot: &XlsxSnapshot, root: &XmlNode, path: &[usize]) -> Result<String, String> {
    let mut hash = 0xcbf29ce484222325;
"""
REVISION_TAIL_OLD = """    hash_node(&mut hash, node, true);
    Ok(format!("{hash:016x}"))
}
"""
REVISION_TAIL_NEW = """    hash_node(&mut hash, node, true);
    if let Some(entry) = referenced_shared_string(snapshot, root, path)? {
        hash_node(&mut hash, entry, true);
    }
    Ok(format!("{hash:016x}"))
}
"""
CALL_1_OLD = "    let revision = address_revision(root, &node_path)?;\n"
CALL_1_NEW = "    let revision = address_revision(snapshot, root, &node_path)?;\n"
CALL_2_OLD = "address_revision(root, &address.node_path)? != address.revision"
CALL_2_NEW = "address_revision(snapshot, root, &address.node_path)? != address.revision"

ORACLE_OLD = """fn assert_independent_spreadsheet_values(bytes: Vec<u8>, case: &serde_json::Value, edited: bool) {
    use calamine::{Data, Reader};
    let mut reference: calamine::Xlsx<_> = calamine::open_workbook_from_rs(std::io::Cursor::new(bytes)).expect("Calamine opens neutral package");
"""
ORACLE_NEW = """/// 🔮️ The package calamine can read: every part verbatim except cell-level `extLst`, which calamine 0.36 refuses
/// (`cells_reader.rs`: "v, f, or is") although ECMA-376 Part 1 §18.3.1.4 allows it — the values it answers are unaffected.
fn calamine_value_projection(bytes: &[u8]) -> Vec<u8> {
    use std::io::{Cursor, Read, Write};
    let mut archive = zip::ZipArchive::new(Cursor::new(bytes)).expect("package ZIP");
    let mut writer = zip::ZipWriter::new(Cursor::new(Vec::new()));
    let options = zip::write::SimpleFileOptions::default().compression_method(zip::CompressionMethod::Stored);
    for index in 0..archive.len() {
        let mut entry = archive.by_index(index).expect("package entry");
        let name = entry.name().to_string();
        let mut content = Vec::new();
        entry.read_to_end(&mut content).expect("package entry bytes");
        if let Ok(mut text) = String::from_utf8(content.clone()) {
            for prefix in ["", "s:"] {
                let (open, close, cell_end) = (format!("<{prefix}extLst>"), format!("</{prefix}extLst>"), format!("</{prefix}c>"));
                let mut from = 0;
                while let Some(found) = text[from..].find(&open) {
                    let start = from + found;
                    let Some(end) = text[start..].find(&close).map(|end| start + end + close.len()) else { break };
                    if text[end..].starts_with(&cell_end) {
                        text.replace_range(start..end, "");
                        from = start;
                    } else {
                        from = end;
                    }
                }
            }
            content = text.into_bytes();
        }
        writer.start_file(name, options).expect("projection entry");
        writer.write_all(&content).expect("projection bytes");
    }
    writer.finish().expect("projection ZIP").into_inner()
}

fn assert_independent_spreadsheet_values(bytes: Vec<u8>, case: &serde_json::Value, edited: bool) {
    use calamine::{Data, Reader};
    let mut reference: calamine::Xlsx<_> = calamine::open_workbook_from_rs(std::io::Cursor::new(calamine_value_projection(&bytes))).expect("Calamine opens neutral package");
"""
SETS = {
    ADDRESS: [(REVISION_OLD, REVISION_NEW, 1), (REVISION_TAIL_OLD, REVISION_TAIL_NEW, 1), (CALL_1_OLD, CALL_1_NEW, 1), (CALL_2_OLD, CALL_2_NEW, 1)],
    TESTS: [(ORACLE_OLD, ORACLE_NEW, 1)],
    FIXTURE: "crlf",
}


DECLARATION = re.compile(r"^(<\?xml(?:[^?]|\?(?!>))*\?>)\s*")
MISC = r"(?:<!--(?:[^-]|-(?!->))*-->|<\?(?!xml)(?:[^?]|\?(?!>))*\?>)"
PROLOG_MISC = re.compile(r"^(" + MISC + r")\s*")
EPILOG_MISC = re.compile(r"\s*(" + MISC + r")$")


def opc_part(path: str) -> bool:
    return path == "[Content_Types].xml" or path.endswith(".rels")


def writer_form(path: str, text: str) -> str:
    """🧾️ The exact text the xlsx package writer materializes for one part: `[Content_Types].xml` and relationship parts through the
    OPC writer (`xml_document_to_opc_text_checked`: declaration + CRLF, nothing between nodes), every other XML part through the XML
    document writer (`xml_document_to_text_checked`: declaration + LF, LF after each prolog node, LF before each epilog node)."""
    declaration = DECLARATION.match(text)
    if opc_part(path):
        return text if declaration is None else declaration.group(1) + "\r\n" + text[declaration.end():]
    head, rest = "", text
    if declaration is not None:
        head, rest = declaration.group(1) + "\n", text[declaration.end():]
    while (misc := PROLOG_MISC.match(rest)) is not None:
        head, rest = head + misc.group(1) + "\n", rest[misc.end():]
    tail = ""
    while (misc := EPILOG_MISC.search(rest)) is not None:
        tail, rest = "\n" + misc.group(1) + tail, rest[:misc.start()]
    return head + rest + tail


def canonical_fixture(text: str):
    fixture = json.loads(text)
    changed = 0
    for case in fixture["cases"]:
        for part in case["parts"]:
            fixed = writer_form(part["path"], part["text"])
            changed += fixed != part["text"]
            part["text"] = fixed
        fixed = writer_form(case["sheetPath"], case["expectedSheetXml"])
        changed += fixed != case["expectedSheetXml"]
        case["expectedSheetXml"] = fixed
    return json.dumps(fixture, indent=2, ensure_ascii=False) + "\n", changed


def key(rel: str) -> str:
    return hashlib.sha256(rel.encode()).hexdigest()[:16]


def main() -> int:
    if REVERT:
        for rel in SETS:
            backup = BACKUP / key(rel)
            if backup.exists():
                (ROOT / rel).write_bytes(backup.read_bytes())
                backup.unlink()
                print(f"REVERTED {rel}")
        return 0
    problems, planned = 0, []
    for rel, hunks in SETS.items():
        path = ROOT / rel
        text = path.read_text()
        if hunks == "crlf":
            text, changed = canonical_fixture(text)
            if not changed:
                print(f"PROBLEM {rel}: already applied")
                problems += 1
                continue
        else:
            for old, new, count in hunks:
                found = text.count(old)
                if found != count:
                    print(f"PROBLEM {rel}: {'already applied' if new in text else f'anchor count {found} != {count}'}: {old[:70]!r}")
                    problems += 1
                    continue
                text = text.replace(old, new)
        planned.append((rel, path, text))
        print(f"{'WRITE' if WRITE else 'DRY'} {rel.rsplit('/🧱️base/', 1)[-1]}")
    if WRITE and problems == 0:
        BACKUP.mkdir(parents=True, exist_ok=True)
        for rel, path, text in planned:
            (BACKUP / key(rel)).write_bytes(path.read_bytes())
            path.write_text(text)
    print(f"{'write' if WRITE and problems == 0 else 'dry-run'}: {len(SETS)} files, {problems} problems")
    return 1 if problems else 0


if __name__ == "__main__":
    sys.exit(main())
