#!/usr/bin/env python3
"""📐️ LB2 p16 (window 3, stdio): ONE canonical ISO 10303-21 (Part-21) codec in the stdio contract, shared by every STEP-family format.

Decision (coordinator, 2026-09-29): the Part-21 exchange structure is the syntax of a whole family of standards (step AP203/214/242,
ifc 2x3/4 — "STEP syntax + a different EXPRESS schema"), so its tokenizer, generic instance graph, canonical writer and value
projection belong to the stdio CONTRACT, not to one artifact that others borrow (step's `io::part21`, re-exported through two
`engine` barrels and imported by ifc, semio and cad through five spellings).

- move: `📐️step/…/🚪️io/📐️part21/` (module + its laws) → `📇️registry/🧬️contract/📐️part21/` (`semio_s_artifact_stdio_contract::part21`);
  every importer (step, ifc, semio, cad) names the contract path; step's `io` module and ifc's `engine` barrel stop
  re-exporting it (no shim); cad gains the contract dependency (+ lock edge).
- canonical JSON projection (schema-first — the ifc 2x3 contract and its TS/GraphQL/Proto twins already describe it; the shared
  type projected serde-style external tagging and snake_case, so every ifc 2x3 document failed its own contract):
  `Part21Value` kind-tagged (`{kind, value}` scalars and decimals, `{kind: list, values}`, `{kind: typed, typeName, values}`,
  `{kind: unset|derived}`), `Part21Instance { id, entities: [{typeName, arguments}] }`, `Part21Header` camelCase,
  `Part21Decimal.exponent` omitted when absent. Hand-written value codecs (the derive cannot place a list under `values` nor
  a decimal under `value`), strict (unknown members refused), path edits through `edit_through_value` (p13).
- laws (contract `📐️part21/🧪️tests`): the projection is exactly the contract shape for every value kind (serde_json literal);
  value codec round trip; the canonical text is a fixed point (`write(parse(write(parse(t)))) == write(parse(t))`); path edits.
  Third-party oracle: `lb2-p16-part21-oracle.py` (IfcOpenShell 0.8.4 reads the canonical IFC 2x3 text the contract writes).
- assets: the three committed ifc 2x3 set-snapshot fixtures are converted to the canonical projection (structural transform,
  proven equal to the documented mapping).

usage: python3 lb2-p16-part21-canonical.py --dry-run | --write | --revert [--root <tree>]   (after p13: `edit_through_value`)
Backups (byte-exact, per root) under `.🧬semio/🌐hub/s14-lb2-backup/p16/<root-hash>/`; the moved files are recreated/removed.
"""
import hashlib, json, os, re, shutil, subprocess, sys

TREE = sys.argv[sys.argv.index("--root") + 1] if "--root" in sys.argv else "/Users/ueli/Documents/semio"
BACKUP = f"/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-lb2-backup/p16/{hashlib.sha256(TREE.encode()).hexdigest()[:12]}"
ART = "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts"
STEP_IO = f"{ART}/📐️step/🏅️standards/🔖️ap214/🪆️subsets/🧱️base/🚪️io"
OLD_MODULE = f"{STEP_IO}/📐️part21/🦀️.rs"
OLD_TESTS = f"{STEP_IO}/📐️part21/🧪️tests/🔬️unit/🦀️.rs"
CONTRACT = "✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract"
NEW_MODULE = f"{CONTRACT}/📐️part21/🦀️.rs"
NEW_TESTS = f"{CONTRACT}/📐️part21/🧪️tests/🔬️unit/🦀️.rs"
CONTRACT_ROOT = f"{CONTRACT}/🦀️.rs"
IFC_ROOT = f"{ART}/🏗️ifc/🦀️.rs"
CAD_MANIFEST = "✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/📦️packages/🦀️rust/Cargo.toml"
LOCK = "Cargo.lock"
IFC2X3_FIXTURES = f"{ART}/🏗️ifc/🏅️standards/🔖️2x3/🪆️subsets/🧱️base/🧫️fixtures"
TARGET = "semio_s_artifact_stdio_contract::part21"
ABSOLUTE = [
    "semio_s_artifact_stdio_step::standards::v_ap214::engine::part21",
    "semio_s_artifact_stdio_step::engine::part21",
    "semio_s_artifact_stdio_ifc::engine::part21",
    "crate::standards::v_ap214::engine::part21",
]
RELATIVE = [(r"(?<![A-Za-z_:])step::engine::part21\b", None), (r"(?<![A-Za-z_:])crate::engine::part21\b", None), (r"(?<![A-Za-z_:])super::super::part21\b", "io"), (r"(?<![A-Za-z_:])super::part21\b", "io")]
SEARCH_ROOTS = ["✏️s"]

