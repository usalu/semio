"""Stage top-level natural source routing and current ordinary event callers."""
from pathlib import Path
from runpy import run_path
import json
import re

ticket = Path(__file__).parent
transport = run_path(str(ticket / "natural-pdf-transport-stage.py"))
staged = transport["staged"]
changes = staged["changes"]
output = staged["output"]
reactor = staged["reactor"]
kernel = transport["kernel"]
change = staged["change"]
emit = staged["emit"]
raw = reactor / "🏃️runtime/🧰️ops/🎞️raw-media"
changes[raw / "📥️natural-source.rs"] = "", (ticket / "natural-pdf-reactor-route.rs").read_text(encoding="utf-8")
gate = transport["staged"]["wire"]["receiver"]["module"] / "🪪️natural-source-gate.rs"
change(gate, [
    ("pub const NATURAL_SOURCE_COMMAND_KIND: u8 = 241;", "pub use semio_framework_replication_command_ingress::NATURAL_SOURCE_COMMAND_KIND;"),
])
change(raw / "🦀️.rs", [
    ('#[path = "📤️picker.rs"]', '#[path = "📥️natural-source.rs"]\nmod natural_source;\n#[path = "📤️picker.rs"]'),
    ("    if let Some(Event::OriginalMediaControl(event))", """    if !matches!(events.as_ref(), Some(Event::OriginalMediaControl(_))) {
        if let Some(status) = natural_source::drive(runtime, events, command_page, closing, cx)? { return Ok(Some(status)); }
    }
    if let Some(Event::OriginalMediaControl(event))"""),
])
change(reactor / "🚪️lifetime/🦀️.rs", [
    ("    raw_media_cursor: usize,", "    raw_media_cursor: usize,\n    natural_source_cursor: usize,"),
    ("cursor: 0, raw_media_cursor: 0 }", "cursor: 0, raw_media_cursor: 0, natural_source_cursor: 0 }"),
    ("    pub fn pending_opening", """    /// 🔁️ Selects one live natural source receipt that can progress without a host page.
    pub fn next_natural_source_poll(&mut self, closing: &[u32]) -> Option<u32> {
        let count = self.slots.values.len();
        for offset in 0..count {
            let index = (self.natural_source_cursor + offset) % count;
            let Some(slot) = self.slots.get(index) else { continue; };
            let instance = slot.cell.lifetime().instance_id;
            if slot.cell.is_live() && !closing.contains(&instance) && slot.natural_source.as_ref().is_some_and(|source| {
                source.page_pending() || source.cancellation_pending() || source.completion_pending()
            }) {
                self.natural_source_cursor = (index + 1) % count;
                return Some(instance);
            }
        }
        None
    }

    pub fn pending_opening"""),
])
change(reactor / "🔄️turn/🌀️poll-kernel-turn/🦀️.rs", [
    ("    let pending_media_control = matches!(events.as_ref(), Some(Event::OriginalMediaControl(_)));",
     "    let pending_media_control = matches!(events.as_ref(), Some(Event::OriginalMediaControl(_)))\n"
     "        || matches!(events.as_ref(), Some(Event::NaturalSourcePrepare(_) | Event::NaturalSourceControl(_)));"),
])

wire = (ticket / "natural-pdf-wire-records.rs").read_text(encoding="utf-8")
records = re.findall(r"pub (?:struct|enum) NaturalSourceWire(?:Declaration|Loan|Ack|Phase|Result) \{[^}]*\}", wire)
assert len(records) == 5
codec = '\n'.join('semio_framework_value::native_json_codec!{\n#[borrowed_json(protocol="crate::projection::KernelJson")]\n#[value(rename_all = "camelCase")]\n' + definition + '\n}' for definition in records)
change(kernel / "📤️return/🔤️json/🎠️kernel/🦀️.rs", [
    ('pub enum CommandIngressStatus {', 'pub enum CommandIngressStatus {'),
])
path = kernel / "📤️return/🔤️json/🎠️kernel/🦀️.rs"
base, revised = changes[path]
offset = revised.rfind("semio_framework_value::native_json_codec!", 0, revised.index("pub enum CommandIngressStatus {"))
assert offset >= 0
changes[path] = base, revised[:offset] + codec + "\n" + revised[offset:]

