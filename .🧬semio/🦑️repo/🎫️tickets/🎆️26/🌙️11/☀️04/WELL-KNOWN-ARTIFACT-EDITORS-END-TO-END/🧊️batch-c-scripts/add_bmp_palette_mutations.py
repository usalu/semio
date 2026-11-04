import json, os, re

ANY = "/home/deck/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/"
MUT = ANY + "🧬️schema/🧬️mutations/"
BIN = ANY + "🚪️io/💾️binary/🧬️mutations/"
TXT = ANY + "🚪️io/📝️text/🧬️mutations/"
OPS = ANY + "🧬️schema/⚙️operations/🦀️.rs"

LEAVES = [
    dict(kind="set-palette-entry", variant="SetPaletteEntry", mod="set_palette_entry", dir="🌈️set-palette-entry", tag=14, verb="set", entity="palette-entry", en="Set palette entry", de="Paletteneintrag setzen",
         fields=[("index", "u32"), ("red", "u8"), ("green", "u8"), ("blue", "u8"), ("reserved", "u8")]),
    dict(kind="insert-palette-entry", variant="InsertPaletteEntry", mod="insert_palette_entry", dir="📥️insert-palette-entry", tag=15, verb="insert", entity="palette-entry", en="Insert palette entry", de="Paletteneintrag einfügen",
         fields=[("index", "u32"), ("red", "u8"), ("green", "u8"), ("blue", "u8"), ("reserved", "u8"), ("colors_used", "u32")]),
    dict(kind="remove-palette-entry", variant="RemovePaletteEntry", mod="remove_palette_entry", dir="📤️remove-palette-entry", tag=16, verb="remove", entity="palette-entry", en="Remove palette entry", de="Paletteneintrag entfernen",
         fields=[("index", "u32"), ("colors_used", "u32")]),
]

def camel(name):
    head, *rest = name.split("_")
    return head + "".join(part.capitalize() for part in rest)

def rw(path, fn):
    s = open(path).read()
    n = fn(s)
    assert n != s, path
    open(path, "w").write(n)

OPS_CODE = '''

/// 🎨️ Replaces one palette entry's color; every sample keeps its index.
pub fn palette_entry_set(image: &BmpImage, index: usize, entry: crate::schema::snapshot::BmpPaletteEntry) -> Result<BmpImage, String> {
    let mut edited = image.clone();
    *edited.palette.get_mut(index).ok_or_else(|| format!("bmp: palette entry {index} is absent"))? = entry;
    edited.validate()?;
    Ok(edited)
}

/// 📥️ Inserts one palette entry and shifts every sample index at or above it so the picture stays identical; `colors_used` is the explicit resulting header value.
pub fn palette_entry_inserted(image: &BmpImage, index: usize, entry: crate::schema::snapshot::BmpPaletteEntry, colors_used: u32) -> Result<BmpImage, String> {
    let BmpPixels::Indexed { indices } = &image.pixels else { return Err("bmp: only indexed images own a palette".into()); };
    if index > image.palette.len() { return Err(format!("bmp: palette position {index} is beyond the palette end {}", image.palette.len())); }
    let mut edited = image.clone();
    edited.palette.insert(index, entry);
    edited.colors_used = colors_used;
    edited.pixels = BmpPixels::Indexed { indices: indices.iter().map(|sample| if usize::from(*sample) >= index { sample.saturating_add(1) } else { *sample }).collect() };
    edited.validate()?;
    Ok(edited)
}

/// 📤️ Removes one palette entry no sample uses and shifts every higher sample index down; `colors_used` is the explicit resulting header value.
pub fn palette_entry_removed(image: &BmpImage, index: usize, colors_used: u32) -> Result<BmpImage, String> {
    let BmpPixels::Indexed { indices } = &image.pixels else { return Err("bmp: only indexed images own a palette".into()); };
    if index >= image.palette.len() { return Err(format!("bmp: palette entry {index} is absent")); }
    if indices.iter().any(|sample| usize::from(*sample) == index) { return Err(format!("bmp: palette entry {index} is still used by samples")); }
    let mut edited = image.clone();
    edited.palette.remove(index);
    edited.colors_used = colors_used;
    edited.pixels = BmpPixels::Indexed { indices: indices.iter().map(|sample| if usize::from(*sample) > index { *sample - 1 } else { *sample }).collect() };
    edited.validate()?;
    Ok(edited)
}
'''

