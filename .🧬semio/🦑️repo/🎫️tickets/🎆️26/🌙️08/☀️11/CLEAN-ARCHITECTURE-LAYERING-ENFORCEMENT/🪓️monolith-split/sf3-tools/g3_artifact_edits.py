#!/usr/bin/env python3
"""🚦️ sf3 target 1, step 2: generation2d and generation3d `io/text/snapshot` stop owning the seven DSL twins and the native template; they re-export the shared records and implement `GenerationNative`.

usage: g3_artifact_edits.py [--write]
Anchors are asserted unique; the 2d and 3d record blocks are compared after normalisation before either is cut.
"""
import re, sys, os

REPO = "/Users/ueli/Documents/semio"
WRITE = "--write" in sys.argv
ART = f"{REPO}/✏️s/🔌️plugins/🌀️procedural/🗿️artifacts"
TAIL = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📝️text/📸️snapshot"
NATIVE = "semio_s_procedural_core_generation_sqlite_native"


def rd(p):
    return open(p, encoding="utf-8").read()


def wr(p, t):
    print(("write " if WRITE else "(dry) ") + p.replace(REPO + "/", ""))
    if WRITE:
        open(p, "w", encoding="utf-8").write(t)


def once(text, anchor):
    assert text.count(anchor) == 1, (text.count(anchor), anchor[:90])
    return text.index(anchor)


def normalise(block):
    lines = [l for l in block.split("\n") if not l.lstrip().startswith("///")]
    text = "\n".join(lines).replace("semio_framework_value::DslValue", "DslValue")
    return re.sub(r"\s+", " ", text).strip()


def cut_block(text, art):
    if art == "2d":
        start = once(text, "/// 🔒️ `ValueDsl` mirrors")
        end_anchor = "pub fn form_generation_from_dsl(generation: FormGenerationDsl) -> FormGeneration {\n    FormGeneration { id: generation.id, name: generation.name, values: generation.values.into_iter().collect() }\n}\n"
    else:
        start = once(text, "//#region 🔖️DslMirror\n#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord)]\npub struct ValueDsl {") + len("//#region 🔖️DslMirror\n")
        m = re.search(r"pub fn form_generation_from_dsl\(generation: FormGenerationDsl\) -> FormGeneration \{\n    FormGeneration \{[^\n]*\n\}\n", text)
        assert m, "3d end"
        end_anchor = m.group(0)
    end = once(text, end_anchor) + len(end_anchor)
    return start, end


def form_free(block):
    cut = block.index("#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord)]\npub struct FormGenerationDsl")
    head = block[:cut]
    body = block[cut:]
    struct = body[: body.index("pub fn form_generation_to_dsl")]
    return head, struct


texts, cuts = {}, {}
for art, name in (("2d", "🌀️generation2d"), ("3d", "🧊️generation3d")):
    p = f"{ART}/{name}/{TAIL}/🦀️.rs"
    texts[art] = rd(p)
    cuts[art] = cut_block(texts[art], art)
b2 = texts["2d"][cuts["2d"][0]:cuts["2d"][1]]
b3 = texts["3d"][cuts["3d"][0]:cuts["3d"][1]]
h2, s2 = form_free(b2)
h3, s3 = form_free(b3)
assert normalise(h2) == normalise(h3), "the record heads of 2d and 3d differ"
assert normalise(s2) == normalise(s3), "FormGenerationDsl differs"
print("record blocks identical after normalisation (heads", len(normalise(h2)), "chars); form conversions differ by design (clone vs DslValue::to_value, both identity on DslValue)")

REEXPORT = (
    f"pub use {NATIVE}::{{\n    camera_from_dsl, camera_to_dsl, form_generation_from_dsl, form_generation_to_dsl, layout_from_dsl, layout_to_dsl, synapse_from_dsl, synapse_to_dsl, widget_from_dsl, widget_to_dsl, CameraJsonDsl, FormGenerationDsl, SynapseSpecDsl, WidgetDsl, WidgetLayoutDsl,\n}};\n"
)