ordinary_files = """
✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🧪️tests/📈️guest-memory/🦀️.rs
✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🧪️tests/🌐️guest/🦀️.rs
🌎️hub/🧩️compositions/🀄️wfc/🧪️tests/🔬️boot-deadline/🦀️.rs
🌎️hub/🧩️compositions/🀄️wfc/🧪️tests/😴️idle-turns/🦀️.rs
🌎️hub/🧩️compositions/🀄️wfc/🧪️tests/🚪️close-ladder/🦀️.rs
🌎️hub/🧩️compositions/🌀️procedural/🧪️tests/🔬️boot-deadline/🦀️.rs
🌎️hub/🧩️compositions/🌀️procedural/🧪️tests/😴️idle-turns/🦀️.rs
🌎️hub/🧩️compositions/🌀️procedural/🧪️tests/🚪️close-ladder/🦀️.rs
🌎️hub/🧩️compositions/🌍️gis/🧪️tests/🌉️component-cold-map-patch/🦀️.rs
🌎️hub/🧩️compositions/🖍️draw/🧪️tests/🌐️guest/🦀️.rs
🌎️hub/🧩️compositions/🧩️puzzle/🧪️tests/🧩️guest-io/👆️interaction/🦀️.rs
🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/📨️plugin-artifact-channel/🧩️instances/🦀️.rs
🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🔖️plugin-binding-and-activation/🦀️.rs
🧰️framework/🛍️products/💻️os/🔨️modules/🏃️run/🧠️wasmtime-node-host/🦀️.rs
🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/📊️scale-bench/🦀️.rs
"""
plugin = staged["plugin"]
ordinary = [Path(line) for line in ordinary_files.strip().splitlines()] + [
    plugin / path for path in (
        "🖥️host/📥️ui-patch/🧪️tests/🧪️component/🦀️.rs",
        "🖥️host/🔁️lifecycle/🧪️tests/🔁️lifecycle/🦀️.rs",
        "🖥️host/🔁️poll/📥️events/🧪️tests/🦀️.rs",
        "🖥️host/🧪️tests/🔬️owned-instance-open/🦀️.rs",
        "🖥️host/🧵️shard/🚚️process-transport/🧪️tests/🔬️unit/🦀️.rs",
        "🧪️tests/🔬️plugin-runtime-plugin-builder-contract-x/🦀️.rs",
    )
]
for path in ordinary:
    base = path.read_text(encoding="utf-8")
    revised, count = re.subn(r"(Event::InstanceOpen\s*\{)(\s*)(?=request\b)",
        r"\1\2purpose: semio_framework::kernel::ActorInstanceOpenPurpose::Ordinary,\2", base)
    # The canonical projection fixture directly depends on the kernel producer.
    if path.name == "🦀️.rs" and path.parent.parent.name == "📥️events":
        revised = revised.replace("purpose: semio_framework::kernel::ActorInstanceOpenPurpose", "purpose: semio_framework_kernel::ActorInstanceOpenPurpose")
    assert count, path
    changes[path] = base, revised

laws = kernel / "📥️events/🧪️tests"

def record(properties):
    return {"type": "object", "properties": properties, "required": list(properties), "additionalProperties": False}

def edit_json(path, update):
    base = path.read_text(encoding="utf-8")
    data = json.loads(base)
    update(data)
    changes[path] = base, json.dumps(data, ensure_ascii=False, indent=2) + "\n"

ident = {"type": "string", "pattern": "^(?!0{16}$)[0-9a-f]{16}$"}
instance = {"type": "integer", "minimum": 1, "maximum": 4294967295}
word = {"type": "integer", "minimum": 1}
extent = {"type": "integer", "minimum": 1, "maximum": 536870912}
declaration = record({"formatKind": {"type":"string", "minLength":1, "maxLength":128}, "extension":{"type":"string", "minLength":2, "maxLength":16},
    "mediaType":{"type":"string", "minLength":3, "maxLength":128}, "binary":{"type":"boolean"}})
prepare = record({"instanceId": instance, "seq": word, "declaration": declaration, "extent": extent})
control = record({"loan": record({"operationId": ident, "generation": ident, "instanceId": instance, "declaration": declaration, "extent": extent}),
    "owner": word, "seq": word, "kind": {"const": 241}, "extent": extent, "control": {"enum": ["begin", "complete", "cancel"]}})

def event_schema(value):
    body = next(entry["properties"]["instanceOpen"] for entry in value["oneOf"] if entry.get("required") == ["instanceOpen"])
    body["properties"]["purpose"] = {"oneOf": [{"const": "ordinary"}, record({"naturalSource": record({"declaration": declaration, "extent": extent})})]}
    body["required"].append("purpose")
    value["oneOf"].extend((record({"naturalSourcePrepare": prepare}), record({"naturalSourceControl": control})))

selected = {"formatKind": "s.stdio.pdf@1.7", "extension": ".pdf", "mediaType": "application/pdf", "binary": True}
selected_extent = 9437877

def event_cases(data):
    data["cases"][0]["input"]["instanceOpen"]["purpose"] = "ordinary"
    data["cases"].extend((
        {"variant":"naturalSourcePrepare", "input":{"naturalSourcePrepare":{"instanceId":17,"seq":76,"declaration":selected,"extent":selected_extent}}},
        {"variant":"naturalSourceControl","input":{"naturalSourceControl":{"loan":{
            "operationId":"000000000000004c","generation":"00000000000000bd","instanceId":17,"declaration":selected,"extent":selected_extent},
            "owner":17,"seq":76,"kind":241,"extent":selected_extent,"control":"begin"}}},
    ))