def leaf_rs(L):
    fields = "\n".join(f"    pub {n}: {t}," for n, t in L["fields"])
    v = L["variant"]
    entry = "crate::schema::snapshot::BmpPaletteEntry { b: self.blue, g: self.green, r: self.red, reserved: self.reserved }"
    if L["kind"] == "set-palette-entry":
        body = f'''impl protocol::ColdMutationKind<BmpSnapshot, BmpMutation> for {v} {{
    fn diff(&self, base: &BmpSnapshot) -> protocol::MutationOutcome<BmpDiff> {{
        if base.image.palette.get(self.index as usize) == Some(&{entry}) {{
            return protocol::MutationOutcome::new(BmpDiff::default());
        }}
        match crate::schema::operations::palette_entry_set(&base.image, self.index as usize, {entry}) {{
            Ok(image) => protocol::MutationOutcome::new(BmpDiff {{ image: Some(image), ..BmpDiff::default() }}),
            Err(message) => protocol::MutationOutcome::refuse(protocol::OutcomeCode::TargetMismatch, message, ["palette-entry"]),
        }}
    }}
    fn inverse(&self, base: &BmpSnapshot) -> Result<Vec<BmpMutation>, semio_framework_value::ValueError> {{
        Ok(base.image.palette.get(self.index as usize).filter(|current| **current != {entry}).map(|current| BmpMutation::{v}(Self {{ index: self.index, red: current.r, green: current.g, blue: current.b, reserved: current.reserved }})).into_iter().collect())
    }}
}}'''
    elif L["kind"] == "insert-palette-entry":
        body = f'''impl protocol::ColdMutationKind<BmpSnapshot, BmpMutation> for {v} {{
    fn diff(&self, base: &BmpSnapshot) -> protocol::MutationOutcome<BmpDiff> {{
        match crate::schema::operations::palette_entry_inserted(&base.image, self.index as usize, {entry}, self.colors_used) {{
            Ok(image) => protocol::MutationOutcome::new(BmpDiff {{ image: Some(image), ..BmpDiff::default() }}),
            Err(message) => protocol::MutationOutcome::refuse(protocol::OutcomeCode::TargetMismatch, message, ["palette-entry"]),
        }}
    }}
    fn inverse(&self, base: &BmpSnapshot) -> Result<Vec<BmpMutation>, semio_framework_value::ValueError> {{
        Ok(crate::schema::operations::palette_entry_inserted(&base.image, self.index as usize, {entry}, self.colors_used).ok().map(|_| BmpMutation::RemovePaletteEntry(crate::schema::mutations::RemovePaletteEntry {{ index: self.index, colors_used: base.image.colors_used }})).into_iter().collect())
    }}
}}'''
    else:
        body = f'''impl protocol::ColdMutationKind<BmpSnapshot, BmpMutation> for {v} {{
    fn diff(&self, base: &BmpSnapshot) -> protocol::MutationOutcome<BmpDiff> {{
        match crate::schema::operations::palette_entry_removed(&base.image, self.index as usize, self.colors_used) {{
            Ok(image) => protocol::MutationOutcome::new(BmpDiff {{ image: Some(image), ..BmpDiff::default() }}),
            Err(message) => protocol::MutationOutcome::refuse(protocol::OutcomeCode::TargetMismatch, message, ["palette-entry"]),
        }}
    }}
    fn inverse(&self, base: &BmpSnapshot) -> Result<Vec<BmpMutation>, semio_framework_value::ValueError> {{
        let removed = crate::schema::operations::palette_entry_removed(&base.image, self.index as usize, self.colors_used).ok();
        Ok(removed.and_then(|_| base.image.palette.get(self.index as usize)).map(|entry| BmpMutation::InsertPaletteEntry(crate::schema::mutations::InsertPaletteEntry {{ index: self.index, red: entry.r, green: entry.g, blue: entry.b, reserved: entry.reserved, colors_used: base.image.colors_used }})).into_iter().collect())
    }}
}}'''
    return f'''//! {L["dir"][:2]} {L["en"]}: an indexed image's palette edit with exact inverse; every sample keeps pointing at the same color.

use crate::schema::diff::BmpDiff;
use crate::schema::mutations::BmpMutation;
use crate::BmpSnapshot;

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct {v} {{
{fields}
}}

impl protocol::MutationKind<BmpSnapshot, BmpMutation> for {v} {{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor {{ verb: "{L["verb"]}", entity: "{L["entity"]}", kind: "{L["kind"]}", record: "{v}" }};

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {{
        semio_framework_ui_locale::LocalizedLabel::native("{L["en"]}", "{L["de"]}")
    }}

    fn target(&self) -> Vec<String> {{
        vec![format!("palette:{{}}", self.index)]
    }}
}}

{body}
'''