problems = []
removed = []


def once(text, old, new, label):
    count = text.count(old)
    if count != 1:
        problems.append(f"{label}: expected 1 anchor, found {count}")
        return text
    return text.replace(old, new)


#region Module
HEADER = (
    "//! 📐️ The canonical ISO 10303-21 (Part-21, \"STEP physical file\") codec of the stdio contract: tokenizer, generic instance\n"
    "//! graph, canonical writer and the one JSON projection every STEP-family contract describes. Shared by every format of the\n"
    "//! family — step (AP203/214/242), ifc (2x3, 4): IFC is literally STEP syntax with another EXPRESS schema — each building its\n"
    "//! typed view on this graph. https://www.iso.org/standard/63141.html\n"
    "//! Escape/matrix logic was pre-verified via a standalone scratch binary (ticket\n"
    "//! `26/08/10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION`); moved here from step's `io::part21` (ticket\n"
    "//! `26/09/23/END-TO-END-OS-HUB-COLLABORATION-MCP`, LB2 p16).\n"
    "\n"
    "use crate::kernel::{edit_through_value, DslValue, FromValue, ToValue, ValueEdit, ValueError};\n"
    "use crate::value_derive;\n"
)

CODEC = '''
//#region 🔖️ValueCodec
/// 🔎️ Takes one member out of a Part-21 JSON record, naming the record and the member when it is missing.
fn part21_member(entries: &mut Vec<(String, DslValue)>, record: &str, key: &str) -> Result<DslValue, ValueError> {
    let index = entries.iter().position(|(candidate, _)| candidate == key).ok_or_else(|| ValueError::new(format!("a Part-21 {record} carries `{key}`")))?;
    Ok(entries.remove(index).1)
}

/// 🚫️ Refuses a member a Part-21 JSON record does not declare.
fn part21_exhausted(entries: &[(String, DslValue)], record: &str) -> Result<(), ValueError> {
    entries.first().map_or(Ok(()), |(key, _)| Err(ValueError::new(format!("a Part-21 {record} carries no `{key}`"))))
}

/// 🌱️ The canonical JSON projection of a Part-21 value: kind-tagged, the payload under `value` (a scalar, a reference, a
/// decimal), `values` (a list) or `typeName` + `values` (a defined-type wrapper); `unset`/`derived` carry the tag alone.
impl ToValue for Part21Value {
    fn to_value(&self) -> DslValue {
        let tagged = |kind: &str, payload: Vec<(&str, DslValue)>| DslValue::object(std::iter::once(("kind".to_string(), DslValue::String(kind.to_string()))).chain(payload.into_iter().map(|(key, value)| (key.to_string(), value))));
        match self {
            Self::Ref(id) => tagged("ref", vec![("value", id.to_value())]),
            Self::Str(text) => tagged("str", vec![("value", text.to_value())]),
            Self::Enum(text) => tagged("enum", vec![("value", text.to_value())]),
            Self::Int(value) => tagged("int", vec![("value", value.to_value())]),
            Self::Real(decimal) => tagged("real", vec![("value", decimal.to_value())]),
            Self::List(items) => tagged("list", vec![("values", items.to_value())]),
            Self::Typed { name, items } => tagged("typed", vec![("typeName", name.to_value()), ("values", items.to_value())]),
            Self::Unset => tagged("unset", Vec::new()),
            Self::Derived => tagged("derived", Vec::new()),
        }
    }
}

/// 🔀️ Decodes exactly the projection [`ToValue`] emits; an unknown kind or member is refused.
impl FromValue for Part21Value {
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        let mut entries = value.into_object()?;
        let kind = String::from_value(part21_member(&mut entries, "value", "kind")?)?;
        let record = format!("{kind} value");
        let decoded = match kind.as_str() {
            "ref" => Self::Ref(u64::from_value(part21_member(&mut entries, &record, "value")?)?),
            "str" => Self::Str(String::from_value(part21_member(&mut entries, &record, "value")?)?),
            "enum" => Self::Enum(String::from_value(part21_member(&mut entries, &record, "value")?)?),
            "int" => Self::Int(i64::from_value(part21_member(&mut entries, &record, "value")?)?),
            "real" => Self::Real(Part21Decimal::from_value(part21_member(&mut entries, &record, "value")?)?),
            "list" => Self::List(Vec::from_value(part21_member(&mut entries, &record, "values")?)?),
            "typed" => Self::Typed { name: String::from_value(part21_member(&mut entries, &record, "typeName")?)?, items: Vec::from_value(part21_member(&mut entries, &record, "values")?)? },
            "unset" => Self::Unset,
            "derived" => Self::Derived,
            other => return Err(ValueError::new(format!("unknown Part-21 value kind `{other}`"))),
        };
        part21_exhausted(&entries, &record)?;
        Ok(decoded)
    }

    fn edit_value_at_path(&mut self, path: &[&str], edit: ValueEdit) -> Result<(), ValueError> {
        edit_through_value(self, path, edit)
    }
}

/// 🌱️ The canonical JSON projection of an instance: its `id` and its `entities` as `{typeName, arguments}` records — a
/// complex instance keeps every entity, in order.
impl ToValue for Part21Instance {
    fn to_value(&self) -> DslValue {
        let entities = self.entities.iter().map(|(name, arguments)| DslValue::object([("typeName".to_string(), name.to_value()), ("arguments".to_string(), arguments.to_value())])).collect();
        DslValue::object([("id".to_string(), self.id.to_value()), ("entities".to_string(), DslValue::Array(entities))])
    }
}

/// 🔀️ Decodes exactly the projection [`ToValue`] emits; an unknown member is refused.
impl FromValue for Part21Instance {
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        let mut entries = value.into_object()?;
        let id = u64::from_value(part21_member(&mut entries, "instance", "id")?)?;
        let DslValue::Array(records) = part21_member(&mut entries, "instance", "entities")? else {
            return Err(ValueError::new("a Part-21 instance carries its entities as an array"));
        };
        part21_exhausted(&entries, "instance")?;
        let entities = records
            .into_iter()
            .map(|record| {
                let mut entity = record.into_object()?;
                let name = String::from_value(part21_member(&mut entity, "entity", "typeName")?)?;
                let arguments = Vec::from_value(part21_member(&mut entity, "entity", "arguments")?)?;
                part21_exhausted(&entity, "entity")?;
                Ok((name, arguments))
            })
            .collect::<Result<Vec<_>, ValueError>>()?;
        Ok(Self { id, entities })
    }

    fn edit_value_at_path(&mut self, path: &[&str], edit: ValueEdit) -> Result<(), ValueError> {
        edit_through_value(self, path, edit)
    }
}
//#endregion 🔖️ValueCodec
'''