def marker(art, snapshot, envelope, prefix):
    n = f"Generation{art}Native"
    s = f"Generation{art}Snapshot"
    d = f"Generation{art}SnapshotDsl"
    g = f"{NATIVE}::GenerationFields"
    return f'''struct {n};

impl {NATIVE}::GenerationNative for {n} {{
    type Snapshot = {s};
    const ENVELOPE_ID: &'static str = "{envelope}";
    const PACK_PREFIX: Option<&'static [u8]> = {prefix};
    fn spec_producer() -> semio_framework_dsl_record::RecordSpecProducer {{
        {d}::__dsl_spec_producer()
    }}
    fn fields(record: &semio_framework_dsl_record::RecordValue, control: &mut semio_framework_value::NativeDecodeControl<'_>) -> Result<{g}, semio_framework_value::ValueError> {{
        let parsed = {d}::__dsl_from_record_controlled(record, control)?;
        Ok({g} {{
            schema: parsed.schema,
            camera: parsed.camera,
            widgets: parsed.widgets,
            synapses: parsed.synapses,
            layout: parsed.layout,
            selected_generation_id: parsed.selected_generation_id,
            preview_text: parsed.preview_text,
            generations: parsed.generations,
        }})
    }}
    fn host(snapshot: &{s}) -> &FlowHostSnapshot {{
        &snapshot.host_snapshot
    }}
    fn state(snapshot: &{s}) -> &GenerationPlayState {{
        snapshot.generation.as_state()
    }}
    fn assemble(host_snapshot: FlowHostSnapshot, generation: semio_framework_artifact_playbook_playbook::GenerationPlayRoot) -> {s} {{
        {s} {{ host_snapshot, generation }}
    }}
}}

pub fn encode_sqlite_native(document: &{s}, encoding: semio_framework_os_kernel::sqlite_snapshot::SnapshotEncoding, control: &mut semio_framework_os_kernel::sqlite_snapshot::SqliteSnapshotControl<'_>, native_owner: &mut semio_framework_os_kernel::NativeSnapshotEncodeOwner<'_, '_>) -> Result<store::io_schema::IoPayload, semio_framework_value::ValueError> {{
    {NATIVE}::encode::<{n}>(document, encoding, control, native_owner)
}}

pub fn decode_sqlite_native(payload: &store::io_schema::IoPayload, control: &mut semio_framework_os_kernel::sqlite_snapshot::SqliteSnapshotControl<'_>, native_owner: &mut semio_framework_os_kernel::NativeSnapshotDecodeOwner<'_, '_>) -> Result<{s}, semio_framework_value::ValueError> {{
    {NATIVE}::decode::<{n}>(payload, control, native_owner)
}}
'''


def carrier_old(art, prefix):
    s = f"Generation{art}Snapshot"
    d = f"Generation{art}SnapshotDsl"
    return (
        f"type ControlledSnapshotDsl = {d};\ntype ControlledSnapshot = {s};\n"
        f'const CONTROLLED_ENVELOPE_ID: &str = "procedural.generation{art}";\n'
        f"const CONTROLLED_PACK_PREFIX: Option<&[u8]> = {prefix};\n"
        '#[path="../../../../../../../../../🫀️core/🧬️generation/🪶️sqlite/🚦️native/🦀️.rs"]mod controlled;\n'
        "pub(crate) use controlled::{decode as decode_sqlite_native,encode as encode_sqlite_native};\n"
    )


def prune_imports(text, art, removed_text):
    tests = rd(f"{ART}/{'🌀️generation2d' if art == '2d' else '🧊️generation3d'}/{TAIL}/🧪️tests/🔬️unit/🦀️.rs")
    out = text
    use_lines = {
        "2d": [
            ("use semio_framework_artifact_flow_flow::neural::{Atom, Dictionary, Value as NeuralValue};\n", ["Atom", "Dictionary", "NeuralValue"]),
            ("use semio_framework_artifact_flow_flow::{CameraJson, FlowHostSnapshot, SynapseSpec, Widget, WidgetLayout};\n", ["CameraJson", "FlowHostSnapshot", "SynapseSpec", "Widget", "WidgetLayout"]),
        ],
        "3d": [
            ("use semio_framework_artifact_flow_flow::neural::{Atom, Dictionary, Value as NeuralValue};\n", ["Atom", "Dictionary", "NeuralValue"]),
            ("use semio_framework_artifact_flow_flow::{CameraJson, FlowHostSnapshot, SynapseSpec, Widget, WidgetLayout};\n", ["CameraJson", "FlowHostSnapshot", "SynapseSpec", "Widget", "WidgetLayout"]),
            ("use semio_framework_artifact_playbook_playbook::{FormGeneration, GenerationPlayState};\n", ["FormGeneration", "GenerationPlayState"]),
        ],
    }[art]
    for line, names in use_lines:
        once(out, line)
        rest = out.replace(line, "")
        code = "\n".join(l for l in rest.split("\n") if not l.lstrip().startswith(("//", "use ", "pub use ")))
        keep = [n for n in names if re.search(r"\b" + n + r"\b", code) or re.search(r"\b" + n + r"\b", tests)]
        dropped = [n for n in names if n not in keep]
        print(f"  {art}: {line.strip()[:70]} keep={keep} drop={dropped}")
        if not dropped:
            continue
        if not keep:
            out = rest
        else:
            def rename(n):
                return "Value as NeuralValue" if n == "NeuralValue" else n
            head = line[: line.index("{")]
            out = out.replace(line, head + "{" + ", ".join(rename(n) for n in keep) + "};\n")
    return out


for art, name in (("2d", "🌀️generation2d"), ("3d", "🧊️generation3d")):
    p = f"{ART}/{name}/{TAIL}/🦀️.rs"
    t = texts[art]
    s, e = cuts[art]
    removed = t[s:e]
    t = t[:s] + REEXPORT + t[e:]
    prefix = 'Some(b"P2D2")' if art == "2d" else "None"
    old = carrier_old(art, prefix)
    once(t, old)
    t = t.replace(old, marker(art, "", f"procedural.generation{art}", prefix))
    t = prune_imports(t, art, removed)
    wr(p, t)