LABELS = {"index": ("Position", "Position"), "red": ("Red", "Rot"), "green": ("Green", "Grün"), "blue": ("Blue", "Blau"), "reserved": ("Reserved byte", "Reserviertes Byte"), "colors_used": ("Colors used", "Verwendete Farben")}

def payload_schema(L):
    props = {}
    required = []
    for order, (name, ty) in enumerate(L["fields"]):
        key = camel(name)
        required.append(key)
        maximum = 255 if ty == "u8" else None
        entry = {"type": "integer", "minimum": 0}
        if maximum is not None:
            entry["maximum"] = maximum
        entry["x-semio-ui"] = {"widget": "stepper", "label": {"en": LABELS[name][0], "de": LABELS[name][1]}, "step": 1, "precision": 0, "group": "palette", "order": (order + 1) * 10}
        props[key] = entry
    return {"$schema": "http://json-schema.org/draft-07/schema#", "$id": f"https://json.schemas.assets.semio-tech.com/s/stdio/bmp/v3/any/mutation/{L['kind']}/schema.json", "title": L["variant"], "type": "object", "additionalProperties": False, "required": required, "properties": props}

def descriptor(L):
    return {"schemaVersion": 1, "owner": f"✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧬️schema/🧬️mutations/{L['dir']}", "semanticKind": L["kind"], "displayName": L["en"], "emoji": L["dir"][:2], "aggregateVariant": L["variant"], "payloadSchema": "🧬️schema/🔣️.json", "textOpcode": L["kind"], "binaryTag": L["tag"], "invertibility": "explicit-mutation", "diffParticipation": "detect", "outcomeClasses": ["applied", "rejected"], "composition": "atomic", "requiredLanguageSurfaces": ["rust", "typescript", "graphql", "protobuf", "json-schema", "text", "binary"]}

def ts_type(ty): return "number"
def leaf_ts(L):
    body = "\n".join(f"  {camel(n)}: {ts_type(t)}" for n, t in L["fields"])
    return f"export interface {L['variant']} {{\n{body}\n}}\n"
def leaf_graphql(L):
    body = "\n".join(f"  {camel(n)}: Int!" for n, t in L["fields"])
    return f"input {L['variant']}Input {{\n{body}\n}}\n"
def agg_graphql_input(L):
    body = "\n".join(f"  {camel(n)}: Int!" for n, t in L["fields"])
    return f"input {L['variant']} {{\n{body}\n}}\n"
def leaf_proto(L):
    body = "\n".join(f"  uint32 {n} = {i + 1};" for i, (n, t) in enumerate(L["fields"]))
    return f"message {L['variant']} {{\n{body}\n}}\n"

def binary_leaf(L):
    v, m, k = L["variant"], L["mod"], L["kind"]
    return f'''//! 💾️ {L["en"]} binary codec.
use crate::schema::mutations::{{BmpMutation, {v}}};
use crate::standards::v_v3::subsets::any::io::binary::mutations::Entry;
pub const BINARY_TAG: u8 = dsl::protocol_record::tag_u8(include_str!("../📡️.protocol.semio"), "{k}");
pub const CODEC: Entry = Entry {{ tag: BINARY_TAG, encode, decode }};
pub fn encode(value: &BmpMutation) -> Option<Result<Vec<u8>, protocol::ProtocolError>> {{
    let BmpMutation::{v}(payload) = value else {{ return None }};
    Some(Ok(semio_framework_pack_json::to_json_string(payload).into_bytes()))
}}
pub fn decode(bytes: &[u8]) -> Result<BmpMutation, protocol::ProtocolError> {{
    let parsed = semio_framework_pack_json::parse_bytes(bytes, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| protocol::ProtocolError::Malformed {{ what: "{k}", offset: 0, detail: error.to_string() }})?;
    let payload = <{v} as semio_framework_value::FromValue>::from_value(semio_framework_pack_json::to_dsl_value(&parsed)).map_err(|error| protocol::ProtocolError::Malformed {{ what: "{k}", offset: 0, detail: error.to_string() }})?;
    Ok(BmpMutation::{v}(payload))
}}
'''