def new_module(text):
    head_end = text.index("use std::fmt;\n")
    text = HEADER + text[head_end:]
    text = once(text, "    pub scale: u32,\n    pub exponent: Option<i32>,\n", "    pub scale: u32,\n    #[value(default, skip_serializing_if = \"Option::is_none\")]\n    pub exponent: Option<i32>,\n", "module: decimal exponent")
    text = once(text, "#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]\npub enum Part21Value {\n", "#[derive(Clone, Debug, PartialEq)]\npub enum Part21Value {\n", "module: value derive")
    text = once(text, "#[derive(Clone, Debug, PartialEq, Default, value_derive::ToValue, value_derive::FromValue)]\npub struct Part21Instance {\n", "#[derive(Clone, Debug, PartialEq, Default)]\npub struct Part21Instance {\n", "module: instance derive")
    text = once(text, "#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]\npub struct Part21Header {\n", "#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]\n#[value(rename_all = \"camelCase\")]\npub struct Part21Header {\n", "module: header casing")
    return once(text, "//#region 🧪️Tests\n", CODEC.lstrip("\n") + "\n//#region 🧪️Tests\n", "module: value codec")


NEW_LAWS = r'''

/// 🌱️ The value projection IS the contract shape (`Part21Value`/`Part21Instance`/`Part21Header` in the ifc 2x3 schema and its
/// TS twin), for every value kind — serde_json is the independent reader.
#[test]
fn value_projection_is_the_canonical_contract_shape() {
    let document = parse_part21("ISO-10303-21;\nHEADER;\nFILE_DESCRIPTION(('d'),'2;1');\nFILE_NAME('n','t',('a'),('o'),'p','s','z');\nFILE_SCHEMA(('IFC2X3'));\nENDSEC;\nDATA;\n#1=X(#2,'s',.E.,3,1.5,(1,2),Y(3.),$,*);\n#2=(A(1)B(2));\nENDSEC;\nEND-ISO-10303-21;\n").expect("parses");
    let json: serde_json::Value = serde_json::from_str(&crate::pack::json::to_json_string(&document)).expect("json");
    assert_eq!(json["header"]["fileSchema"], serde_json::json!([{ "kind": "list", "values": [{ "kind": "str", "value": "IFC2X3" }] }]));
    assert_eq!(
        json["instances"][0],
        serde_json::json!({ "id": 1, "entities": [{ "typeName": "X", "arguments": [
            { "kind": "ref", "value": 2 }, { "kind": "str", "value": "s" }, { "kind": "enum", "value": "E" }, { "kind": "int", "value": 3 },
            { "kind": "real", "value": { "negative": false, "coefficient": "15", "scale": 1 } },
            { "kind": "list", "values": [{ "kind": "int", "value": 1 }, { "kind": "int", "value": 2 }] },
            { "kind": "typed", "typeName": "Y", "values": [{ "kind": "real", "value": { "negative": false, "coefficient": "3", "scale": 0 } }] },
            { "kind": "unset" }, { "kind": "derived" }
        ] }] })
    );
    assert_eq!(json["instances"][1]["entities"], serde_json::json!([{ "typeName": "A", "arguments": [{ "kind": "int", "value": 1 }] }, { "typeName": "B", "arguments": [{ "kind": "int", "value": 2 }] }]));
    let reopened: Part21Document = crate::pack::json::from_json_str(&crate::pack::json::to_json_string(&document)).expect("the projection decodes");
    assert_eq!(reopened, document);
    assert!(crate::pack::json::from_json_str::<Part21Value>(r#"{"kind":"str","value":"s","extra":1}"#).is_err(), "an undeclared member is refused");
    assert!(crate::pack::json::from_json_str::<Part21Value>(r#"{"kind":"blob"}"#).is_err(), "an unknown kind is refused");
}

/// ✍️ The canonical writer is a fixed point: writing a parsed canonical text reproduces it byte for byte.
#[test]
fn canonical_text_is_a_fixed_point() {
    let canonical = write_part21(&parse_part21(FIXTURE).expect("parse"));
    assert_eq!(write_part21(&parse_part21(&canonical).expect("reparse canonical")), canonical);
}

/// 🧭️ Path edits address the projection: an argument inside a complex instance edits in place.
#[test]
fn path_edits_address_the_projection() {
    let mut document = parse_part21(FIXTURE).expect("parse");
    let path = ["instances", "0", "entities", "0", "arguments", "0"];
    document.edit_value_at_path(&path, ValueEdit::Set(DslValue::object([("kind".to_string(), DslValue::String("str".into())), ("value".to_string(), DslValue::String("edited".into()))]))).expect("argument edit");
    assert_eq!(document.instances[0].entities[0].1[0], Part21Value::Str("edited".into()));
}
'''