edit_json(laws / "🧬️.json", event_schema)
edit_json(laws / "🔣️.json", event_cases)
change(laws / "🟦️.ts", [(".toEqual({count:24})", ".toEqual({count:26})")])
actor_fixture = plugin / "🧬️schema/🔁️persistent-local/📊️actor-input.json"
base = actor_fixture.read_text(encoding="utf-8")
revised, migrated = re.subn(r'("instanceOpen":\{)("request":)', r'\1"purpose":"ordinary",\2', base)
assert migrated == 3, migrated
changes[actor_fixture] = base, revised
emit("natural-source-integration.patch")
events = output / "event-law"
events.mkdir(exist_ok=True)
(events / "schema.json").write_text(changes[laws / "🧬️.json"][1], encoding="utf-8")
(events / "cases.json").write_text(changes[laws / "🔣️.json"][1], encoding="utf-8")

schema = plugin / "🧬️schema"
for path in schema.rglob("*.wit"):
    target = output / "wit" / path.relative_to(schema)
    target.parent.mkdir(parents=True, exist_ok=True)
    text = changes[path][1] if path in changes else path.read_text(encoding="utf-8")
    target.write_text(text, encoding="utf-8")

scope = output / "receiver-check"
(scope / "route.rs").write_text(changes[raw / "📥️natural-source.rs"][1].split("\n", 1)[1], encoding="utf-8")
life = changes[reactor / "🚪️lifetime/🦀️.rs"][1]
start = life.index("/// 🧭 Routes an authenticated natural source turn")
end = life.index("/// 🗃️ Fixed allocation-bound ownership", start)
(scope / "slot-routes.rs").write_text(life[start:end], encoding="utf-8")
crate = (scope / "lib.rs").read_text(encoding="utf-8").replace('mod natural_source;', 'pub mod natural_source;').replace('mod natural_source_gate;', 'pub mod natural_source_gate;')
crate += (ticket / "natural-pdf-route-metadata.rs").read_text(encoding="utf-8").split("\n", 1)[1]
crate += """
extern crate self as semio_plugin_abi;

pub mod wasip2 {
    wit_bindgen::generate!({
        world: "actor",
        path: WIT_STAGE,
        generate_unused_types: true,
        disable_custom_section_link_helpers: true,
    });
}

mod actual_event_lift { include!("event-lift.rs"); }
mod natural_wit_conversion {
    use crate::wasip2::semio::framework::events as wit_events;
    use crate::wasip2::exports::semio::framework::reactor as wit_reactor;
    use wit_reactor as wit;
""" .replace("WIT_STAGE", json.dumps(str((output / "wit").resolve()), ensure_ascii=False))
native = transport["native"]
converters = [
    ("host_purpose", f"purpose: &{native}::ActorInstanceOpenPurpose", "wit_events::ActorInstanceOpenPurpose"),
    ("guest_purpose", "payload: wit_events::InstanceOpenEvent", f"{native}::ActorInstanceOpenPurpose"),
    ("wit_prepare", f"event: &{native}::NaturalSourcePrepare", "wit_events::NaturalSourcePrepare"),
    ("kernel_prepare", "event: wit_events::NaturalSourcePrepare", f"{native}::NaturalSourcePrepare"),
    ("wire_control", f"event: &{native}::NaturalSourceControlEvent", "wit_events::NaturalSourceControlEvent"),
    ("typed_control", "event: wit_events::NaturalSourceControlEvent", f"{native}::NaturalSourceControlEvent"),
    ("host_ack", "ack: wit::NaturalSourceAck", f"{native}::NaturalSourceWireAck"),
    ("guest_ack", f"ack: {native}::NaturalSourceWireAck", "wit::NaturalSourceAck"),
]
for name, argument, value in converters:
    crate += f"    fn {name}({argument}) -> {value} {{ {transport[name]} }}\n"
crate += "}\n"
crate += """
type OpenSource = (String, String, wire::ActorInstanceOpenPurpose);
use semio_framework_value::retirement::controlled::ControlledRetirement;
fn check_open_source_retirement(
    payload: OpenSource,
) -> Result<ControlledRetirement<OpenSource>, (semio_framework_value::ValueError, OpenSource)> {
    ControlledRetirement::new(payload)
}
"""
lift = changes[reactor / "🧬️wit-bridge/🎟️event-lift/🦀️.rs"][1]
(scope / "event-lift.rs").write_text(lift.split("\n", 1)[1], encoding="utf-8")
(scope / "lib.rs").write_text(crate, encoding="utf-8")