def text_leaf(L):
    v, m, k = L["variant"], L["mod"], L["kind"]
    return f'''//! 📝️ {L["en"]} text codec.
use crate::schema::mutations::BmpMutation;
use crate::standards::v_v3::subsets::any::io::text::mutations::Entry;
pub const TEXT_OPCODE: &str = "{k}";
pub const CODEC: Entry = Entry {{ opcode: TEXT_OPCODE, print, parse }};
pub fn print(value: &BmpMutation) -> Option<String> {{
    let BmpMutation::{v}(payload) = value else {{ return None }};
    Some(format!("{{TEXT_OPCODE}} payload={{}}", semio_framework_pack_json::to_json_string(payload).as_bytes().iter().map(|byte| format!("{{byte:02x}}")).collect::<String>()))
}}
pub fn parse(line: &str) -> Result<BmpMutation, semio_framework_diagnostic::TextError> {{
    let (opcode, arguments) = line.split_once(' ').unwrap_or((line, ""));
    if opcode != TEXT_OPCODE {{
        return Err(error(format!("expected {{TEXT_OPCODE}}")));
    }}
    let encoded = arguments.strip_prefix("payload=").ok_or_else(|| error("missing payload"))?;
    if !encoded.len().is_multiple_of(2) {{
        return Err(error("payload has odd hexadecimal length"));
    }}
    let bytes = (0..encoded.len()).step_by(2).map(|index| u8::from_str_radix(&encoded[index..index + 2], 16).map_err(|failure| failure.to_string())).collect::<Result<Vec<_>, _>>().map_err(error)?;
    crate::standards::v_v3::subsets::any::io::binary::mutations::{m}::decode(&bytes).map_err(|failure| error(failure.to_string()))
}}
fn error(message: impl Into<String>) -> semio_framework_diagnostic::TextError {{
    semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, message, semio_framework_diagnostic::TextSpan::at(1, 1))
}}
'''

def canonical_arm(L):
    v = L["variant"]
    parts = ",".join(f'("{camel(n)}",number(v.{n}))' for n, t in L["fields"])
    return f'            Self::{v}(v)=>("{L["kind"]}",object([{parts}])),\n'