def new_tests(text):
    text = text.replace("#[semio_framework_async_macros::async_test]\nasync fn ", "#[test]\nfn ")
    if "async" in text:
        problems.append("tests: async remains")
    return text + NEW_LAWS
#endregion Module


#region Importers
def rewrite_imports(text, path):
    for old in ABSOLUTE:
        text = text.replace(old, TARGET)
    for pattern, scope in RELATIVE:
        if scope == "io" and not (path.startswith(STEP_IO + "/") and "/🔮️oracles/" not in path):
            continue
        if pattern.startswith("(?<![A-Za-z_:])crate::engine::part21") and not (path.startswith(f"{ART}/📐️step/") or path.startswith(f"{ART}/🏗️ifc/")):
            continue
        text = re.sub(pattern, TARGET, text)
    return re.sub(r"(?<![A-Za-z_:])engine::part21\b", TARGET, text)


def importers():
    patterns = ["engine::part21", "super::part21", "super::super::part21"]
    found = set()
    for pattern in patterns:
        out = subprocess.run(["/usr/bin/grep", "-rl", "--include=🦀️.rs", "--exclude-dir=node_modules", pattern, *SEARCH_ROOTS], cwd=TREE, capture_output=True, text=True).stdout.split("\n")
        found.update(path for path in out if path and "/🔮️oracles/" not in path and not path.startswith(f"{STEP_IO}/📐️part21/"))
    return sorted(found)


