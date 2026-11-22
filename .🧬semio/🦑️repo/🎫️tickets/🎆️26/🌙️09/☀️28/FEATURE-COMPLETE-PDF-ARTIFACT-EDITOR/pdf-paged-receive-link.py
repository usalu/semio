"""🔬 Link the PDF paged Pack seam to the product codec compiler cohort."""
import hashlib
import json
import os
import re
from pathlib import Path
import subprocess
import sys

ROOT = Path(__file__).resolve().parent
CACHE = Path(sys.argv[1]).resolve()
OUT = ROOT / "🗑️generated" / "pdf-reopen-record"
OUT.mkdir(parents=True, exist_ok=True)
RUSTC = ["repo", "tools", "exec", "--light", "rustup", "--", "run", "nightly-2026-07-20", "rustc"]
PDF = Path("✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base")
case = json.loads((ROOT / "pdf-paged-receive.json").read_text())
if "PDF_REOPEN_ORACLE_MODULE_PATH" in os.environ:
    sys.path.insert(0, os.environ["PDF_REOPEN_ORACLE_MODULE_PATH"])
import fitz

fixture_path = PDF / "🔨️modules/🗜️filters/🪣️stream-data/🧪️tests/🔬️unit/🧪️streams.pdf"
with fitz.open(fixture_path) as pdf:
    assert len(pdf) == case["pdfPages"]
    assert pdf[0].get_contents() == [case["streamObject"]]
    stream = pdf.xref_stream(case["streamObject"])
    assert len(stream) == case["streamBytes"]
    assert hashlib.sha256(stream).hexdigest() == case["streamSha256"]
(OUT / "stream.bin").write_bytes(stream)
text = (PDF / "🚪️io/📝️text/📸️snapshot/🦀️.rs").read_text()
table = text.split("const FIELDS:", 1)[1].split("];", 1)[0]
declared = re.findall(r'\((\d+),"([^"]+)",(true|false)\)', table)
assert len(declared) == 32
fields = []
ordinary = []
values = []
for field, key, optional in declared:
    field = int(field)
    shape = "Text" if field <= 2 else "Value"
    fields.append(f'{{ let mut f = BorrowedFieldSpec::new({field}, "{key}", BorrowedShape::{shape}); f.optional = {optional}; f }}')
    ordinary.append(f'{{ let mut f = FieldSpec::new({field}, "{key}", Shape::{shape}); f.optional = {optional}; f }}')
    if field == 1:
        value = 'FieldValue::Text("stdio.pdf.1.7".into())'
    elif field == 2:
        value = 'FieldValue::Text("1.7".into())'
    elif field == 30:
        value = f'''FieldValue::Value(DslValue::Array(vec![DslValue::Object(vec![
            ("id".into(), DslValue::Object(vec![("num".into(), DslValue::Number(semio_framework_value::Number::UInt({case["streamObject"]}))), ("gen".into(), DslValue::Number(semio_framework_value::Number::UInt(0)))])),
            ("value".into(), DslValue::Object(vec![("kind".into(), DslValue::String("stream".into())), ("dict".into(), DslValue::Array(vec![])), ("data".into(), DslValue::Bytes(include_bytes!("stream.bin").to_vec())), ("filters".into(), DslValue::Array(vec![]))]))
        ])]))'''
    else:
        value = 'FieldValue::Value(DslValue::Null)' if optional == "true" else 'FieldValue::Value(DslValue::Array(vec![]))'
    values.append(f'    record.fields.insert({field}, {value});')
(OUT / "fields.rs").write_text(f"const PAGE: usize = {case['pageBytes']};\nconst CUTS: &[usize] = &{case['cancellationCuts']};\n" + "static FIELDS: [BorrowedFieldSpec; 32] = [\n" + ",\n".join(fields) + """,\n];
fn fixture() -> (Pages, RecordValue) {
    let declared = RecordSpec::new(None, RecordLayout::Lines, vec![
""" + ",\n".join(ordinary) + """,\n    ]);
    let mut record = RecordValue::default();
""" + "\n".join(values) + """
    let inner = pack::record::encode_document(&declared, &record, &Default::default()).unwrap();
    let envelope = semio_format::SemioEnvelope::from_envelope_id(ENVELOPE, semio_format::Component::Pack, 1).unwrap();
    let wrapped = semio_format::wrap_binary(&envelope, &inner);
    (Pages::new(&wrapped), record)
}
""")

def find(package):
    return sorted(CACHE.glob(f"{package}/*/out/*.rmeta"))[0]

def metadata(path):
    return subprocess.run(RUSTC + ["-Zls=all", str(path)], capture_output=True, text=True, check=True).stdout

pack = find("semio-framework-pack")
dependencies = {match[1]: match[2] for match in re.finditer(r"^\d+ ([a-z0-9_]+)-([a-f0-9]{16}) hash", metadata(pack), re.M)}
names = dict(pack=pack, semio_os_semio=find("semio-os-semio"))
for name in ("semio_framework_dsl_record", "semio_framework_value", "semio_framework_value_native_owned", "semio_framework_value_testing_effect_bank"):
    names[name] = next(CACHE.glob(f"*/*/out/lib{name}-{dependencies[name]}.rmeta"))
protocols = list(CACHE.glob("semio-framework-replication/*/out/libprotocol-*.rmeta"))
names["protocol"] = next(q for q in protocols if f"semio_framework_replication_wire_core-{dependencies['semio_framework_replication_wire_core']}" in metadata(q))
response = OUT / "rustc.rsp"
flags = ["--edition=2021", "--crate-name", "pdf_paged_receive_law", "--test", str(ROOT / "pdf-paged-receive-law.rs"), "-o", str(OUT / "law")]
for name, path in names.items():
    for extension in (".rmeta", ".rlib"):
        flags += ["--extern", f"{name}={path.with_suffix(extension)}"]
for directory in CACHE.glob("*/*/out"):
    flags += ["-L", f"dependency={directory}"]
response.write_text("\n".join(flags) + "\n")
subprocess.run(RUSTC + ["@" + str(response)], check=True)
subprocess.run([OUT / "law", "--nocapture"], check=True)