def main():
    if "palette_entry_set" in open(OPS).read():
        print("already applied"); return
    rw(OPS, lambda s: s + OPS_CODE)
    for L in LEAVES:
        d = MUT + L["dir"] + "/"
        os.makedirs(d + "🧬️schema", exist_ok=True)
        open(d + "🦀️.rs", "w").write(leaf_rs(L))
        open(d + "🔣️.json", "w").write(json.dumps(descriptor(L), ensure_ascii=False, indent=2) + "\n")
        open(d + "🧬️schema/🔣️.json", "w").write(json.dumps(payload_schema(L), ensure_ascii=False, indent=2) + "\n")
        open(d + "🟦️.ts", "w").write(leaf_ts(L))
        open(d + "🔗️.graphql", "w").write(leaf_graphql(L))
        open(d + "🛰️.proto", "w").write(leaf_proto(L))
        for base, fn in ((BIN, binary_leaf), (TXT, text_leaf)):
            os.makedirs(base + L["dir"], exist_ok=True)
            open(base + L["dir"] + "/🦀️.rs", "w").write(fn(L))
    # aggregate rs
    def agg_rs(s):
        s = s.replace("pub use super::replace_samples::ReplaceSamples;\n", "pub use super::replace_samples::ReplaceSamples;\n" + "".join(f"pub use super::{L['mod']}::{L['variant']};\n" for L in LEAVES), 1)
        s = s.replace("    ReplaceSamples(ReplaceSamples),\n}", "    ReplaceSamples(ReplaceSamples),\n" + "".join(f"    {L['variant']}({L['variant']}),\n" for L in LEAVES) + "}", 1)
        s = s.replace("        BmpMutation::ReplaceSamples(ReplaceSamples { region: crate::schema::snapshot::BmpRegion { x: 0, y: 0, width: 1, height: 1 }, indices: Vec::new(), samples: vec![crate::schema::snapshot::BmpNativeSample::default()] }),\n",
            "        BmpMutation::ReplaceSamples(ReplaceSamples { region: crate::schema::snapshot::BmpRegion { x: 0, y: 0, width: 1, height: 1 }, indices: Vec::new(), samples: vec![crate::schema::snapshot::BmpNativeSample::default()] }),\n        BmpMutation::SetPaletteEntry(SetPaletteEntry { index: 0, red: 0, green: 0, blue: 0, reserved: 0 }),\n        BmpMutation::InsertPaletteEntry(InsertPaletteEntry { index: 0, red: 0, green: 0, blue: 0, reserved: 0, colors_used: 0 }),\n        BmpMutation::RemovePaletteEntry(RemovePaletteEntry { index: 0, colors_used: 0 }),\n", 1)
        return s
    rw(MUT + "🦀️.rs", agg_rs)
    # aggregate json
    def agg_json(s):
        data = json.loads(s)
        for L in LEAVES:
            data["oneOf"].append({"type": "object", "additionalProperties": False, "required": ["mutation", "payload"], "properties": {"mutation": {"const": L["kind"]}, "payload": {"$ref": f"https://json.schemas.assets.semio-tech.com/s/stdio/bmp/v3/any/mutation/{L['kind']}/schema.json"}}})
        return json.dumps(data, ensure_ascii=False, indent=2) + "\n"
    rw(MUT + "🔣️.json", agg_json)
    # graphql
    def agg_graphql(s):
        s = s.replace("replaceSamples: ReplaceSamples }", "replaceSamples: ReplaceSamples " + " ".join(f"{L['variant'][0].lower() + L['variant'][1:]}: {L['variant']}" for L in LEAVES) + " }", 1)
        return s.rstrip("\n") + "\n\n" + "\n".join(agg_graphql_input(L) for L in LEAVES)
    rw(MUT + "🔗️.graphql", agg_graphql)
    def agg_proto(s):
        s = s.replace("ReplaceSamples replace_samples = 13; }", "ReplaceSamples replace_samples = 13; " + " ".join(f"{L['variant']} {L['mod']} = {L['tag']};" for L in LEAVES) + " }", 1)
        return s.rstrip("\n") + "\n\n" + "\n".join(leaf_proto(L) for L in LEAVES)
    rw(MUT + "🛰️.proto", agg_proto)
    def agg_ts(s):
        s = s.replace("export type BmpMutation =", "".join(leaf_ts(L) + "\n" for L in LEAVES) + "export type BmpMutation =", 1)
        s = s.replace("{ readonly mutation: 'replace-samples'; readonly payload: ReplaceSamples };", "{ readonly mutation: 'replace-samples'; readonly payload: ReplaceSamples }" + "".join(f"\n  | {{ readonly mutation: '{L['kind']}'; readonly payload: {L['variant']} }}" for L in LEAVES) + ";", 1)
        return s
    rw(MUT + "🟦️.ts", agg_ts)
    rw(MUT + "🔏️canonical/🦀️.rs", lambda s: s.replace("        };Ok(Some(object([(\"mutation\",text(kind))", "".join(canonical_arm(L) for L in LEAVES) + "        };Ok(Some(object([(\"mutation\",text(kind))", 1))
    rw(MUT + "📦️packages/🦀️rust/🪞️mirror/🦀️.rs", lambda s: s.replace("                        pub mod replace_samples;\n", "                        pub mod replace_samples;\n" + "".join(f'                        #[path = "../../../{L["dir"]}/🦀️.rs"]\n                        pub mod {L["mod"]};\n' for L in LEAVES), 1))
    def reg(s):
        s = s.replace("replace_samples::CODEC];", "replace_samples::CODEC, " + ", ".join(f"{L['mod']}::CODEC" for L in LEAVES) + "];", 1)
        return s.rstrip("\n") + "\n\n" + "\n\n".join(f'#[path = "{L["dir"]}/🦀️.rs"]\npub mod {L["mod"]};' for L in LEAVES) + "\n"
    rw(BIN + "🦀️.rs", reg)
    rw(TXT + "🦀️.rs", reg)
    rw(BIN + "📡️.protocol.semio", lambda s: s.rstrip("\n") + "\n" + "".join(f"record {L['kind']} tag={L['tag']}\nfield payload bytes\n" for L in LEAVES))
    def grammar(s):
        s = s.replace("# Direct identities: replace-image, paint-indexed-region, paint-direct-region, replace-samples", "# Direct identities: replace-image, paint-indexed-region, paint-direct-region, replace-samples, set-palette-entry, insert-palette-entry, remove-palette-entry", 1)
        s = s.replace("| replace-samples\n", "| replace-samples | set-palette-entry | insert-palette-entry | remove-palette-entry\n", 1)
        return s.rstrip("\n") + "\n" + "".join(f'{L["kind"]} = "{L["kind"]}" "payload" "=" hex\n' for L in LEAVES)
    rw(TXT + "📖️.grammar.semio", grammar)
    contract = ANY + "🧪️tests/🧫️fixtures/📜️direct-mutation-contract/🔣️.json"
    def roster(s):
        data = json.loads(s)
        for L in LEAVES:
            data["mutations"].append({"kind": L["kind"], "variant": L["variant"], "directory": L["dir"], "textOpcode": L["kind"], "binaryTag": L["tag"], "requiredFiles": ["🦀️.rs", "🔣️.json", "🧬️schema/🔣️.json", "🟦️.ts", "🔗️.graphql", "🛰️.proto"]})
        return json.dumps(data, ensure_ascii=False, indent=2) + "\n"
    rw(contract, roster)
    print("applied")

main()