def step_io(text):
    return once(
        text,
        "/// 📐 Shared ISO 10303-21 tokenizer + generic graph — public, importable cross-artifact (ifc\n/// reuses it) and cross-plugin (📐️cad reuses it too) — dissolved out of `⚙️engine`\n/// (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES).\n#[path = \"📐️part21/🦀️.rs\"]\npub mod part21;\n",
        "",
        "step io: part21 module",
    )


def ifc_root(text):
    return once(text, "    pub use semio_s_artifact_stdio_step::engine::part21;\n", "", "ifc root: part21 re-export")


def contract_root(text):
    return once(text, '#[path = "✏️editing/🦀️.rs"]\npub mod editing;\n', '#[path = "✏️editing/🦀️.rs"]\npub mod editing;\n/// 📐️ The canonical ISO 10303-21 codec every STEP-family artifact shares.\n#[path = "📐️part21/🦀️.rs"]\npub mod part21;\n', "contract root: part21")


def cad_manifest(text):
    return once(text, "semio-s-artifact-stdio-step = { workspace = true }\n", "semio-s-artifact-stdio-contract = { workspace = true }\nsemio-s-artifact-stdio-step = { workspace = true }\n", "cad: contract dependency")


def lock(text):
    block = re.search(r'\[\[package\]\]\nname = "semio-s-artifact-cad-cad"\nversion = "[^"]+"\ndependencies = \[\n(.*?)\n\]', text, re.S)
    if not block:
        problems.append("lock: semio-s-artifact-cad-cad")
        return text
    lines = block.group(1).split("\n")
    names = [line.strip().strip(",").strip('"') for line in lines]
    if "semio-s-artifact-stdio-contract" in names:
        problems.append("lock: cad already depends on the contract")
        return text
    index = max(i for i, name in enumerate(names) if name < "semio-s-artifact-stdio-contract")
    lines.insert(index + 1, ' "semio-s-artifact-stdio-contract",')
    lines = [line if line.endswith(",") else line + "," for line in lines[:-1]] + [lines[-1]]
    return text[: block.start(1)] + "\n".join(lines) + text[block.end(1) :]
#endregion Importers


