#!/usr/bin/env python3
"""🚦️ sf3 target 1, step 1: writes the source files of the shared generation-native crate (`semio-s-procedural-core-generation-sqlite-native`) by cutting the live template and the generation2d records.

usage: g2_native_files.py [--write]
Every anchor is asserted unique; the template files are rewritten from their live text, never retyped.
"""
import re, sys, os

REPO = "/Users/ueli/Documents/semio"
WRITE = "--write" in sys.argv
NATIVE = f"{REPO}/✏️s/🔌️plugins/🌀️procedural/🫀️core/🧬️generation/🪶️sqlite/🚦️native"
ART = f"{REPO}/✏️s/🔌️plugins/🌀️procedural/🗿️artifacts"
TAIL = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📝️text/📸️snapshot/🦀️.rs"
S2 = f"{ART}/🌀️generation2d/{TAIL}"


def rd(path):
    return open(path, encoding="utf-8").read()


def wr(path, text):
    print(("write " if WRITE else "(dry) ") + path.replace(REPO + "/", ""), len(text.split("\n")), "lines")
    if WRITE:
        os.makedirs(os.path.dirname(path), exist_ok=True)
        open(path, "w", encoding="utf-8").write(text)


def once(text, anchor):
    assert text.count(anchor) == 1, (text.count(anchor), anchor[:80])
    return text.index(anchor)


s2 = rd(S2)
start = once(s2, "/// 🔒️ `ValueDsl` mirrors")
end_anchor = "pub fn form_generation_from_dsl(generation: FormGenerationDsl) -> FormGeneration {\n    FormGeneration { id: generation.id, name: generation.name, values: generation.values.into_iter().collect() }\n}\n"
end = once(s2, end_anchor) + len(end_anchor)
block = s2[start:end]

def widen(block):
    out, inside = [], False
    for line in block.split("\n"):
        if re.match(r"pub (struct|enum) \w+ \{$", line):
            inside = line.startswith("pub struct")
        elif line == "}":
            inside = False
        elif inside and re.match(r"^    (\w+): ", line):
            line = "    pub(crate) " + line[4:]
        out.append(line)
    return "\n".join(out)

records = widen(block)
records = records.replace("semio_framework_value::DslValue", "DslValue")
header = '''//! 🧬️ The seven local DSL twins of the generation host and play-state records, shared by generation 2d and 3d, with the conversions to and from the foreign `flow` and `playbook` types.
//!
//! The foreign types cannot carry the derive themselves (orphan rule), so the twins live here and both artifacts re-export them.
use semio_framework_os_flow_contract::neural::{Atom, Dictionary, Value as NeuralValue};
use semio_framework_os_flow_contract::{CameraJson, SynapseSpec, Widget, WidgetLayout};
use semio_framework_os_playbook_contract::FormGeneration;
use semio_framework_value::DslValue;
use std::collections::BTreeMap;

'''
records_text = header + records
assert "pub(crate) pub(crate)" not in records_text
wr(f"{NATIVE}/🧬️records/🦀️.rs", records_text)

old_root = rd(f"{NATIVE}/🦀️.rs")
old_rows = rd(f"{NATIVE}/📏️rows/🦀️.rs")

doc = old_root
doc = doc.replace("//! 🚦️ Literal eight-field Generation native mirrors with cumulative construction controls.\nuse super::*;\n", "//! 🧱️ Literal eight-field Generation native document codec with cumulative construction controls, generic over the artifact snapshot.\n", 1)
assert doc != old_root
imports_old = "use semio_framework_os_flow_contract::{OrderedMap,WidgetLayout,neural::{ColdDictionaryBuilder,ColdOwner,ColdRetire}};\n"
imports_new = ("use super::{GenerationNative,rows,values};\n"
               "use super::records::{DictEntryDsl,SynapseSpecDsl,ValueDsl,WidgetDsl,camera_from_dsl,camera_to_dsl,layout_to_dsl,synapse_from_dsl};\n"
               "use semio_framework_os_flow_contract::{FlowHostSnapshot,OrderedMap,Widget,WidgetLayout,neural::{Atom,ColdDictionaryBuilder,ColdOwner,ColdRetire,Dictionary,Value as NeuralValue}};\n"
               "use semio_framework_os_playbook_contract::{FormGeneration,GenerationPlayState};\n"
               "use semio_framework_os_kernel as store;\n")
once(doc, imports_old)
doc = doc.replace(imports_old, imports_new)
for anchor in ["#[path=\"🌱️value/🦀️.rs\"]mod values;\n", "#[path=\"📏️rows/🦀️.rs\"]mod rows;\n"]:
    once(doc, anchor)
    doc = doc.replace(anchor, "")