IFC2X3_IO_TESTS = f"{ART}/🏗️ifc/🏅️standards/🔖️2x3/🪆️subsets/🧱️base/🚪️io/🧪️tests/🔬️unit/🦀️.rs"
IFC2X3_LAW = r'''

/// 📐️ Every committed IFC 2x3 fixture reads through the contract's canonical Part-21 codec: the canonical text re-reads as the
/// same graph and is a fixed point. With `SEMIO_PART21_ORACLE_OUT` naming a directory, each file's canonical JSON projection is
/// written there beside its source path for the IfcOpenShell oracle (ticket `26/09/23/END-TO-END-OS-HUB-COLLABORATION-MCP`,
/// `wp-lb2/lb2-p16-part21-oracle.py`).
#[test]
fn committed_ifc2x3_fixtures_read_through_the_canonical_part21_codec() {
    use semio_s_artifact_stdio_contract::part21::{parse_part21, write_part21};
    let mut pending = vec![std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../🏅️standards/🔖️2x3")];
    let mut files = Vec::new();
    while let Some(directory) = pending.pop() {
        for entry in std::fs::read_dir(&directory).expect("fixture directory") {
            let path = entry.expect("fixture entry").path();
            if path.is_dir() {
                pending.push(path);
            } else if path.extension().is_some_and(|extension| extension == "ifc") {
                files.push(path);
            }
        }
    }
    files.sort();
    assert!(files.len() >= 40, "the committed IFC 2x3 fixtures are found: {}", files.len());
    let out = std::env::var_os("SEMIO_PART21_ORACLE_OUT").map(std::path::PathBuf::from);
    for (index, path) in files.iter().enumerate() {
        let text = String::from_utf8(std::fs::read(path).expect("fixture bytes")).expect("utf-8 fixture");
        let document = parse_part21(&text).unwrap_or_else(|error| panic!("{}: {error}", path.display()));
        let canonical = write_part21(&document);
        let reread = parse_part21(&canonical).expect("canonical text re-reads");
        assert_eq!(reread, document, "{}: the canonical text re-reads as the same graph", path.display());
        assert_eq!(write_part21(&reread), canonical, "{}: the canonical text is a fixed point", path.display());
        if let Some(out) = &out {
            std::fs::write(out.join(format!("{index:03}.json")), format!("{{\"source\":{},\"document\":{}}}", dsl::json::to_json_string(&path.to_string_lossy().to_string()), dsl::json::to_json_string(&document))).expect("oracle projection");
        }
    }
}
'''


def ifc2x3_io_tests(text):
    return text + IFC2X3_LAW


#region Fixtures
def canonical_value(node):
    if node in ("Unset", "Derived"):
        return {"kind": node.lower()}
    (tag, payload), = node.items()
    if tag in ("Ref", "Str", "Enum", "Int"):
        return {"kind": tag.lower(), "value": payload}
    if tag == "Real":
        return {"kind": "real", "value": {key: value for key, value in payload.items() if not (key == "exponent" and value is None)}}
    if tag == "List":
        return {"kind": "list", "values": [canonical_value(item) for item in payload]}
    if tag == "Typed":
        return {"kind": "typed", "typeName": payload["name"], "values": [canonical_value(item) for item in payload["items"]]}
    raise ValueError(f"unknown Part-21 value {tag}")


def canonical_document(document):
    header = document["header"]
    return {
        "header": {"fileDescription": [canonical_value(v) for v in header["file_description"]], "fileName": [canonical_value(v) for v in header["file_name"]], "fileSchema": [canonical_value(v) for v in header["file_schema"]]},
        "instances": [{"id": instance["id"], "entities": [{"typeName": name, "arguments": [canonical_value(v) for v in arguments]} for name, arguments in instance["entities"]]} for instance in document["instances"]],
    }


def convert_documents(node):
    if isinstance(node, dict):
        return {key: (canonical_document(value) if key == "document" and isinstance(value, dict) and "file_description" in value.get("header", {}) else convert_documents(value)) for key, value in node.items()}
    if isinstance(node, list):
        return [convert_documents(value) for value in node]
    return node


def fixture(text, label):
    document = json.loads(text)
    converted = convert_documents(document)
    if converted == document:
        problems.append(f"{label}: no Part-21 document")
        return text
    return json.dumps(converted, indent=2, ensure_ascii=False) + ("\n" if text.endswith("\n") else "")


def fixture_files():
    out = subprocess.run(["/usr/bin/grep", "-rl", "--include=🔣️.json", "file_description", IFC2X3_FIXTURES], cwd=TREE, capture_output=True, text=True).stdout.split("\n")
    return sorted(path for path in out if path)
#endregion Fixtures


def plan():
    edits = {CONTRACT_ROOT: contract_root}
    if os.path.isfile(os.path.join(TREE, CAD_MANIFEST)):
        edits[CAD_MANIFEST] = cad_manifest
        edits[LOCK] = lock
    for path in importers():
        edits[path] = lambda text, path=path: rewrite_imports(text, path)
    edits[f"{STEP_IO}/🦀️.rs"] = lambda text, previous=edits.get(f"{STEP_IO}/🦀️.rs"): step_io(previous(text) if previous else text)
    edits[IFC_ROOT] = lambda text, previous=edits.get(IFC_ROOT): previous(ifc_root(text)) if previous else ifc_root(text)
    edits[IFC2X3_IO_TESTS] = lambda text, previous=edits.get(IFC2X3_IO_TESTS): ifc2x3_io_tests(previous(text) if previous else text)
    for path in fixture_files():
        edits[path] = lambda text, path=path: fixture(text, path)
    return edits


def main():
    mode = next((flag for flag in ("--dry-run", "--write", "--revert") if flag in sys.argv), None)
    if mode is None:
        print(__doc__)
        sys.exit(2)
    edits = plan()
    moves = {NEW_MODULE: (OLD_MODULE, new_module), NEW_TESTS: (OLD_TESTS, new_tests)}
    if mode == "--revert":
        for path in list(edits) + [OLD_MODULE, OLD_TESTS]:
            source = os.path.join(BACKUP, path)
            if os.path.isfile(source):
                os.makedirs(os.path.dirname(os.path.join(TREE, path)), exist_ok=True)
                shutil.copyfile(source, os.path.join(TREE, path))
                print("restored", path)
        for path in moves:
            if os.path.isfile(os.path.join(TREE, path)):
                os.remove(os.path.join(TREE, path))
                print("removed", path)
        return
    staged = {}
    for path, edit in edits.items():
        before = open(os.path.join(TREE, path), encoding="utf-8").read()
        after = edit(before)
        if after == before:
            problems.append(f"{path}: unchanged")
        staged[path] = (before, after)
    created = {}
    for new, (old, transform) in moves.items():
        if os.path.exists(os.path.join(TREE, new)) or not os.path.isfile(os.path.join(TREE, old)):
            problems.append(f"move {old} -> {new}: source missing or target exists")
            continue
        created[new] = (old, transform(open(os.path.join(TREE, old), encoding="utf-8").read()))
    leftovers = [path for path, (_, after) in staged.items() if re.search(r"(?<![A-Za-z_])(engine|super)::part21\b", after) and "/🔮️oracles/" not in path]
    for path in leftovers:
        problems.append(f"{path}: an engine::part21 path remains")
    for problem in problems:
        print("PROBLEM", problem)
    print(f"{len(staged)} files edited, {len(created)} moved, {len(problems)} problems")
    if mode == "--write" and not problems:
        for path, (before, after) in staged.items():
            backup = os.path.join(BACKUP, path)
            os.makedirs(os.path.dirname(backup), exist_ok=True)
            if not os.path.exists(backup):
                open(backup, "w", encoding="utf-8").write(before)
            open(os.path.join(TREE, path), "w", encoding="utf-8").write(after)
        for new, (old, content) in created.items():
            backup = os.path.join(BACKUP, old)
            os.makedirs(os.path.dirname(backup), exist_ok=True)
            if not os.path.exists(backup):
                shutil.copyfile(os.path.join(TREE, old), backup)
            os.makedirs(os.path.dirname(os.path.join(TREE, new)), exist_ok=True)
            open(os.path.join(TREE, new), "w", encoding="utf-8").write(content)
            os.remove(os.path.join(TREE, old))
        for directory in (os.path.dirname(OLD_TESTS), os.path.dirname(os.path.dirname(OLD_TESTS)), os.path.dirname(OLD_MODULE)):
            full = os.path.join(TREE, directory)
            if os.path.isdir(full) and not os.listdir(full):
                os.rmdir(full)
        print("written; backups under", BACKUP)
    sys.exit(1 if problems else 0)


if __name__ == "__main__":
    main()