err = doc[doc.index("fn error(e:ValueError)"):]
err = err[:err.index("\n") + 1]
assert err.startswith("fn error(e:ValueError)->semio_framework_diagnostic::TextError{")
doc = doc.replace(err, "")
sub = [
    ("fn encode_record_native(encoding:", "fn encode_record_native<K:GenerationNative>(encoding:"),
    ("fn decode_record_native(construct:impl FnOnce(&semio_framework_dsl_record::RecordValue,&mut NativeDecodeControl<'_>)->Result<ControlledSnapshot,ValueError>", "fn decode_record_native<K:GenerationNative>(construct:impl FnOnce(&semio_framework_dsl_record::RecordValue,&mut NativeDecodeControl<'_>)->Result<K::Snapshot,ValueError>"),
    ("native_owner:&mut semio_framework_os_kernel::NativeSnapshotDecodeOwner<'_, '_>)->Result<ControlledSnapshot,ValueError>{\n use semio_framework_os_kernel::sqlite_snapshot::SqliteSnapshotPhase;", "native_owner:&mut semio_framework_os_kernel::NativeSnapshotDecodeOwner<'_, '_>)->Result<K::Snapshot,ValueError>{\n use semio_framework_os_kernel::sqlite_snapshot::SqliteSnapshotPhase;"),
    ("pub(crate) fn encode(document:&ControlledSnapshot,", "pub fn encode<K:GenerationNative>(document:&K::Snapshot,"),
    ("pub(crate) fn decode(payload:", "pub fn decode<K:GenerationNative>(payload:"),
    ("Result<ControlledSnapshot,ValueError>{let maximum=control.limits().max_rows;", "Result<K::Snapshot,ValueError>{let maximum=control.limits().max_rows;"),
    (" encode_record_native(encoding,|c|{\n  rows::owned(document,maximum,maximum_bytes,c)?;", " encode_record_native::<K>(encoding,|c|{\n  rows::owned(K::host(document),K::state(document),maximum,maximum_bytes,c)?;"),
    ("let host=&document.host_snapshot;let generation=&document.generation;", "let host=K::host(document);let generation=K::state(document);"),
    ("decode_record_native(|record,c|{rows::borrowed(record,maximum,maximum_bytes,c)?;let parsed=ControlledSnapshotDsl::__dsl_from_record_controlled(record,c)?;let result=(||->Result<ControlledSnapshot,ValueError>{", "decode_record_native::<K>(|record,c|{rows::borrowed(record,maximum,maximum_bytes,c)?;let parsed=K::fields(record,c)?;let result=(||->Result<K::Snapshot,ValueError>{"),
    ("Ok(ControlledSnapshot{host_snapshot:host.take(),generation:generation.take().into()})})();", "Ok(K::assemble(host.take(),generation.take().into()))})();"),
]
for a, b in sub:
    once(doc, a)
    doc = doc.replace(a, b)
assert doc.count("ControlledSnapshotDsl::__dsl_spec_producer()") == 4, doc.count("ControlledSnapshotDsl::__dsl_spec_producer()")
doc = doc.replace("ControlledSnapshotDsl::__dsl_spec_producer()", "K::spec_producer()")
assert doc.count("CONTROLLED_ENVELOPE_ID") == 2 and doc.count("CONTROLLED_PACK_PREFIX") == 2
doc = doc.replace("CONTROLLED_ENVELOPE_ID", "K::ENVELOPE_ID").replace("CONTROLLED_PACK_PREFIX", "K::PACK_PREFIX")
for leftover in ["ControlledSnapshot", "super::*"]:
    assert leftover not in doc, leftover
wr(f"{NATIVE}/🧱️document/🦀️.rs", doc)

rows = old_rows
a = "use super::*;\n"
once(rows, a)
rows = rows.replace(a, "use semio_framework_os_flow_contract::{FlowHostSnapshot,Widget,neural::{Atom,Dictionary,Value as NeuralValue}};\nuse semio_framework_os_playbook_contract::GenerationPlayState;\n")
sig = "pub(super) fn owned<C:NativeSchemaControl>(snapshot:&ControlledSnapshot,maximum:usize"
once(rows, sig)
rows = rows.replace(sig, "pub(super) fn owned<C:NativeSchemaControl>(host:&FlowHostSnapshot,state:&GenerationPlayState,maximum:usize")
for x, y in [(" for widget in &snapshot.host_snapshot.widgets{", " for widget in &host.widgets{"), (" for generation in &snapshot.generation.generations{", " for generation in &state.generations{")]:
    once(rows, x)
    rows = rows.replace(x, y)
head, tail = rows.split("pub(super) fn owned")
body, rest = tail.split("fn field(", 1)
body = body.replace("snapshot.host_snapshot.", "host.").replace("snapshot.generation.", "state.")
rows = head + "pub(super) fn owned" + body + "fn field(" + rest
assert "snapshot." not in body and "snapshot." not in rows.replace("semantic snapshot", "")
wr(f"{NATIVE}/📏️rows/🦀️.rs", rows)

new_s2 = s2[:start] + s2[end:]
wr("/private/tmp/claude-501/-Users-ueli--claude-spares-spare-PBncC5/6c384ddf-f4e2-43bf-8d3c-c66daff05e49/scratchpad/sf3-s2-preview.rs", new_s2)
