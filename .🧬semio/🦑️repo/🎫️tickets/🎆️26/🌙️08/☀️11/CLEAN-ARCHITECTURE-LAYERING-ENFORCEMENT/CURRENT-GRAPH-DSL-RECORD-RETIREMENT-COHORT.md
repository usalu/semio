# Current Graph DSL Record Retirement Cohort

Read-only current source audit. No source writes, Cargo/compiler runs or claims of runtime success. The actual four-normal-dependency manifest belongs to semio-framework-dsl-record under 🗣️dsl/🧬️schema. Older eight-dependency semio-framework-pack-record proposal under 🎒️pack/🌱️value is a DIFFERENT owner; it is not superseded by this DSL manifest and must not overwrite it.

## Current executable symbol scope

Graph's actual code uses Record APIs and Value APIs through dsl_core. Prior raw grep included comment-only DslEnum/IdiomHooks/hooks/passthrough/parse_wire/Wire references; those do NOT require new derive/lexical exports. idiom_hooks already directly uses semio_framework_dsl; actual derive sites use value_derive, whose generated identities use neutral semio_framework_value. Keep existing direct lexical DSL/Value/Diagnostic dependencies. No record-derive normal dependency is needed solely for actual Graph current code.

Current neutral record exports own Shape/FieldValue/RecordSpec/RecordValue/layout/spec/control producer, native_encoding, Writer/JoinMode/wire parser and binding APIs by own pub definitions/pub use binding::*. It has private use imports for ValueError/ValueRefusalKind, DslValue/Number, NativeDecodeControl/NativeEncodeControl; FromValue/ToValue are not root exports. Therefore Graph→Record alias redirection alone fails those names. Public reexports can legally expose original neutral Value identities without new trait/wrapper definitions.

## Smallest genuine direct-owner cohort (Root clean-owner decision)

Do not add Value pub reexports to neutral Record merely to preserve Graph umbrella alias. Existing private imports reflect Value ownership and remain unchanged. Remove Graph dsl_core alias entirely and route each executable reference to its exact owner.

| Current symbols | Replacement |
|---|---|
| DslValue, FromValue, ToValue, ValueError, NativeEncodeControl, NativeDecodeControl | semio_framework_value::... |
| DslField, FieldValue, RecordValue, RecordSpec, RecordSpecProducer, FieldSpec, RecordLayout, Shape, NativeSchemaControl, WireValue/WireNode/WireEdgeLabel, Writer, JoinMode, parse_wire_text, print_shape, producer, native_encoding, __rt | semio_framework_dsl_record::... |
| schema::producer | semio_framework_dsl_record::producer (canonical direct module, no schema alias) |

Actual code-only roster contains11 Rust files (seven normal production/wiring and four cfg test files), plus Graph Cargo manifest. Split the PropertyDef binder's mixed use declaration: Record owns schema/field/spec/layout/shape/control; Value owns encode/decode controls and ValueError. Remove nested `use crate::dsl_core` helper imports after all their qualified names become exact owner paths. No record public export/source mutation is necessary for this Graph retirement.

Replace Graph OS normal dependency with actual neutral DSL-record path; preserve existing direct Value/DSL/Diagnostic/Value derive edges. Remove extern crate OS as dsl_core. Code-only runtime idiom hooks already use direct lexical DSL. DslEnum is comment-only, so no record derive dependency needed by Graph. Keep Graph public Jack module named dsl.

Graph DslField impl targets are owned PropertyValue/PropertyBag/PropertyDef and macro scalar targets PropertyKind/PortDirection. No current Graph foreign ValueType/DslValue DslField impl was found; ValueType decoder hook is a function, not orphan impl. Neutral Record binding owns DslField for ValueType/DslValue legally. Value derives use neutral Value expansion paths; unresolved compiler-generated count is UNKNOWN until sole worker compiler metadata, not asserted zero by text search.

Current root has no neutral DSL-record literal member: admit canonical member registration only with exact root current-byte review and worker evidence; preserve actual foreign four-dependency DSL manifest. Root workspace/wholeOS membership deletion blocker remains separate.

Exact affected Rust roster:

- `🧰️framework/🔨️modules/🕸️graph/⚙️engine/🦀️.rs`
- `🧰️framework/🔨️modules/🕸️graph/⚙️engine/🧪️tests/🔬️unit/🦀️.rs`
- `🧰️framework/🔨️modules/🕸️graph/📦️packages/🦀️rust/🦀️.rs`
- `🧰️framework/🔨️modules/🕸️graph/🗣️dsl/🦀️.rs`
- `🧰️framework/🔨️modules/🕸️graph/🗣️dsl/🧪️tests/🔬️unit/🦀️.rs`
- `🧰️framework/🔨️modules/🕸️graph/🛂️manifest/🗂️properties/🦀️.rs`
- `🧰️framework/🔨️modules/🕸️graph/🛂️manifest/🚦️properties/🦀️.rs`
- `🧰️framework/🔨️modules/🕸️graph/🛂️manifest/🦀️.rs`
- `🧰️framework/🔨️modules/🕸️graph/🛂️manifest/🧪️tests/🏷️type/🦀️.rs`
- `🧰️framework/🔨️modules/🕸️graph/🛂️manifest/🧪️tests/🔬️unit/🦀️.rs`
- `🧰️framework/🔨️modules/🕸️graph/🛂️manifest/🪆️binding/🦀️.rs`

## Limits and blockers beyond minimal direct edge

Distinct product OS schema/binding copies still define nominally distinct DslField/Shape/FieldValue/RecordValue. Graph public signatures/trait impls now use neutral identity; OS consumer/native pack integration may fail until it adopts the same canonical neutral DSL Record. Eliminating the direct Graph normal OS edge alone is valid progress but not proof of one complete public contract or whole-product build.

Current neutral Record normal closure includes DSL, Value, Diagnostic, Pack JSON (four direct deps). Its tests require record-derive/async macros/serde_json; cfg/refusal/unit/macro source must be independently compiled and checked. Missing self alias for generated ::semio_framework_dsl_record paths within unit tests may require explicit `extern crate self as semio_framework_dsl_record` after actual compiler evidence. No blanket derive dependency/export should be added because comments mention DslEnum.

Current Graph native tests use the same dsl_core alias, public owned PropertyValue/Type and Graph Value derives. Actual source-only leftover neutral Record tests mentioning OS remain potential deletion/cfg bind hazards; verify mounted test imports, not orphan file matches. Normal closure must be compiled under deletion proof and root registration handled separately; current root wholeOS member declarations prevent claiming actual registered root command deletion independence.

## Code-only symbol receipt

```json
[
  "DslField",
  "DslValue",
  "FieldValue",
  "FromValue",
  "JoinMode",
  "NativeDecodeControl",
  "NativeEncodeControl",
  "NativeSchemaControl",
  "Shape",
  "ToValue",
  "ValueError",
  "WireEdgeLabel",
  "WireNode",
  "WireValue",
  "Writer",
  "__rt",
  "native_encoding",
  "parse_wire_text",
  "print_shape",
  "producer",
  "schema"
]
```

## Full current before receipts

### 🧰️framework/🔨️modules/🗣️dsl/🧬️schema/🦀️.rs

SHA-256 `4cb90af61211cdb308967c32cb37114a1a849fe3b83d893eb010e6c559d3f1b1`; 114826 bytes.

```
//! 🧬️ `dsl_schema` — the data-driven declarative grammar engine: technologies describe their
//! document/op grammar as `RecordSpec`/`Shape` DATA (not code), and this crate parses text against
//! that data into a generic `Cst` (walked by typed binders that `dsl_derive` will generate) and
//! prints it back via a chunk `Writer` that structurally guarantees the newline law: every
//! grammar renders both as multi-line canonical `Document` text and as one space-joined `Inline`
//! line, and both re-parse to the same value.

use semio_framework_dsl::format_f64;
use semio_framework_dsl::lex;
use semio_framework_dsl::parse_f64;
use semio_framework_diagnostic::Limits;
use semio_framework_dsl::SpannedToken;
use semio_framework_diagnostic::TextError;
use semio_framework_diagnostic::TextSpan;
use semio_framework_dsl::TokenClass;
use semio_framework_dsl::TokenKind;
use std::collections::{HashMap, HashSet};
use crate::notation as dsl_notation;
#[path = "🖋️notation/🦀️.rs"]
pub mod notation;
#[path = "🪆️binding/🦀️.rs"]
mod binding;
pub use binding::*;
#[path = "🛫️encode/🦀️.rs"]
pub mod native_encoding;
use semio_framework_value::NativeDecodeControl;
use semio_framework_value::{ValueError,ValueRefusalKind};

#[path = "🛬️decoding/🦀️.rs"]
mod controlled_decoding;
pub use controlled_decoding::{parse_exact_controlled,parse_expr_text_controlled};

#[path = "🛫️encoding/🦀️.rs"]
mod controlled_encoding;
pub use controlled_encoding::{print_controlled,print_expr_controlled};

#[path = "🏭️producer/🦀️.rs"]
pub mod producer;
pub use producer::{NativeSchemaControl,RecordSpecProducer};

//#region 🔖️Shape
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RecordLayout {
    /// All fields printed as space-joined `key=value` tokens on one logical unit.
    Inline,
    /// Each field printed as its own line/statement (norm-family style) in Document mode;
    /// collapses to the same space-joined form as `Inline` when rendered in `JoinMode::Inline`.
    Lines,
    /// `<name> = <keyword>(arg1=val1 arg2=val2)` — a named call assignment (the graph-family
    /// construction-chain notation, e.g. `extrude = brep.solid.extrude(profile=w1 axis=v1)`).
    /// Requires exactly one field marked [`FieldSpec::call_name`] (printed before `=`, must have
    /// `Shape::Text`) and `RecordSpec.keyword` set (the dotted call target after `=`, e.g.
    /// `"brep.solid.extrude"` — printed and matched as one token since `.` is `dsl_core`
    /// ident-continue). Every other field prints/parses exactly as it would under `Inline`
    /// (positional bare, keyed as `key=value`), just inside the parens instead of bare after the
    /// keyword.
    Call,
}

/// 🧩️ What one field's value looks like, textually. Covers all 16 grammar-shape
/// primitives found across the 32 hand-rolled implementations this engine replaces.
#[derive(Clone, Debug)]
pub enum Shape {
    Bool,
    Int,
    UInt,
    Float,
    Text,
    Bytes64,
    /// Unit-variant keyword table: `(tag, ordinal)` pairs.
    Enum(Vec<(String, u32)>),
    /// Packed `x,y,z` — `len = Some(n)` enforces arity.
    Tuple(Box<Shape>, Option<usize>),
    /// 🧾️ Bracketed scalar lists `[a b c]`; each direct record item is braced `[ { fields } { } ]`.
    List(Box<Shape>),
    /// 📄️ Inline nested `key=value` run using another record's fields. Direct list items are braced. Lazy for the same
    /// reason `Statements` is: a self-referential `#[derive(DslRecord)]` struct (a field whose type
    /// recurses back to the struct itself, e.g. a dynamic-value type with a nested-dictionary-of-
    /// itself field) would otherwise recurse infinitely just building its own `RecordSpec`.
    Record(RecordSpecProducer),
    /// Wraps the inner shape in `{ ... }`.
    Block(Box<Shape>),
    /// Keyword-dispatched, order-preserving repeated records: `(keyword, spec_fn)` per variant.
    /// `spec_fn` is a zero-capture `fn` pointer, not an eagerly-built `RecordSpec` — a genuinely
    /// self-referential grammar (a recursive block tree whose own variant table contains itself)
    /// would otherwise recurse infinitely just building the table. Calling `(spec_fn.ordinary)()` one level at
    /// a time bottoms out naturally at real documents' finite depth instead.
    Statements(Vec<(String, RecordSpecProducer)>),
    /// `{ key=value ... }` block, keys sorted on canonical print.
    Map(Box<Shape>),
    /// Dynamic JSON-equivalent literal.
    Value,
    /// Structure-of-Arrays columnar table: `key [col:TYPE ...] { v11 v12 ...  v21 v22 ... }`.
    /// `fn() -> RecordSpec` is the SAME lazy self-referential seam `Record`/`Statements` use.
    /// Parses to `FieldValue::List(Vec<FieldValue::Record>)` — identical to `List(Record)` — so
    /// no binder/diff/derive path needs to know a field is a table rather than a verbose AoS list.
    /// Only a record's OWN keyword-prefixed field prints/parses the compact bare SoA form above
    /// (`print_record`/`parse_record_body`'s dedicated lookahead); a `Table` reached any other way
    /// (a table row's own column, a list element, the generic `key=` keyed dispatch) prints/parses
    /// as the bracketed AoS list `[ {...} {...} ]` instead — the bare form has no bracket of its
    /// own to mark where it ends, so it's only safe directly after a record's leading keyword.
    Table(RecordSpecProducer),
    /// Graph endpoint literal: `id[:kind][@port][->|--id2[:kind2][@port2]]{props}`.
    Wire,
    /// A `Shape::Float` refinement: prints/parses with a glued unit suffix (`210GPa`). The value
    /// is stored in `unit`'s declared unit; a compatible alien suffix on parse (`210000MPa`)
    /// converts into it, an incompatible one (wrong dimension) is a parse error. No suffix at all
    /// means the bare number is already in the declared unit.
    Quantity(&'static semio_framework_dsl::UnitSpec),
    /// A `Shape::Quantity` restricted to angle units (`deg`/`rad`/`turn`) — kept as its own variant
    /// (rather than reusing `Quantity` with an angle unit) so `shape_type_name`/table headers can
    /// tell a length from a rotation at a glance (`NUM` vs `QTY` vs `ANG`).
    Angle(&'static semio_framework_dsl::UnitSpec),
    /// A `Shape::Text` refinement: a checked reference to an entity of the named kind (e.g.
    /// `"material"`). Prints/parses identically to `Text` (bare-preferred) — the only difference
    /// is semantic (a paired `FieldSpec.defines` anchor lets `LanguageService::validate` flag a
    /// dangling reference), so it needs no dedicated parse/print arm, only a distinct type name.
    Ref(&'static str),
    /// `@x,y[,z,...]` — a placement/position literal, `dims` coordinates. Value is
    /// `FieldValue::Tuple` (same representation `Shape::Tuple` uses) with exactly `dims` floats.
    Coord(u8),
    /// `^x,y,z` — a unit direction/axis vector, always exactly 3 floats. Value is
    /// `FieldValue::Tuple` — distinct from `Coord(3)` only by its `^` sigil and `DIR` type tag,
    /// so a reader never confuses "where" from "which way".
    Dir,
    /// `WxHxD` (glued, no separator token — see `parse_dim`) — `dims` size components. Value is
    /// `FieldValue::Tuple` with exactly `dims` floats.
    Dim(u8),
    /// `(lo..hi)` or `(lo..hi,step)` — value is `FieldValue::Tuple` of 2 or 3 floats (no dedicated
    /// `RangeValue` type: a range IS a small tuple, just printed with `..` instead of `,` between
    /// the first two elements).
    Range,
    /// `xN` — a bare count/multiplicity literal. Value is `FieldValue::UInt`.
    Count,
    /// `(expr)` — an arithmetic formula literal, always outer-parenthesized. Value is the ONE
    /// genuinely new `FieldValue` variant this engine adds (`FieldValue::Expr`) — everything else
    /// in this Shape reuses an existing representation.
    Expr,
    /// Fenced verbatim text in Document mode (`` ```lang\ncontent\n``` ``), escaped-quoted `Text`
    /// in Inline mode — both parse to the same `FieldValue::Text`, the "Document/Inline agree" law
    /// applied to a shape whose Document form needs raw multi-line content. `lang` is this field's
    /// DECLARED embedded language (e.g. `"jack"`); an authored fence's own lang tag must be empty
    /// or match it.
    Embed(&'static str),
    /// Fence language taken from a sibling Text field named by this key (see `#[dsl(lang_from)]`).
    EmbedFrom(&'static str),
}

#[derive(Clone, Debug)]
pub struct FieldSpec {
    pub id: u16,
    /// Empty for positional-only fields.
    pub key: String,
    /// `Some(n)` = nth positional token right after the keyword, in declaration order among
    /// positional fields.
    pub position: Option<u8>,
    pub shape: Shape,
    pub optional: bool,
    /// Splice a nested record's fields directly into this record (shared doc/op field schemas).
    pub flatten: bool,
    /// Paired with a sibling field's `Shape::Ref(kind)`: this field's value is the canonical id of
    /// an entity of kind `kind`. `None` for every field that isn't such an anchor. Not wire/hash
    /// relevant (LanguageService-only, see `Shape::Ref`'s doc comment) — purely an authoring aid.
    pub defines: Option<&'static str>,
    /// The one field a `RecordLayout::Call` spec prints before `=` and parses as the assignment
    /// target — see [`RecordLayout::Call`]. Always `false` outside a `Call`-layout spec; ignored
    /// (never printed/parsed specially) for any other layout.
    pub is_call_name: bool,
}

impl FieldSpec {
    // 🚫️async: E1 pure spec builder consumed by E4 fn-pointer slots (Shape::Record) and derive-macro output — see R9
    pub fn new(id: u16, key: &str, shape: Shape) -> Self {
        Self { id, key: key.to_string(), position: None, shape, optional: false, flatten: false, defines: None, is_call_name: false }
    }

    // 🚫️async: E1 pure spec builder consumed by E4 fn-pointer slots (Shape::Record) and derive-macro output — see R9
    pub fn positional(mut self, index: u8) -> Self {
        self.position = Some(index);
        self
    }

    // 🚫️async: E1 pure spec builder consumed by E4 fn-pointer slots (Shape::Record) and derive-macro output — see R9
    pub fn optional(mut self) -> Self {
        self.optional = true;
        self
    }

    // 🚫️async: E1 pure spec builder consumed by E4 fn-pointer slots (Shape::Record) and derive-macro output — see R9
    pub fn flatten(mut self) -> Self {
        self.flatten = true;
        self
    }

    // 🚫️async: E1 pure spec builder consumed by E4 fn-pointer slots (Shape::Record) and derive-macro output — see R9
    pub fn defines(mut self, kind: &'static str) -> Self {
        self.defines = Some(kind);
        self
    }

    /// 📛️ Marks this field as the one printed before `=` / parsed as the assignment target
    /// in a `RecordLayout::Call` spec. See [`RecordLayout::Call`].
    // 🚫️async: E1 pure spec builder consumed by E4 fn-pointer slots (Shape::Record) and derive-macro output — see R9
    pub fn call_name(mut self) -> Self {
        self.is_call_name = true;
        self
    }
}

#[derive(Clone, Debug)]
pub struct RecordSpec {
    pub keyword: Option<String>,
    pub layout: RecordLayout,
    pub fields: Vec<FieldSpec>,
}

impl RecordSpec {
    // 🚫️async: E1 pure spec builder consumed by E4 fn-pointer slots (Shape::Record) and derive-macro output — see R9
    pub fn new(keyword: Option<&str>, layout: RecordLayout, fields: Vec<FieldSpec>) -> Self {
        Self { keyword: keyword.map(|k| k.to_string()), layout, fields }
    }

    /// 🏗️ Same as [`Self::new`] but takes an already-owned keyword — what
    /// `dsl_derive`-generated code builds from a spliced `String` literal.
    // 🚫️async: E1 pure spec builder consumed by E4 fn-pointer slots (Shape::Record) and derive-macro output — see R9
    pub fn new_owned(keyword: Option<String>, layout: RecordLayout, fields: Vec<FieldSpec>) -> Self {
        Self { keyword, layout, fields }
    }
}

pub struct GrammarSpec {
    pub name: String,
    pub root: RecordSpec,
}
//#endregion 🔖️Shape

//#region 🔖️JsonSchema
// 🎫️ ticket 26/08/17/LLM-FIRST-OS-VIA-THE-SEMIO-OS-MCP-GATEWAY packet P3-manifest-schema §3.2: the
// gateway's JSON Schema derivation is primarily `ActionArgDef::json_schema()` (manifest-declared
// args); THIS is the fallback for `app_commands!` payload structs whose only declared shape is a
// `RecordSpec` (`#[derive(dsl::DslRecord)]`) — the catalog compiler tags whatever it emits from here
// `x-semio-confidence: "payload"`, not this module's concern.
/// 📐️ JSON Schema 2020-12 for one `Shape` leaf/node — recurses through `Tuple`/`List`/
/// `Record`/`Block`/`Statements`/`Map`/`Table`. `Quantity`/`Angle` carry their unit as
/// `x-semio-unit`; `Ref(kind)` carries the referenced entity kind as `x-semio-ref`; every shape with
/// no native JSON Schema vocabulary (`Bytes64`/`Wire`/`Coord`/`Dir`/`Dim`/`Range`/`Count`/`Expr`/
/// `Embed`/`EmbedFrom`) additionally carries `x-semio-shape` naming the exact `Shape` variant.
pub fn shape_json_schema(shape: &Shape) -> semio_framework_pack_json::Value {
    use semio_framework_pack_json::{object, Value};
    let number_array = |min: u64, max: u64, extra: Option<(&str, &str)>| {
        let mut fields = vec![("type".to_string(), Value::from("array")), ("items".to_string(), object([("type".to_string(), Value::from("number"))])), ("minItems".to_string(), Value::from(min)), ("maxItems".to_string(), Value::from(max))];
        if let Some((key, value)) = extra {
            fields.push((key.to_string(), Value::from(value)));
        }
        object(fields)
    };
    match shape {
        Shape::Bool => object([("type".to_string(), Value::from("boolean"))]),
        Shape::Int => object([("type".to_string(), Value::from("integer"))]),
        Shape::UInt => object([("type".to_string(), Value::from("integer")), ("minimum".to_string(), Value::from(0u64))]),
        Shape::Float => object([("type".to_string(), Value::from("number"))]),
        Shape::Text => object([("type".to_string(), Value::from("string"))]),
        Shape::Bytes64 => object([("type".to_string(), Value::from("string")), ("contentEncoding".to_string(), Value::from("base64")), ("x-semio-shape".to_string(), Value::from("bytes64"))]),
        Shape::Enum(variants) => object([("type".to_string(), Value::from("string")), ("enum".to_string(), Value::Array(variants.iter().map(|(tag, _)| Value::from(tag.clone())).collect()))]),
        Shape::Tuple(inner, len) => {
            let items = shape_json_schema(inner);
            let mut fields = vec![("type".to_string(), Value::from("array")), ("items".to_string(), items)];
            if let Some(len) = len {
                fields.push(("minItems".to_string(), Value::from(*len as u64)));
                fields.push(("maxItems".to_string(), Value::from(*len as u64)));
            }
            object(fields)
        }
        Shape::List(inner) => {
            let items = shape_json_schema(inner);
            object([("type".to_string(), Value::from("array")), ("items".to_string(), items)])
        }
        Shape::Record(spec_fn) => record_spec_json_schema(&(spec_fn.ordinary)()),
        Shape::Block(inner) => shape_json_schema(inner),
        Shape::Statements(variants) => {
            let mut one_of = Vec::with_capacity(variants.len());
            for (keyword, spec_fn) in variants {
                let mut entry = record_spec_json_schema(&(spec_fn.ordinary)());
                if let Value::Object(map) = &mut entry {
                    map.insert("x-semio-keyword", Value::from(keyword.clone()));
                }
                one_of.push(entry);
            }
            object([("type".to_string(), Value::from("array")), ("items".to_string(), object([("oneOf".to_string(), Value::Array(one_of))]))])
        }
        Shape::Map(inner) => {
            let additional_properties = shape_json_schema(inner);
            object([("type".to_string(), Value::from("object")), ("additionalProperties".to_string(), additional_properties)])
        }
        Shape::Value => Value::Object(semio_framework_pack_json::Object::new()),
        Shape::Table(spec_fn) => {
            let items = record_spec_json_schema(&(spec_fn.ordinary)());
            object([("type".to_string(), Value::from("array")), ("items".to_string(), items)])
        }
        Shape::Wire => object([("type".to_string(), Value::from("string")), ("x-semio-shape".to_string(), Value::from("wire"))]),
        Shape::Quantity(unit) => object([("type".to_string(), Value::from("number")), ("x-semio-unit".to_string(), Value::from(unit.symbol))]),
        Shape::Angle(unit) => object([("type".to_string(), Value::from("number")), ("x-semio-unit".to_string(), Value::from(unit.symbol)), ("x-semio-shape".to_string(), Value::from("angle"))]),
        Shape::Ref(kind) => object([("type".to_string(), Value::from("string")), ("x-semio-ref".to_string(), Value::from(*kind))]),
        Shape::Coord(dims) => number_array(*dims as u64, *dims as u64, Some(("x-semio-shape", "coord"))),
        Shape::Dir => number_array(3, 3, Some(("x-semio-shape", "dir"))),
        Shape::Dim(dims) => number_array(*dims as u64, *dims as u64, Some(("x-semio-shape", "dim"))),
        Shape::Range => number_array(2, 3, Some(("x-semio-shape", "range"))),
        Shape::Count => object([("type".to_string(), Value::from("integer")), ("minimum".to_string(), Value::from(0u64)), ("x-semio-shape".to_string(), Value::from("count"))]),
        Shape::Expr => object([("type".to_string(), Value::from("string")), ("x-semio-shape".to_string(), Value::from("expr"))]),
        Shape::Embed(lang) => object([("type".to_string(), Value::from("string")), ("x-semio-shape".to_string(), Value::from("embed")), ("x-semio-lang".to_string(), Value::from(*lang))]),
        Shape::EmbedFrom(key) => object([("type".to_string(), Value::from("string")), ("x-semio-shape".to_string(), Value::from("embed")), ("x-semio-lang-from".to_string(), Value::from(*key))]),
    }
}

/// 📐️ JSON Schema 2020-12 object for one `RecordSpec` — one property per `FieldSpec.key`
/// (positional-only fields, whose `key` is empty, are omitted — no name to key a JSON object
/// property on), `flatten`ed nested-record fields splice their own fields into THIS SAME properties
/// map rather than nesting, mirroring what `flatten` means at parse/print altitude. `required` lists
/// every non-`optional`, non-empty-key field.
pub fn record_spec_json_schema(spec: &RecordSpec) -> semio_framework_pack_json::Value {
    use semio_framework_pack_json::{Object, Value};
    let mut properties = Object::new();
    let mut required: Vec<Value> = Vec::new();
    collect_record_spec_properties(spec, &mut properties, &mut required);
    let mut map = Object::new();
    map.insert("type", Value::from("object"));
    map.insert("properties", Value::Object(properties));
    if let Some(keyword) = &spec.keyword {
        map.insert("x-semio-keyword", Value::from(keyword.clone()));
    }
    if !required.is_empty() {
        map.insert("required", Value::Array(required));
    }
    Value::Object(map)
}

fn collect_record_spec_properties(spec: &RecordSpec, properties: &mut semio_framework_pack_json::Object, required: &mut Vec<semio_framework_pack_json::Value>) {
    use semio_framework_pack_json::Value;
    for field in &spec.fields {
        if field.flatten {
            if let Shape::Record(spec_fn) = &field.shape {
                collect_record_spec_properties(&(spec_fn.ordinary)(), properties, required);
                continue;
            }
        }
        if field.key.is_empty() {
            continue;
        }
        properties.insert(field.key.clone(), shape_json_schema(&field.shape));
        if !field.optional {
            required.push(Value::from(field.key.clone()));
        }
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️json-schema/🦀️.rs"]
mod json_schema_tests;
//#endregion 🔖️JsonSchema

//#region 🔖️Value
use semio_framework_value::{DslValue,NativeEncodeControl,Number};

/// 🕸️ One endpoint (and optional edge) of a wire-literal.
#[derive(Clone, Debug, PartialEq, Default)]
pub struct WireNode {
    pub id: String,
    pub kind: Option<String>,
    pub port: Option<String>,
}

/// 🏷️ Optional id/kind label on a wire edge (`-[e1:Connection]->` / fused `-e1:Connection>`).
#[derive(Clone, Debug, PartialEq, Default)]
pub struct WireEdgeLabel {
    pub id: Option<String>,
    pub kind: Option<String>,
}

impl WireEdgeLabel {
    pub fn is_empty(&self) -> bool {
        self.id.is_none() && self.kind.is_none()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct WireValue {
    pub from: WireNode,
    /// `Some((directed, to))` if this line describes an edge, `None` for a bare node declaration.
    pub edge: Option<(bool, WireNode)>,
    pub edge_label: WireEdgeLabel,
    pub properties: DslValue,
}

/// 🌳️ The parsed representation of one field's value — what a typed binder converts
/// to/from a concrete Rust value. Doubles as this v1 engine's "Cst": simplified (semantic, not a
/// full lossless syntax tree) but sufficient for round-tripping, diagnostics, and highlighting;
/// a real green/red tree can replace it later behind the same `parse`/`Writer` API.
#[derive(Clone, Debug, PartialEq)]
pub enum FieldValue {
    Bool(bool),
    Int(i64),
    UInt(u64),
    Float(f64),
    Text(String),
    Bytes64(Vec<u8>),
    Enum(u32),
    Tuple(Vec<FieldValue>),
    List(Vec<FieldValue>),
    Record(RecordValue),
    Block(Box<FieldValue>),
    Statements(Vec<(String, RecordValue)>),
    Map(Vec<(String, FieldValue)>),
    Value(DslValue),
    Wire(WireValue),
    Expr(ExprValue),
    Absent,
}

#[path = "🧩️record/🗂️fields/🦀️.rs"]
mod record_fields;
pub use record_fields::RecordFields;

#[derive(Clone, Debug, PartialEq, Default)]
pub struct RecordValue {
    pub fields: RecordFields,
}

impl RecordValue {
    // 🚫️async: E1 pure map lookup, consumed by `Iterator::any`/`Option::and_then` sync closures
    // (`print_record_fields`) and by dozens of `assert_eq!(value.get(id), ...)` test call sites
    // that compare its result directly (never ``ed) — see R9
    pub fn get(&self, id: u16) -> Option<&FieldValue> {
        self.fields.get(&id)
    }
}

/// 🌳️ Alias naming the parse product per the engine's design vocabulary.
pub type Cst = RecordValue;
//#endregion 🔖️Value

//#region 🔖️Expr
/// ➕️ Arithmetic operators `Shape::Expr` supports — standard left-associative precedence
/// (`*`/`/` bind tighter than `+`/`-`), plus a call form for named functions (`min(a, b)`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExprOp {
    Add,
    Sub,
    Mul,
    Div,
}

impl ExprOp {
    // 🚫️async: E1 pure, consumed by `print_expr_prec` (forced sync — its `Call` arm feeds a
    // `Display`-formatted `Iterator::map(...).join(...)` closure chain, R9) and inlined directly
    // into `format!` args elsewhere in this impl — see R9
    fn precedence(self) -> u8 {
        match self {
            ExprOp::Add | ExprOp::Sub => 1,
            ExprOp::Mul | ExprOp::Div => 2,
        }
    }

    // 🚫️async: E1 pure, same R9 chain as `precedence` above
    fn symbol(self) -> &'static str {
        match self {
            ExprOp::Add => "+",
            ExprOp::Sub => "-",
            ExprOp::Mul => "*",
            ExprOp::Div => "/",
        }
    }
}

/// 🧮️ The parsed body of a `Shape::Expr` field — a small formula AST, e.g.
/// `1.35*G + 1.5*Q` parses to `Binary(Add, Binary(Mul, Num(1.35), Var("G")), Binary(Mul,
/// Num(1.5), Var("Q")))`. Deliberately NOT a general-purpose scripting language (no assignment, no
/// control flow, no boolean logic) — it's a formula literal, one notch above a bare number.
#[derive(Clone, Debug, PartialEq)]
pub enum ExprValue {
    Num(f64),
    /// A snake_case reference to a sibling field/symbol, resolved by the consuming technology
    /// (e.g. a norm calc-sheet's own `given`/prior `clause` definitions) — this engine only
    /// parses/prints the name, it never evaluates it.
    Var(String),
    Neg(Box<ExprValue>),
    Binary(ExprOp, Box<ExprValue>, Box<ExprValue>),
    Call(String, Vec<ExprValue>),
}
//#endregion 🔖️Expr

//#region 🔖️Cursor
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SourceMode {
    Document,
    Inline,
}

struct Cursor {
    tokens: Vec<SpannedToken>,
    pos: usize,
    limits: Limits,
}

impl Cursor {
    // 🚫️async: E1 pure in-memory cursor consumed by `Iterator::position` sync closures (`:1345`, `:1354` via `at_keyword`) — see R9.
    // The whole impl block is one call graph (`peek`/`peek_at`/`span`/`advance`/`expect`/`at_attr_key`/`at_keyword` all call each
    // other with no suspension point ever possible), so the language barrier on `at_keyword` propagates to every method here.
    //
    // `SourceMode` no longer participates in parsing (its only consumer, `RawLines`, is gone —
    // `Shape::Text` now accepts `Ident|Text` identically regardless of Document/Inline); it stays
    // a `ParseOptions`/`parse` public-API distinction only, still meaningful to callers choosing
    // between `dsl::__rt::parse_document_record`/`parse_inline_record`.
    fn new(tokens: Vec<SpannedToken>, limits: Limits) -> Self {
        let tokens: Vec<SpannedToken> = tokens.into_iter().filter(|t| !t.kind.is_trivia()).collect();
        Self { tokens, pos: 0, limits }
    }

    fn peek(&self) -> &SpannedToken {
        &self.tokens[self.pos.min(self.tokens.len() - 1)]
    }

    fn peek_at(&self, offset: usize) -> &SpannedToken {
        let idx = (self.pos + offset).min(self.tokens.len() - 1);
        &self.tokens[idx]
    }

    fn span(&self) -> TextSpan {
        self.peek().span
    }

    fn advance(&mut self) -> SpannedToken {
        let token = self.tokens[self.pos.min(self.tokens.len() - 1)].clone();
        if self.pos < self.tokens.len() - 1 {
            self.pos += 1;
        }
        token
    }

    fn expect(&mut self, kind: TokenKind) -> Result<SpannedToken, TextError> {
        if self.peek().kind == kind {
            Ok(self.advance())
        } else {
            Err(TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,format!("expected {:?}, found {:?} '{}'", kind, self.peek().kind, self.peek().text.as_str()),self.span()))
        }
    }

    /// 🔎️ Whether the next token is an `Ident` that is followed by `=` — the LL(2)
    /// lookahead that makes the grammar newline-insensitive: a bare ident followed by `=` is
    /// always a `key=value` attribute, never the start of a new statement.
    fn at_attr_key(&self) -> Option<String> {
        if self.peek().kind == TokenKind::Ident && self.peek_at(1).kind == TokenKind::Equals {
            Some(self.peek().text.as_str().to_string())
        } else {
            None
        }
    }

    fn at_keyword(&self, keyword: &str) -> bool {
        self.peek().kind == TokenKind::Ident && self.peek().text.as_str().as_ref() == keyword
    }

    fn dynamic_attr_key(&self) -> Result<Option<String>, TextError> {
        if self.peek().kind == TokenKind::Text && self.peek_at(1).kind == TokenKind::Equals {
            semio_framework_dsl::unescape_text(&self.peek().text.as_str(), false).map(Some).map_err(|message| TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,message,self.span()))
        } else {
            Ok(self.at_attr_key())
        }
    }
}
//#endregion 🔖️Cursor

//#region 🔖️Parser
pub struct ParseOptions {
    pub limits: Limits,
    pub mode: SourceMode,
}

impl Default for ParseOptions {
    fn default() -> Self {
        Self { limits: Limits::default(), mode: SourceMode::Document }
    }
}

/// ✂️ The structural seam between lexing and parsing: everything downstream of a token
/// vector is grammar-only and needs no raw source bytes (the parser is token-only — no shape
/// still consumes verbatim source text the way the deleted `RawLines` shape once did). Exists so
/// a caller that already has tokens (e.g. an incremental relexer) can skip `parse`'s own lex pass.
pub fn parse_tokens(tokens: Vec<SpannedToken>, spec: &RecordSpec, opts: &ParseOptions) -> Result<Cst, TextError> {
    let mut cursor = Cursor::new(tokens, opts.limits);
    parse_record_body(&mut cursor, spec, 0)
}

pub fn parse(text: &str, spec: &RecordSpec, opts: &ParseOptions) -> Result<Cst, TextError> {
    let tokens = lex(text, &opts.limits, false)?;
    parse_tokens(tokens, spec, opts)
}

/// 🛑️ Parses one terminal record and rejects every token outside its schema-owned body.
pub fn parse_exact(text: &str, spec: &RecordSpec, opts: &ParseOptions) -> Result<Cst, TextError> {
    let mut cursor = Cursor::new(lex(text, &opts.limits, false)?, opts.limits);
    let record = parse_record_body(&mut cursor, spec, 0)?;
    cursor.expect(TokenKind::Eof)?;
    Ok(record)
}

fn ident_like_text(token: &SpannedToken) -> String {
    token.text.as_str().to_string()
}

fn parse_scalar(cursor: &mut Cursor, shape: &Shape) -> Result<FieldValue, TextError> {
    match shape {
        Shape::Bool => {
            let token = cursor.expect(TokenKind::Ident)?;
            match token.text.as_str().as_ref() {
                "true" => Ok(FieldValue::Bool(true)),
                "false" => Ok(FieldValue::Bool(false)),
                other => Err(TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,format!("expected 'true' or 'false', found '{other}'"),token.span)),
            }
        }
        Shape::Int => {
            let token = cursor.expect(TokenKind::Int)?;
            let value: i64 = token.text.as_str().parse().map_err(|_| TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,format!("invalid integer '{}'", token.text.as_str()),token.span))?;
            Ok(FieldValue::Int(value))
        }
        Shape::UInt => {
            let token = cursor.expect(TokenKind::Int)?;
            let value: u64 = token.text.as_str().parse().map_err(|_| TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,format!("invalid unsigned integer '{}'", token.text.as_str()),token.span))?;
            Ok(FieldValue::UInt(value))
        }
        Shape::Float => {
            let is_float_token = matches!(cursor.peek().kind, TokenKind::Float | TokenKind::Int) || (cursor.peek().kind == TokenKind::Ident && matches!(cursor.peek().text.as_str().as_ref(), "nan" | "inf" | "-inf"));
            if !is_float_token {
                return Err(TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,format!("expected a float, found {:?} '{}'", cursor.peek().kind, cursor.peek().text.as_str()),cursor.span()));
            }
            let token = cursor.advance();
            let value = parse_f64(&token.text.as_str()).map_err(|e| TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,e,token.span))?;
            Ok(FieldValue::Float(value))
        }
        Shape::Text => parse_scalar_text(cursor),
        Shape::Bytes64 => {
            let token = cursor.expect(TokenKind::Text)?;
            let bytes = base64_decode(&token.text.as_str()).map_err(|e| TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,e,token.span))?;
            Ok(FieldValue::Bytes64(bytes))
        }
        Shape::Enum(variants) => {
            let token = cursor.expect(TokenKind::Ident)?;
            let text = token.text.as_str();
            variants.iter().find(|(tag, _)| tag == text.as_ref()).map(|(_, ordinal)| FieldValue::Enum(*ordinal)).ok_or_else(|| TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,format!("unknown enum tag '{text}'"),token.span))
        }
        Shape::Quantity(declared) | Shape::Angle(declared) => parse_quantity(cursor, declared),
        Shape::Ref(_) => parse_scalar_text(cursor),
        Shape::Embed(declared_lang) => parse_embed(cursor, declared_lang),
        Shape::EmbedFrom(_) => Err(TextError::new(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"EmbedFrom field must be parsed in record context",cursor.span())),
        Shape::Count => {
            if cursor.peek().kind != TokenKind::Ident {
                return Err(TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,format!("expected a count literal like 'x24', found {:?} '{}'", cursor.peek().kind, cursor.peek().text.as_str()),cursor.span()));
            }
            let token = cursor.advance();
            let text = token.text.as_str();
            let digits = text.strip_prefix('x').ok_or_else(|| TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,format!("expected a count literal like 'x24', found '{text}'"),token.span))?;
            let value: u64 = digits.parse().map_err(|_| TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,format!("invalid count literal 'x{digits}'"),token.span))?;
            Ok(FieldValue::UInt(value))
        }
        other => Err(TextError::new(semio_framework_value::ValueRefusalKind::UnsupportedOwner,format!("shape {other:?} is not a scalar"),cursor.span())),
    }
}

/// 🧮️ Precedence-climbing entry point for `Shape::Expr`'s body (called with the caller's
/// outer `(`/`)` already consumed). `min_prec` is the lowest operator precedence this call is
/// willing to keep consuming at — the standard technique for turning a flat token stream into a
/// precedence-correct tree without a separate tokenize-then-shunting-yard pass.
fn parse_expr(cursor: &mut Cursor, min_prec: u8) -> Result<ExprValue, TextError> {
    let lhs = parse_expr_unary(cursor)?;
    parse_expr_continue(cursor, min_prec, lhs)
}

/// 🧮️ The loop body of `parse_expr`, factored out so the glued-negative-number case below
/// can re-enter it with an ALREADY-PARSED left operand instead of calling `parse_expr_unary` again
/// (which would re-consume nothing, since the token was already consumed to build that operand).
fn parse_expr_continue(cursor: &mut Cursor, min_prec: u8, mut lhs: ExprValue) -> Result<ExprValue, TextError> {
    loop {
        // The shared lexer glues a leading `-` onto an immediately-following digit as ONE negative
        // number token (`y=-2`'s existing, load-bearing behavior — see dsl_core's lexer) — so
        // `10-2` lexes as `Int(10), Int(-2)`, not `Int(10), Minus, Int(2)`. Detect that shape here
        // and reinterpret it as `Sub` with a positive right operand, rather than requiring authors
        // to always space out `-` (canonical PRINT output always does; hand-written input may not).
        let glued_negative = matches!(cursor.peek().kind, TokenKind::Float | TokenKind::Int) && cursor.peek().text.as_str().starts_with('-');
        let (op, prec) = if glued_negative {
            (ExprOp::Sub, ExprOp::Sub.precedence())
        } else {
            match cursor.peek().kind {
                TokenKind::Plus => (ExprOp::Add, ExprOp::Add.precedence()),
                TokenKind::Minus => (ExprOp::Sub, ExprOp::Sub.precedence()),
                TokenKind::Star => (ExprOp::Mul, ExprOp::Mul.precedence()),
                TokenKind::Slash => (ExprOp::Div, ExprOp::Div.precedence()),
                _ => break,
            }
        };
        if prec < min_prec {
            break;
        }
        let rhs = if glued_negative {
            let token = cursor.advance();
            let value = parse_f64(&token.text.as_str()).map_err(|e| TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,e,token.span))?;
            parse_expr_continue(cursor, prec + 1, ExprValue::Num(-value))?
        } else {
            cursor.advance();
            parse_expr(cursor, prec + 1)?
        };
        lhs = ExprValue::Binary(op, Box::new(lhs), Box::new(rhs));
    }
    Ok(lhs)
}

fn parse_expr_unary(cursor: &mut Cursor) -> Result<ExprValue, TextError> {
    if cursor.peek().kind == TokenKind::Minus {
        cursor.advance();
        return Ok(ExprValue::Neg(Box::new(parse_expr_unary(cursor)?)));
    }
    parse_expr_primary(cursor)
}

fn parse_expr_primary(cursor: &mut Cursor) -> Result<ExprValue, TextError> {
    match cursor.peek().kind {
        TokenKind::Float | TokenKind::Int => {
            let token = cursor.advance();
            let value = parse_f64(&token.text.as_str()).map_err(|e| TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,e,token.span))?;
            Ok(ExprValue::Num(value))
        }
        TokenKind::Ident | TokenKind::Text => {
            let token = cursor.advance();
            let name = if token.kind==TokenKind::Text{semio_framework_dsl::unescape_text(&token.text.as_str(),false).map_err(|error|TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,error,token.span))?}else{ident_like_text(&token)};
            if cursor.peek().kind == TokenKind::LParen {
                cursor.advance();
                let mut args = Vec::new();
                if cursor.peek().kind != TokenKind::RParen {
                    loop {
                        args.push(parse_expr(cursor, 0)?);
                        if cursor.peek().kind == TokenKind::Comma {
                            cursor.advance();
                            continue;
                        }
                        break;
                    }
                }
                cursor.expect(TokenKind::RParen)?;
                Ok(ExprValue::Call(name, args))
            } else {
                Ok(ExprValue::Var(name))
            }
        }
        TokenKind::LParen => {
            cursor.advance();
            let inner = parse_expr(cursor, 0)?;
            cursor.expect(TokenKind::RParen)?;
            Ok(inner)
        }
        other => Err(TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,format!("expected a number, variable, or '(', found {other:?} '{}'", cursor.peek().text.as_str()),cursor.span())),
    }
}

/// 🧮️ Standalone entry point for parsing a bare expression body (no surrounding `(`/`)`,
/// unlike `Shape::Expr`'s own field-value grammar) — what `pack_value`'s decoder calls to turn the
/// canonical string it stored back into an `ExprValue`, since decode has no `Cursor` of its own.
pub fn parse_expr_text(text: &str) -> Result<ExprValue, TextError> {
    let tokens = lex(text, &Limits::default(), false)?;
    let mut cursor = Cursor::new(tokens, Limits::default());
    let value = parse_expr(&mut cursor, 0)?;
    cursor.expect(TokenKind::Eof)?;
    Ok(value)
}

/// 🎨️ Canonical `Shape::Expr` printer. Parenthesizes the minimum necessary to guarantee
/// `parse_expr(print_expr(e)) == e` for EVERY tree shape (not just canonically-left-nested ones):
/// a `Binary` right operand is parenthesized whenever its own precedence isn't STRICTLY higher
/// than the parent's (so even a commutative `a+(b+c)` keeps its parens — losing them would
/// reparse as the structurally different `(a+b)+c`), and a left operand only when strictly lower
/// (left-associativity already makes equal precedence safe there).
// 🚫️async: E1 pure AST pretty-printer — the `Call` arm's `Iterator::map(...).join(...)` recurses
// through `print_expr_prec` inside a sync closure, and both are inlined directly into `format!`
// args elsewhere in this fn, which requires `Display`, not `Future` — see R9
pub fn print_expr(expr: &ExprValue) -> String {
    print_expr_prec(expr, 0)
}

fn print_expr_prec(expr: &ExprValue, min_prec: u8) -> String {
    let (body, own_prec) = match expr {
        ExprValue::Num(v) => (format_f64(*v), 255),
        ExprValue::Var(name) => (expression_name(name), 255),
        ExprValue::Call(name, args) => {
            let joined = args.iter().map(|a| print_expr_prec(a, 0)).collect::<Vec<_>>().join(", ");
            (format!("{}({joined})",expression_name(name)), 255)
        }
        // min_prec=4 is higher than every binary op (max 2) and Neg's own rank (3), so a nested
        // Binary OR another Neg always gets parenthesized — the latter specifically avoids ever
        // printing adjacent `--`, which would relex as `DashArrow`, not two `Minus` tokens.
        ExprValue::Neg(inner) => (format!("-{}", print_expr_prec(inner, 4)), 3),
        ExprValue::Binary(op, l, r) => {
            let prec = op.precedence();
            let l_text = print_expr_prec(l, prec);
            let r_text = print_expr_prec(r, prec + 1);
            (format!("{l_text} {} {r_text}", op.symbol()), prec)
        }
    };
    if own_prec < min_prec {
        format!("({body})")
    } else {
        body
    }
}

fn expression_name(name:&str)->String{if semio_framework_dsl::is_bare_ident(name){name.to_string()}else{format!("\"{}\"",semio_framework_dsl::escape_text(name))}}

/// 📛️ `Shape::Text`'s own body, factored out so `Shape::Ref` (identical grammar, distinct
/// type only) can share it without a redundant match arm duplicating both branches.
fn parse_scalar_text(cursor: &mut Cursor) -> Result<FieldValue, TextError> {
    match cursor.peek().kind {
        TokenKind::Text => {
            let token = cursor.advance();
            let text = semio_framework_dsl::unescape_text(&token.text.as_str(), false).map_err(|e| TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,e,token.span))?;
            Ok(FieldValue::Text(text))
        }
        TokenKind::Ident => {
            let token = cursor.advance();
            Ok(FieldValue::Text(ident_like_text(&token)))
        }
        other => Err(TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,format!("expected Text, found {other:?} '{}'", cursor.peek().text.as_str()),cursor.span())),
    }
}

/// 🗣️ `Shape::Embed`'s parse: a `Fence` token (Document mode — see `dsl_core`'s lexer for
/// the `lang\u{0}content` encoding) with an empty or matching lang tag, OR anything
/// `parse_scalar_text` already accepts (Inline mode's escaped-quoted fallback) — both converge on
/// the same `FieldValue::Text`, which is what makes Document/Inline renders agree.
fn parse_embed(cursor: &mut Cursor, declared_lang: &str) -> Result<FieldValue, TextError> {
    if cursor.peek().kind == TokenKind::Fence {
        let token = cursor.advance();
        let raw = token.text.as_str();
        let (lang, content) = raw.split_once('\u{0}').ok_or_else(|| TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,"malformed fence token (missing separator)",token.span))?;
        if !lang.is_empty() && !declared_lang.is_empty() && lang != declared_lang {
            return Err(TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,format!("fence declares lang '{lang}', field expects '{declared_lang}'"),token.span));
        }
        return Ok(FieldValue::Text(content.to_string()));
    }
    parse_scalar_text(cursor)
}

fn sibling_text_field<'a>(record: &'a RecordValue, spec: &RecordSpec, lang_key: &str) -> Option<&'a str> {
    let field = spec.fields.iter().find(|f| f.key == lang_key)?;
    match record.get(field.id)? {
        FieldValue::Text(text) => Some(text.as_str()),
        _ => None,
    }
}

fn parse_field_shape(cursor: &mut Cursor, field: &FieldSpec, spec: &RecordSpec, record: &RecordValue, depth: usize) -> Result<FieldValue, TextError> {
    if let Shape::EmbedFrom(lang_key) = &field.shape {
        let declared = sibling_text_field(record, spec, lang_key).unwrap_or("");
        return parse_embed(cursor, declared);
    }
    parse_shape(cursor, &field.shape, depth)
}

/// 📐️ Shared parse for `Shape::Quantity`/`Shape::Angle`: a number, optionally followed by a
/// GLUED (no whitespace between — the lexer already ends a numeric token exactly where the next
/// `Ident` token begins for input like `210GPa`) unit-symbol ident. No suffix means the number is
/// already expressed in `declared`'s unit; a suffix converts, erroring if the dimensions differ.
fn parse_quantity(cursor: &mut Cursor, declared: &'static semio_framework_dsl::UnitSpec) -> Result<FieldValue, TextError> {
    let is_number_token = matches!(cursor.peek().kind, TokenKind::Float | TokenKind::Int) || (cursor.peek().kind == TokenKind::Ident && matches!(cursor.peek().text.as_str().as_ref(), "nan" | "inf" | "-inf"));
    if !is_number_token {
        return Err(TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,format!("expected a quantity, found {:?} '{}'", cursor.peek().kind, cursor.peek().text.as_str()),cursor.span()));
    }
    let number_token = cursor.advance();
    let value = parse_f64(&number_token.text.as_str()).map_err(|e| TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,e,number_token.span))?;
    let suffix = cursor.peek();
    if suffix.kind == TokenKind::Ident && suffix.byte_range.0 == number_token.byte_range.1 {
        let suffix_token = cursor.advance();
        let symbol = suffix_token.text.as_str().to_string();
        let suffix_unit = semio_framework_dsl::unit_by_symbol(&symbol).ok_or_else(|| TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,format!("unknown unit '{symbol}'"),suffix_token.span))?;
        let converted = if value.is_nan(){if suffix_unit.dimension!=declared.dimension{return Err(TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,format!("unit '{symbol}' is not compatible with expected unit '{}'",declared.symbol),suffix_token.span));}value}else{semio_framework_dsl::convert(value,suffix_unit,declared).ok_or_else(||TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,format!("unit '{symbol}' is not compatible with expected unit '{}'",declared.symbol),suffix_token.span))?};
        Ok(FieldValue::Float(converted))
    } else {
        Ok(FieldValue::Float(value))
    }
}

/// 🔢️ Reads a complete numeric token for coordinate, direction, dimension and range components.
fn parse_plain_number(cursor: &mut Cursor) -> Result<f64, TextError> {
    if !(matches!(cursor.peek().kind,TokenKind::Float|TokenKind::Int)||(cursor.peek().kind==TokenKind::Ident&&matches!(cursor.peek().text.as_str().as_ref(),"nan"|"inf"|"-inf"))) {
        return Err(TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,format!("expected a number, found {:?} '{}'", cursor.peek().kind, cursor.peek().text.as_str()),cursor.span()));
    }
    let token = cursor.advance();
    parse_f64(&token.text.as_str()).map_err(|e| TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,e,token.span))
}

/// 📍️ Shared body for `Shape::Coord`/`Shape::Dir`: a fixed-arity comma-separated run of
/// plain numbers, with no delimiter of its own (the caller already consumed the `@`/`^` sigil).
fn parse_fixed_number_tuple(cursor: &mut Cursor, arity: usize, what: &str) -> Result<FieldValue, TextError> {
    let mut items = Vec::with_capacity(arity);
    loop {
        items.push(FieldValue::Float(parse_plain_number(cursor)?));
        if items.len() == arity {
            break;
        }
        cursor.expect(TokenKind::Comma)?;
    }
    if cursor.peek().kind == TokenKind::Comma {
        return Err(TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,format!("{what} literal expects exactly {arity} components"),cursor.span()));
    }
    Ok(FieldValue::Tuple(items))
}

/// 📏️ `Shape::Dim`'s `WxHxD` grammar: the FIRST number is an ordinary `Float|Int` token;
/// every number after it is glued (no whitespace, no comma) onto an `x`-prefixed ident — the
/// lexer has no notion of a bare `x` operator (digits/`.` are ident-continue, so `x0.12x0.24`
/// lexes as ONE `Ident` token), so this splits that single glued token on `x` itself rather than
/// looping token-by-token the way `parse_fixed_number_tuple` does.
fn parse_dim(cursor: &mut Cursor, dims: usize) -> Result<FieldValue, TextError> {
    let first_token = cursor.peek().clone();
    let first = parse_plain_number(cursor)?;
    let mut items = vec![FieldValue::Float(first)];
    if dims > 1 {
        let suffix = cursor.peek();
        if suffix.kind != TokenKind::Ident || suffix.byte_range.0 != first_token.byte_range.1 {
            return Err(TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,format!("dimension literal expects {dims} components glued with 'x' (e.g. '2x3'), found only one"),cursor.span()));
        }
        let suffix_token = cursor.advance();
        let suffix_text = suffix_token.text.as_str();
        let parts: Vec<&str> = suffix_text.split('x').collect();
        // `"x0.12x0.24".split('x')` yields `["", "0.12", "0.24"]` — the leading empty piece is the
        // text before the first `x`, which is always empty since the suffix itself starts with it.
        if parts.first() != Some(&"") || parts.len() != dims {
            return Err(TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,format!("dimension literal expects {dims} components glued with 'x', found '{}{}'", format_f64(first), suffix_text),suffix_token.span));
        }
        for part in &parts[1..] {
            let value = parse_f64(part).map_err(|_| TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,format!("invalid dimension component '{part}'"),suffix_token.span))?;
            items.push(FieldValue::Float(value));
        }
    }
    Ok(FieldValue::Tuple(items))
}

fn parse_shape(cursor: &mut Cursor, shape: &Shape, depth: usize) -> Result<FieldValue, TextError> {
    cursor.limits.check_depth(depth, cursor.span())?;
    match shape {
        Shape::Bool | Shape::Int | Shape::UInt | Shape::Float | Shape::Text | Shape::Bytes64 | Shape::Enum(_) | Shape::Quantity(_) | Shape::Angle(_) | Shape::Ref(_) | Shape::Count | Shape::Embed(_) => parse_scalar(cursor, shape),
        Shape::EmbedFrom(_) => Err(TextError::new(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"EmbedFrom field must be parsed in record context",cursor.span())),
        Shape::Coord(dims) => {
            cursor.expect(TokenKind::At)?;
            parse_fixed_number_tuple(cursor, *dims as usize, "coordinate")
        }
        Shape::Dir => {
            cursor.expect(TokenKind::Caret)?;
            parse_fixed_number_tuple(cursor, 3, "direction")
        }
        Shape::Dim(dims) => parse_dim(cursor, *dims as usize),
        Shape::Range => {
            cursor.expect(TokenKind::LParen)?;
            let lo = parse_plain_number(cursor)?;
            cursor.expect(TokenKind::DotDot)?;
            let hi = parse_plain_number(cursor)?;
            let mut items = vec![FieldValue::Float(lo), FieldValue::Float(hi)];
            if cursor.peek().kind == TokenKind::Comma {
                cursor.advance();
                items.push(FieldValue::Float(parse_plain_number(cursor)?));
            }
            cursor.expect(TokenKind::RParen)?;
            Ok(FieldValue::Tuple(items))
        }
        Shape::Expr => {
            cursor.expect(TokenKind::LParen)?;
            let value = parse_expr(cursor, 0)?;
            cursor.expect(TokenKind::RParen)?;
            Ok(FieldValue::Expr(value))
        }
        Shape::Tuple(elem, len) => {
            let mut items = Vec::new();
            loop {
                items.push(parse_shape(cursor, elem, depth + 1)?);
                if cursor.peek().kind == TokenKind::Comma {
                    cursor.advance();
                    continue;
                }
                break;
            }
            if let Some(expected_len) = len {
                if items.len() != *expected_len {
                    return Err(TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,format!("tuple expects {} elements, found {}", expected_len, items.len()),cursor.span()));
                }
            }
            Ok(FieldValue::Tuple(items))
        }
        Shape::List(elem) => {
            cursor.expect(TokenKind::LBracket)?;
            let mut items = Vec::new();
            while cursor.peek().kind != TokenKind::RBracket {
                let pos_before = cursor.pos;
                let value = if let Shape::Record(make) = elem.as_ref() {
                    cursor.expect(TokenKind::LBrace)?;
                    let record = parse_record_body(cursor, &(make.ordinary)(), depth + 1)?;
                    cursor.expect(TokenKind::RBrace)?;
                    FieldValue::Record(record)
                } else {
                    parse_shape(cursor, elem, depth + 1)?
                };
                items.push(value);
                if cursor.pos == pos_before {
                    return Err(TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,format!("list element made no progress at {:?} '{}' — likely an unrecognized field key", cursor.peek().kind, cursor.peek().text.as_str()),cursor.span()));
                }
                cursor.limits.check_nodes(items.len(), cursor.span())?;
            }
            cursor.expect(TokenKind::RBracket)?;
            Ok(FieldValue::List(items))
        }
        Shape::Record(spec_fn) => Ok(FieldValue::Record(parse_record_body(cursor, &(spec_fn.ordinary)(), depth + 1)?)),
        Shape::Block(inner) => {
            cursor.expect(TokenKind::LBrace)?;
            let value = parse_shape(cursor, inner, depth + 1)?;
            cursor.expect(TokenKind::RBrace)?;
            Ok(FieldValue::Block(Box::new(value)))
        }
        Shape::Statements(variants) => {
            let mut out = Vec::new();
            while let Some(keyword) = current_keyword(cursor) {
                let Some((_, spec_fn)) = variants.iter().find(|(kw, _)| kw == &keyword) else { break };
                // `parse_record_body` consumes the keyword itself (see its own check below); we
                // only peek here to decide whether this token starts a known variant at all.
                let record = parse_record_body(cursor, &(spec_fn.ordinary)(), depth + 1)?;
                out.push((keyword, record));
                cursor.limits.check_nodes(out.len(), cursor.span())?;
                if cursor.peek().kind == TokenKind::RBrace || cursor.peek().kind == TokenKind::Eof {
                    break;
                }
            }
            Ok(FieldValue::Statements(out))
        }
        Shape::Map(inner) => {
            cursor.expect(TokenKind::LBrace)?;
            let mut entries = Vec::new();
            while let Some(key) = cursor.dynamic_attr_key()? {
                cursor.advance();
                cursor.expect(TokenKind::Equals)?;
                let record = matches!(inner.as_ref(), Shape::Record(_));
                if record { cursor.expect(TokenKind::LBrace)?; }
                let value = parse_shape(cursor, inner, depth + 1)?;
                if record { cursor.expect(TokenKind::RBrace)?; }
                entries.push((key, value));
            }
            cursor.expect(TokenKind::RBrace)?;
            Ok(FieldValue::Map(entries))
        }
        Shape::Value => Ok(FieldValue::Value(parse_dsl_value(cursor, depth + 1)?)),
        Shape::Table(spec_fn) => {
            validate_table_columns(&(spec_fn.ordinary)())?;
            parse_table_list(cursor, *spec_fn, depth)
        }
        Shape::Wire => Ok(FieldValue::Wire(parse_wire(cursor)?)),
    }
}

fn current_keyword(cursor: &Cursor) -> Option<String> {
    if cursor.peek().kind == TokenKind::Ident && cursor.at_attr_key().is_none() {
        Some(cursor.peek().text.as_str().to_string())
    } else {
        None
    }
}

fn parse_dsl_value(cursor: &mut Cursor, depth: usize) -> Result<DslValue, TextError> {
    cursor.limits.check_depth(depth, cursor.span())?;
    match cursor.peek().kind {
        TokenKind::LBrace => {
            cursor.advance();
            let mut entries = Vec::new();
            while let Some(key) = cursor.dynamic_attr_key()? {
                cursor.advance();
                cursor.expect(TokenKind::Equals)?;
                entries.push((key, parse_dsl_value(cursor, depth + 1)?));
            }
            cursor.expect(TokenKind::RBrace)?;
            Ok(DslValue::Object(entries))
        }
        TokenKind::LBracket => {
            cursor.advance();
            let mut items = Vec::new();
            while cursor.peek().kind != TokenKind::RBracket {
                items.push(parse_dsl_value(cursor, depth + 1)?);
            }
            cursor.expect(TokenKind::RBracket)?;
            Ok(DslValue::Array(items))
        }
        TokenKind::Text => {
            let token = cursor.advance();
            let text = semio_framework_dsl::unescape_text(&token.text.as_str(), false).map_err(|e| TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,e,token.span))?;
            Ok(DslValue::String(text))
        }
        TokenKind::Int => {
            let token = cursor.advance();
            let text = token.text.as_str();
            if let Ok(v) = text.parse::<u64>() {
                Ok(DslValue::Number(Number::UInt(v)))
            } else if let Ok(v) = text.parse::<i64>() {
                Ok(DslValue::Number(Number::Int(v)))
            } else {
                let value = parse_f64(&text).map_err(|e| TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,e,token.span))?;
                Ok(DslValue::Number(Number::Float(value)))
            }
        }
        TokenKind::Float => {
            let token = cursor.advance();
            let value = parse_f64(&token.text.as_str()).map_err(|e| TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,e,token.span))?;
            Ok(DslValue::Number(Number::Float(value)))
        }
        TokenKind::Ident => {
            let token = cursor.advance();
            match token.text.as_str().as_ref() {
                "bytes64"=>{cursor.expect(TokenKind::LParen)?;let token=cursor.expect(TokenKind::Text)?;let bytes=base64_decode(&token.text.as_str()).map_err(|error|TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,error,token.span))?;cursor.expect(TokenKind::RParen)?;Ok(DslValue::Bytes(bytes))},
                "null" => Ok(DslValue::Null),
                "true" => Ok(DslValue::Bool(true)),
                "false" => Ok(DslValue::Bool(false)),
                "nan"|"inf"=>Ok(DslValue::Number(Number::Float(parse_f64(&token.text.as_str()).map_err(|error|TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,error,token.span))?))),
                other => Err(TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,format!("expected a value literal, found ident '{other}'"),token.span)),
            }
        }
        other => Err(TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,format!("expected a value literal, found {other:?}"),cursor.span())),
    }
}

/// 🕸️ Parses one wire literal. `<-` is accepted sugar only: normalized here by swapping
/// the two endpoints, so the stored `WireValue` (and everything reprinted from it) only ever
/// holds `->`/`--` or fused labeled arrows — `b<-a` and `a->b` parse to the identical value.
fn parse_wire(cursor: &mut Cursor) -> Result<WireValue, TextError> {
    fn parse_wire_label(cursor: &mut Cursor) -> Result<WireEdgeLabel, TextError> {
        cursor.expect(TokenKind::LBracket)?;
        let id = if cursor.peek().kind == TokenKind::Ident { Some(ident_like_text(&cursor.advance())) } else { None };
        let kind = if cursor.peek().kind == TokenKind::Colon {
            cursor.advance();
            Some(ident_like_text(&cursor.expect(TokenKind::Ident)?))
        } else {
            None
        };
        let label = WireEdgeLabel { id, kind };
        if label.is_empty() {
            return Err(TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,"edge label `[...]` must name an id and/or a `:kind`",cursor.span()));
        }
        cursor.expect(TokenKind::RBracket)?;
        Ok(label)
    }

    let mut from = parse_wire_node(cursor)?;
    let mut edge_label = WireEdgeLabel::default();
    let edge = match cursor.peek().kind {
        TokenKind::Arrow => {
            cursor.advance();
            let to = parse_wire_node(cursor)?;
            Some((true, to))
        }
        TokenKind::DashArrow => {
            cursor.advance();
            let to = parse_wire_node(cursor)?;
            Some((false, to))
        }
        TokenKind::BackArrow => {
            cursor.advance();
            if cursor.peek().kind == TokenKind::LBracket {
                edge_label = parse_wire_label(cursor)?;
                cursor.expect(TokenKind::Minus)?;
            }
            let to = parse_wire_node(cursor)?;
            let swapped_to = std::mem::replace(&mut from, to);
            Some((true, swapped_to))
        }
        TokenKind::Minus if cursor.peek_at(1).kind == TokenKind::LBracket => {
            cursor.advance();
            edge_label = parse_wire_label(cursor)?;
            let directed = match cursor.peek().kind {
                TokenKind::Arrow => {
                    cursor.advance();
                    true
                }
                TokenKind::DashArrow => {
                    cursor.advance();
                    false
                }
                other => {
                    return Err(TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,format!("expected `->` or `--` to close a labeled edge, found {other:?}"),cursor.span()));
                }
            };
            let to = parse_wire_node(cursor)?;
            Some((directed, to))
        }
        TokenKind::EdgeArrow => {
            let token = cursor.advance();
            let (directed, label) = dsl_notation::decode_fused_edge_arrow(&token.text.as_str())?;
            edge_label = WireEdgeLabel { id: label.id, kind: label.kind };
            let to = parse_wire_node(cursor)?;
            Some((directed, to))
        }
        _ => None,
    };
    let properties = if cursor.peek().kind == TokenKind::LBrace { parse_dsl_value(cursor, 0)? } else { DslValue::Object(Vec::new()) };
    Ok(WireValue { from, edge, edge_label, properties })
}

/// 🔌️ Small public entry point other crates (the graph wire module, trinity) can call
/// directly to lex + parse one standalone wire literal, without needing a `RecordSpec` around it.
pub fn parse_wire_text(text: &str) -> Result<WireValue, TextError> {
    let limits = Limits::default();
    let tokens = lex(text, &limits, false)?;
    let mut cursor = Cursor::new(tokens, limits);
    parse_wire(&mut cursor)
}

fn parse_wire_node(cursor: &mut Cursor) -> Result<WireNode, TextError> {
    let FieldValue::Text(id) = parse_scalar_text(cursor)? else { unreachable!() };
    let kind = if cursor.peek().kind == TokenKind::Colon {
        cursor.advance();
        Some(match parse_scalar_text(cursor)? { FieldValue::Text(value) => value, _ => unreachable!() })
    } else {
        None
    };
    let port = if cursor.peek().kind == TokenKind::At {
        cursor.advance();
        Some(match parse_scalar_text(cursor)? { FieldValue::Text(value) => value, _ => unreachable!() })
    } else {
        None
    };
    Ok(WireNode { id, kind, port })
}

/// 🧾️ Parses one record: its own leading keyword if `spec.keyword` declares one (the
/// `Statements` dispatcher only peeks to choose a variant — consuming it is always this
/// function's job, so a spec is self-contained regardless of whether it's reached via `parse`
/// directly, `Shape::Record`, or a `Statements` variant), positional fields in declaration order,
/// then order-independent `key=value` attributes (LL(2): an `Ident` followed by `=` is always a
/// key), until a token that is neither a known key nor an unfilled positional slot — which ends
/// the record (it belongs to whatever comes next: a new statement, a closing brace, or EOF).
fn parse_record_body(cursor: &mut Cursor, spec: &RecordSpec, depth: usize) -> Result<RecordValue, TextError> {
    cursor.limits.check_depth(depth, cursor.span())?;
    if spec.layout == RecordLayout::Call {
        return parse_call_record(cursor, spec, depth);
    }
    if let Some(keyword) = &spec.keyword {
        if cursor.at_keyword(keyword) {
            cursor.advance();
        } else {
            return Err(TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,format!("expected keyword '{keyword}', found {:?} '{}'", cursor.peek().kind, cursor.peek().text.as_str()),cursor.span()));
        }
    }
    parse_record_fields(cursor, spec, depth)
}

/// 📛️ Parses a `RecordLayout::Call` record: `<name> = <keyword>(args)`. The parenthesized
/// argument list is parsed by the exact same [`parse_record_fields`] loop every other layout uses
/// — it naturally stops at the first token that matches neither a positional slot nor a known
/// key (here, always `)`), so no special "bounded sub-cursor" is needed to keep it from reading
/// past the closing paren.
fn parse_call_record(cursor: &mut Cursor, spec: &RecordSpec, depth: usize) -> Result<RecordValue, TextError> {
    let name_field = spec.fields.iter().find(|f| f.is_call_name).ok_or_else(|| TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,"RecordLayout::Call requires exactly one field marked call_name()",cursor.span()))?;
    let name = ident_like_text(&cursor.expect(TokenKind::Ident)?);
    cursor.expect(TokenKind::Equals)?;
    let keyword = spec.keyword.as_deref().ok_or_else(|| TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,"RecordLayout::Call requires RecordSpec.keyword (the call target)",cursor.span()))?;
    if !cursor.at_keyword(keyword) {
        return Err(TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,format!("expected call target '{keyword}', found {:?} '{}'", cursor.peek().kind, cursor.peek().text.as_str()),cursor.span()));
    }
    cursor.advance();
    cursor.expect(TokenKind::LParen)?;
    let mut record = parse_record_fields(cursor, spec, depth)?;
    cursor.expect(TokenKind::RParen)?;
    record.fields.insert(name_field.id, FieldValue::Text(name));
    Ok(record)
}

/// 🧾️ Parses a record's fields: positional fields in declaration order, then order-
/// independent `key=value` attributes (LL(2): an `Ident` followed by `=` is always a key), until a
/// token that is neither a known key nor an unfilled positional slot — which ends the record (it
/// belongs to whatever comes next: a new statement, a closing brace/paren, or EOF). Excludes any
/// field marked `call_name()` from both candidate sets: that field is consumed by the caller
/// (`RecordLayout::Call`'s `<name> =` prefix) before this function ever runs, for a Call-layout
/// spec, and no field is ever marked `call_name()` under any other layout.
fn parse_record_fields(cursor: &mut Cursor, spec: &RecordSpec, depth: usize) -> Result<RecordValue, TextError> {
    let mut record = RecordValue::default();
    let positional: Vec<&FieldSpec> = {
        let mut p: Vec<&FieldSpec> = spec.fields.iter().filter(|f| f.position.is_some() && !f.is_call_name).collect();
        p.sort_by_key(|f| f.position.unwrap());
        p
    };
    for field in &positional {
        if field.optional {
            // An explicit `_` placeholder always means "absent, but consume the slot" — this is
            // what keeps LATER positionals aligned when an earlier optional one is skipped (see
            // `print_record`'s matching print-side logic). Only positional contexts ever see a
            // `Placeholder` token; keyed optionals are simply omitted instead.
            if cursor.peek().kind == TokenKind::Placeholder {
                cursor.advance();
                record.fields.insert(field.id, FieldValue::Absent);
                continue;
            }
            if !can_start_positional(cursor, &field.shape) {
                record.fields.insert(field.id, FieldValue::Absent);
                continue;
            }
        }
        let value = parse_field_shape(cursor, field, spec, &record, depth + 1)?;
        record.fields.insert(field.id, value);
    }

    // `Statements` fields have no field-level key at all — they're recognized purely by matching
    // one of their own variants' keywords, so at most one such field may appear per record.
    // `Block` fields are also excluded from the `key=value` loop below: their own key acts as a
    // bare leading keyword (`children { ... }`, no `=`) — `Table` fields (bare `key [...] {...}`
    // SoA form) are handled the same way, via their own lookahead branch below.
    let statements_field = spec.fields.iter().find(|f| f.position.is_none() && matches!(f.shape, Shape::Statements(_)));
    let mut keyed: Vec<&FieldSpec> = spec.fields.iter().filter(|f| f.position.is_none() && !f.key.is_empty() && !f.is_call_name && !matches!(f.shape, Shape::Statements(_))).collect();

    loop {
        if let Some(key) = cursor.dynamic_attr_key()? {
            let Some(index) = keyed.iter().position(|f| !matches!(f.shape, Shape::Block(_)) && f.key == key) else { break };
            let field = keyed.remove(index);
            cursor.advance();
            cursor.expect(TokenKind::Equals)?;
            let value = parse_field_shape(cursor, field, spec, &record, depth + 1)?;
            record.fields.insert(field.id, value);
            continue;
        }
        // `Table`'s bare SoA form: the keyword directly followed by `[` (no `=`) — distinct from
        // the AoS-verbose `key=[...]` form already handled by the `at_attr_key` branch above.
        if let Some(index) = keyed.iter().position(|f| matches!(f.shape, Shape::Table(_)) && cursor.at_keyword(&f.key) && cursor.peek_at(1).kind == TokenKind::LBracket) {
            let field = keyed.remove(index);
            let Shape::Table(spec_fn) = &field.shape else { unreachable!() };
            let spec_fn = *spec_fn;
            cursor.advance();
            let value = parse_table_soa(cursor, spec_fn, depth + 1)?;
            record.fields.insert(field.id, value);
            continue;
        }
        let Some(index) = keyed.iter().position(|f| matches!(f.shape, Shape::Block(_)) && cursor.at_keyword(&f.key)) else { break };
        let field = keyed.remove(index);
        cursor.advance();
        let value = parse_field_shape(cursor, field, spec, &record, depth + 1)?;
        record.fields.insert(field.id, value);
    }
    for field in keyed {
        if !record.fields.contains_key(&field.id){record.fields.insert(field.id,FieldValue::Absent);}
    }

    if let Some(field) = statements_field {
        let value = parse_field_shape(cursor, field, spec, &record, depth + 1)?;
        record.fields.insert(field.id, value);
    }

    Ok(record)
}

// 🚫️async: E1 pure lookahead over the now-sync `Cursor`, called inline in a plain `if` with no
// await anywhere at its one call site — see R9
fn can_start_positional(cursor: &Cursor, shape: &Shape) -> bool {
    match shape {
        Shape::Bool | Shape::Enum(_) => cursor.peek().kind == TokenKind::Ident,
        Shape::Int | Shape::UInt => cursor.peek().kind == TokenKind::Int,
        Shape::Float | Shape::Quantity(_) | Shape::Angle(_) | Shape::Dim(_) => matches!(cursor.peek().kind, TokenKind::Float | TokenKind::Int),
        Shape::Ref(_) => matches!(cursor.peek().kind, TokenKind::Text | TokenKind::Placeholder),
        Shape::Count => cursor.peek().kind == TokenKind::Ident,
        Shape::Coord(_) => cursor.peek().kind == TokenKind::At,
        Shape::Dir => cursor.peek().kind == TokenKind::Caret,
        Shape::Range | Shape::Expr => cursor.peek().kind == TokenKind::LParen,
        Shape::Embed(_) | Shape::EmbedFrom(_) => matches!(cursor.peek().kind, TokenKind::Fence | TokenKind::Text | TokenKind::Placeholder),
        // Only `Text|Placeholder` — NOT bare `Ident` — may start an optional positional `Text`
        // field: an unquoted bare-ident value here would be indistinguishable from the next
        // statement's leading keyword, so this deliberately narrower check (versus `Shape::Text`
        // parsing `Ident|Text` everywhere else) resolves that ambiguity.
        Shape::Text => matches!(cursor.peek().kind, TokenKind::Text | TokenKind::Placeholder),
        Shape::Bytes64 => cursor.peek().kind == TokenKind::Text,
        Shape::List(_) => cursor.peek().kind == TokenKind::LBracket,
        Shape::Block(_) | Shape::Map(_) => cursor.peek().kind == TokenKind::LBrace,
        _ => true,
    }
}

//#region 🔖️Table
/// 🚧️ Which shapes have a fixed/bounded token extent and may therefore be a `Table`
/// column: an unbounded `Tuple` (`len: None`, comma-separated until... forever) and `Statements`
/// (repeats until a non-matching keyword) both need an external delimiter to know where they end
/// — fine inside `[ ]`/`{ }` brackets, fatal inside a table row where the ONLY thing marking a
/// row boundary is "we've now read exactly `columns.len()` values".
// 🚫️async: E1 pure, inlined directly into `format!` args alongside `shape_type_name` (Display,
// not Future) — see R9
fn shape_is_self_delimiting(shape: &Shape) -> bool {
    !matches!(shape, Shape::Statements(_) | Shape::Tuple(_, None))
}

/// 🚧️ Spec-build-time validation for a `Table`'s element `RecordSpec` — called wherever a
/// `Shape::Table(spec_fn)` is first evaluated (both parse paths, and printing), since `spec_fn` is
/// a lazy pointer rather than an eagerly-built value there is no earlier moment to check it at.
fn validate_table_columns(spec: &RecordSpec) -> Result<(), TextError> {
    for field in &spec.fields {
        if !shape_is_self_delimiting(&field.shape) {
            return Err(TextError::new(semio_framework_value::ValueRefusalKind::UnsupportedOwner,format!("table column '{}' has a non-self-delimiting shape ({}) and cannot be a table column", field.key, shape_type_name(&field.shape)),TextSpan::at(1, 1)));
        }
    }
    Ok(())
}

/// 🏷️ UPPERCASE schema type tag for a `Shape` — what a `Table` header prints per column
/// (`id:TEXT`), per the unified syntax law (`UPPERCASE` for engine shapes, `PascalCase` reserved
/// for technology-declared domain kinds).
// 🚫️async: E1 pure, inlined directly into `format!` args at both call sites (Display, not Future) — see R9
pub fn shape_type_name(shape: &Shape) -> &'static str {
    match shape {
        Shape::Bool => "BOOL",
        Shape::Int => "INT",
        Shape::UInt => "UINT",
        Shape::Float => "NUM",
        Shape::Text => "TEXT",
        Shape::Bytes64 => "BYTES",
        Shape::Enum(_) => "ENUM",
        Shape::Tuple(_, _) => "TUPLE",
        Shape::List(_) => "LIST",
        Shape::Record(_) => "REC",
        Shape::Block(_) => "BLOCK",
        Shape::Statements(_) => "STMT",
        Shape::Map(_) => "MAP",
        Shape::Value => "VAL",
        Shape::Table(_) => "TABLE",
        Shape::Wire => "WIRE",
        Shape::Quantity(_) => "QTY",
        Shape::Angle(_) => "ANG",
        Shape::Ref(_) => "REF",
        Shape::Coord(_) => "CRD",
        Shape::Dir => "DIR",
        Shape::Dim(_) => "DIM",
        Shape::Range => "RNG",
        Shape::Count => "CNT",
        Shape::Expr => "EXPR",
        Shape::Embed(_) | Shape::EmbedFrom(_) => "EMBED",
    }
}

/// 📊️ Parses the bare SoA form of a `Table` field: `[col:TYPE ...] { v11 v12 ...  v21 v22
/// ... }`, cursor positioned right after the field's own keyword has already been consumed. The
/// header names columns (in the order values then appear per row); a `:TYPE` suffix is accepted
/// but not required to resolve a column (it's a human/printer-facing tag, not load-bearing for
/// parsing — the column's real shape always comes from the element `RecordSpec`), which is what
/// lets a hand-written header omit types the engine can already infer. Rows have NO separator —
/// reading exactly `columns.len()` values per row is what makes a row self-delimiting, which is
/// also why every column shape must itself be self-delimiting (`validate_table_columns`).
fn parse_table_soa(cursor: &mut Cursor, spec_fn: RecordSpecProducer, depth: usize) -> Result<FieldValue, TextError> {
    let element_spec = (spec_fn.ordinary)();
    validate_table_columns(&element_spec)?;
    cursor.expect(TokenKind::LBracket)?;
    let mut columns: Vec<&FieldSpec> = Vec::new();
    while cursor.peek().kind != TokenKind::RBracket {
        let key_token = cursor.expect(TokenKind::Ident)?;
        let key = key_token.text.as_str().to_string();
        if cursor.peek().kind == TokenKind::Colon {
            cursor.advance();
            cursor.expect(TokenKind::Ident)?; // type tag — documentation only, not re-validated here
        }
        let field_spec = element_spec.fields.iter().find(|f| f.key == key).ok_or_else(|| TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,format!("unknown table column '{key}'"),key_token.span))?;
        columns.push(field_spec);
    }
    cursor.expect(TokenKind::RBracket)?;
    cursor.expect(TokenKind::LBrace)?;
    let mut rows = Vec::new();
    while cursor.peek().kind != TokenKind::RBrace {
        let mut record = RecordValue::default();
        for field_spec in &columns {
            if cursor.peek().kind == TokenKind::Placeholder {
                cursor.advance();
                record.fields.insert(field_spec.id, FieldValue::Absent);
                continue;
            }
            let value = parse_table_cell(cursor, &field_spec.shape, depth + 1)?;
            record.fields.insert(field_spec.id, value);
        }
        for field_spec in &element_spec.fields {
            if !record.fields.contains_key(&field_spec.id){record.fields.insert(field_spec.id,FieldValue::Absent);}
        }
        rows.push(FieldValue::Record(record));
        cursor.limits.check_nodes(rows.len(), cursor.span())?;
    }
    cursor.expect(TokenKind::RBrace)?;
    Ok(FieldValue::List(rows))
}

/// 🧱️ Reads one table cell's value. Every table-safe shape is bounded by its own bracket or
/// a fixed token count (`validate_table_columns`/`shape_is_self_delimiting`) — EXCEPT a bare
/// `Shape::Record` column, which prints as a flat run of `key=value` tokens with no bracket of its
/// own (a table row has no `field=` prefix to give it one, unlike a Record-shaped field elsewhere).
/// Two adjacent columns of the SAME record type (or any two types sharing a field name) are then
/// genuinely ambiguous: `parse_record_body`'s keyed loop for column N keeps matching `key=value`
/// tokens for as long as the key is one of ITS OWN not-yet-filled fields, so a column-N field left
/// absent (never printed) silently lets column N's parse run on and swallow column N+1's
/// same-named token instead of stopping at the column boundary. Braced here for exactly that
/// reason — every other shape already round-trips through the ordinary `parse_shape`.
fn parse_table_cell(cursor: &mut Cursor, shape: &Shape, depth: usize) -> Result<FieldValue, TextError> {
    if let Shape::Record(spec_fn) = shape {
        cursor.expect(TokenKind::LBrace)?;
        let record = parse_record_body(cursor, &(spec_fn.ordinary)(), depth + 1)?;
        cursor.expect(TokenKind::RBrace)?;
        return Ok(FieldValue::Record(record));
    }
    parse_shape(cursor, shape, depth)
}

/// 📋️ The AoS-list form for a `Table` value reached anywhere other than a record's own
/// leading keyword-prefixed field: `[ {row-fields} {row-fields} ... ]`. Each row is brace-wrapped
/// for the same reason a `Shape::Record` table COLUMN is (`parse_table_cell` above) — a table row
/// type is commonly declared with no keyword of its own (a header already gives every row its
/// column order, so SoA rows don't need one), so without a bracket of its own, one row's absent
/// field could let its parse run on into the next row's same-named token exactly like the
/// column-vs-column case. Bracing every row here removes that ambiguity regardless of whether the
/// row type happens to declare a keyword or not.
fn parse_table_list(cursor: &mut Cursor, spec_fn: RecordSpecProducer, depth: usize) -> Result<FieldValue, TextError> {
    cursor.expect(TokenKind::LBracket)?;
    let mut items = Vec::new();
    while cursor.peek().kind != TokenKind::RBracket {
        cursor.expect(TokenKind::LBrace)?;
        let record = parse_record_body(cursor, &(spec_fn.ordinary)(), depth + 1)?;
        cursor.expect(TokenKind::RBrace)?;
        items.push(FieldValue::Record(record));
        cursor.limits.check_nodes(items.len(), cursor.span())?;
    }
    cursor.expect(TokenKind::RBracket)?;
    Ok(FieldValue::List(items))
}

/// 📋️ Prints the braced AoS-list form `parse_table_list` reads back. Ordinary `[ ]` spacing
/// (a space just inside, per the general list rule — NOT the header's own tight-glued exception).
fn print_table_list(spec_fn: RecordSpecProducer, items: &[FieldValue], writer: &mut Writer) {
    writer.atom("[");
    for item in items {
        let FieldValue::Record(record) = item else { continue };
        writer.atom("{");
        writer.glue();
        print_record(record, &(spec_fn.ordinary)(), writer);
        writer.glue();
        writer.atom("}");
    }
    writer.atom("]");
}
//#endregion 🔖️Table

fn base64_decode(text:&str)->Result<Vec<u8>,String>{semio_framework_value::bytes::decode_base64(text)}
fn base64_encode(bytes:&[u8])->String{semio_framework_value::bytes::encode_base64(bytes)}
//#endregion 🔖️Parser

//#region 🔖️Writer
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JoinMode {
    Document,
    Inline,
}

/// ✍️ A chunk tree that renders in either join mode — the structural half of the newline
/// law. `atom` asserts its argument contains no raw `\n` (Document mode still separates atoms with
/// synthesized whitespace, never embeds one inside an atom), so `render(Inline)` joining every
/// chunk with a single space can never produce an embedded newline.
///
/// 📏️ Canonical spacing rules (both join modes, structurally guaranteed — never hand-tuned
/// per callsite): never a space adjacent to `=` (`key=[ a b ]`, not `key= [ a b ]` — the printer
/// achieves this by pushing a bare `key=` atom, calling [`Writer::glue`], then printing the
/// value); exactly one space between sibling atoms; exactly one space just inside `[ ]`/`{ }` when
/// rendered inline (`[ a b ]`, not `[a b]`) — EXCEPT a `Table` header's `[ ]`, which is glued
/// tight on both sides (`[id:TEXT x:NUM]`) since it's a fixed one-shot header, not a
/// space-joined element list; a space appears before a keyword-led block's `{` (`children {
/// ... }`) but never before a glued composite's `{` (`data={ ... }`).
#[derive(Default)]
pub struct Writer {
    chunks: Vec<Chunk>,
    indent: usize,
}

enum Chunk {
    Atom(String),
    OpenBlock,
    CloseBlock,
    NewRecord,
    /// 🧲️ One-shot marker: the very next `Atom`/`OpenBlock` chunk renders with NO
    /// preceding separator (space in Inline mode, space-or-newline-continuation in Document mode)
    /// — consumed by that one chunk, then normal spacing resumes. See [`Writer::glue`].
    Glue,
    /// 📜️ `Shape::Embed`'s payload — the one chunk kind whose Document and Inline renders
    /// genuinely differ in FORM (fenced block vs. escaped quoted string), not just spacing.
    Verbatim {
        lang: String,
        content: String,
    },
}

impl Writer {
    pub fn new() -> Self {
        Self { chunks: Vec::new(), indent: 0 }
    }

    pub fn atom(&mut self, s: impl AsRef<str>) {
        let s = s.as_ref();
        debug_assert!(!s.contains('\n'), "Writer::atom must not contain a raw newline: {s:?}");
        self.chunks.push(Chunk::Atom(s.to_string()));
    }

    pub fn key_value(&mut self, key: &str, value: impl AsRef<str>) {
        self.atom(format!("{key}={}", value.as_ref()));
    }

    pub fn open_block(&mut self) {
        self.chunks.push(Chunk::OpenBlock);
        self.indent += 1;
    }

    pub fn close_block(&mut self) {
        self.indent = self.indent.saturating_sub(1);
        self.chunks.push(Chunk::CloseBlock);
    }

    pub fn new_record(&mut self) {
        self.chunks.push(Chunk::NewRecord);
    }

    /// 🧲️ Fuses the next pushed chunk onto whatever precedes it, with no separator, in
    /// BOTH join modes — the mechanism behind every `key=value`/`key=[...]`/`key={...}` fusion in
    /// this printer. Replaces the old approach of mutating an already-pushed atom's string in
    /// place (which only worked for single-atom scalar values): `glue()` composes with arbitrarily
    /// structured values (nested blocks, lists, whole sub-records) since it's a rendering-time
    /// join, not a string-splice.
    pub fn glue(&mut self) {
        self.chunks.push(Chunk::Glue);
    }

    /// 📜️ Pushes a `Shape::Embed` payload — content MAY contain raw newlines (unlike
    /// [`Self::atom`], which forbids them), since Document mode renders it as a fence.
    pub fn verbatim(&mut self, lang: &str, content: &str) {
        self.chunks.push(Chunk::Verbatim { lang: lang.to_string(), content: content.to_string() });
    }

    pub fn render(&self, mode: JoinMode) -> String {
        match mode {
            JoinMode::Inline => {
                let mut parts: Vec<String> = Vec::new();
                let mut glued = false;
                let mut push = |piece: String, glued: &mut bool| {
                    if *glued {
                        if let Some(last) = parts.last_mut() {
                            last.push_str(&piece);
                        } else {
                            parts.push(piece);
                        }
                    } else {
                        parts.push(piece);
                    }
                    *glued = false;
                };
                for chunk in &self.chunks {
                    match chunk {
                        Chunk::Glue => glued = true,
                        Chunk::Atom(s) => push(s.clone(), &mut glued),
                        Chunk::OpenBlock => push("{".to_string(), &mut glued),
                        Chunk::CloseBlock => push("}".to_string(), &mut glued),
                        Chunk::NewRecord => {}
                        Chunk::Verbatim { content, .. } => push(format!("\"{}\"", semio_framework_dsl::escape_text(content)), &mut glued),
                    }
                }
                parts.join(" ")
            }
            JoinMode::Document => {
                let mut out = String::new();
                let mut indent = 0usize;
                let mut line_open = false;
                let mut glued = false;
                let push_indent = |out: &mut String, indent: usize| {
                    for _ in 0..indent {
                        out.push_str("  ");
                    }
                };
                for chunk in &self.chunks {
                    match chunk {
                        Chunk::Glue => glued = true,
                        Chunk::Atom(s) => {
                            if !line_open {
                                push_indent(&mut out, indent);
                                line_open = true;
                            } else if !glued {
                                out.push(' ');
                            }
                            out.push_str(s);
                            glued = false;
                        }
                        Chunk::OpenBlock => {
                            if glued {
                                out.push('{');
                            } else {
                                out.push_str(" {");
                            }
                            out.push('\n');
                            line_open = false;
                            indent += 1;
                            glued = false;
                        }
                        Chunk::CloseBlock => {
                            if line_open {
                                out.push('\n');
                                line_open = false;
                            }
                            indent = indent.saturating_sub(1);
                            push_indent(&mut out, indent);
                            out.push('}');
                            out.push('\n');
                        }
                        Chunk::NewRecord => {
                            if line_open {
                                out.push('\n');
                                line_open = false;
                            }
                        }
                        Chunk::Verbatim { lang, content } => {
                            if !line_open {
                                push_indent(&mut out, indent);
                            } else if !glued {
                                out.push(' ');
                            }
                            out.push_str("```");
                            out.push_str(lang);
                            out.push('\n');
                            out.push_str(content);
                            if !content.is_empty() {
                                out.push('\n');
                            }
                            out.push_str("```");
                            line_open = true;
                            glued = false;
                        }
                    }
                }
                if line_open {
                    out.push('\n');
                }
                out
            }
        }
    }
}

/// 🥇️ Field print order within one record — NOT declaration order: keyword, then
/// positionals (unchanged), then keyed fields grouped scalar-before-composite-before-table-
/// before-statements, ties broken by original declaration order (a stable sort over an
/// already-declaration-order slice achieves this for free). Metadata/scalars land before large
/// nested/tabular blocks, which is friendlier to lazy loading/streaming readers — parsing stays
/// completely order-independent, so this is a print-only change.
// 🚫️async: E1 pure, consumed by `Iterator::sort_by_key`'s sync closure (its `u8` result must be
// `Ord`, which `impl Future<Output = u8>` is not) — see R9
fn keyed_field_rank(shape: &Shape) -> u8 {
    match shape {
        Shape::Bool
        | Shape::Int
        | Shape::UInt
        | Shape::Float
        | Shape::Text
        | Shape::Bytes64
        | Shape::Enum(_)
        | Shape::Tuple(_, _)
        | Shape::Quantity(_)
        | Shape::Angle(_)
        | Shape::Ref(_)
        | Shape::Coord(_)
        | Shape::Dir
        | Shape::Dim(_)
        | Shape::Range
        | Shape::Count
        | Shape::Expr => 0,
        Shape::List(_) | Shape::Map(_) | Shape::Record(_) | Shape::Block(_) | Shape::Value | Shape::Wire => 1,
        Shape::Table(_) => 2,
        Shape::Statements(_) => 3,
        // Ranks LAST of all: a multi-line fence dwarfs everything else in a record, so it should
        // print after every scalar/composite/table field, not interleaved among them.
        Shape::Embed(_) | Shape::EmbedFrom(_) => 4,
    }
}

pub fn print_record(value: &RecordValue, spec: &RecordSpec, writer: &mut Writer) {
    if spec.layout == RecordLayout::Call {
        print_call_record(value, spec, writer);
        return;
    }
    if let Some(keyword) = &spec.keyword {
        writer.atom(keyword);
    }
    print_record_fields(value, spec, writer);
}

/// 📛️ Prints a `RecordLayout::Call` record: `<name> = <keyword>(args)`. The argument list
/// is built by [`print_record_fields`] — the exact same field-printing logic every other layout
/// uses — rendered to its own `JoinMode::Inline` string and glued onto the keyword inside parens,
/// so a positional/keyed field prints identically here as it would under `Inline` layout.
fn print_call_record(value: &RecordValue, spec: &RecordSpec, writer: &mut Writer) {
    let Some(name_field) = spec.fields.iter().find(|f| f.is_call_name) else {
        debug_assert!(false, "RecordLayout::Call requires exactly one field marked call_name()");
        return;
    };
    let name_text = match value.get(name_field.id) {
        Some(fv @ FieldValue::Text(_)) => scalar_to_text(fv),
        _ => String::new(),
    };
    writer.atom(name_text);
    writer.atom("=");
    if let Some(keyword) = &spec.keyword {
        writer.atom(keyword);
    }
    let mut args_writer = Writer::new();
    print_record_fields(value, spec, &mut args_writer);
    let args_text = args_writer.render(JoinMode::Inline);
    writer.glue();
    writer.atom(format!("({args_text})"));
}

/// 🖨️ Prints a record's fields: positional bare in declaration order, then order-
/// independent `key=value` attributes. Excludes any field marked `call_name()` — see
/// [`parse_record_fields`]'s matching doc comment for why.
fn print_record_fields(value: &RecordValue, spec: &RecordSpec, writer: &mut Writer) {
    let mut positional: Vec<&FieldSpec> = spec.fields.iter().filter(|f| f.position.is_some() && !f.is_call_name).collect();
    positional.sort_by_key(|f| f.position.unwrap());
    for (index, field) in positional.iter().enumerate() {
        match value.get(field.id) {
            Some(fv) if !matches!(fv, FieldValue::Absent) => print_shape(fv, &field.shape, writer),
            _ => {
                // An absent OPTIONAL positional prints as `_` only if some LATER positional in
                // this same record is actually present — that's what keeps slots aligned for the
                // reader (and reparse). A run of trailing absents needs no placeholder at all.
                let later_present = positional[index + 1..].iter().any(|f| matches!(value.get(f.id), Some(fv) if !matches!(fv, FieldValue::Absent)));
                if later_present {
                    writer.atom("_");
                }
            }
        }
    }

    let mut keyed: Vec<&FieldSpec> = spec.fields.iter().filter(|f| f.position.is_none() && !f.key.is_empty() && !f.is_call_name).collect();
    keyed.sort_by_key(|f| keyed_field_rank(&f.shape));
    for field in keyed {
        match value.get(field.id) {
            Some(FieldValue::Absent) | None => continue,
            Some(fv) => match &field.shape {
                Shape::EmbedFrom(lang_key) => {
                    writer.new_record();
                    writer.atom(format!("{}=", field.key));
                    writer.glue();
                    let lang = spec
                        .fields
                        .iter()
                        .find(|f| f.key == *lang_key)
                        .and_then(|f| value.get(f.id))
                        .and_then(|v| match v {
                            FieldValue::Text(t) => Some(t.as_str()),
                            _ => None,
                        })
                        .unwrap_or("plaintext");
                    if let FieldValue::Text(content) = fv {
                        writer.verbatim(lang, content);
                    }
                }
                // `Statements` items each carry their own leading keyword — no field-level key at
                // all is ever printed for this shape.
                Shape::Statements(_) => print_shape(fv, &field.shape, writer),
                // `Block`'s own key is a bare leading keyword, not a `key=value` attribute
                // (`children { ... }`, never `children={...}`).
                Shape::Block(_) => {
                    writer.new_record();
                    writer.atom(&field.key);
                    print_shape(fv, &field.shape, writer);
                }
                // `Table`'s own key is likewise a bare leading keyword, but — unlike `Block` —
                // it must always go through the dedicated SoA writer (`print_table`), never the
                // generic `print_shape` dispatch: that dispatch renders `Table` as the bracketed
                // AoS list (see its `Shape::Table` arm below) so a `Table` value reached any OTHER
                // way (nested inside a table row, a list, ...) stays self-delimiting. Only here,
                // directly after a record's own leading keyword, is the bare `[col:TYPE ...]
                // {rows}` form reachable on the parse side (`parse_record_body`'s dedicated
                // bare-SoA lookahead) — printing it via `print_shape` here would silently regress
                // to the AoS form for every top-level table field.
                Shape::Table(spec_fn) => {
                    writer.new_record();
                    writer.atom(&field.key);
                    if let FieldValue::List(items) = fv {
                        print_table(*spec_fn, items, writer);
                    }
                }
                _ => {
                    writer.atom(format!("{}=", field.key));
                    print_key_value(field, fv, writer);
                }
            },
        }
    }
}

/// 🧲️ `key=` was just pushed by the caller — glue the value onto it with no separator,
/// then print it normally (composed, not string-spliced, so this handles arbitrarily structured
/// values exactly like a bare `print_shape` call would).
fn print_key_value(field: &FieldSpec, value: &FieldValue, writer: &mut Writer) {
    writer.glue();
    match (&field.shape, value) {
        (Shape::Enum(variants), FieldValue::Enum(ordinal)) => {
            if let Some((tag, _)) = variants.iter().find(|(_, o)| o == ordinal) {
                writer.atom(tag);
            }
        }
        _ => print_shape(value, &field.shape, writer),
    }
}

fn scalar_to_text(value: &FieldValue) -> String {
    match value {
        FieldValue::Bool(b) => b.to_string(),
        FieldValue::Int(i) => i.to_string(),
        FieldValue::UInt(u) => u.to_string(),
        FieldValue::Float(f) => format_f64(*f),
        // Bare (unquoted) whenever the text lexes back as exactly this one ident — the printer's
        // half of the "strings bare-preferred" law; `is_bare_ident` also excludes reserved literal
        // idents (`_`/`true`/`false`/`null`/`nan`/`inf`) and number-shaped text, which always fall
        // through to the quoted+escaped form instead.
        FieldValue::Text(s) => {
            if semio_framework_dsl::is_bare_ident(s) {
                s.clone()
            } else {
                format!("\"{}\"", semio_framework_dsl::escape_text(s))
            }
        }
        FieldValue::Bytes64(bytes) => format!("\"{}\"", base64_encode(bytes)),
        FieldValue::Enum(_) => String::new(), // resolved by caller via variants table when needed
        _ => String::new(),
    }
}

/// 🔢️ Renders one `FieldValue::Tuple` element as bare text for the `Coord`/`Dir`/`Dim`/
/// `Range` printers above — every element of those tuples is always `FieldValue::Float` by
/// construction (their parsers only ever push `FieldValue::Float`), so this panics rather than
/// falling back on a malformed value, matching the rest of this module's "trust the parser built
/// this" convention for shapes whose `FieldValue` invariant is enforced entirely at parse time.
// 🚫️async: E1 pure, passed as a bare fn item into `Iterator::map` sync closures at every call site — see R9
fn number_tuple_component(value: &FieldValue) -> String {
    match value {
        FieldValue::Float(v) => format_f64(*v),
        other => panic!("Coord/Dir/Dim/Range tuple element must be Float, found {other:?}"),
    }
}

pub fn print_shape(value: &FieldValue, shape: &Shape, writer: &mut Writer) {
    match (value, shape) {
        // Must precede the generic scalar arm below: that arm's shape pattern is `_` and would
        // otherwise swallow every `FieldValue::Float` regardless of shape, printing a bare number
        // with no unit suffix even for a `Quantity`/`Angle` field.
        (FieldValue::Float(v), Shape::Quantity(unit) | Shape::Angle(unit)) => {
            writer.atom(format!("{}{}", format_f64(*v), unit.symbol));
        }
        (FieldValue::UInt(v), Shape::Count) => {
            writer.atom(format!("x{v}"));
        }
        (FieldValue::Tuple(items), Shape::Coord(_)) => {
            writer.atom(format!("@{}", items.iter().map(number_tuple_component).collect::<Vec<_>>().join(",")));
        }
        (FieldValue::Tuple(items), Shape::Dir) => {
            writer.atom(format!("^{}", items.iter().map(number_tuple_component).collect::<Vec<_>>().join(",")));
        }
        (FieldValue::Tuple(items), Shape::Dim(_)) => {
            writer.atom(items.iter().map(number_tuple_component).collect::<Vec<_>>().join("x"));
        }
        (FieldValue::Tuple(items), Shape::Range) => {
            let parts: Vec<String> = items.iter().map(number_tuple_component).collect();
            let body = match parts.as_slice() {
                [lo, hi] => format!("{lo}..{hi}"),
                [lo, hi, step] => format!("{lo}..{hi},{step}"),
                _ => parts.join(","),
            };
            writer.atom(format!("({body})"));
        }
        (FieldValue::Expr(expr), Shape::Expr) => {
            writer.atom(format!("({})", print_expr(expr)));
        }
        (FieldValue::Text(content), Shape::Embed(lang)) => {
            writer.verbatim(lang, content);
        }
        (FieldValue::Text(content), Shape::EmbedFrom(lang_key)) => {
            // Fallback when print_shape is called without sibling resolution — prefer plaintext fence.
            let _ = lang_key;
            writer.verbatim("plaintext", content);
        }
        (FieldValue::Bool(_) | FieldValue::Int(_) | FieldValue::UInt(_) | FieldValue::Float(_) | FieldValue::Text(_) | FieldValue::Bytes64(_), _) => {
            writer.atom(scalar_to_text(value));
        }
        (FieldValue::Enum(ordinal), Shape::Enum(variants)) => {
            if let Some((tag, _)) = variants.iter().find(|(_, o)| o == ordinal) {
                writer.atom(tag);
            }
        }
        (FieldValue::Tuple(items), Shape::Tuple(elem, _)) => {
            let mut rendered: Vec<String> = Vec::with_capacity(items.len());
            for item in items {
                let mut sub = Writer::new();
                print_shape(item, elem, &mut sub);
                rendered.push(sub.render(JoinMode::Inline));
            }
            writer.atom(rendered.join(","));
        }
        // A `Table` reached here (NOT via `print_record`'s own keyed-field dispatch, which calls
        // `print_table` directly) is nested inside another shape — a table row's own column, a
        // list element, ... — where the bare `key [col:TYPE ...] {rows}` form has no bracket of
        // its own to mark where it ends. Render the braced-row AoS list instead (see
        // `print_table_list`), matching what `parse_shape`'s own `Shape::Table` arm parses in
        // every one of these same contexts.
        (FieldValue::List(items), Shape::Table(spec_fn)) => print_table_list(*spec_fn, items, writer),
        (FieldValue::List(items), Shape::List(elem)) => {
            writer.atom("[");
            for item in items {
                if matches!(elem.as_ref(), Shape::Record(_)) {
                    writer.atom("{");
                    print_shape(item, elem, writer);
                    writer.atom("}");
                } else {
                    print_shape(item, elem, writer);
                }
            }
            writer.atom("]");
        }
        (FieldValue::Record(record), Shape::Record(spec_fn)) => {
            print_record(record, &(spec_fn.ordinary)(), writer);
        }
        (FieldValue::Block(inner_value), Shape::Block(inner_shape)) => {
            writer.open_block();
            print_shape(inner_value, inner_shape, writer);
            writer.close_block();
        }
        (FieldValue::Statements(items), Shape::Statements(variants)) => {
            for (keyword, record) in items {
                writer.new_record();
                if let Some((_, spec_fn)) = variants.iter().find(|(kw, _)| kw == keyword) {
                    print_record(record, &(spec_fn.ordinary)(), writer);
                }
            }
        }
        (FieldValue::Map(entries), Shape::Map(inner)) => {
            writer.open_block();
            let mut sorted = entries.iter().collect::<Vec<_>>();
            sorted.sort_by(|a, b| a.0.cmp(&b.0));
            for (key, value) in &sorted {
                writer.atom(dynamic_key_text(key));
                writer.glue();
                if matches!(inner.as_ref(), Shape::Record(_)) {
                    writer.atom("{");
                    print_shape(value, inner, writer);
                    writer.atom("}");
                } else { print_shape(value, inner, writer); }
            }
            writer.close_block();
        }
        (FieldValue::Value(dsl_value), Shape::Value) => print_dsl_value(dsl_value, writer),
        (FieldValue::Wire(wire), Shape::Wire) => print_wire(wire, writer),
        _ => {}
    }
}

/// 📊️ Always prints the compact SoA form — this (not the parser, which still accepts the
/// verbose AoS form too) is what makes `canonicalize` migrate old AoS documents to SoA
/// automatically. Header `[ ]` is glued tight on both sides (`[id:TEXT x:NUM]`); rows have no
/// separator, one row per line in Document mode purely for readability (`new_record` is a no-op
/// in Inline mode).
fn print_table(spec_fn: RecordSpecProducer, items: &[FieldValue], writer: &mut Writer) {
    let element_spec = (spec_fn.ordinary)();
    writer.atom("[");
    writer.glue();
    for field in &element_spec.fields {
        writer.atom(format!("{}:{}", field.key, shape_type_name(&field.shape)));
    }
    writer.glue();
    writer.atom("]");
    writer.open_block();
    for item in items {
        writer.new_record();
        let FieldValue::Record(record) = item else { continue };
        for field in &element_spec.fields {
            match record.get(field.id) {
                Some(fv) if !matches!(fv, FieldValue::Absent) => print_table_cell(fv, &field.shape, writer),
                _ => writer.atom("_"),
            }
        }
    }
    writer.close_block();
}

/// 🧱️ Prints one table cell's value. See `parse_table_cell` for why a bare `Shape::Record`
/// column is brace-wrapped here — `{ }` glued tight on both sides, the same technique the header's
/// own `[ ]` uses, so bracing never disturbs the "no space just inside" canonical spacing rule for
/// a one-shot wrapper — and every other shape is left to the ordinary `print_shape`, already
/// self-delimiting.
fn print_table_cell(value: &FieldValue, shape: &Shape, writer: &mut Writer) {
    if let (FieldValue::Record(record), Shape::Record(spec_fn)) = (value, shape) {
        writer.atom("{");
        writer.glue();
        print_record(record, &(spec_fn.ordinary)(), writer);
        writer.glue();
        writer.atom("}");
        return;
    }
    print_shape(value, shape, writer);
}

fn dynamic_key_text(key: &str) -> String {
    if semio_framework_dsl::is_bare_ident(key) { format!("{key}=") } else { format!("\"{}\"=", semio_framework_dsl::escape_text(key)) }
}

fn print_dsl_value(value: &DslValue, writer: &mut Writer) {
    match value {
        DslValue::Null => writer.atom("null"),
        DslValue::Bool(b) => writer.atom(b.to_string()),
        DslValue::Number(Number::UInt(n)) => writer.atom(n.to_string()),
        DslValue::Number(Number::Int(n)) => writer.atom(n.to_string()),
        DslValue::Number(Number::Float(n)) => {
            let mut value = format_f64(*n);
            if n.is_finite() && !value.contains(['.', 'e', 'E']) {
                value.push_str(".0");
            }
            writer.atom(value);
        }
        DslValue::String(s) => writer.atom(format!("\"{}\"", semio_framework_dsl::escape_text(s))),
        DslValue::Bytes(bytes)=>writer.atom(format!("bytes64(\"{}\")",base64_encode(bytes))),
        DslValue::Array(items) => {
            writer.atom("[");
            for item in items {
                print_dsl_value(item, writer);
            }
            writer.atom("]");
        }
        DslValue::Object(entries) => {
            let mut sorted = entries.iter().collect::<Vec<_>>();
            sorted.sort_by(|a, b| a.0.cmp(&b.0));
            writer.open_block();
            for (key, value) in &sorted {
                writer.atom(dynamic_key_text(key));
                writer.glue();
                print_dsl_value(value, writer);
            }
            writer.close_block();
        }
    }
}

fn print_wire(wire: &WireValue, writer: &mut Writer) {
    let map_node = |node: &WireNode| dsl_notation::EdgeNode { id: node.id.clone(), kind: node.kind.clone(), port: node.port.clone() };
    let edge = dsl_notation::EdgeValue {
        from: map_node(&wire.from),
        link: wire.edge.as_ref().map(|(directed, to)| dsl_notation::EdgeLink { directed: *directed, label: dsl_notation::EdgeLabel { id: wire.edge_label.id.clone(), kind: wire.edge_label.kind.clone() }, to: map_node(to) }),
    };
    writer.atom(dsl_notation::print_edge(&edge));
    if !matches!(&wire.properties, DslValue::Object(entries) if entries.is_empty()) {
        print_dsl_value(&wire.properties, writer);
    }
}

/// 🔁️ Prints `value` against `spec` in the given join mode — the top-level entry point
/// `dsl_derive`-generated code calls from `ArtifactDsl::print_dsl`/`OpText::print_op`.
pub fn print(value: &RecordValue, spec: &RecordSpec, mode: JoinMode) -> String {
    let mut writer = Writer::new();
    print_record(value, spec, &mut writer);
    writer.render(mode)
}
//#endregion 🔖️Writer

//#region 🔖️Canonicalize
/// ♻️ `canonicalize(canonicalize(x)) == canonicalize(x)`: reprints whatever `parse`
/// produces from `text`, which is the fixpoint every technology's `print_dsl` output must already
/// be at (the round-trip law), so this doubles as the idempotence check.
pub fn canonicalize(text: &str, spec: &RecordSpec, opts: &ParseOptions) -> Result<String, TextError> {
    let value = parse(text, spec, opts)?;
    Ok(print(&value, spec, JoinMode::Document))
}
//#endregion 🔖️Canonicalize

//#region 🔖️Language
/// 🎨️ Generic editor surface over any `RecordSpec` — the generalization of
/// `math::graph::dsl`'s hand-rolled `LanguageService`.
pub struct LanguageService<'g> {
    pub spec: &'g RecordSpec,
}

pub struct CompletionItem {
    pub label: String,
    pub detail: Option<String>,
}

impl<'g> LanguageService<'g> {
    pub fn new(spec: &'g RecordSpec) -> Self {
        Self { spec }
    }

    fn keywords(&self) -> Vec<String> {
        let mut out = Vec::new();
        collect_keywords(self.spec, &mut out, &mut HashSet::new(), &mut HashSet::new());
        out
    }

    pub fn semantic_tokens(&self, text: &str) -> Vec<(TokenClass, TextSpan)> {
        let limits = Limits::default();
        let tokens = lex(text, &limits, true).unwrap_or_default();
        let keywords = self.keywords();
        let keyword_refs: Vec<&str> = keywords.iter().map(String::as_str).collect();
        semio_framework_dsl::token_classes(&tokens, &keyword_refs)
    }

    pub fn diagnostics(&self, text: &str) -> Vec<TextError> {
        match parse(text, self.spec, &ParseOptions::default()) {
            Ok(_) => Vec::new(),
            Err(e) => vec![e],
        }
    }

    /// 💡️ Completions at `offset`: every key not yet used in the record enclosing the
    /// cursor, plus every keyword reachable from the root. A simple, always-available baseline —
    /// full context-sensitive narrowing is a natural follow-up once `Cst` gains node addressing.
    pub fn completions(&self, _text: &str, _offset: usize) -> Vec<CompletionItem> {
        let mut items: Vec<CompletionItem> = self.spec.fields.iter().filter(|f| !f.key.is_empty()).map(|f| CompletionItem { label: f.key.clone(), detail: Some(format!("{:?}", f.shape)) }).collect();
        for keyword in self.keywords() {
            items.push(CompletionItem { label: keyword, detail: None });
        }
        items
    }
}

// 🚫️async: E1 pure tree walk, mutually recursive with `collect_shape_keywords` below through match
// arms whose tail expression must resolve to the same `()` type in every arm — see R9
fn collect_keywords(spec: &RecordSpec, out: &mut Vec<String>, seen: &mut HashSet<String>, seen_records: &mut HashSet<usize>) {
    if let Some(kw) = &spec.keyword {
        out.push(kw.clone());
    }
    for field in &spec.fields {
        collect_shape_keywords(&field.shape, out, seen, seen_records);
    }
}

/// 🔁️ `seen` guards against a genuinely self-referential `Statements` table (a recursive
/// block tree whose own variant list contains itself): each `(spec_fn.ordinary)()` call is only expanded the
/// first time its keyword is reached, so the keyword set — which is always finite, even when the
/// grammar's real nesting isn't — is collected exactly once instead of infinitely. `seen_records`
/// is the same guard for a self-referential `Shape::Record` (a `#[derive(DslRecord)]` struct field
/// whose type recurses back to itself, e.g. a dynamic-value type nesting a map of itself) — a bare
/// Record has no keyword to key on, so this tracks the `fn() -> RecordSpec` pointer's own address
/// instead (two calls to the same generated `__dsl_spec` always share one code address).
// 🚫️async: E1 pure, same mutual-recursion R9 case as `collect_keywords` above
fn collect_shape_keywords(shape: &Shape, out: &mut Vec<String>, seen: &mut HashSet<String>, seen_records: &mut HashSet<usize>) {
    match shape {
        Shape::Record(spec_fn) => {
            if seen_records.insert(spec_fn.ordinary as usize) {
                collect_keywords(&(spec_fn.ordinary)(), out, seen, seen_records);
            }
        }
        Shape::Block(inner) => collect_shape_keywords(inner, out, seen, seen_records),
        Shape::Statements(variants) => {
            for (kw, spec_fn) in variants {
                out.push(kw.clone());
                if seen.insert(kw.clone()) {
                    collect_keywords(&(spec_fn.ordinary)(), out, seen, seen_records);
                }
            }
        }
        Shape::List(inner) | Shape::Tuple(inner, _) | Shape::Map(inner) => collect_shape_keywords(inner, out, seen, seen_records),
        _ => {}
    }
}
//#endregion 🔖️Language

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests





#[cfg(test)]
#[path = "🧪️tests/⚠️refusal/🦀️.rs"]
mod refusal_tests;

#[cfg(test)]
#[path = "🧪️tests/🪆️refusal/🦀️.rs"]
mod field_refusal_tests;

#[cfg(test)]
#[path = "🧪️tests/🔢️number-refusal/🦀️.rs"]
mod number_refusal_tests;

```

### 🧰️framework/🔨️modules/🗣️dsl/🧬️schema/📦️packages/🦀️rust/Cargo.toml

SHA-256 `a5b711cf2d190abf50aec059369ca6fb5b11dcc42243f943f70717da7b39bbf4`; 1081 bytes.

```
[package]
workspace = "../../../../../.."
name = "semio-framework-dsl-record"
version = "0.1.0"
edition = "2021"
rust-version.workspace = true

[package.metadata.semio]
role = "framework"
id = "dsl-record"

[lints]
workspace = true

[lib]
name = "semio_framework_dsl_record"
path = "../../🦀️.rs"

[dependencies]
semio-framework-dsl = { path = "../../../📦️packages/🦀️rust" }
semio-framework-value = { path = "../../../../🌱️value/📦️packages/🦀️rust" }
semio-framework-diagnostic = { path = "../../../../⚠️diagnostic/📦️packages/🦀️rust" }
semio-framework-pack-json = { path = "../../../../🎒️pack/🔤️json/📦️packages/🦀️rust" }

[dev-dependencies]
semio-framework-async-macros = { path = "../../../../⏳️async/✨️macros/📦️packages/🦀️rust" }
semio-framework-dsl-record-derive = { path = "../../✨️derive/📦️packages/🦀️rust" }
serde_json = { version = "1.0", features = ["float_roundtrip"] }

[[test]]
name = "field_refusal_public"
path = "../../🧪️tests/🪆️refusal/🚪️public/🦀️.rs"

```

### 🧰️framework/🔨️modules/🗣️dsl/🧬️schema/🪆️binding/🦀️.rs

SHA-256 `15ef13fe87c6010457d2850c8c2189da493cfe1243ae748eebe829be94685152`; 31060 bytes.

```
//! 🪆️ Generic record field and variant construction under explicit owned controls.
use crate::*;
use semio_framework_dsl::{UnitSpec,unit_by_symbol};
//#region 🔖️Field
/// 🔗️ Bridges a concrete Rust field type to the engine's `Shape`/`FieldValue` — every
/// primitive implements it directly; `#[derive(DslRecord)]`/`#[derive(DslScalar)]` implement it
/// for technology-declared nested types, so composition (a record field whose type is another
/// derived record or enum) works transparently through the same trait.
pub trait DslField: Sized {
    // 🚫️async: E4 fn-pointer transitivity — `Shape::Record`/`Table`/`Statements` hold
    // `fn() -> RecordSpec`; every `shape()` implementation ultimately feeds one, directly or
    // through a derived `__dsl_spec` — see R9.
    fn shape() -> Shape;
    /// 🏭️ Constructs only the explicitly declared shape metadata under caller admission.
    fn shape_controlled<C:NativeSchemaControl>(control:&mut C)->Result<Shape,ValueError>{control.checkpoint()?;Err(ValueError::new(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"field owner has no controlled native schema implementation"))}
    fn to_value(&self) -> FieldValue;
    /// 🛫️ Projects explicitly owned fields under cumulative output admission and cancellation.
    fn to_value_controlled(&self,control:&mut NativeEncodeControl<'_>)->Result<FieldValue,ValueError>{control.checkpoint()?;Err(ValueError::new(ValueRefusalKind::UnsupportedOwner,"field owner has no controlled native projection implementation"))}
    /// 📑️ Projects a record without an intermediate boxed field carrier.
    fn to_record_controlled(&self,control:&mut NativeEncodeControl<'_>)->Result<RecordValue,ValueError>{control.checkpoint()?;Err(ValueError::new(ValueRefusalKind::UnsupportedOwner,"field owner has no controlled record projection implementation"))}
    fn from_value(value: &FieldValue) -> Result<Self, String>;
    /// 🧹️ Retires a completed field according to its owner after partial reconstruction fails.
    fn retire_decoded(self) { drop(self); }
    /// 🛬️ Constructs an owned field under the caller's cumulative allocation and work control.
    fn from_value_controlled(_value: &FieldValue, control: &mut NativeDecodeControl<'_>) -> Result<Self,ValueError> {
        control.checkpoint()?;
        Err(ValueError::new(ValueRefusalKind::UnsupportedOwner,"field owner has no controlled native construction implementation"))
    }
    /// 📑️ Binds a record view without cloning a temporary FieldValue carrier.
    fn from_record_controlled(_record:&RecordValue,control:&mut NativeDecodeControl<'_>)->Result<Self,ValueError>{
        control.checkpoint()?;
        Err(ValueError::new(ValueRefusalKind::UnsupportedOwner,"field owner has no controlled record construction implementation"))
    }
}

/// 📦️ Boxed ownership preserves the inner field's schema, value, and decoding errors.
impl<T: DslField> DslField for Box<T> {
    fn to_value_controlled(&self,control:&mut NativeEncodeControl<'_>)->Result<FieldValue,ValueError>{T::to_value_controlled(self.as_ref(),control)}
    fn to_record_controlled(&self,control:&mut NativeEncodeControl<'_>)->Result<RecordValue,ValueError>{T::to_record_controlled(self.as_ref(),control)}

    fn retire_decoded(self) { T::retire_decoded(*self); }
    fn shape() -> Shape {
        T::shape()
    }
    fn shape_controlled<C:NativeSchemaControl>(control:&mut C)->Result<Shape,ValueError>{T::shape_controlled(control)}
    fn to_value(&self) -> FieldValue {
        T::to_value(self.as_ref())
    }
    fn from_value(value: &FieldValue) -> Result<Self, String> {
        T::from_value(value).map(Box::new)
    }
    fn from_value_controlled(value: &FieldValue, control: &mut NativeDecodeControl<'_>) -> Result<Self,ValueError> {
        control.charge(std::mem::size_of::<T>())?;
        control.scoped_stage(|control|T::from_value_controlled(value, control)).map(Box::new)
    }
    fn from_record_controlled(record:&RecordValue,control:&mut NativeDecodeControl<'_>)->Result<Self,ValueError>{control.charge(std::mem::size_of::<T>())?;control.scoped_stage(|control|T::from_record_controlled(record,control)).map(Box::new)}
}

macro_rules! impl_dsl_field_int {
    ($ty:ty, $shape:expr, $variant:ident, $as_ty:ty) => {
        impl DslField for $ty {
            // 🚫️async: E4 — see `DslField::shape`'s tag above.
            fn shape() -> Shape {
                $shape
            }
            fn shape_controlled<C:NativeSchemaControl>(control:&mut C)->Result<Shape,ValueError>{control.checkpoint()?;Ok($shape)}
            fn to_value(&self) -> FieldValue {
                FieldValue::$variant(*self as $as_ty)
            }
            fn to_value_controlled(&self,control:&mut NativeEncodeControl<'_>)->Result<FieldValue,ValueError>{control.step()?;Ok(FieldValue::$variant(*self as $as_ty))}
            fn from_value(value: &FieldValue) -> Result<Self, String> {
                match value {
                    FieldValue::$variant(v) => <$ty>::try_from(*v).map_err(|_| format!("integer {v} out of range for {}", stringify!($ty))),
                    other => Err(format!("expected {}, found {other:?}", stringify!($variant))),
                }
            }
            fn from_value_controlled(value: &FieldValue, control: &mut NativeDecodeControl<'_>) -> Result<Self,ValueError> {
                control.step()?;
                match value { FieldValue::$variant(number)=><$ty>::try_from(*number).map_err(|_|ValueError::new(ValueRefusalKind::InvalidValue,format!("integer {number} out of range for {}",stringify!($ty)))),_=>Err(ValueError::new(ValueRefusalKind::InvalidValue,concat!("expected ",stringify!($variant)))) }
            }
        }
    };
}

impl_dsl_field_int!(i8, Shape::Int, Int, i64);
impl_dsl_field_int!(i16, Shape::Int, Int, i64);
impl_dsl_field_int!(i32, Shape::Int, Int, i64);
impl_dsl_field_int!(i64, Shape::Int, Int, i64);
impl_dsl_field_int!(isize, Shape::Int, Int, i64);
impl_dsl_field_int!(u8, Shape::UInt, UInt, u64);
impl_dsl_field_int!(u16, Shape::UInt, UInt, u64);
impl_dsl_field_int!(u32, Shape::UInt, UInt, u64);
impl_dsl_field_int!(u64, Shape::UInt, UInt, u64);
impl_dsl_field_int!(usize, Shape::UInt, UInt, u64);

impl DslField for bool {
    // 🚫️async: E4 — see `DslField::shape`'s tag above.
    fn shape() -> Shape {
        Shape::Bool
    }
    fn shape_controlled<C:NativeSchemaControl>(control:&mut C)->Result<Shape,ValueError>{control.checkpoint()?;Ok(Shape::Bool)}
    fn to_value(&self) -> FieldValue {
        FieldValue::Bool(*self)
    }
    fn to_value_controlled(&self,control:&mut NativeEncodeControl<'_>)->Result<FieldValue,ValueError>{control.step()?;Ok(FieldValue::Bool(*self))}
    fn from_value(value: &FieldValue) -> Result<Self, String> {
        match value {
            FieldValue::Bool(b) => Ok(*b),
            other => Err(format!("expected Bool, found {other:?}")),
        }
    }
    fn from_value_controlled(value: &FieldValue, control: &mut NativeDecodeControl<'_>) -> Result<Self,ValueError> { control.step()?;match value {FieldValue::Bool(value)=>Ok(*value),_=>Err(ValueError::new(ValueRefusalKind::InvalidValue,"expected Bool"))} }
}

impl DslField for f32 {
    // 🚫️async: E4 — see `DslField::shape`'s tag above.
    fn shape() -> Shape {
        Shape::Float
    }
    fn shape_controlled<C:NativeSchemaControl>(control:&mut C)->Result<Shape,ValueError>{control.checkpoint()?;Ok(Shape::Float)}
    fn to_value(&self) -> FieldValue {
        let bits=self.to_bits();let value=if bits&0x7f800000==0x7f800000&&bits&0x7fffff!=0{f64::from_bits(((bits as u64&0x80000000)<<32)|0x7ff0000000000000|((bits as u64&0x7fffff)<<29))}else{*self as f64};FieldValue::Float(value)
    }
    fn to_value_controlled(&self,control:&mut NativeEncodeControl<'_>)->Result<FieldValue,ValueError>{control.step()?;Ok(<Self as DslField>::to_value(self))}
    fn from_value(value: &FieldValue) -> Result<Self, String> {
        match value {
            FieldValue::Float(f)=>{let bits=f.to_bits();if bits&0x7ff0000000000000==0x7ff0000000000000&&bits&0xfffffffffffff!=0{if bits&0x1fffffff!=0{return Err("NaN word is not exactly representable at binary32 width".into());}Ok(f32::from_bits(((bits>>32)as u32&0x80000000)|0x7f800000|((bits>>29)as u32&0x7fffff)))}else{Ok(*f as f32)}},
            other => Err(format!("expected Float, found {other:?}")),
        }
    }
    fn from_value_controlled(value: &FieldValue, control: &mut NativeDecodeControl<'_>) -> Result<Self,ValueError> {
        control.step()?;
        match value {FieldValue::Float(value)=>{let bits=value.to_bits();if bits&0x7ff0000000000000==0x7ff0000000000000&&bits&0xfffffffffffff!=0{if bits&0x1fffffff!=0{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"NaN word is not exactly representable at binary32 width"));}Ok(f32::from_bits(((bits>>32)as u32&0x80000000)|0x7f800000|((bits>>29)as u32&0x7fffff)))}else{Ok(*value as f32)}},_=>Err(ValueError::new(ValueRefusalKind::InvalidValue,"expected Float"))}
    }
}

impl DslField for f64 {
    // 🚫️async: E4 — see `DslField::shape`'s tag above.
    fn shape() -> Shape {
        Shape::Float
    }
    fn shape_controlled<C:NativeSchemaControl>(control:&mut C)->Result<Shape,ValueError>{control.checkpoint()?;Ok(Shape::Float)}
    fn to_value(&self) -> FieldValue {
        FieldValue::Float(*self)
    }
    fn to_value_controlled(&self,control:&mut NativeEncodeControl<'_>)->Result<FieldValue,ValueError>{control.step()?;Ok(FieldValue::Float(*self))}
    fn from_value(value: &FieldValue) -> Result<Self, String> {
        match value {
            FieldValue::Float(f) => Ok(*f),
            other => Err(format!("expected Float, found {other:?}")),
        }
    }
    fn from_value_controlled(value: &FieldValue, control: &mut NativeDecodeControl<'_>) -> Result<Self,ValueError> { control.step()?;match value{FieldValue::Float(value)=>Ok(*value),_=>Err(ValueError::new(ValueRefusalKind::InvalidValue,"expected Float"))} }
}

/// 🔤️ `String` binds as `Shape::Text` — the one string shape. The parser accepts either a
/// bare `Ident` token or a quoted `Text` token wherever `Text` is expected; the printer emits bare
/// (unquoted) whenever `crate::os_dsl::is_bare_ident` holds for the value, quoted+escaped otherwise —
/// so bare-vs-quoted is entirely a printing decision now, not a separate shape a field opts into.
impl DslField for String {
    // 🚫️async: E4 — see `DslField::shape`'s tag above.
    fn shape() -> Shape {
        Shape::Text
    }
    fn shape_controlled<C:NativeSchemaControl>(control:&mut C)->Result<Shape,ValueError>{control.checkpoint()?;Ok(Shape::Text)}
    fn to_value(&self) -> FieldValue {
        FieldValue::Text(self.clone())
    }
    fn to_value_controlled(&self,control:&mut NativeEncodeControl<'_>)->Result<FieldValue,ValueError>{control.step()?;Ok(FieldValue::Text(control.copy_text(self)?))}
    fn from_value(value: &FieldValue) -> Result<Self, String> {
        match value {
            FieldValue::Text(s) => Ok(s.clone()),
            other => Err(format!("expected Text, found {other:?}")),
        }
    }
    fn from_value_controlled(value: &FieldValue, control: &mut NativeDecodeControl<'_>) -> Result<Self,ValueError> {
        control.step()?;
        match value { FieldValue::Text(text)=>control.copy_text(text),_=>Err(ValueError::new(ValueRefusalKind::InvalidValue,"expected Text")) }
    }
}

/// 🔌️ A wire literal as a plain struct field (or inside a `#[dsl(table)]` `Vec` as a
/// `WIRE`-typed column) — thin `DslField` wrapper around `crate::WireValue` so adopter
/// technologies never need to hand-roll their own `Shape::Wire` binding.
#[derive(Clone, Debug, PartialEq)]
pub struct Wire(pub WireValue);

impl DslField for Wire {
    // 🚫️async: E4 — see `DslField::shape`'s tag above.
    fn shape() -> Shape {
        Shape::Wire
    }
    fn shape_controlled<C:NativeSchemaControl>(control:&mut C)->Result<Shape,ValueError>{control.checkpoint()?;Ok(Shape::Wire)}
    fn to_value(&self) -> FieldValue {
        FieldValue::Wire(self.0.clone())
    }
    fn from_value(value: &FieldValue) -> Result<Self, String> {
        match value {
            FieldValue::Wire(w) => Ok(Wire(w.clone())),
            other => Err(format!("expected Wire, found {other:?}")),
        }
    }
}
/// 📚️ General recursion seam: `#[derive(DslRecord)]`/`#[derive(DslScalar)]` fields classify
/// `Vec<T>`/`[T; N]` directly (so their own printed shape stays field-specific), but a NESTED
/// collection — `Vec<Vec<T>>`, a fixed-size array field, ... — needs its inner element type to
/// satisfy `DslField` itself. These two blanket impls close that gap generically instead of adding
/// a special-cased `FieldKind` for every depth of nesting.
impl<T: DslField> DslField for Vec<T> {
    fn to_value_controlled(&self,control:&mut NativeEncodeControl<'_>)->Result<FieldValue,ValueError>{native_encoding::project_list(self,control).map(FieldValue::List)}

    fn retire_decoded(self) { for value in self { T::retire_decoded(value); } }
    // 🚫️async: E4 — see `DslField::shape`'s tag above.
    fn shape() -> Shape {
        Shape::List(Box::new(T::shape()))
    }
    fn shape_controlled<C:NativeSchemaControl>(control:&mut C)->Result<Shape,ValueError>{control.scoped_depth(64,|control|Ok(Shape::List(crate::producer::boxed(T::shape_controlled(control)?,control)?)))}
    // 🔁 `Iterator::map` cannot await per-element (residue shape 1) and `T::to_value`/`from_value`
    // are AFIT over an arbitrary implementor, so — unlike a known-pure leaf fn — R9 does not apply;
    // the fix is a plain sequential loop that awaits each element in turn.
    fn to_value(&self) -> FieldValue {
        let mut items = Vec::with_capacity(self.len());
        for item in self {
            items.push(item.to_value());
        }
        FieldValue::List(items)
    }
    fn from_value(value: &FieldValue) -> Result<Self, String> {
        match value {
            FieldValue::List(items) => {
                let mut out = Vec::with_capacity(items.len());
                for item in items {
                    out.push(T::from_value(item)?);
                }
                Ok(out)
            }
            other => Err(format!("expected List, found {other:?}")),
        }
    }
    fn from_value_controlled(value: &FieldValue, control: &mut NativeDecodeControl<'_>) -> Result<Self,ValueError> {
        control.step()?;
        match value {
            FieldValue::List(items)=>__rt::decode_list_controlled(items,control),
            _=>Err(ValueError::new(ValueRefusalKind::InvalidValue,"expected List")),
        }
    }
}

/// 🗺️ Same recursion seam as `Vec<T>`, for a `BTreeMap<String, T>` that's itself nested
/// (e.g. `Option<BTreeMap<String, T>>`) rather than a bare top-level field — `#[derive(DslRecord)]`
/// classifies a *bare* `BTreeMap<String, T>` field directly via its own dedicated `FieldKind`
/// (same `Shape::Map` this produces), so the two never conflict.
impl<T: DslField> DslField for std::collections::BTreeMap<String, T> {
    fn to_value_controlled(&self,control:&mut NativeEncodeControl<'_>)->Result<FieldValue,ValueError>{native_encoding::project_map(self,control)}

    fn retire_decoded(self) { for (_,value) in self { T::retire_decoded(value); } }
    // 🚫️async: E4 — see `DslField::shape`'s tag above.
    fn shape() -> Shape {
        Shape::Map(Box::new(T::shape()))
    }
    fn shape_controlled<C:NativeSchemaControl>(control:&mut C)->Result<Shape,ValueError>{control.scoped_depth(64,|control|Ok(Shape::Map(crate::producer::boxed(T::shape_controlled(control)?,control)?)))}
    // 🔁 Same R9-doesn't-apply reasoning as `Vec<T>` above: sequential loop, not `.map().collect()`.
    fn to_value(&self) -> FieldValue {
        let mut entries = Vec::with_capacity(self.len());
        for (k, v) in self {
            entries.push((k.clone(), v.to_value()));
        }
        FieldValue::Map(entries)
    }
    fn from_value(value: &FieldValue) -> Result<Self, String> {
        match value {
            FieldValue::Map(entries) => {
                let mut out = Self::new();
                for (k, v) in entries {
                    out.insert(k.clone(), T::from_value(v)?);
                }
                Ok(out)
            }
            other => Err(format!("expected Map, found {other:?}")),
        }
    }
    fn from_value_controlled(value: &FieldValue, control: &mut NativeDecodeControl<'_>) -> Result<Self,ValueError> {
        control.step()?;
        match value {
            FieldValue::Map(entries)=>{
                let slot=std::mem::size_of::<(String,T)>().checked_add(128).ok_or_else(||ValueError::new(ValueRefusalKind::OwnershipLimit,"map slot size overflow"))?;
                control.charge(entries.len().checked_mul(slot).ok_or_else(||ValueError::new(ValueRefusalKind::OwnershipLimit,"map ownership size overflow"))?)?;
                let mut output=__rt::DecodedFieldOwner::new(Self::new(),Self::retire_decoded);
                for (key,value) in entries {if let Some(previous)=output.as_mut().insert(control.copy_text(key)?,control.scoped_stage(|control|T::from_value_controlled(value,control))?){T::retire_decoded(previous);}}
                Ok(output.take())
            },
            _=>Err(ValueError::new(ValueRefusalKind::InvalidValue,"expected Map")),
        }
    }
}

/// 📐️ Fixed-arity `Shape::Tuple(_, Some(N))` — a packed `x,y,z`-style literal for any `N`.
impl<T: DslField, const N: usize> DslField for [T; N] {
    fn to_value_controlled(&self,control:&mut NativeEncodeControl<'_>)->Result<FieldValue,ValueError>{native_encoding::project_list(self,control).map(FieldValue::Tuple)}

    fn retire_decoded(self) { for value in self { T::retire_decoded(value); } }
    // 🚫️async: E4 — see `DslField::shape`'s tag above.
    fn shape() -> Shape {
        Shape::Tuple(Box::new(T::shape()), Some(N))
    }
    fn shape_controlled<C:NativeSchemaControl>(control:&mut C)->Result<Shape,ValueError>{control.scoped_depth(64,|control|Ok(Shape::Tuple(crate::producer::boxed(T::shape_controlled(control)?,control)?,Some(N))))}
    // 🔁 Same R9-doesn't-apply reasoning as `Vec<T>` above: sequential loop, not `.map().collect()`.
    fn to_value(&self) -> FieldValue {
        let mut items = Vec::with_capacity(N);
        for item in self {
            items.push(item.to_value());
        }
        FieldValue::Tuple(items)
    }
    fn from_value(value: &FieldValue) -> Result<Self, String> {
        match value {
            FieldValue::Tuple(items) if items.len() == N => {
                let mut converted: Vec<T> = Vec::with_capacity(N);
                for item in items {
                    converted.push(T::from_value(item)?);
                }
                converted.try_into().map_err(|_| format!("expected {N} items, got a length mismatch"))
            }
            other => Err(format!("expected a {N}-item Tuple, found {other:?}")),
        }
    }
    fn from_value_controlled(value: &FieldValue, control: &mut NativeDecodeControl<'_>) -> Result<Self,ValueError> {
        control.step()?;
        match value {
            FieldValue::Tuple(items) if items.len()==N=>{let mut output=__rt::DecodedFieldOwner::new(control.allocate_vec::<T>(N)?,<Vec<T> as DslField>::retire_decoded);for item in items {output.as_mut().push(control.scoped_stage(|control|T::from_value_controlled(item,control))?);}output.take().try_into().map_err(|values:Vec<T>|{<Vec<T> as DslField>::retire_decoded(values);ValueError::new(ValueRefusalKind::InvariantViolated,"tuple arity mismatch")})},
            _=>Err(ValueError::new(ValueRefusalKind::InvalidValue,format!("expected a {N}-item Tuple"))),
        }
    }
}

/// 🌱️ Schema-less dynamic literal — binds as `Shape::Value`.
impl DslField for DslValue {
    // 🚫️async: E4 — see `DslField::shape`'s tag above.
    fn shape() -> Shape {
        Shape::Value
    }
    fn shape_controlled<C:NativeSchemaControl>(control:&mut C)->Result<Shape,ValueError>{control.checkpoint()?;Ok(Shape::Value)}
    fn to_value(&self) -> FieldValue {
        FieldValue::Value(self.clone())
    }
    fn to_value_controlled(&self,control:&mut NativeEncodeControl<'_>)->Result<FieldValue,ValueError>{<Self as semio_framework_value::ToValue>::to_value_controlled(self,control).map(FieldValue::Value)}
    fn from_value_controlled(value:&FieldValue,control:&mut NativeDecodeControl<'_>)->Result<Self,ValueError>{match value{FieldValue::Value(value)=><Self as semio_framework_value::FromValue>::from_value_controlled(value,control),_=>Err(ValueError::new(ValueRefusalKind::InvalidValue,"expected an intrinsic Value field"))}}
    fn retire_decoded(self){<Self as semio_framework_value::FromValue>::retire_decoded(self)}
    fn from_value(value: &FieldValue) -> Result<Self, String> {
        match value {
            FieldValue::Value(dsl_value) => Ok(dsl_value.clone()),
            other => Err(format!("expected Value, found {other:?}")),
        }
    }
}
//#endregion 🔖️Field

//#region 🔖️Variants
/// 🌿️ Bridges an enum whose variants are each their own keyword-tagged record — the type
/// bound for `#[dsl(statements)] Vec<T>` collection fields and for `#[derive(DslOps)]` operation
/// enums. `#[derive(DslEnum)]`-with-struct-variants and `#[derive(DslOps)]` both implement this.
pub trait DslVariants: Sized {
    /// 🐌️ Lazy: each entry is a zero-capture `fn` pointer, not an eagerly-built `RecordSpec`
    /// — a self-referential grammar's own `variants()` would otherwise need to recurse infinitely
    /// just to construct this list. See [`Shape::Statements`]'s doc comment for the full rationale.
    // 🚫️async: E4 — the returned `Vec<(String, fn() -> RecordSpec)>` IS a fn-pointer table, and
    // `Shape::Statements(<T>::variants())` is itself called from inside a sync `__dsl_spec` — see R9.
    fn variants() -> Vec<(String, RecordSpecProducer)>;
    /// 🌿️ Owns literal variant labels and their lazy controlled schema producers.
    fn variants_controlled<C:NativeSchemaControl>(control:&mut C)->Result<Vec<(String,RecordSpecProducer)>,ValueError>{control.checkpoint()?;Err(ValueError::new(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"variant owner has no controlled native schema implementation"))}
    fn to_named_record(&self) -> (String, RecordValue);
    /// 🌿️ Projects a declared tagged variant under the same cumulative output control.
    fn to_named_record_controlled(&self,control:&mut NativeEncodeControl<'_>)->Result<(String,RecordValue),ValueError>{control.checkpoint()?;Err(ValueError::new(ValueRefusalKind::UnsupportedOwner,"variant owner has no controlled native projection implementation"))}
    /// ⚠️ Returns `TextError` (not `String`, unlike [`DslField::from_value`]) so
    /// generated bodies can `?`-propagate it directly — this is the same error type
    /// `crate::os_spr::OpText::parse_op`/`crate::os_store::ArtifactDsl::parse_dsl` already return, and the derive's
    /// `#[dsl(statements)]` field codegen composes it without any conversion at every nesting depth.
    fn from_named_record(keyword: &str, record: &RecordValue) -> Result<Self, TextError>;
    /// 🌲️ Retires a completed tagged value through its domain owner.
    fn retire_decoded_variant(self) { drop(self); }
    /// 🌿️ Constructs a declared variant without invoking an unchecked owner binding.
    fn from_named_record_controlled(_keyword:&str,_record:&RecordValue,control:&mut NativeDecodeControl<'_>)->Result<Self,ValueError>{
        control.checkpoint()?;
        Err(ValueError::new(ValueRefusalKind::UnsupportedOwner,"variant owner has no controlled native construction implementation"))
    }
}
//#endregion 🔖️Variants

//#region 🔖️Runtime
/// ⚙️ Helpers remaining after P6 flag day — DslField/DslVariants derive bodies only (codec paths deleted).
pub mod __rt {
    use super::*;

    /// 🧹️ Holds a completed typed field until construction commits or invokes its actual owner retirement.
    pub struct DecodedFieldOwner<T> { value:Option<T>, retire:fn(T) }
    impl<T> DecodedFieldOwner<T> {
        /// 📥️ Adopts one owned field with its declared retirement function.
        pub fn new(value:T,retire:fn(T))->Self { Self{value:Some(value),retire} }
        /// 🌿️ Allows bounded construction inside the guarded owned collection.
        pub fn as_mut(&mut self)->&mut T { self.value.as_mut().expect("decoded owner already transferred") }
        /// 📤️ Transfers ownership only after every required constructor succeeds.
        pub fn take(mut self)->T { self.value.take().expect("decoded owner already transferred") }
    }
    impl<T> Drop for DecodedFieldOwner<T> { fn drop(&mut self){if let Some(value)=self.value.take(){(self.retire)(value);}} }

    /// 📋️ Binds declared list elements with one known collection workload and cumulative ownership.
    pub fn decode_list_controlled<T:DslField>(items:&[FieldValue],control:&mut NativeDecodeControl<'_>)->Result<Vec<T>,ValueError>{
        control.scoped_stage(|control|{control.begin_stage(items.len())?;let mut output=DecodedFieldOwner::new(control.allocate_vec::<T>(items.len())?,<Vec<T> as DslField>::retire_decoded);for item in items{output.as_mut().push(control.scoped_stage(|control|{control.begin_stage(0)?;T::from_value_controlled(item,control)})?);control.step()?;}Ok(output.take())})
    }
    /// 🌿️ Binds tagged variants with exact collection progress and declared variant retirement.
    pub fn decode_statements_controlled<T:DslVariants>(items:&[(String,RecordValue)],control:&mut NativeDecodeControl<'_>)->Result<Vec<T>,ValueError>{
        control.scoped_stage(|control|{control.begin_stage(items.len())?;let mut output=DecodedFieldOwner::new(control.allocate_vec::<T>(items.len())?,|values:Vec<T>|{for value in values{T::retire_decoded_variant(value);}});for(keyword,record)in items{output.as_mut().push(control.scoped_stage(|control|{control.begin_stage(0)?;T::from_named_record_controlled(keyword,record,control)})?);control.step()?;}Ok(output.take())})
    }

    /// 📐️ Resolves a `#[dsl(unit = "...")]`/`#[dsl(angle = "...")]` symbol at spec-build
    /// time. An unknown symbol is a derive-time misuse (a typo'd unit string, caught the first time
    /// the generated `__dsl_spec` runs — every RecordSpec-law test exercises this), so it panics
    /// rather than threading a `Result` through the whole spec-building call chain, matching
    /// `newtype_variant_spec`'s convention above.
    pub fn unit_for_derive(symbol: &'static str) -> &'static UnitSpec {
        unit_by_symbol(symbol).unwrap_or_else(|| panic!("dsl: unknown unit symbol '{symbol}' in #[dsl(unit = ...)]/#[dsl(angle = ...)]"))
    }

    /// 📦️ Single-field tuple ("newtype") enum variant support — `Variant(Body)` delegates its
    /// whole `RecordSpec`/value to `Body`'s own `DslField` impl rather than wrapping it in one
    /// positional field, so `Body` prints/parses identically whether reached through the enum or on
    /// its own. `Body` must have `Shape::Record` (i.e. itself come from `#[derive(DslRecord)]` or
    /// `#[derive(DslArtifact)]`) — anything else is a derive-time misuse, hence the panic rather than
    /// a `Result` (there is no sensible recoverable path for a grammar that's wrong at compile time).
    // 🚫️async: E4 — this fn's VALUE is cast `as fn() -> RecordSpec` at every newtype-variant call
    // site (`✨️derive/🦀️.rs`'s `dsl_variants_codegen`), and it calls the now-sync `DslField::shape`.
    pub fn newtype_variant_spec<T: DslField>() -> RecordSpec {
        match T::shape() {
            Shape::Record(spec_fn) => (spec_fn.ordinary)(),
            other => panic!("newtype variant's inner type must have Record shape, found {other:?}"),
        }
    }

    /// 🪆️ Delegates a declared record variant through its explicit lazy schema producer.
    pub fn newtype_variant_producer<T:DslField>()->RecordSpecProducer{
        RecordSpecProducer{ordinary:newtype_variant_spec::<T>,decoding:|control|{match T::shape_controlled(control)?{Shape::Record(producer)=>producer.decode(control),_=>Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue,"newtype variant requires a controlled Record schema"))}},encoding:|control|{match T::shape_controlled(control)?{Shape::Record(producer)=>producer.encode(control),_=>Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue,"newtype variant requires a controlled Record schema"))}}}
    }

    pub fn newtype_variant_to_record<T: DslField>(inner: &T) -> RecordValue {
        match inner.to_value() {
            FieldValue::Record(record) => record,
            other => panic!("newtype variant's inner type must produce a Record value, found {other:?}"),
        }
    }

    pub fn newtype_variant_from_record<T: DslField>(record: &RecordValue) -> Result<T, TextError> {
        T::from_value(&FieldValue::Record(record.clone())).map_err(|message|TextError::new(ValueRefusalKind::InvalidValue,message,TextSpan::at(1,1)))
    }
}

//#endregion 🔖️Runtime

//#region 🔖️OpTextRt
/// 🔤️ Handcrafted `OpText` helper — the text twin of [`variants_binary`].
///
/// An operation line is ONE terminal keyword-tagged record, so it parses through
/// [`parse_exact`], which rejects every token outside the variant's own schema body: a trailing
/// `unknown-field 1` is not a second statement, it is garbage the line must refuse. Plain
/// [`parse`] stops at the end of the record it recognises and silently drops the rest, which is
/// the document-mode contract, not the op-line one.
pub mod variants_text {
    use super::{print, DslVariants, JoinMode, Limits, ParseOptions, SourceMode, TextError};

    pub fn parse_op<T: DslVariants>(line: &str) -> Result<T, TextError> {
        let variants = T::variants();
        for (keyword, spec_fn) in &variants {
            if line == keyword.as_str() || line.starts_with(&format!("{keyword} ")) {
                let record = super::parse_exact(line, &(spec_fn.ordinary)(), &ParseOptions { limits: Limits::default(), mode: SourceMode::Inline })?;
                return T::from_named_record(keyword, &record);
            }
        }
        Err(TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,format!("unknown operation line '{line}'"),semio_framework_diagnostic::TextSpan::at(1,1)))
    }

    pub fn print_op<T: DslVariants>(op: &T) -> String {
        let (keyword, record) = op.to_named_record();
        let variants = T::variants();
        let spec_fn = variants.iter().find(|(key, _)| key == &keyword).map(|(_, spec)| *spec).expect("variant spec must exist for its own keyword");
        print(&record, &(spec_fn.ordinary)(), JoinMode::Inline)
    }
}
//#endregion 🔖️OpTextRt

#[path = "🏷️type/🦀️.rs"]
mod value_type_binding;

```

### 🧰️framework/🔨️modules/🗣️dsl/🧬️schema/🪆️binding/🏷️type/🦀️.rs

SHA-256 `61e0f42742d14cf3b9792ee931568da791f4865678d8dbfc65f67ae1fdca08c0`; 1350 bytes.

```
//! 🏷️ Canonical seven-variant value types bind through their actual owned intrinsic tree.
use semio_framework_value::{FromValue, ToValue, ValueType};
use crate::{DslField, FieldValue, Shape, NativeSchemaControl, NativeEncodeControl, NativeDecodeControl, ValueError};
impl DslField for ValueType {
    fn shape() -> Shape { Shape::Value }
    fn shape_controlled<C:NativeSchemaControl>(control:&mut C)->Result<Shape,ValueError>{control.checkpoint()?;Ok(Shape::Value)}
    fn to_value(&self)->FieldValue{FieldValue::Value(ToValue::to_value(self))}
    fn to_value_controlled(&self,control:&mut NativeEncodeControl<'_>)->Result<FieldValue,ValueError>{Ok(FieldValue::Value(ToValue::to_value_controlled(self,control)?))}
    fn from_value(value:&FieldValue)->Result<Self,String>{match value{FieldValue::Value(value)=><Self as FromValue>::from_value(value.clone()).map_err(ValueError::into_message),_=>Err("expected a typed value declaration".into())}}
    fn from_value_controlled(value:&FieldValue,control:&mut NativeDecodeControl<'_>)->Result<Self,ValueError>{match value{FieldValue::Value(value)=><Self as FromValue>::from_value_controlled(value,control),_=>Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue,"expected a typed value declaration"))}}
    fn retire_decoded(self){<Self as FromValue>::retire_decoded(self)}
}

```

### 🧰️framework/🔨️modules/🕸️graph/📦️packages/🦀️rust/Cargo.toml

SHA-256 `c3194f523c7829426c4fec0fce9fbb979c1b067e8fff25eebf91d48ba6c0cc09`; 2107 bytes.

```
[package]
workspace = "../../../../.."
name = "semio-framework-graph"
version = "0.1.0"
edition = "2021"
rust-version.workspace = true
description = "Graph vocabulary, index-based algorithms, drawing layouts and the compile-time graph manifest registry — the framework-internal graph surface every framework crate may name without reaching a plugin"
build = false

[package.metadata.semio]
role = "framework"
id = "graph"

[lints]
workspace = true

[lib]
name = "semio_framework_graph"
path = "🦀️.rs"

[dependencies]
semio-framework-pack-json = { path = "../../../🎒️pack/🔤️json/📦️packages/🦀️rust" }
semio-framework-diagnostic = { path = "../../../⚠️diagnostic/📦️packages/🦀️rust" }
semio-framework-dsl = { path = "../../../🗣️dsl/📦️packages/🦀️rust" }
semio-framework-ui-locale = { path = "../../../🖱️ui/🌐️locale/📦️packages/🦀️rust" }
semio-framework-value = { path = "../../../🌱️value/📦️packages/🦀️rust" }
semio-framework-ui-contract = { path = "../../../🖱️ui/🧬️contract/📦️packages/🦀️rust", package = "semio-framework-ui-contract" }
geometry = { path = "../../../📐️geometry/📦️packages/🦀️rust", package = "semio-framework-geometry" }
semio-framework-os-kernel = { path = "../../../../🛍️products/💻️os/📦️packages/🦀️rust", package = "semio-framework-os-kernel" }
semio-framework-value-derive = { path = "../../../🌱️value/✨️derive/📦️packages/🦀️rust", package = "semio-framework-value-derive" }
# ⚠️ `serde`/`serde_json` are fully removed — every type in this crate carries `ToValue`/
# `FromValue` instead of `Serialize`/`Deserialize`. `⚙️engine/🦀️.rs`'s `property_bag_from_value`/
# `property_bag_to_value` now take/return `dsl_core::DslValue`; `♾️infinite`'s board ports
# (the former external caller) bridge their own `serde_json::Value`-typed `user_data` field at
# the call site via the existing `DslValue::from` bridge. RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS, 26/09/02.

[dev-dependencies]
serde_json="1"

```

### 🧰️framework/🔨️modules/🕸️graph/📦️packages/🦀️rust/🦀️.rs

SHA-256 `006ecd36c9b70df435081f18d12ccb96dbf13865e327679e2c6355eb1212495c`; 2150 bytes.

```
//! 🕸️ The semio graph framework module: storage and view vocabulary, index-based algorithms, drawing layouts, the compile-time manifest registry, and the Jack graph query language.
//!
//! Each domain is a `🦀️.rs` in the owner tree; this entry file is pure wiring.

// 🃏️ `QueryableGraph` (Jack) has async trait methods; Send-ness comes structurally from the
// concrete/generic caller, never from a `+ Send` bound on the trait method — see R3 and R7.
#![allow(async_fn_in_trait)]

// 🃏️ Renamed `dsl` → `dsl_core` (wave MATHEND) to free the crate-root name `dsl` for
// `pub mod dsl` (Jack) below — both alias the identical crate. `🛂️manifest` updated to match.
extern crate semio_framework_os_kernel as dsl_core;
extern crate semio_framework_value_derive as value_derive;

#[path = "../../⚙️engine/🦀️.rs"]
mod engine;
pub use engine::*;

#[path = "../../🧮️algorithms/🦀️.rs"]
pub mod algorithms;

#[path = "../../🖊️drawing/🦀️.rs"]
pub mod drawing;

#[path = "../../🛂️manifest/🦀️.rs"]
pub mod manifest;

// 🃏️ wave MATHEND (ticket 26/08/12/DISSOLVE-KERNELS-AND-MODULES-INTO-EVENT-SOURCED-ARTIFACTS):
// Jack relocated verbatim from `🧰️framework/🔨️modules/🧮️math/🕸️graph/🗣️dsl` — measured NOT
// cleanly splittable into a framework "core" + plugin "language-service" half as first hypothesized:
// `complete`/`format` are needed by BOTH the framework-tier `DslIdiom` self-registration seam
// (`idiom_hooks`) AND `🔱️trinity`'s explicit LSP-style calls, and `complete`/`hover` share private
// helpers (`collect_bound_vars`, `lex_spanned`) that would need to become new public API or be
// duplicated (forbidden) to cross a crate boundary. Independently, Jack (a generic pattern-matching
// query language over graphs, plus its own editor tooling) passes the domain-neutral test — it names
// no domain, structurally analogous to `💻️os/🔨️modules/🗣️dsl`'s already-framework-tier
// diagnostic/completion machinery. See that wave's report for the full evidence and reasoning.
#[path = "../../🗣️dsl/🦀️.rs"]
pub mod dsl;

```

### 🧰️framework/🔨️modules/🕸️graph/🛂️manifest/🪆️binding/🦀️.rs

SHA-256 `375c41a61d33b060d3dac7b0cea50421edbaef6f42142fd6cfcc06ddeaacba0d`; 6927 bytes.

```
//! 📋️ Actual property declarations and port directions own their typed native fields.
use super::{PropertyDef,PropertyKind,PortDirection};
use crate::dsl_core::{DslField,FieldValue,RecordValue,RecordSpec,RecordSpecProducer,FieldSpec,RecordLayout,Shape,NativeSchemaControl,NativeEncodeControl,NativeDecodeControl,ValueError};
use semio_framework_value::{FromValue,ValueRefusalKind,ValueType,DecodedValue};
macro_rules! scalar {
    ($ty:ty,$first:ident,$second:ident,$first_name:literal,$second_name:literal)=>{
        impl DslField for $ty {
            fn shape()->Shape{Shape::Enum(vec![($first_name.into(),0),($second_name.into(),1)])}
            fn shape_controlled<C:NativeSchemaControl>(control:&mut C)->Result<Shape,ValueError>{control.scoped_stage(|control|{control.begin_stage(2)?;let mut values=control.allocate_vec(2)?;values.push((control.copy_text($first_name)?,0));control.step()?;values.push((control.copy_text($second_name)?,1));control.step()?;Ok(Shape::Enum(values))})}
            fn to_value(&self)->FieldValue{FieldValue::Enum(match self{Self::$first=>0,Self::$second=>1})}
            fn to_value_controlled(&self,control:&mut NativeEncodeControl<'_>)->Result<FieldValue,ValueError>{control.checkpoint()?;Ok(<Self as DslField>::to_value(self))}
            fn from_value(value:&FieldValue)->Result<Self,String>{match value{FieldValue::Enum(0)=>Ok(Self::$first),FieldValue::Enum(1)=>Ok(Self::$second),_=>Err("invalid declared enum ordinal".into())}}
            fn from_value_controlled(value:&FieldValue,control:&mut NativeDecodeControl<'_>)->Result<Self,ValueError>{control.checkpoint()?;match value{FieldValue::Enum(0)=>Ok(Self::$first),FieldValue::Enum(1)=>Ok(Self::$second),_=>Err(ValueError::new(ValueRefusalKind::InvalidValue,"invalid declared enum ordinal"))}}
        }
    };
}
scalar!(PropertyKind,Data,Derived,"data","derived");
scalar!(PortDirection,In,Out,"in","out");
fn ordinary_spec()->RecordSpec{RecordSpec::new(None,RecordLayout::Inline,vec![FieldSpec::new(0,"name",<String as DslField>::shape()),FieldSpec::new(1,"kind",<PropertyKind as DslField>::shape()),FieldSpec::new(2,"value-type",<ValueType as DslField>::shape()),FieldSpec::new(3,"expr",<String as DslField>::shape()).optional()])}
fn controlled_spec<C:NativeSchemaControl>(control:&mut C)->Result<RecordSpec,ValueError>{control.scoped_stage(|control|{
    control.begin_stage(4)?;let mut fields=control.allocate_vec(4)?;
    for(id,key,shape,optional)in[(0,"name",<String as DslField>::shape_controlled(control)?,false),(1,"kind",<PropertyKind as DslField>::shape_controlled(control)?,false),(2,"value-type",<ValueType as DslField>::shape_controlled(control)?,false),(3,"expr",<String as DslField>::shape_controlled(control)?,true)]{let mut field=crate::dsl_core::schema::producer::field(id,key,shape,control)?;field.optional=optional;fields.push(field);control.step()?;}
    crate::dsl_core::schema::producer::record(None,RecordLayout::Inline,fields,control)
})}
fn decoding_spec(control:&mut NativeDecodeControl<'_>)->Result<RecordSpec,ValueError>{controlled_spec(control)}
fn encoding_spec(control:&mut NativeEncodeControl<'_>)->Result<RecordSpec,ValueError>{controlled_spec(control)}
fn producer()->RecordSpecProducer{RecordSpecProducer{ordinary:ordinary_spec,decoding:decoding_spec,encoding:encoding_spec}}
fn retire(value:PropertyDef){<ValueType as FromValue>::retire_decoded(value.value_type)}
impl DslField for PropertyDef {
    fn shape()->Shape{Shape::Record(producer())}
    fn shape_controlled<C:NativeSchemaControl>(control:&mut C)->Result<Shape,ValueError>{control.checkpoint()?;Ok(Shape::Record(producer()))}
    fn to_value(&self)->FieldValue{FieldValue::Record(RecordValue{fields:[(0,<String as DslField>::to_value(&self.name)),(1,<PropertyKind as DslField>::to_value(&self.kind)),(2,<ValueType as DslField>::to_value(&self.value_type)),(3,self.expr.as_ref().map(<String as DslField>::to_value).unwrap_or(FieldValue::Absent))].into_iter().collect()})}
    fn to_record_controlled(&self,control:&mut NativeEncodeControl<'_>)->Result<RecordValue,ValueError>{control.scoped_stage(|control|{
        control.begin_stage(4)?;let mut record=crate::dsl_core::native_encoding::EncodedRecord::new(4,control)?;
        record.insert(0,<String as DslField>::to_value_controlled(&self.name,control)?)?;control.step()?;
        record.insert(1,<PropertyKind as DslField>::to_value_controlled(&self.kind,control)?)?;control.step()?;
        record.insert(2,<ValueType as DslField>::to_value_controlled(&self.value_type,control)?)?;control.step()?;
        record.insert(3,match &self.expr{Some(value)=><String as DslField>::to_value_controlled(value,control)?,None=>FieldValue::Absent})?;control.step()?;
        Ok(record.take())
    })}
    fn to_value_controlled(&self,control:&mut NativeEncodeControl<'_>)->Result<FieldValue,ValueError>{let record=crate::dsl_core::native_encoding::EncodedRecord::from_record(Self::to_record_controlled(self,control)?);Ok(FieldValue::Record(record.take()))}
    fn from_value(value:&FieldValue)->Result<Self,String>{let FieldValue::Record(record)=value else{return Err("expected property declaration record".into())};let get=|id|record.get(id).ok_or_else(||"missing property declaration field".to_string());Ok(Self{name:<String as DslField>::from_value(get(0)?)?,kind:<PropertyKind as DslField>::from_value(get(1)?)?,value_type:<ValueType as DslField>::from_value(get(2)?)?,expr:match get(3)?{FieldValue::Absent=>None,value=>Some(<String as DslField>::from_value(value)?)}})}
    fn from_record_controlled(record:&RecordValue,control:&mut NativeDecodeControl<'_>)->Result<Self,ValueError>{control.scoped_stage(|control|{
        control.begin_stage(4)?;if record.fields.len()!=4{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"invalid property declaration fields"));}
        let get=|id|record.get(id).ok_or_else(||ValueError::new(ValueRefusalKind::InvalidValue,"missing property declaration field"));
        let name=<String as DslField>::from_value_controlled(get(0)?,control)?;control.step()?;
        let kind=<PropertyKind as DslField>::from_value_controlled(get(1)?,control)?;control.step()?;
        let value_type=DecodedValue::new(<ValueType as DslField>::from_value_controlled(get(2)?,control)?,<ValueType as FromValue>::retire_decoded);control.step()?;
        let expr=match get(3)?{FieldValue::Absent=>None,value=>Some(<String as DslField>::from_value_controlled(value,control)?)};
        let value=DecodedValue::new(Self{name,kind,value_type:value_type.take(),expr},retire);control.step()?;Ok(value.take())
    })}
    fn from_value_controlled(value:&FieldValue,control:&mut NativeDecodeControl<'_>)->Result<Self,ValueError>{match value{FieldValue::Record(record)=>Self::from_record_controlled(record,control),_=>Err(ValueError::new(ValueRefusalKind::InvalidValue,"expected property declaration record"))}}
    fn retire_decoded(self){retire(self)}
}

```

### 🧰️framework/🔨️modules/🕸️graph/🛂️manifest/🗂️properties/🦀️.rs

SHA-256 `20004a364e0de469a0428221b8fb7870c04eee4d53d09eb16015b37f85b85ddc`; 8702 bytes.

```
//! 🗂️ Literal unique graph properties own concrete sorted contiguous member slots.
use super::PropertyValue;
use semio_framework_value::{DecodedValue,DslValue,FromValue,NativeDecodeControl,NativeEncodeControl,ToValue,ValueError,ValueRefusalKind};
#[derive(Clone,Debug,Default,PartialEq)]
pub struct PropertyBag { members:Vec<(String,PropertyValue)> }
impl PropertyBag {
 pub fn new()->Self{Self::default()}
 pub fn len(&self)->usize{self.members.len()}
 pub fn is_empty(&self)->bool{self.members.is_empty()}
 pub fn iter(&self)->impl DoubleEndedIterator<Item=(&String,&PropertyValue)>+ExactSizeIterator{self.members.iter().map(|(key,value)|(key,value))}
 pub fn keys(&self)->impl DoubleEndedIterator<Item=&String>+ExactSizeIterator{self.members.iter().map(|(key,_)|key)}
 pub fn values(&self)->impl DoubleEndedIterator<Item=&PropertyValue>+ExactSizeIterator{self.members.iter().map(|(_,value)|value)}
 pub fn into_values(self)->impl DoubleEndedIterator<Item=PropertyValue>+ExactSizeIterator{self.members.into_iter().map(|(_,value)|value)}
 pub fn get(&self,key:&str)->Option<&PropertyValue>{self.members.binary_search_by(|(name,_)|name.as_str().cmp(key)).ok().map(|index|&self.members[index].1)}
 pub fn get_mut(&mut self,key:&str)->Option<&mut PropertyValue>{self.members.binary_search_by(|(name,_)|name.as_str().cmp(key)).ok().map(|index|&mut self.members[index].1)}
 pub fn contains_key(&self,key:&str)->bool{self.get(key).is_some()}
 pub fn insert(&mut self,key:String,value:PropertyValue)->Option<PropertyValue>{match self.members.binary_search_by(|(name,_)|name.cmp(&key)){Ok(index)=>Some(std::mem::replace(&mut self.members[index].1,value)),Err(index)=>{self.members.insert(index,(key,value));None}}}
 pub fn remove(&mut self,key:&str)->Option<PropertyValue>{self.members.binary_search_by(|(name,_)|name.as_str().cmp(key)).ok().map(|index|self.members.remove(index).1)}
 pub fn pop_last(&mut self)->Option<(String,PropertyValue)>{self.members.pop()}
 pub fn successor(&self,key:&str)->Option<(&String,&PropertyValue)>{self.members.get(self.members.partition_point(|(name,_)|name.as_str()<=key)).map(|(key,value)|(key,value))}
 pub fn first_key_value(&self)->Option<(&String,&PropertyValue)>{self.members.first().map(|(key,value)|(key,value))}
 pub fn from_admitted(members:Vec<(String,PropertyValue)>)->Self{Self{members}}
 pub fn admitted_insert(&mut self,key:String,value:PropertyValue,control:&mut NativeDecodeControl<'_>)->Result<(),ValueError>{
  self.insert_controlled(key,value,&mut||control.step())
 }
 pub fn insert_controlled(&mut self,key:String,value:PropertyValue,checkpoint:&mut dyn FnMut()->Result<(),ValueError>)->Result<(),ValueError>{
  let value=DecodedValue::new(value,PropertyValue::retire_decoded);
  let mut index=0;while index<self.members.len(){checkpoint()?;match compare(&self.members[index].0,&key,checkpoint)?{std::cmp::Ordering::Less=>index+=1,std::cmp::Ordering::Equal=>{PropertyValue::retire_decoded(std::mem::replace(&mut self.members[index].1,value.take()));return Ok(())},std::cmp::Ordering::Greater=>break}}
  if self.members.len()==self.members.capacity(){return Err(ValueError::new(ValueRefusalKind::InvariantViolated,"property map exceeds admitted member slots"));}
  self.members.push((key,value.take()));let mut position=self.members.len()-1;while position>index{self.members.swap(position,position-1);position-=1;checkpoint()?;}Ok(())
 }
}
fn compare(left:&str,right:&str,checkpoint:&mut dyn FnMut()->Result<(),ValueError>)->Result<std::cmp::Ordering,ValueError>{for(index,(left,right))in left.bytes().zip(right.bytes()).enumerate(){if index%256==0{checkpoint()?;}if left!=right{return Ok(left.cmp(&right))}}Ok(left.len().cmp(&right.len()))}
impl<const N:usize> From<[(String,PropertyValue);N]> for PropertyBag{fn from(values:[(String,PropertyValue);N])->Self{values.into_iter().collect()}}
impl FromIterator<(String,PropertyValue)> for PropertyBag{fn from_iter<T:IntoIterator<Item=(String,PropertyValue)>>(values:T)->Self{let mut result=Self::new();for(key,value)in values{if let Some(previous)=result.insert(key,value){PropertyValue::retire_decoded(previous)}}result}}
impl IntoIterator for PropertyBag{type Item=(String,PropertyValue);type IntoIter=std::vec::IntoIter<Self::Item>;fn into_iter(self)->Self::IntoIter{self.members.into_iter()}}
impl<'a> IntoIterator for &'a PropertyBag{type Item=(&'a String,&'a PropertyValue);type IntoIter=std::iter::Map<std::slice::Iter<'a,(String,PropertyValue)>,fn(&'a(String,PropertyValue))->Self::Item>;fn into_iter(self)->Self::IntoIter{self.members.iter().map(|(key,value)|(key,value))}}
impl semio_framework_value::retirement::RetireOwned for PropertyBag{fn retirement(self)->Box<dyn semio_framework_value::retirement::RetirementCursor>{semio_framework_value::retirement::RetireOwned::retirement(self.members)}}
impl FromValue for PropertyBag {
 fn from_value(value:DslValue)->Result<Self,ValueError>{let DslValue::Object(fields)=value else{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"graph property object required"))};fields.into_iter().map(|(key,value)|PropertyValue::from_value(value).map(|value|(key,value))).collect()}
 fn from_value_controlled(value:&DslValue,control:&mut NativeDecodeControl<'_>)->Result<Self,ValueError>{control.scoped_depth(64,|control|control.scoped_stage(|control|{let fields=value.object_controlled(control)?;control.begin_stage(0)?;let mut output=DecodedValue::new(Self::from_admitted(control.allocate_vec(fields.len())?),<Self as FromValue>::retire_decoded);for(key,value)in fields{let key=control.copy_text(key)?;let value=PropertyValue::from_value_controlled(value,control)?;output.get_mut().admitted_insert(key,value,control)?;}control.checkpoint()?;Ok(output.take())}))}
 fn default_value_controlled(control:&mut NativeDecodeControl<'_>)->Result<Self,ValueError>{control.checkpoint()?;Ok(Self::new())}
 fn retire_decoded(self){for(_,value)in self.members{PropertyValue::retire_decoded(value)}}
}
impl ToValue for PropertyBag {
 fn to_value(&self)->DslValue{DslValue::Object(self.iter().map(|(key,value)|(key.clone(),value.to_value())).collect())}
 fn to_value_controlled(&self,control:&mut NativeEncodeControl<'_>)->Result<DslValue,ValueError>{control.scoped_depth(64,|control|control.scoped_stage(|control|{control.begin_stage(0)?;let mut output=DslValue::object_encoding_controlled(self.len(),control)?;for(key,value)in self{let value=value.to_value_controlled(control)?;DslValue::push_encoding_controlled(output.get_mut(),key,value,control)?;control.step()?;}control.checkpoint()?;Ok(DslValue::Object(output.take()))}))}
}
impl dsl_core::DslField for PropertyBag {
 fn shape()->dsl_core::Shape{dsl_core::Shape::Map(Box::new(dsl_core::Shape::Value))}
 fn shape_controlled<C:dsl_core::NativeSchemaControl>(control:&mut C)->Result<dsl_core::Shape,ValueError>{dsl_core::producer::boxed(dsl_core::Shape::Value,control).map(dsl_core::Shape::Map)}
 fn to_value(&self)->dsl_core::FieldValue{dsl_core::FieldValue::Map(self.iter().map(|(key,value)|(key.clone(),<PropertyValue as dsl_core::DslField>::to_value(value))).collect())}
 fn from_value(value:&dsl_core::FieldValue)->Result<Self,String>{let dsl_core::FieldValue::Map(fields)=value else{return Err("graph property map required".into())};fields.iter().map(|(key,value)|<PropertyValue as dsl_core::DslField>::from_value(value).map(|value|(key.clone(),value))).collect()}
 fn to_value_controlled(&self,control:&mut NativeEncodeControl<'_>)->Result<dsl_core::FieldValue,ValueError>{control.scoped_stage(|control|{control.begin_stage(0)?;let mut output=dsl_core::__rt::DecodedFieldOwner::new(control.allocate_vec(self.len())?,|values:Vec<(String,dsl_core::FieldValue)>|{for(_,value)in values{dsl_core::native_encoding::retire_field(value)}});for(key,value)in self{let key=control.copy_text(key)?;let value=<PropertyValue as dsl_core::DslField>::to_value_controlled(value,control)?;output.as_mut().push((key,value));control.step()?;}Ok(dsl_core::FieldValue::Map(output.take()))})}
 fn from_value_controlled(value:&dsl_core::FieldValue,control:&mut NativeDecodeControl<'_>)->Result<Self,ValueError>{let dsl_core::FieldValue::Map(fields)=value else{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"graph property map required"))};control.scoped_stage(|control|{control.begin_stage(0)?;let mut output=DecodedValue::new(Self::from_admitted(control.allocate_vec(fields.len())?),<Self as FromValue>::retire_decoded);for(key,value)in fields{let key=control.copy_text(key)?;let value=<PropertyValue as dsl_core::DslField>::from_value_controlled(value,control)?;output.get_mut().admitted_insert(key,value,control)?;}Ok(output.take())})}
 fn retire_decoded(self){<Self as FromValue>::retire_decoded(self)}
}

```

### 🧰️framework/🔨️modules/🕸️graph/🗣️dsl/🦀️.rs

SHA-256 `dbd6e61e3420e856a04ee4e541cfdc2ab302dd95d80211b3f3b66f99fd164509`; 96726 bytes.

```
//! 🃏️ Shared Jack query language for graph frameworks.
//!
//! 🚚 Relocated verbatim from `🧰️framework/🔨️modules/🧮️math/🕸️graph/🗣️dsl` in ticket 26/08/12/
//! DISSOLVE-KERNELS-AND-MODULES-INTO-EVENT-SOURCED-ARTIFACTS wave MATHEND — see that wave's report
//! for why the framework/plugin split hypothesis was measured and rejected (real coupling between
//! `DslIdiom` and the language-service surface). `dsl_core` (crate-root alias for
//! `semio_framework_os_kernel`, renamed from `dsl` by this same wave to free that name for this
//! module) is this file's own `os_dsl`/`dsl`-derive dependency.

// #region ⚠️ Errors
/// 🚧️ Unified failure mode for jack parsing/execution, wire-literal parsing, and fixture ingestion.
#[derive(Debug)]
pub enum GraphDslError {
    /// 🧾️ Fixture or query-result JSON failed to parse or serialize.
    Json(semio_framework_pack_json::JsonError),
    /// 🔤️ A string literal was never closed (Jack's own dual-quote pre-scan, `dsl_core` only
    /// natively lexes `"..."`).
    UnterminatedString,
    /// ❓️ A byte outside the token grammar was found (Jack's own pre-scan for `'`/`"`/`!=`, ahead
    /// of delegating the rest of the alphabet to `os_dsl::lex`).
    UnexpectedChar(char),
    /// 🔢️ A numeric literal did not parse as a float (defensive — `os_dsl::lex` only ever
    /// accumulates well-formed digit runs, so this should be unreachable in practice).
    NumberFormat(std::num::ParseFloatError),
    /// ➡️ Parser expected one token shape and found another.
    UnexpectedToken { expected: String, found: String },
    /// 🪝️ A wire-literal edge was missing a mandatory `@port` on one of its endpoints (this
    /// module's own DAG domain rule, enforced on top of the unified wire grammar).
    EdgeTargetMissingPort,
    /// 🕸️ A jack pattern had no nodes.
    EmptyPattern,
    /// 🚫️ CREATE/DELETE/SET/MERGE are not supported on read-only queryable graphs.
    UnsupportedMutation,
    /// 📞️ A `CALL` named a procedure outside this module's small owned registry (see
    /// [`call_procedure`]) — full arbitrary-procedure dispatch is deferred to unifying
    /// semio_compose_rs's Architect query language onto Jack (Wave 2 / P9).
    UnknownProcedure(String),
    /// 🔢️ A `CALL` supplied the wrong number of positional arguments for the named procedure.
    ProcedureArity { name: String, expected: usize, found: usize },
    /// 🪪️ A supplied manifest must match the snapshot's declared identity.
    ManifestIdentity { expected: String, actual: String },
    /// 🔡️ A lexical/grammar error surfaced verbatim by the unified `dsl_core`/`dsl_schema` engine —
    /// used by both the wire-literal delegate (`dsl_core::parse_wire_text`) and Jack's
    /// `dsl_core`-backed lexer.
    Lex(semio_framework_diagnostic::TextError),
}

impl std::fmt::Display for GraphDslError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Json(error) => write!(formatter, "invalid json: {error}"),
            Self::UnterminatedString => formatter.write_str("unterminated string literal"),
            Self::UnexpectedChar(character) => write!(formatter, "unexpected character '{character}'"),
            Self::NumberFormat(error) => write!(formatter, "invalid number literal: {error}"),
            Self::UnexpectedToken { expected, found } => write!(formatter, "expected {expected}, got {found}"),
            Self::EdgeTargetMissingPort => formatter.write_str("edge target requires @port"),
            Self::EmptyPattern => formatter.write_str("empty pattern"),
            Self::UnsupportedMutation => formatter.write_str("mutating jack clauses are not supported on this graph domain"),
            Self::UnknownProcedure(name) => write!(formatter, "unknown CALL procedure '{name}'"),
            Self::ProcedureArity { name, expected, found } => write!(formatter, "procedure '{name}' expects {expected} argument(s), got {found}"),
            Self::ManifestIdentity { expected, actual } => write!(formatter, "snapshot manifest '{actual}' does not match supplied manifest '{expected}'"),
            Self::Lex(error) => error.fmt(formatter),
        }
    }
}

impl std::error::Error for GraphDslError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Json(error) => Some(error),
            Self::NumberFormat(error) => Some(error),
            Self::Lex(error) => Some(error),
            _ => None,
        }
    }
}

impl From<semio_framework_pack_json::JsonError> for GraphDslError {
    fn from(error: semio_framework_pack_json::JsonError) -> Self {
        Self::Json(error)
    }
}

impl From<std::num::ParseFloatError> for GraphDslError {
    fn from(error: std::num::ParseFloatError) -> Self {
        Self::NumberFormat(error)
    }
}

impl From<semio_framework_diagnostic::TextError> for GraphDslError {
    fn from(error: semio_framework_diagnostic::TextError) -> Self {
        Self::Lex(error)
    }
}
// #endregion ⚠️ Errors

pub mod queryable {
    // #region queryable
    //! 🔍️ Queryable graph interface for Jack.

    use crate::dsl::GraphDslError;
    use crate::manifest::{GraphManifest, PropertyBag, PropertyValue};
    use semio_framework_pack_json::Value;
    use std::collections::{BTreeMap, BTreeSet};

    // #region 🔖️QueryableEdge
    /// 🪢️ Edge row exposed to Jack matching.
    #[derive(Clone, Debug, PartialEq)]
    pub struct QueryableEdge {
        pub id: String,
        pub kind: String,
        pub source_node_id: String,
        pub target_node_id: String,
        pub source_port: Option<String>,
        pub target_port: Option<String>,
        pub properties: PropertyBag,
    }
    // #endregion 🔖️QueryableEdge

    // #region 🔖️QueryableGraph
    /// 🕸️ Read-only graph surface for Jack query execution.
    pub trait QueryableGraph {
        fn manifest(&self) -> Option<&GraphManifest>;
        fn node_ids(&self) -> Vec<String>;
        fn node_kind(&self, id: &str) -> Option<String>;
        fn node_name(&self, id: &str) -> Option<String>;
        fn node_property(&self, id: &str, key: &str) -> Option<PropertyValue>;
        fn edges(&self) -> Vec<QueryableEdge>;
        fn subgraph_fixture_json(&self, node_ids: &BTreeSet<String>, edge_ids: &BTreeSet<String>) -> Option<String>;
    }

    pub fn manifest_node_kinds<G: QueryableGraph>(graph: &G) -> Vec<String> {
        let mut kinds = BTreeSet::new();
        for id in graph.node_ids() {
            if let Some(kind) = graph.node_kind(id.as_str()) {
                kinds.insert(kind);
            }
        }
        if let Some(manifest) = graph.manifest() {
            for def in &manifest.node_kinds {
                kinds.insert(def.id.clone());
            }
        }
        kinds.into_iter().collect()
    }

    pub fn manifest_edge_kinds<G: QueryableGraph>(graph: &G) -> Vec<String> {
        let mut kinds = BTreeSet::new();
        for edge in graph.edges() {
            kinds.insert(edge.kind.clone());
        }
        if let Some(manifest) = graph.manifest() {
            for def in &manifest.edge_kinds {
                kinds.insert(def.id.clone());
            }
        }
        kinds.into_iter().collect()
    }

    pub fn manifest_property_names<G: QueryableGraph>(graph: &G) -> Vec<String> {
        let mut props = BTreeSet::from(["id".to_string(), "name".to_string(), "kind".to_string()]);
        for id in graph.node_ids() {
            for key in ["label", "text"] {
                if graph.node_property(id.as_str(), key).is_some() {
                    props.insert(key.to_string());
                }
            }
            if let Some(PropertyValue::Object(map)) = graph.node_property(id.as_str(), "__all") {
                for key in map.keys() {
                    props.insert(key.clone());
                }
            }
        }
        props.into_iter().collect()
    }

    pub fn manifest_port_kinds<G: QueryableGraph>(graph: &G) -> Vec<String> {
        let mut kinds = BTreeSet::new();
        for edge in graph.edges() {
            if let Some(port) = &edge.source_port {
                kinds.insert(port.clone());
            }
            if let Some(port) = &edge.target_port {
                kinds.insert(port.clone());
            }
        }
        if let Some(manifest) = graph.manifest() {
            for def in &manifest.port_kinds {
                kinds.insert(def.id.clone());
            }
        }
        kinds.into_iter().collect()
    }
    // #endregion 🔖️QueryableGraph

    // #region 🔖️BoardQueryableGraph
    fn json_to_property_bag(value: &Value) -> PropertyBag {
        let dsl_core::DslValue::Object(entries) = semio_framework_pack_json::to_dsl_value(value) else {
            return PropertyBag::default();
        };
        entries.into_iter().filter_map(|(k, v)| dsl_core::FromValue::from_value(v).ok().map(|pv| (k, pv))).collect()
    }

    fn split_endpoint(endpoint: &str, handle_to_node: &BTreeMap<String, String>) -> (String, Option<String>) {
        if let Some(node_id) = handle_to_node.get(endpoint) {
            return (node_id.clone(), None);
        }
        if let Some((node, port)) = endpoint.split_once('@') {
            let node_id = handle_to_node.get(node).cloned().unwrap_or_else(|| node.to_string());
            return (node_id, Some(port.to_string()));
        }
        if let Some((node, port)) = endpoint.rsplit_once(':') {
            let node_id = handle_to_node.get(node).cloned().unwrap_or_else(|| node.to_string());
            return (node_id, Some(port.to_string()));
        }
        if let Some((node, port)) = endpoint.rsplit_once('.') {
            if handle_to_node.contains_key(endpoint) {
                return (handle_to_node[endpoint].clone(), None);
            }
            return (node.to_string(), Some(port.to_string()));
        }
        (endpoint.to_string(), None)
    }

    /// 🧩️ Jack query target over board/scene fixture JSON.
    pub struct BoardQueryableGraph {
        manifest: Option<GraphManifest>,
        nodes: BTreeMap<String, (String, String, PropertyBag)>,
        edges: Vec<QueryableEdge>,
        raw_fixture: Value,
    }

    impl BoardQueryableGraph {
        pub fn from_host_snapshot_json(json: &str, manifest: Option<GraphManifest>) -> Result<Self, GraphDslError> {
            let raw: Value = semio_framework_pack_json::parse(json, semio_framework_pack_json::JsonMemberPolicy::Reject)?;
            if let (Some(manifest), Some(id)) = (&manifest, raw.get("manifestId").and_then(Value::as_str)) {
                if manifest.id != id {
                    return Err(GraphDslError::ManifestIdentity { expected: manifest.id.clone(), actual: id.to_string() });
                }
            }
            let mut nodes = BTreeMap::new();
            let mut handle_to_node = BTreeMap::new();
            if let Some(rows) = raw.get("nodes").and_then(|v| v.as_array()) {
                for row in rows {
                    let Some(obj) = row.as_object() else { continue };
                    let Some(id) = obj.get("id").and_then(|v| v.as_str()) else { continue };
                    let kind = obj.get("nodeKind").or_else(|| obj.get("node_kind")).or_else(|| obj.get("kind")).and_then(|v| v.as_str()).unwrap_or("").to_string();
                    let name = obj.get("text").or_else(|| obj.get("name")).or_else(|| obj.get("label")).and_then(|v| v.as_str()).unwrap_or(id).to_string();
                    let mut properties = match obj.get("userData").or_else(|| obj.get("user_data")) {
                        Some(v) => json_to_property_bag(v),
                        None => PropertyBag::default(),
                    };
                    for (key, value) in obj.iter() {
                        if matches!(key, "id" | "nodeKind" | "node_kind" | "kind" | "text" | "name" | "label" | "handles" | "x" | "y" | "shape" | "radius" | "width" | "height" | "userData" | "user_data") {
                            continue;
                        }
                        if let Ok(prop) = <PropertyValue as dsl_core::FromValue>::from_value(semio_framework_pack_json::to_dsl_value(value)) {
                            properties.insert(key.to_string(), prop);
                        }
                    }
                    nodes.insert(id.to_string(), (kind, name, properties));
                    if let Some(handles) = obj.get("handles").and_then(|v| v.as_array()) {
                        for handle in handles {
                            if let Some(hid) = handle.get("id").and_then(|v| v.as_str()) {
                                handle_to_node.insert(hid.to_string(), id.to_string());
                            }
                        }
                    }
                }
            }
            let mut edges = Vec::new();
            if let Some(rows) = raw.get("edges").and_then(|v| v.as_array()) {
                for row in rows {
                    let Some(obj) = row.as_object() else { continue };
                    let Some(id) = obj.get("id").and_then(|v| v.as_str()) else { continue };
                    let Some(source) = obj.get("source").and_then(|v| v.as_str()) else { continue };
                    let Some(target) = obj.get("target").and_then(|v| v.as_str()) else { continue };
                    let kind = obj.get("edgeKind").or_else(|| obj.get("edge_kind")).or_else(|| obj.get("kind")).and_then(|v| v.as_str()).unwrap_or("").to_string();
                    let properties = match obj.get("userData").or_else(|| obj.get("user_data")) {
                        Some(v) => json_to_property_bag(v),
                        None => PropertyBag::default(),
                    };
                    let (source_node_id, source_port) = split_endpoint(source, &handle_to_node);
                    let (target_node_id, target_port) = split_endpoint(target, &handle_to_node);
                    edges.push(QueryableEdge { id: id.to_string(), kind, source_node_id, target_node_id, source_port, target_port, properties });
                }
            }
            Ok(Self { manifest, nodes, edges, raw_fixture: raw })
        }

        pub fn from_object_snapshot_json(json: &str, manifest: Option<GraphManifest>) -> Result<Self, GraphDslError> {
            let raw: Value = semio_framework_pack_json::parse(json, semio_framework_pack_json::JsonMemberPolicy::Reject)?;
            let mut fixture = raw.clone();
            if fixture.get("nodes").and_then(|v| v.as_array()).is_none() {
                if let Some(objects) = raw.get("objects").and_then(|v| v.as_array()) {
                    let nodes: Vec<Value> = objects
                        .iter()
                        .filter_map(|row| {
                            let obj = row.as_object()?;
                            let id = obj.get("id").and_then(|v| v.as_str())?;
                            let kind = obj.get("objectKind").or_else(|| obj.get("kind")).and_then(|v| v.as_str()).unwrap_or("Object");
                            let name = obj.get("name").or_else(|| obj.get("label")).and_then(|v| v.as_str()).unwrap_or(id);
                            Some(Value::Object(semio_framework_pack_json::Object::from_iter([("id".to_string(), Value::from(id)), ("nodeKind".to_string(), Value::from(kind)), ("text".to_string(), Value::from(name))])))
                        })
                        .collect();
                    if let Some(object) = fixture.as_object_mut() {
                        object.insert("nodes", Value::Array(nodes));
                    }
                }
            }
            Self::from_host_snapshot_json(&semio_framework_pack_json::to_string(&fixture), manifest)
        }


    }

    impl QueryableGraph for BoardQueryableGraph {
        fn manifest(&self) -> Option<&GraphManifest> {
            self.manifest.as_ref()
        }

        fn node_ids(&self) -> Vec<String> {
            self.nodes.keys().cloned().collect()
        }

        fn node_kind(&self, id: &str) -> Option<String> {
            self.nodes.get(id).map(|(kind, _, _)| kind.clone())
        }

        fn node_name(&self, id: &str) -> Option<String> {
            self.nodes.get(id).map(|(_, name, _)| name.clone())
        }

        fn node_property(&self, id: &str, key: &str) -> Option<PropertyValue> {
            let (_, name, properties) = self.nodes.get(id)?;
            match key {
                "id" => Some(PropertyValue::String(id.to_string())),
                "name" | "label" | "text" => Some(PropertyValue::String(name.clone())),
                "kind" => self.node_kind(id).map(PropertyValue::String),
                "__all" => Some(PropertyValue::Object(properties.clone())),
                _ => properties.get(key).cloned(),
            }
        }

        fn edges(&self) -> Vec<QueryableEdge> {
            self.edges.clone()
        }

        fn subgraph_fixture_json(&self, node_ids: &BTreeSet<String>, edge_ids: &BTreeSet<String>) -> Option<String> {
            let mut fixture = self.raw_fixture.clone();
            if let Some(nodes) = fixture.get_mut("nodes").and_then(|v| v.as_array_mut()) {
                nodes.retain(|row| row.get("id").and_then(|v| v.as_str()).is_some_and(|id| node_ids.contains(id)));
            }
            if let Some(edges) = fixture.get_mut("edges").and_then(|v| v.as_array_mut()) {
                edges.retain(|row| row.get("id").and_then(|v| v.as_str()).is_some_and(|id| edge_ids.contains(id)));
            }
            Some(semio_framework_pack_json::to_string(&fixture))
        }
    }
    // #endregion 🔖️BoardQueryableGraph
    // #endregion queryable
}

pub mod wire {
    // #region wire
    //! 🔌️ Wire-literal compiled DAG text notation — delegates all lexing/parsing/printing to
    //! `dsl_schema`'s unified `Shape::Wire` grammar (`->`/`<-`/`--`, `{k=v}` double-quoted
    //! properties), keeping only this module's own public row types (`WireNode`/`WireEdge`) and
    //! its domain-specific rule that an edge's ports are mandatory on both ends — a validation
    //! layer on top of the shared parse, not a syntax difference. ~8 downstream crates depend on
    //! these exact type/function signatures, unchanged by this unification.

    use crate::dsl::GraphDslError;
    use crate::manifest::{PropertyBag, PropertyValue};

    // #region 🔖️WireTypes
    /// 🧩️ Neutral node row for wire-literal emission.
    #[derive(Clone, Debug, PartialEq)]
    pub struct WireNode {
        pub id: String,
        pub kind: String,
        pub port: Option<String>,
        pub properties: PropertyBag,
    }

    /// 🪢️ Neutral edge row for wire-literal emission.
    #[derive(Clone, Debug, PartialEq)]
    pub struct WireEdge {
        pub from: String,
        pub from_port: String,
        pub to: String,
        pub to_port: String,
        pub directed: bool,
        pub properties: PropertyBag,
    }
    // #endregion 🔖️WireTypes

    // #region 🔖️PropertyBridge
    /// 🌉️ `crate::manifest::PropertyValue` <-> `dsl_core::DslValue` — the two crates'
    /// dynamic-JSON-equivalent literal types are structurally identical, so this is a pure reshape.
    fn dsl_value_from_property_value(value: &PropertyValue) -> dsl_core::DslValue {
        match value {
            PropertyValue::Null => dsl_core::DslValue::Null,
            PropertyValue::Bool(b) => dsl_core::DslValue::Bool(*b),
            PropertyValue::Number(n) => dsl_core::DslValue::float(*n),
            PropertyValue::String(s) => dsl_core::DslValue::String(s.clone()),
            PropertyValue::Array(items) => {
                let mut out = Vec::with_capacity(items.len());
                for item in items {
                    out.push(dsl_value_from_property_value(item));
                }
                dsl_core::DslValue::Array(out)
            }
            PropertyValue::Object(map) => {
                let mut out = Vec::with_capacity(map.len());
                for (k, v) in map {
                    out.push((k.clone(), dsl_value_from_property_value(v)));
                }
                dsl_core::DslValue::Object(out)
            }
        }
    }

    fn property_value_from_dsl_value(value: &dsl_core::DslValue) -> PropertyValue {
        match value {
            dsl_core::DslValue::Null => PropertyValue::Null,
            dsl_core::DslValue::Bool(b) => PropertyValue::Bool(*b),
            dsl_core::DslValue::Number(n) => PropertyValue::Number(n.as_f64()),
            dsl_core::DslValue::String(s) => PropertyValue::String(s.clone()),
            dsl_core::DslValue::Bytes(bytes) => PropertyValue::Array(bytes.iter().map(|byte| PropertyValue::Number(f64::from(*byte))).collect()),
            dsl_core::DslValue::Array(items) => {
                let mut out = Vec::with_capacity(items.len());
                for item in items {
                    out.push(property_value_from_dsl_value(item));
                }
                PropertyValue::Array(out)
            }
            dsl_core::DslValue::Object(entries) => {
                let mut out = PropertyBag::new();
                for (k, v) in entries {
                    out.insert(k.clone(), property_value_from_dsl_value(v));
                }
                PropertyValue::Object(out)
            }
        }
    }

    fn properties_to_dsl_object(properties: &PropertyBag) -> dsl_core::DslValue {
        let mut out = Vec::with_capacity(properties.len());
        for (k, v) in properties {
            out.push((k.clone(), dsl_value_from_property_value(v)));
        }
        dsl_core::DslValue::Object(out)
    }

    fn properties_from_dsl_value(value: &dsl_core::DslValue) -> PropertyBag {
        match value {
            dsl_core::DslValue::Object(entries) => {
                let mut out = PropertyBag::new();
                for (k, v) in entries {
                    out.insert(k.clone(), property_value_from_dsl_value(v));
                }
                out
            }
            _ => PropertyBag::new(),
        }
    }
    // #endregion 🔖️PropertyBridge

    // #region 🔖️WireLiteral
    fn render_wire_line(value: &dsl_core::WireValue) -> String {
        // 🚨️ `dsl_core::Writer::new`/`print_shape`/`Writer::render` are all sync in `dsl_core` —
        // none of them suspend, so no `.await` belongs on any of them.
        let mut writer = dsl_core::Writer::new();
        dsl_core::print_shape(&dsl_core::FieldValue::Wire(value.clone()), &dsl_core::Shape::Wire, &mut writer);
        writer.render(dsl_core::JoinMode::Inline)
    }

    /// 📝️ Render wire-literal text from neutral node/edge rows, one unified `dsl_core::Wire`
    /// statement per line.
    pub fn wire_literal_from_dag(nodes: &[WireNode], edges: &[WireEdge]) -> String {
        let mut lines = Vec::new();
        for node in nodes {
            let value = dsl_core::WireValue {
                from: dsl_core::WireNode { id: node.id.clone(), kind: Some(node.kind.clone()), port: node.port.clone() },
                edge: None,
                edge_label: dsl_core::WireEdgeLabel::default(),
                properties: properties_to_dsl_object(&node.properties),
            };
            lines.push(render_wire_line(&value));
        }
        for edge in edges {
            let from_kind = nodes.iter().find(|n| n.id == edge.from).map_or("node", |n| n.kind.as_str());
            let to_kind = nodes.iter().find(|n| n.id == edge.to).map_or("node", |n| n.kind.as_str());
            let value = dsl_core::WireValue {
                from: dsl_core::WireNode { id: edge.from.clone(), kind: Some(from_kind.to_string()), port: Some(edge.from_port.clone()) },
                edge: Some((edge.directed, dsl_core::WireNode { id: edge.to.clone(), kind: Some(to_kind.to_string()), port: Some(edge.to_port.clone()) })),
                edge_label: dsl_core::WireEdgeLabel::default(),
                properties: properties_to_dsl_object(&edge.properties),
            };
            lines.push(render_wire_line(&value));
        }
        lines.join("\n")
    }

    /// 🔍️ Parse wire-literal text into neutral node/edge rows. Delegates lexing+parsing to
    /// `dsl_core::parse_wire_text` (the one unified wire grammar — `->`/`<-` sugar/`--`,
    /// `{k=v}` double-quoted properties) one statement (line) at a time, then enforces this
    /// module's own DAG domain rule on top: an edge's ports are mandatory on BOTH ends (the
    /// shared grammar itself leaves ports optional on every endpoint — that's the engine's
    /// business, not a syntax difference this module should encode into the lexer/parser).
    pub fn dag_from_wire_literal(text: &str) -> Result<(Vec<WireNode>, Vec<WireEdge>), GraphDslError> {
        let mut nodes = Vec::new();
        let mut edges = Vec::new();
        for line in text.lines() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }
            let value = dsl_core::parse_wire_text(line)?;
            match value.edge {
                None => nodes.push(WireNode { id: value.from.id, kind: value.from.kind.unwrap_or_else(|| "node".to_string()), port: value.from.port, properties: properties_from_dsl_value(&value.properties) }),
                Some((directed, to)) => {
                    let from_port = value.from.port.ok_or(GraphDslError::EdgeTargetMissingPort)?;
                    let to_port = to.port.ok_or(GraphDslError::EdgeTargetMissingPort)?;
                    edges.push(WireEdge { from: value.from.id, from_port, to: to.id, to_port, directed, properties: properties_from_dsl_value(&value.properties) });
                }
            }
        }
        Ok((nodes, edges))
    }
    // #endregion 🔖️WireLiteral

    // #region 🔖️Tests
    #[cfg(test)]
    include!("🧪️tests/🔬️wire-unit/🦀️.rs");
    // #endregion 🔖️Tests
    // #endregion wire
}

pub use queryable::{manifest_edge_kinds, manifest_node_kinds, manifest_port_kinds, manifest_property_names, BoardQueryableGraph, QueryableEdge, QueryableGraph};
pub use wire::{dag_from_wire_literal, wire_literal_from_dag, WireEdge, WireNode};

use crate::manifest::PropertyValue;
use std::collections::{BTreeMap, BTreeSet};

// #region jack_impl

// #region 🔖️Ast
/// 🌳️ Jack query abstract syntax tree.
#[derive(Clone, Debug, PartialEq)]
pub struct Query {
    pub clauses: Vec<Clause>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Clause {
    Match(Vec<Pattern>),
    Where(Expr),
    /// 🔭️ `WITH <items>` — projects the current bindings down to just the named vars (dropping
    /// everything else out of scope), no more than [`ReturnItem`] itself models: no aliasing, no
    /// `DISTINCT`, no `ORDER BY`/`SKIP`/`LIMIT`. A trailing filter is just the next
    /// [`Clause::Where`] in sequence — it needs no special-casing here.
    With(Vec<ReturnItem>),
    /// 🌀️ `UNWIND <source> AS <var>` — see [`UnwindClause`].
    Unwind(UnwindClause),
    /// 📞️ `CALL <name>(<args>...)` — see [`CallClause`] and [`call_procedure`].
    Call(CallClause),
    Return(Vec<ReturnItem>),
    Create(Pattern),
    Delete(Vec<String>),
    Set(Vec<Assignment>),
    Merge(Pattern),
}

/// 🌀️ `UNWIND <source> AS <var>` — flattens a list-valued source into per-row bindings of `var`.
#[derive(Clone, Debug, PartialEq)]
pub struct UnwindClause {
    pub source: ReturnItem,
    pub var: String,
}

/// 📞️ `CALL <name>(<args>...)` — a named procedure invocation with positional scalar arguments.
#[derive(Clone, Debug, PartialEq)]
pub struct CallClause {
    pub name: String,
    pub args: Vec<PropertyValue>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Pattern {
    pub nodes: Vec<PatternNode>,
    pub edge: Option<PatternEdge>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PatternNode {
    pub var: String,
    pub kind: String,
    pub port: Option<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PatternEdge {
    pub var: Option<String>,
    pub kind: Option<String>,
    pub directed: bool,
    pub right: PatternNode,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ReturnItem {
    Var(String),
    Property { var: String, prop: String },
}

#[derive(Clone, Debug, PartialEq)]
pub struct Assignment {
    pub var: String,
    pub prop: String,
    pub value: PropertyValue,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
    Eq { var: String, prop: String, value: PropertyValue },
    Ne { var: String, prop: String, value: PropertyValue },
    And(Box<Expr>, Box<Expr>),
    Or(Box<Expr>, Box<Expr>),
}

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub enum QueryResultKind {
    #[default]
    Table,
    Graph,
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct QueryResult {
    #[value(default)]
    pub kind: QueryResultKind,
    pub columns: Vec<String>,
    pub rows: Vec<Vec<PropertyValue>>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub graph_fixture_json: Option<String>,
}

impl QueryResult {
    pub fn table(columns: Vec<String>, rows: Vec<Vec<PropertyValue>>) -> Self {
        Self { kind: QueryResultKind::Table, columns, rows, graph_fixture_json: None }
    }

    pub fn graph(columns: Vec<String>, graph_fixture_json: String) -> Self {
        Self { kind: QueryResultKind::Graph, columns, rows: vec![], graph_fixture_json: Some(graph_fixture_json) }
    }
}
// #endregion 🔖️Ast

// #region 🔖️Lexer
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub enum TokenClass {
    Keyword,
    Ident,
    Number,
    String,
    Operator,
    Punctuation,
    Error,
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct TokenSpan {
    pub class: TokenClass,
    pub start: usize,
    pub end: usize,
}

#[derive(Clone, Debug, PartialEq)]
enum Token {
    KwMatch,
    KwWhere,
    KwReturn,
    KwCreate,
    KwDelete,
    KwSet,
    KwMerge,
    KwWith,
    KwUnwind,
    KwCall,
    KwAs,
    Ident(String),
    Number(f64),
    StringLit(String),
    LParen,
    RParen,
    LBracket,
    RBracket,
    Colon,
    Comma,
    Dot,
    Eq,
    Ne,
    Arrow,
    /// ➖️ A bare `-`, the Cypher spelling of the pattern connector in `(a)-[r]->(b)`. `DashArrow`
    /// (`--`) is this grammar's own equal-status spelling; both reach the same `PatternEdge`.
    Dash,
    DashArrow,
    BackArrow,
    At,
    And,
    Or,
    Eof,
}

#[derive(Clone, Debug, PartialEq)]
struct SpannedToken {
    token: Token,
    start: usize,
    end: usize,
}

fn token_class(token: &Token) -> TokenClass {
    match token {
        Token::KwMatch | Token::KwWhere | Token::KwReturn | Token::KwCreate | Token::KwDelete | Token::KwSet | Token::KwMerge | Token::KwWith | Token::KwUnwind | Token::KwCall | Token::KwAs | Token::And | Token::Or => TokenClass::Keyword,
        Token::Ident(_) => TokenClass::Ident,
        Token::Number(_) => TokenClass::Number,
        Token::StringLit(_) => TokenClass::String,
        Token::Eq | Token::Ne | Token::Arrow | Token::Dash | Token::DashArrow | Token::BackArrow | Token::At => TokenClass::Operator,
        Token::LParen | Token::RParen | Token::LBracket | Token::RBracket | Token::Colon | Token::Comma | Token::Dot => TokenClass::Punctuation,
        Token::Eof => TokenClass::Punctuation,
    }
}

fn push_spanned(tokens: &mut Vec<SpannedToken>, token: Token, start: usize, end: usize) {
    tokens.push(SpannedToken { token, start, end });
}

/// 🔑️ Uppercases and matches against Jack's clause/logic keyword table; anything else stays a
/// plain variable/property/kind identifier. Case-insensitive (Cypher heritage — `match`, `Match`,
/// `MATCH` are all the same token), unlike `dsl_core`'s own grammars which are case-sensitive.
fn keyword_or_ident(text: String) -> Token {
    match text.to_ascii_uppercase().as_str() {
        "MATCH" => Token::KwMatch,
        "WHERE" => Token::KwWhere,
        "RETURN" => Token::KwReturn,
        "CREATE" => Token::KwCreate,
        "DELETE" => Token::KwDelete,
        "SET" => Token::KwSet,
        "MERGE" => Token::KwMerge,
        "WITH" => Token::KwWith,
        "UNWIND" => Token::KwUnwind,
        "CALL" => Token::KwCall,
        "AS" => Token::KwAs,
        "AND" => Token::And,
        "OR" => Token::Or,
        _ => Token::Ident(text),
    }
}

/// 🪚️ `dsl_core` treats `.` as ident-continue (so `a.name` lexes as ONE ident there), but Jack's
/// `var.prop` property-access grammar needs `.` as its own token — splits it back apart here,
/// checking each piece against the keyword table too (defensive; keywords never legitimately
/// contain a dot, but this keeps the one keyword-recognition path authoritative).
fn push_ident_or_keyword_with_dots(text: &str, start: usize, out: &mut Vec<SpannedToken>) {
    let mut offset = 0usize;
    for (idx, part) in text.split('.').enumerate() {
        if idx > 0 {
            push_spanned(out, Token::Dot, start + offset, start + offset + 1);
            offset += 1;
        }
        if !part.is_empty() {
            let end = start + offset + part.len();
            push_spanned(out, keyword_or_ident(part.to_string()), start + offset, end);
        }
        offset += part.len();
    }
}

/// 🔬️ Converts one already-lexed `dsl_core` segment (containing no quotes or `!=` — those are
/// scanned by [`lex_spanned`] itself, ahead of delegating everything else) into Jack's own
/// richer, grammar-aware token stream.
fn push_dsl_core_segment(segment: &str, base_offset: usize, forgiving: bool, out: &mut Vec<SpannedToken>) -> Result<(), GraphDslError> {
    if segment.is_empty() {
        return Ok(());
    }
    let raw = semio_framework_dsl::lex(segment, &semio_framework_diagnostic::Limits::default(), forgiving).map_err(GraphDslError::Lex)?;
    for token in raw {
        if token.kind.is_trivia() || token.kind == semio_framework_dsl::TokenKind::Eof {
            continue;
        }
        let start = base_offset + token.byte_range.0 as usize;
        let end = base_offset + token.byte_range.1 as usize;
        let text = token.text.as_str().to_string();
        match token.kind {
            semio_framework_dsl::TokenKind::Ident => push_ident_or_keyword_with_dots(&text, start, out),
            // A lone `_` is `dsl_core`'s placeholder sigil; Jack has no placeholder concept of its
            // own, so it round-trips as an ordinary one-character identifier.
            semio_framework_dsl::TokenKind::Placeholder => push_spanned(out, Token::Ident(text), start, end),
            semio_framework_dsl::TokenKind::Int | semio_framework_dsl::TokenKind::Float => {
                let n: f64 = text.parse().map_err(GraphDslError::NumberFormat)?;
                push_spanned(out, Token::Number(n), start, end);
            }
            semio_framework_dsl::TokenKind::LParen => push_spanned(out, Token::LParen, start, end),
            semio_framework_dsl::TokenKind::RParen => push_spanned(out, Token::RParen, start, end),
            semio_framework_dsl::TokenKind::LBracket => push_spanned(out, Token::LBracket, start, end),
            semio_framework_dsl::TokenKind::RBracket => push_spanned(out, Token::RBracket, start, end),
            semio_framework_dsl::TokenKind::Colon => push_spanned(out, Token::Colon, start, end),
            semio_framework_dsl::TokenKind::Comma => push_spanned(out, Token::Comma, start, end),
            semio_framework_dsl::TokenKind::Equals => push_spanned(out, Token::Eq, start, end),
            semio_framework_dsl::TokenKind::At => push_spanned(out, Token::At, start, end),
            semio_framework_dsl::TokenKind::Arrow => push_spanned(out, Token::Arrow, start, end),
            // ➖️ A bare `-` is the pattern connector every Cypher-shaped query writes
            // (`(a)-[r:Kind]->(b)`), which is also what this dialect's own executor parser accepts
            // and what every committed example query in the repo uses. It used to fall into the
            // stray-character bucket below and every such query was reported
            // `unexpected character '-'` by lint/complete/hover/format while running perfectly.
            semio_framework_dsl::TokenKind::Minus => push_spanned(out, Token::Dash, start, end),
            semio_framework_dsl::TokenKind::DashArrow => push_spanned(out, Token::DashArrow, start, end),
            semio_framework_dsl::TokenKind::BackArrow => push_spanned(out, Token::BackArrow, start, end),
            // Double-quoted text delegated straight through `dsl_core` — unreachable in practice
            // since `lex_spanned` pre-scans and consumes every quote itself before ever
            // delegating a segment, kept only for defensive completeness.
            semio_framework_dsl::TokenKind::Text => push_spanned(out, Token::StringLit(text), start, end),
            // `{`/`}` aren't part of Jack's grammar (no map/object literals) — same "stray
            // character" treatment as an outright `os_dsl::TokenKind::Error` below. P2-M1's
            // promoted `< > & $ ;` tokens and STEP's `DotEnum` literal join this bucket too —
            // Jack has no grammar concept for any of them either.
            semio_framework_dsl::TokenKind::EdgeArrow
            | semio_framework_dsl::TokenKind::LBrace
            | semio_framework_dsl::TokenKind::RBrace
            | semio_framework_dsl::TokenKind::Caret
            | semio_framework_dsl::TokenKind::DotDot
            | semio_framework_dsl::TokenKind::Plus
            | semio_framework_dsl::TokenKind::Star
            | semio_framework_dsl::TokenKind::Slash
            | semio_framework_dsl::TokenKind::Fence
            | semio_framework_dsl::TokenKind::Lt
            | semio_framework_dsl::TokenKind::Gt
            | semio_framework_dsl::TokenKind::Amp
            | semio_framework_dsl::TokenKind::Dollar
            | semio_framework_dsl::TokenKind::Semicolon
            | semio_framework_dsl::TokenKind::DotEnum
            | semio_framework_dsl::TokenKind::Error => {
                if forgiving {
                    push_spanned(out, Token::Ident(text), start, end);
                } else {
                    return Err(GraphDslError::UnexpectedChar(text.chars().next().unwrap_or('?')));
                }
            }
            semio_framework_dsl::TokenKind::Whitespace | semio_framework_dsl::TokenKind::Newline | semio_framework_dsl::TokenKind::Comment | semio_framework_dsl::TokenKind::Eof => {
                unreachable!("trivia/Eof filtered above")
            }
        }
    }
    Ok(())
}

/// 🔬️ Jack's own lexer: unifies on `os_dsl::lex` for the shared token alphabet (idents,
/// numbers, punctuation, `(`/`)`/`[`/`]`, `->`/`--`/`<-`) but keeps two genuinely Cypher-specific
/// pieces local, since neither fits `dsl_core`'s grammar-independent alphabet: dual-quote strings
/// (`'x'`/`"x"` — Cypher heritage; `dsl_core` only ever lexes `"..."`) and the `!=` comparison
/// operator (`dsl_core` has no relational operators at all — it's a structural DSL alphabet, not
/// an expression language). Both are pre-scanned as their own tokens; every remaining run of
/// characters is delegated whole to `os_dsl::lex` and converted via [`push_dsl_core_segment`].
fn lex_spanned(input: &str, forgiving: bool) -> Result<Vec<SpannedToken>, GraphDslError> {
    let bytes = input.as_bytes();
    let mut tokens = Vec::new();
    let mut i = 0usize;
    let mut seg_start = 0usize;

    while i < bytes.len() {
        let c = bytes[i];
        if c == b'\'' || c == b'"' {
            push_dsl_core_segment(&input[seg_start..i], seg_start, forgiving, &mut tokens)?;
            let quote = c;
            let start = i;
            i += 1;
            let content_start = i;
            let mut closed = false;
            while i < bytes.len() {
                if bytes[i] == b'\\' && i + 1 < bytes.len() {
                    i += 2;
                    continue;
                }
                if bytes[i] == quote {
                    closed = true;
                    break;
                }
                i += 1;
            }
            let raw = String::from_utf8_lossy(&bytes[content_start..i]).into_owned();
            if !closed {
                if forgiving {
                    push_spanned(&mut tokens, Token::StringLit(raw), start, i);
                    seg_start = i;
                    break;
                }
                return Err(GraphDslError::UnterminatedString);
            }
            i += 1;
            let text = semio_framework_dsl::unescape_text(&raw, forgiving).unwrap_or(raw);
            push_spanned(&mut tokens, Token::StringLit(text), start, i);
            seg_start = i;
            continue;
        }
        if c == b'!' && i + 1 < bytes.len() && bytes[i + 1] == b'=' {
            push_dsl_core_segment(&input[seg_start..i], seg_start, forgiving, &mut tokens)?;
            push_spanned(&mut tokens, Token::Ne, i, i + 2);
            i += 2;
            seg_start = i;
            continue;
        }
        i += 1;
    }
    push_dsl_core_segment(&input[seg_start..bytes.len()], seg_start, forgiving, &mut tokens)?;
    push_spanned(&mut tokens, Token::Eof, input.len(), input.len());
    Ok(tokens)
}

fn lex(input: &str) -> Result<Vec<Token>, GraphDslError> {
    lex_spanned(input, false).map(|spanned| spanned.into_iter().map(|row| row.token).collect())
}

/// 🎨️ Tokenize jack source for editor highlighting (never fails).
pub fn tokenize(input: &str) -> Vec<TokenSpan> {
    // 🔀️ Rewritten from `.map(..)` — `token_class` is async and cannot be called inside the sync
    // closure that used to build each `TokenSpan` (R10 residue shape #1).
    let rows = lex_spanned(input, true).unwrap_or_default();
    let mut out = Vec::with_capacity(rows.len());
    for row in rows {
        if matches!(row.token, Token::Eof) {
            continue;
        }
        let mut class = token_class(&row.token);
        if matches!(row.token, Token::StringLit(_)) {
            let quote = input.as_bytes().get(row.start);
            if quote == Some(&b'\'') || quote == Some(&b'"') {
                let closed = input.as_bytes().get(row.end.saturating_sub(1)) == quote;
                if !closed {
                    class = TokenClass::Error;
                }
            }
        }
        out.push(TokenSpan { class, start: row.start, end: row.end });
    }
    out
}
// #endregion 🔖️Lexer

// #region 🔖️Language
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct Completion {
    pub label: String,
    pub kind: String,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
    pub insert: String,
}

const CLAUSE_KEYWORDS: &[&str] = &["MATCH", "WHERE", "RETURN", "CREATE", "DELETE", "SET", "MERGE", "WITH", "UNWIND", "CALL"];
const LOGIC_KEYWORDS: &[&str] = &["AND", "OR"];

fn completion_prefix(source: &str, cursor: usize) -> String {
    let cursor = cursor.min(source.len());
    let bytes = source.as_bytes();
    let mut start = cursor;
    while start > 0 {
        let c = bytes[start - 1];
        if c.is_ascii_alphanumeric() || c == b'_' {
            start -= 1;
        } else {
            break;
        }
    }
    source[start..cursor].to_string()
}

fn tokens_before_cursor(tokens: &[SpannedToken], cursor: usize) -> &[SpannedToken] {
    let mut end = tokens.len();
    for (i, row) in tokens.iter().enumerate() {
        if row.start >= cursor && !matches!(row.token, Token::Eof) {
            end = i;
            break;
        }
    }
    &tokens[..end]
}

fn after_colon_kind_context(source: &str, cursor: usize) -> Option<bool> {
    let cursor = cursor.min(source.len());
    let before = &source[..cursor];
    let colon = before.rfind(':')?;
    let after = &before[colon + 1..];
    // 🩹️ an `@` after the kind name means the cursor moved into the port segment (`kind@port`);
    // bail so `after_at_port_context` can offer port completions instead of kind completions.
    if after.chars().any(|c| c.is_whitespace() || matches!(c, '(' | ')' | '[' | ']' | ',' | '@')) {
        return None;
    }
    let left = &before[..colon];
    let bracket = left.rfind('[');
    let paren = left.rfind('(');
    let in_bracket = match (bracket, paren) {
        (Some(b), Some(p)) => b > p,
        (Some(_), None) => true,
        _ => false,
    };
    Some(in_bracket)
}

fn after_dot_property_context(source: &str, cursor: usize) -> bool {
    let cursor = cursor.min(source.len());
    let before = &source[..cursor];
    let Some(dot) = before.rfind('.') else {
        return false;
    };
    let after = &before[dot + 1..];
    !after.chars().any(|c| c.is_whitespace() || matches!(c, '(' | ')' | '[' | ']' | ',' | ':'))
}
fn open_bracket_kind(tokens: &[SpannedToken]) -> Option<char> {
    let mut paren = 0i32;
    let mut bracket = 0i32;
    for row in tokens.iter().rev() {
        match row.token {
            Token::RParen => paren += 1,
            Token::LParen if paren > 0 => paren -= 1,
            Token::LParen if paren == 0 && bracket == 0 => return Some('('),
            Token::RBracket => bracket += 1,
            Token::LBracket if bracket > 0 => bracket -= 1,
            Token::LBracket if bracket == 0 && paren == 0 => return Some('['),
            _ => {}
        }
    }
    None
}

fn collect_bound_vars(tokens: &[SpannedToken]) -> BTreeSet<String> {
    let mut vars = BTreeSet::new();
    let mut i = 0;
    while i + 2 < tokens.len() {
        if matches!(tokens[i].token, Token::LParen | Token::LBracket) {
            if let Token::Ident(var) = &tokens[i + 1].token {
                if matches!(tokens[i + 2].token, Token::Colon) {
                    vars.insert(var.clone());
                }
            }
        }
        i += 1;
    }
    vars
}

fn in_where_clause(tokens: &[SpannedToken]) -> bool {
    let mut seen_where = false;
    let mut seen_return = false;
    for row in tokens {
        match row.token {
            Token::KwWhere => seen_where = true,
            Token::KwReturn => seen_return = true,
            _ => {}
        }
    }
    seen_where && !seen_return
}

fn graph_node_kinds<G: QueryableGraph>(graph: &G) -> Vec<String> {
    manifest_node_kinds(graph)
}

fn graph_edge_kinds<G: QueryableGraph>(graph: &G) -> Vec<String> {
    manifest_edge_kinds(graph)
}

fn graph_property_names<G: QueryableGraph>(graph: &G) -> Vec<String> {
    manifest_property_names(graph)
}

fn graph_port_kinds<G: QueryableGraph>(graph: &G) -> Vec<String> {
    manifest_port_kinds(graph)
}

fn after_at_port_context(source: &str, cursor: usize) -> bool {
    let cursor = cursor.min(source.len());
    let before = &source[..cursor];
    let Some(at) = before.rfind('@') else {
        return false;
    };
    let after = &before[at + 1..];
    !after.chars().any(|c| c.is_whitespace() || matches!(c, '(' | ')' | '[' | ']' | ',' | '-' | '>' | '@'))
}

fn filter_completions(candidates: impl IntoIterator<Item = (String, String, Option<String>)>, prefix: &str) -> Vec<Completion> {
    let prefix_lower = prefix.to_ascii_lowercase();
    let mut out = Vec::new();
    for (label, kind, detail) in candidates {
        if prefix.is_empty() || label.to_ascii_lowercase().starts_with(&prefix_lower) {
            out.push(Completion { insert: label.clone(), label, kind, detail });
        }
    }
    out.sort_by(|a, b| a.label.cmp(&b.label));
    out
}

/// 🔎️ Context-aware jack completions for the editor.
pub fn complete<G: QueryableGraph>(graph: &G, source: &str, cursor: usize) -> Vec<Completion> {
    let cursor = cursor.min(source.len());
    let prefix = completion_prefix(source, cursor);
    let tokens = lex_spanned(source, true).unwrap_or_default();
    let before = tokens_before_cursor(&tokens, cursor);

    if let Some(in_bracket) = after_colon_kind_context(source, cursor) {
        let kinds = if in_bracket { graph_edge_kinds(graph).into_iter().map(|name| (name, "edgeKind".into(), None)).collect::<Vec<_>>() } else { graph_node_kinds(graph).into_iter().map(|name| (name, "nodeKind".into(), None)).collect::<Vec<_>>() };
        return filter_completions(kinds, &prefix);
    }

    if after_dot_property_context(source, cursor) {
        let props = graph_property_names(graph).into_iter().map(|name| (name, "property".into(), None)).collect::<Vec<_>>();
        return filter_completions(props, &prefix);
    }

    if after_at_port_context(source, cursor) {
        let ports = graph_port_kinds(graph).into_iter().map(|name| (name, "portKind".into(), None)).collect::<Vec<_>>();
        return filter_completions(ports, &prefix);
    }

    if let Some(last) = before.last() {
        if matches!(last.token, Token::At) {
            let ports = graph_port_kinds(graph).into_iter().map(|name| (name, "portKind".into(), None)).collect::<Vec<_>>();
            return filter_completions(ports, &prefix);
        }
        if matches!(last.token, Token::Colon) {
            let kinds = if open_bracket_kind(before) == Some('[') {
                graph_edge_kinds(graph).into_iter().map(|name| (name, "edgeKind".into(), None)).collect::<Vec<_>>()
            } else {
                graph_node_kinds(graph).into_iter().map(|name| (name, "nodeKind".into(), None)).collect::<Vec<_>>()
            };
            return filter_completions(kinds, &prefix);
        }
        if matches!(last.token, Token::Dot) {
            let props = graph_property_names(graph).into_iter().map(|name| (name, "property".into(), None)).collect::<Vec<_>>();
            return filter_completions(props, &prefix);
        }
    }

    if in_where_clause(before) {
        let logic = filter_completions(LOGIC_KEYWORDS.iter().map(|kw| (kw.to_string(), "keyword".into(), None)), &prefix);
        if !logic.is_empty() {
            return logic;
        }
    }

    let vars = collect_bound_vars(before);
    if !vars.is_empty() && prefix.chars().next().is_some_and(|c| c.is_ascii_alphabetic() || c == '_') {
        let var_items = vars.into_iter().map(|name| (name, "variable".into(), None)).collect::<Vec<_>>();
        let filtered = filter_completions(var_items, &prefix);
        if !filtered.is_empty() {
            return filtered;
        }
    }

    filter_completions(CLAUSE_KEYWORDS.iter().map(|kw| (kw.to_string(), "keyword".into(), None)), &prefix)
}
// #endregion 🔖️Language

// #region 🔖️LanguageService
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub enum DiagnosticSeverity {
    Error,
    Warning,
    Information,
    Info,
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct Diagnostic {
    pub start: usize,
    pub end: usize,
    pub severity: DiagnosticSeverity,
    pub message: String,
    // 🌉️ `default` added even though the serde attribute list only carries `skip_serializing_if`:
    // serde treats an `Option<T>` field as implicitly optional on deserialize with no explicit
    // `default`, but `#[derive(ToValue, FromValue)]` has no such special case — a genuinely omitted
    // wire key needs `default` spelled out or `FromValue` errors "missing field" instead of `None`.
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct Hover {
    pub start: usize,
    pub end: usize,
    pub contents: String,
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct SemanticToken {
    pub start: usize,
    pub end: usize,
    pub class: String,
}

fn collect_pattern_vars(pattern: &Pattern, out: &mut BTreeSet<String>) {
    for node in &pattern.nodes {
        out.insert(node.var.clone());
    }
    if let Some(edge) = &pattern.edge {
        if let Some(var) = &edge.var {
            out.insert(var.clone());
        }
        out.insert(edge.right.var.clone());
    }
}

fn collect_clause_bound_vars(clauses: &[Clause]) -> BTreeSet<String> {
    let mut vars = BTreeSet::new();
    for clause in clauses {
        match clause {
            Clause::Match(patterns) => {
                for pattern in patterns {
                    collect_pattern_vars(pattern, &mut vars);
                }
            }
            Clause::Create(pattern) | Clause::Merge(pattern) => collect_pattern_vars(pattern, &mut vars),
            _ => {}
        }
    }
    vars
}

fn collect_referenced_vars(clauses: &[Clause]) -> Vec<(String, usize, usize)> {
    let mut refs = Vec::new();
    for clause in clauses {
        match clause {
            Clause::Return(items) => {
                for item in items {
                    match item {
                        ReturnItem::Var(v) => refs.push((v.clone(), 0, v.len())),
                        ReturnItem::Property { var, .. } => refs.push((var.clone(), 0, var.len())),
                    }
                }
            }
            Clause::Delete(vars) => {
                for var in vars {
                    refs.push((var.clone(), 0, var.len()));
                }
            }
            Clause::Set(assignments) => {
                for assignment in assignments {
                    refs.push((assignment.var.clone(), 0, assignment.var.len()));
                }
            }
            Clause::Where(expr) => collect_expr_vars(expr, &mut refs),
            _ => {}
        }
    }
    refs
}

fn collect_expr_vars(expr: &Expr, refs: &mut Vec<(String, usize, usize)>) {
    match expr {
        Expr::Eq { var, .. } | Expr::Ne { var, .. } => refs.push((var.clone(), 0, var.len())),
        Expr::And(a, b) | Expr::Or(a, b) => {
            collect_expr_vars(a, refs);
            collect_expr_vars(b, refs);
        }
    }
}

fn semantic_lints<G: QueryableGraph>(graph: &G, query: &Query, source: &str) -> Vec<Diagnostic> {
    let mut out = Vec::new();
    let node_kinds = graph_node_kinds(graph).into_iter().collect::<BTreeSet<_>>();
    let edge_kinds = graph_edge_kinds(graph).into_iter().collect::<BTreeSet<_>>();
    let bound = collect_clause_bound_vars(&query.clauses);
    for clause in &query.clauses {
        match clause {
            Clause::Match(patterns) => {
                for pattern in patterns {
                    for node in &pattern.nodes {
                        if !node_kinds.contains(&node.kind) {
                            if let Some((start, end)) = find_kind_span(source, &node.kind) {
                                out.push(Diagnostic { start, end, severity: DiagnosticSeverity::Error, message: format!("unknown node kind '{}'", node.kind), code: Some("jack/unknown-node-kind".into()) });
                            }
                        }
                    }
                    if let Some(edge) = &pattern.edge {
                        if let Some(kind) = &edge.kind {
                            if !edge_kinds.contains(kind) {
                                if let Some((start, end)) = find_kind_span(source, kind) {
                                    out.push(Diagnostic { start, end, severity: DiagnosticSeverity::Error, message: format!("unknown edge kind '{}'", kind), code: Some("jack/unknown-edge-kind".into()) });
                                }
                            }
                        }
                    }
                }
            }
            Clause::Create(pattern) | Clause::Merge(pattern) => {
                for node in &pattern.nodes {
                    if !node_kinds.contains(&node.kind) {
                        if let Some((start, end)) = find_kind_span(source, &node.kind) {
                            out.push(Diagnostic { start, end, severity: DiagnosticSeverity::Error, message: format!("unknown node kind '{}'", node.kind), code: Some("jack/unknown-node-kind".into()) });
                        }
                    }
                }
            }
            _ => {}
        }
    }
    for (var, _, _) in collect_referenced_vars(&query.clauses) {
        if !bound.contains(&var) {
            if let Some((start, end)) = find_ident_span(source, &var) {
                out.push(Diagnostic { start, end, severity: DiagnosticSeverity::Error, message: format!("variable '{var}' is not bound by MATCH"), code: Some("jack/unbound-variable".into()) });
            }
        }
    }
    out
}

fn find_kind_span(source: &str, kind: &str) -> Option<(usize, usize)> {
    let needle = format!(":{kind}");
    let start = source.find(&needle)?;
    Some((start + 1, start + needle.len()))
}

fn find_ident_span(source: &str, ident: &str) -> Option<(usize, usize)> {
    let mut from = 0;
    while let Some(rel) = source[from..].find(ident) {
        let start = from + rel;
        let end = start + ident.len();
        let before = source.as_bytes().get(start.wrapping_sub(1));
        let after = source.as_bytes().get(end);
        let boundary_before = before.is_none_or(|c| !c.is_ascii_alphanumeric() && *c != b'_');
        let boundary_after = after.is_none_or(|c| !c.is_ascii_alphanumeric() && *c != b'_');
        if boundary_before && boundary_after {
            return Some((start, end));
        }
        from = end;
    }
    None
}

/// 🩺️ Lint jack source with syntax and semantic diagnostics.
pub fn lint<G: QueryableGraph>(graph: &G, source: &str) -> Vec<Diagnostic> {
    let mut out = Vec::new();
    for span in tokenize(source) {
        if span.class == TokenClass::Error {
            out.push(Diagnostic { start: span.start, end: span.end, severity: DiagnosticSeverity::Error, message: "unterminated string literal".into(), code: Some("jack/unterminated-string".into()) });
        }
    }
    match parse(source) {
        Ok(query) => out.extend(semantic_lints(graph, &query, source)),
        Err(err) => {
            let end = source.len().max(1);
            out.push(Diagnostic { start: 0, end, severity: DiagnosticSeverity::Error, message: err.to_string(), code: Some("jack/parse-error".into()) });
        }
    }
    out
}

fn format_token(tok: &Token) -> String {
    match tok {
        Token::KwMatch => "MATCH".into(),
        Token::KwWhere => "WHERE".into(),
        Token::KwReturn => "RETURN".into(),
        Token::KwCreate => "CREATE".into(),
        Token::KwDelete => "DELETE".into(),
        Token::KwSet => "SET".into(),
        Token::KwMerge => "MERGE".into(),
        Token::KwWith => "WITH".into(),
        Token::KwUnwind => "UNWIND".into(),
        Token::KwCall => "CALL".into(),
        Token::KwAs => "AS".into(),
        Token::And => "AND".into(),
        Token::Or => "OR".into(),
        Token::Ident(s) => s.clone(),
        Token::Number(n) => {
            if n.fract() == 0.0 {
                format!("{}", *n as i64)
            } else {
                n.to_string()
            }
        }
        // 🩹️ unified syntax law: strings always PRINT double-quoted with `dsl_core`'s canonical
        // escape, regardless of which quote style the source used.
        Token::StringLit(s) => format!("\"{}\"", semio_framework_dsl::escape_text(s)),
        Token::LParen => "(".into(),
        Token::RParen => ")".into(),
        Token::LBracket => "[".into(),
        Token::RBracket => "]".into(),
        Token::Colon => ":".into(),
        Token::Comma => ",".into(),
        Token::Dot => ".".into(),
        Token::Eq => "=".into(),
        Token::Ne => "!=".into(),
        Token::Arrow => "->".into(),
        Token::Dash => "-".into(),
        Token::DashArrow => "--".into(),
        Token::BackArrow => "<-".into(),
        Token::At => "@".into(),
        Token::Eof => String::new(),
    }
}

/// 🪞️ Format jack source canonically (idempotent).
pub fn format(source: &str) -> Result<String, GraphDslError> {
    let tokens = lex_spanned(source, false)?;
    let mut out = String::new();
    let mut line_open = false;
    let mut i = 0;
    while i < tokens.len() {
        let row = &tokens[i];
        if matches!(row.token, Token::Eof) {
            break;
        }
        match &row.token {
            Token::KwMatch | Token::KwWhere | Token::KwReturn | Token::KwCreate | Token::KwDelete | Token::KwSet | Token::KwMerge | Token::KwWith | Token::KwUnwind | Token::KwCall => {
                if !out.is_empty() {
                    out.push('\n');
                }
                out.push_str(&format_token(&row.token));
                out.push(' ');
                line_open = true;
            }
            Token::Comma => {
                out.push_str(", ");
            }
            Token::Arrow => {
                out.push_str("->");
            }
            Token::Dash => {
                out.push_str("-");
            }
            Token::DashArrow => {
                out.push_str("--");
            }
            Token::BackArrow => {
                out.push_str("<-");
            }
            Token::And | Token::Or | Token::KwAs => {
                out.push(' ');
                out.push_str(&format_token(&row.token));
                out.push(' ');
            }
            Token::Eq | Token::Ne => {
                out.push(' ');
                out.push_str(&format_token(&row.token));
                out.push(' ');
            }
            _ => {
                if line_open && !out.ends_with(' ') && !out.ends_with('\n') && !matches!(row.token, Token::RParen | Token::RBracket | Token::Comma | Token::Dot) {
                    let prev = tokens.get(i.saturating_sub(1)).map(|t| &t.token);
                    if !matches!(prev, Some(Token::LParen | Token::LBracket | Token::Colon | Token::Dot | Token::Arrow | Token::Dash | Token::DashArrow | Token::BackArrow)) {
                        out.push(' ');
                    }
                }
                out.push_str(&format_token(&row.token));
            }
        }
        i += 1;
    }
    Ok(out.trim().to_string())
}

fn hover_word_at(source: &str, cursor: usize) -> Option<(usize, usize, String)> {
    let cursor = cursor.min(source.len());
    if cursor > source.len() {
        return None;
    }
    let bytes = source.as_bytes();
    let mut start = cursor;
    while start > 0 {
        let c = bytes[start - 1];
        if c.is_ascii_alphanumeric() || c == b'_' || c == b':' || c == b'.' {
            start -= 1;
        } else {
            break;
        }
    }
    let mut end = cursor;
    while end < bytes.len() {
        let c = bytes[end];
        if c.is_ascii_alphanumeric() || c == b'_' || c == b':' || c == b'.' {
            end += 1;
        } else {
            break;
        }
    }
    if start == end {
        return None;
    }
    Some((start, end, source[start..end].to_string()))
}

/// 💬️ Hover information at cursor.
pub fn hover<G: QueryableGraph>(graph: &G, source: &str, cursor: usize) -> Option<Hover> {
    let (start, end, word) = hover_word_at(source, cursor)?;
    let upper = word.to_ascii_uppercase();
    if CLAUSE_KEYWORDS.iter().any(|kw| *kw == upper) || LOGIC_KEYWORDS.iter().any(|kw| *kw == upper) {
        return Some(Hover { start, end, contents: format!("Jack keyword `{upper}`") });
    }
    if graph_node_kinds(graph).iter().any(|kind| kind == &word) {
        return Some(Hover { start, end, contents: format!("Node kind `{word}`") });
    }
    if graph_edge_kinds(graph).iter().any(|kind| kind == &word) {
        return Some(Hover { start, end, contents: format!("Edge kind `{word}`") });
    }
    if graph_property_names(graph).iter().any(|prop| prop == &word) {
        return Some(Hover { start, end, contents: format!("Property `{word}`") });
    }
    if collect_bound_vars(&lex_spanned(source, true).unwrap_or_default()).contains(&word) {
        return Some(Hover { start, end, contents: format!("Bound variable `{word}`") });
    }
    None
}

/// 🎨️ Semantic token classes for LSP highlighting.
pub fn semantic_tokens(source: &str) -> Vec<SemanticToken> {
    tokenize(source)
        .into_iter()
        .map(|span| SemanticToken {
            start: span.start,
            end: span.end,
            class: match span.class {
                TokenClass::Keyword => "keyword",
                TokenClass::Ident => "ident",
                TokenClass::Number => "number",
                TokenClass::String => "string",
                TokenClass::Operator => "operator",
                TokenClass::Punctuation => "punctuation",
                TokenClass::Error => "error",
            }
            .into(),
        })
        .collect()
}
// #endregion 🔖️LanguageService

// #region 🔖️DslIdiom
/// 🔌️ Registers Jack as a `dsl_core::IdiomHooks` entry (the `DslIdiom` seam's Route B — an EMBEDDED
/// idiom hosted inside another document's `Shape::Embed("jack")` field) so `canonicalize`
/// normalizes embedded Jack text through this crate's own `format`/`tokenize`. Hand-built rather
/// than `dsl_core::hooks_for::<I: DslIdiom>()`: that helper needs `DslIdiom::print(ast) -> String`, and
/// Jack has no AST-to-text printer (`format` re-derives canonical text token-by-token from SOURCE,
/// not from a `Query`) — `IdiomHooks` itself only needs function pointers, so it's built directly
/// from the language-service surface Jack already has, no printer required.
// 🚫️async: E4 fn-pointer slot — builds an `IdiomHooks` whose fields are plain `fn` pointers; an
// `async fn`'s value cannot coerce to `fn`, so this stays sync. See R2 E4, mirrors
// `dsl_core::hooks_for`/`dsl_core::passthrough_hooks`.
pub fn idiom_hooks() -> semio_framework_dsl::IdiomHooks {
    semio_framework_dsl::IdiomHooks { lang: "jack", canonicalize: idiom_canonicalize, classify: idiom_classify, complete: idiom_complete }
}

// 🚫️async: E4 fn-pointer slot (`IdiomHooks.canonicalize`) — see R2 E4.
fn idiom_canonicalize(text: &str) -> Result<String, semio_framework_diagnostic::TextError> {
    format(text).map_err(|error|match error{GraphDslError::Lex(error)=>error,GraphDslError::Json(error)=>semio_framework_diagnostic::TextError::new(error.kind(),error.to_string(),semio_framework_diagnostic::TextSpan::at(1,1)),GraphDslError::UnsupportedMutation=>semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"mutating jack clauses are not supported on this graph domain",semio_framework_diagnostic::TextSpan::at(1,1)),error @ (GraphDslError::UnterminatedString|GraphDslError::UnexpectedChar(_)|GraphDslError::NumberFormat(_)|GraphDslError::UnexpectedToken{..}|GraphDslError::EdgeTargetMissingPort|GraphDslError::EmptyPattern|GraphDslError::UnknownProcedure(_)|GraphDslError::ProcedureArity{..}|GraphDslError::ManifestIdentity{..})=>semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,error.to_string(),semio_framework_diagnostic::TextSpan::at(1,1))})
}

// 🚫️async: E4 fn-pointer slot (`IdiomHooks.classify`) — see R2 E4.
fn idiom_classify(text: &str) -> Vec<(semio_framework_dsl::TokenClass, semio_framework_diagnostic::TextSpan)> {
    tokenize(text)
        .into_iter()
        .map(|span| {
            let class = match span.class {
                TokenClass::Keyword => semio_framework_dsl::TokenClass::Keyword,
                TokenClass::Ident => semio_framework_dsl::TokenClass::Ident,
                TokenClass::Number => semio_framework_dsl::TokenClass::Number,
                TokenClass::String => semio_framework_dsl::TokenClass::String,
                TokenClass::Operator => semio_framework_dsl::TokenClass::Operator,
                TokenClass::Punctuation => semio_framework_dsl::TokenClass::Punctuation,
                TokenClass::Error => semio_framework_dsl::TokenClass::Error,
            };
            (class, byte_range_to_span(text, span.start, span.end))
        })
        .collect()
}

// 🚫️async: E4 fn-pointer slot (`IdiomHooks.complete`) — see R2 E4.
fn idiom_complete(text: &str, offset: usize) -> Vec<semio_framework_dsl::CompletionItem> {
    // Jack's own `complete` needs a `QueryableGraph`-bounded generic for schema-aware suggestions
    // (node/edge kinds, property names) that the generic `DslIdiom`/embed-host seam has no graph to
    // supply — an empty graph still exercises the syntax-only completions (clause/logic keywords).
    struct EmptyGraph;
    impl QueryableGraph for EmptyGraph {
        fn manifest(&self) -> Option<&crate::manifest::GraphManifest> {
            None
        }
        fn node_ids(&self) -> Vec<String> {
            Vec::new()
        }
        fn node_kind(&self, _id: &str) -> Option<String> {
            None
        }
        fn node_name(&self, _id: &str) -> Option<String> {
            None
        }
        fn node_property(&self, _id: &str, _key: &str) -> Option<PropertyValue> {
            None
        }
        fn edges(&self) -> Vec<QueryableEdge> {
            Vec::new()
        }
        fn subgraph_fixture_json(&self, _node_ids: &BTreeSet<String>, _edge_ids: &BTreeSet<String>) -> Option<String> {
            None
        }
    }
    complete(&EmptyGraph, text, offset).into_iter().map(|c| semio_framework_dsl::CompletionItem { label: c.label, detail: c.detail }).collect()
}

/// 📍️ Converts a byte-offset half-open range into `os_dsl::TextSpan`'s 1-based line/column/
/// length form — Jack's own spans are byte offsets (`TokenSpan`/`SemanticToken`), `dsl_core`'s are
/// line/column, so this is the one place that needs the source text to translate between them.
fn byte_range_to_span(text: &str, start: usize, end: usize) -> semio_framework_diagnostic::TextSpan {
    let mut line = 1u32;
    let mut column = 1u32;
    for (i, ch) in text.char_indices() {
        if i >= start {
            break;
        }
        if ch == '\n' {
            line += 1;
            column = 1;
        } else {
            column += 1;
        }
    }
    let length = text.get(start..end).map_or(0, |s| s.chars().count()) as u32;
    semio_framework_diagnostic::TextSpan::with_length(line, column, length)
}
// #endregion 🔖️DslIdiom

// #region 🔖️Parser
struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, pos: 0 }
    }

    fn peek(&self) -> &Token {
        self.tokens.get(self.pos).unwrap_or(&Token::Eof)
    }

    fn bump(&mut self) -> Token {
        let t = self.peek().clone();
        if !matches!(t, Token::Eof) {
            self.pos += 1;
        }
        t
    }

    fn expect_ident(&mut self) -> Result<String, GraphDslError> {
        match self.bump() {
            Token::Ident(s) => Ok(s),
            other => Err(GraphDslError::UnexpectedToken { expected: "ident".into(), found: format!("{other:?}") }),
        }
    }

    fn parse_query(&mut self) -> Result<Query, GraphDslError> {
        let mut clauses = Vec::new();
        while !matches!(self.peek(), Token::Eof) {
            clauses.push(self.parse_clause()?);
        }
        Ok(Query { clauses })
    }

    fn parse_clause(&mut self) -> Result<Clause, GraphDslError> {
        match self.peek() {
            Token::KwMatch => {
                self.bump();
                let mut patterns = vec![self.parse_pattern()?];
                while matches!(self.peek(), Token::Comma) {
                    self.bump();
                    patterns.push(self.parse_pattern()?);
                }
                Ok(Clause::Match(patterns))
            }
            Token::KwWhere => {
                self.bump();
                Ok(Clause::Where(self.parse_expr()?))
            }
            Token::KwReturn => {
                self.bump();
                let mut items = vec![self.parse_return_item()?];
                while matches!(self.peek(), Token::Comma) {
                    self.bump();
                    items.push(self.parse_return_item()?);
                }
                Ok(Clause::Return(items))
            }
            Token::KwCreate => {
                self.bump();
                Ok(Clause::Create(self.parse_pattern()?))
            }
            Token::KwDelete => {
                self.bump();
                let mut vars = vec![self.expect_ident()?];
                while matches!(self.peek(), Token::Comma) {
                    self.bump();
                    vars.push(self.expect_ident()?);
                }
                Ok(Clause::Delete(vars))
            }
            Token::KwSet => {
                self.bump();
                let mut items = vec![self.parse_assignment()?];
                while matches!(self.peek(), Token::Comma) {
                    self.bump();
                    items.push(self.parse_assignment()?);
                }
                Ok(Clause::Set(items))
            }
            Token::KwMerge => {
                self.bump();
                Ok(Clause::Merge(self.parse_pattern()?))
            }
            Token::KwWith => {
                self.bump();
                let mut items = vec![self.parse_return_item()?];
                while matches!(self.peek(), Token::Comma) {
                    self.bump();
                    items.push(self.parse_return_item()?);
                }
                Ok(Clause::With(items))
            }
            Token::KwUnwind => {
                self.bump();
                let source = self.parse_return_item()?;
                self.expect(&Token::KwAs)?;
                let var = self.expect_ident()?;
                Ok(Clause::Unwind(UnwindClause { source, var }))
            }
            Token::KwCall => {
                self.bump();
                let name = self.expect_ident()?;
                self.expect(&Token::LParen)?;
                let mut args = Vec::new();
                if !matches!(self.peek(), Token::RParen) {
                    args.push(self.parse_value()?);
                    while matches!(self.peek(), Token::Comma) {
                        self.bump();
                        args.push(self.parse_value()?);
                    }
                }
                self.expect(&Token::RParen)?;
                Ok(Clause::Call(CallClause { name, args }))
            }
            other => Err(GraphDslError::UnexpectedToken { expected: "clause start (MATCH/WHERE/RETURN/CREATE/DELETE/SET/MERGE/WITH/UNWIND/CALL)".into(), found: format!("{other:?}") }),
        }
    }

    /// 🕸️ Pattern grammar over the unified token alphabet — `dsl_core` has no standalone `-`
    /// token (only `->`/`--`/`<-`), so the leading connector before a bracketed edge label is
    /// always `--` or `<-`, never a bare dash (real Cypher's `-[r]->`/`<-[r]-` shape, adapted to
    /// this repo's alphabet). `<-` at the front means the edge points INTO `left`; represented by
    /// swapping which parsed node plays "left" so the stored `PatternEdge.right` is always the
    /// forward-direction target, mirroring `dsl_schema`'s own wire `<-` normalization.
    fn parse_pattern(&mut self) -> Result<Pattern, GraphDslError> {
        self.expect(&Token::LParen)?;
        let left = self.parse_pattern_node()?;
        self.expect(&Token::RParen)?;
        match self.peek().clone() {
            Token::Arrow => {
                self.bump();
                let right = self.parse_bracketed_pattern_node()?;
                Ok(Pattern { nodes: vec![left], edge: Some(PatternEdge { var: None, kind: None, directed: true, right }) })
            }
            // ➖️ `-` and `--` are the SAME connector. Cypher (and this dialect's own executor parser,
            // and every committed example query in the repo) writes `(a)-[r:Kind]->(b)`; `--` is this
            // grammar's own equal-status spelling. Accepting only `--` made every real query fail
            // `unexpected character '-'` in lint/complete/hover/format while the executor ran it fine.
            Token::Dash | Token::DashArrow => {
                self.bump();
                if matches!(self.peek(), Token::LBracket) {
                    let (edge_var, edge_kind) = self.parse_edge_label()?;
                    let directed = match self.peek() {
                        Token::Arrow => {
                            self.bump();
                            true
                        }
                        Token::Dash | Token::DashArrow => {
                            self.bump();
                            false
                        }
                        other => return Err(GraphDslError::UnexpectedToken { expected: "->, - or --".into(), found: format!("{other:?}") }),
                    };
                    let right = self.parse_bracketed_pattern_node()?;
                    Ok(Pattern { nodes: vec![left], edge: Some(PatternEdge { var: edge_var, kind: edge_kind, directed, right }) })
                } else {
                    let right = self.parse_bracketed_pattern_node()?;
                    Ok(Pattern { nodes: vec![left], edge: Some(PatternEdge { var: None, kind: None, directed: false, right }) })
                }
            }
            Token::BackArrow => {
                self.bump();
                if matches!(self.peek(), Token::LBracket) {
                    let (edge_var, edge_kind) = self.parse_edge_label()?;
                    match self.peek() {
                        Token::Dash | Token::DashArrow => {
                            self.bump();
                        }
                        other => return Err(GraphDslError::UnexpectedToken { expected: "- or --".into(), found: format!("{other:?}") }),
                    }
                    let right = self.parse_bracketed_pattern_node()?;
                    Ok(Pattern { nodes: vec![right], edge: Some(PatternEdge { var: edge_var, kind: edge_kind, directed: true, right: left }) })
                } else {
                    let right = self.parse_bracketed_pattern_node()?;
                    Ok(Pattern { nodes: vec![right], edge: Some(PatternEdge { var: None, kind: None, directed: true, right: left }) })
                }
            }
            _ => Ok(Pattern { nodes: vec![left], edge: None }),
        }
    }

    fn parse_bracketed_pattern_node(&mut self) -> Result<PatternNode, GraphDslError> {
        self.expect(&Token::LParen)?;
        let node = self.parse_pattern_node()?;
        self.expect(&Token::RParen)?;
        Ok(node)
    }

    fn parse_edge_label(&mut self) -> Result<(Option<String>, Option<String>), GraphDslError> {
        self.expect(&Token::LBracket)?;
        let edge_var = if matches!(self.peek(), Token::Ident(_)) { Some(self.expect_ident()?) } else { None };
        let edge_kind = if matches!(self.peek(), Token::Colon) {
            self.bump();
            Some(self.expect_ident()?)
        } else {
            None
        };
        self.expect(&Token::RBracket)?;
        Ok((edge_var, edge_kind))
    }

    fn parse_pattern_node(&mut self) -> Result<PatternNode, GraphDslError> {
        let var = self.expect_ident()?;
        self.expect(&Token::Colon)?;
        let kind = self.expect_ident()?;
        let port = if matches!(self.peek(), Token::At) {
            self.bump();
            Some(self.expect_ident()?)
        } else {
            None
        };
        Ok(PatternNode { var, kind, port })
    }

    fn parse_return_item(&mut self) -> Result<ReturnItem, GraphDslError> {
        let var = self.expect_ident()?;
        if matches!(self.peek(), Token::Dot) {
            self.bump();
            let prop = self.expect_ident()?;
            Ok(ReturnItem::Property { var, prop })
        } else {
            Ok(ReturnItem::Var(var))
        }
    }

    fn parse_assignment(&mut self) -> Result<Assignment, GraphDslError> {
        let var = self.expect_ident()?;
        self.expect(&Token::Dot)?;
        let prop = self.expect_ident()?;
        self.expect(&Token::Eq)?;
        let value = self.parse_value()?;
        Ok(Assignment { var, prop, value })
    }

    fn parse_expr(&mut self) -> Result<Expr, GraphDslError> {
        self.parse_or_expr()
    }

    fn parse_or_expr(&mut self) -> Result<Expr, GraphDslError> {
        let mut left = self.parse_and_expr()?;
        while matches!(self.peek(), Token::Or) {
            self.bump();
            let right = self.parse_and_expr()?;
            left = Expr::Or(Box::new(left), Box::new(right));
        }
        Ok(left)
    }

    fn parse_and_expr(&mut self) -> Result<Expr, GraphDslError> {
        let mut left = self.parse_cmp_expr()?;
        while matches!(self.peek(), Token::And) {
            self.bump();
            let right = self.parse_cmp_expr()?;
            left = Expr::And(Box::new(left), Box::new(right));
        }
        Ok(left)
    }

    fn parse_cmp_expr(&mut self) -> Result<Expr, GraphDslError> {
        let var = self.expect_ident()?;
        self.expect(&Token::Dot)?;
        let prop = self.expect_ident()?;
        match self.bump() {
            Token::Eq => Ok(Expr::Eq { var, prop, value: self.parse_value()? }),
            Token::Ne => Ok(Expr::Ne { var, prop, value: self.parse_value()? }),
            other => Err(GraphDslError::UnexpectedToken { expected: "= or !=".into(), found: format!("{other:?}") }),
        }
    }

    fn parse_value(&mut self) -> Result<PropertyValue, GraphDslError> {
        match self.bump() {
            Token::Number(n) => Ok(PropertyValue::Number(n)),
            Token::StringLit(s) => Ok(PropertyValue::String(s)),
            Token::Ident(s) if s.eq_ignore_ascii_case("true") => Ok(PropertyValue::Bool(true)),
            Token::Ident(s) if s.eq_ignore_ascii_case("false") => Ok(PropertyValue::Bool(false)),
            Token::Ident(s) if s.eq_ignore_ascii_case("null") => Ok(PropertyValue::Null),
            other => Err(GraphDslError::UnexpectedToken { expected: "value".into(), found: format!("{other:?}") }),
        }
    }

    fn expect(&mut self, want: &Token) -> Result<(), GraphDslError> {
        if std::mem::discriminant(self.peek()) == std::mem::discriminant(want) {
            self.bump();
            Ok(())
        } else {
            Err(GraphDslError::UnexpectedToken { expected: format!("{want:?}"), found: format!("{:?}", self.peek()) })
        }
    }
}

/// 🔍️ Parse a jack query string.
pub fn parse(query: &str) -> Result<Query, GraphDslError> {
    let tokens = lex(query)?;
    Parser::new(tokens).parse_query()
}
// #endregion 🔖️Parser

// #region 🔖️Executor
/// 🎯️ Variable binding in a match row.
#[derive(Clone, Debug, Default)]
pub struct Binding {
    pub nodes: BTreeMap<String, String>,
    pub edges: BTreeMap<String, String>,
    /// 🍇️ Scalar/list/object values bound by `WITH`'s property projection, `UNWIND`, and `CALL` —
    /// entries here have no backing graph entity, unlike `nodes`/`edges`.
    pub values: BTreeMap<String, PropertyValue>,
}

/// ▶️ Execute a read-only jack query against a queryable graph.
pub fn execute<G: QueryableGraph>(graph: &G, query: &Query) -> Result<QueryResult, GraphDslError> {
    let mut bindings: Vec<Binding> = vec![Binding::default()];
    let mut return_items: Option<Vec<ReturnItem>> = None;
    for clause in &query.clauses {
        match clause {
            Clause::Match(patterns) => bindings = match_patterns(graph, patterns)?,
            Clause::Where(expr) => {
                // 🔀️ Rewritten from `.retain(..)` — `eval_expr` is async (it may call the
                // caller-provided `QueryableGraph`, which is not assumed I/O-free) and cannot run
                // inside a sync `retain` predicate (R10 residue shape #1).
                let mut kept = Vec::with_capacity(bindings.len());
                for b in bindings {
                    if eval_expr(graph, &b, expr) {
                        kept.push(b);
                    }
                }
                bindings = kept;
            }
            Clause::Return(items) => return_items = Some(items.clone()),
            Clause::Create(_) | Clause::Delete(_) | Clause::Set(_) | Clause::Merge(_) => {
                return Err(GraphDslError::UnsupportedMutation);
            }
            Clause::With(items) => {
                bindings = bindings.iter().map(|binding| project_binding(binding, items)).collect();
            }
            Clause::Unwind(unwind) => {
                let mut next = Vec::new();
                for binding in &bindings {
                    for element in unwind_elements(resolve_item_value(graph, binding, &unwind.source)) {
                        let mut b = binding.clone();
                        b.values.insert(unwind.var.clone(), element);
                        next.push(b);
                    }
                }
                bindings = next;
            }
            Clause::Call(call) => {
                let (column, results) = call_procedure(graph, call)?;
                let mut next = Vec::with_capacity(bindings.len() * results.len());
                for binding in &bindings {
                    for value in &results {
                        let mut b = binding.clone();
                        b.values.insert(column.clone(), value.clone());
                        next.push(b);
                    }
                }
                bindings = next;
            }
        }
    }
    if let Some(items) = return_items {
        return Ok(build_return(graph, &bindings, &items));
    }
    Ok(QueryResult::table(vec![], vec![]))
}

/// ▶️ Parse and execute jack in one step.
pub fn run_query<G: QueryableGraph>(graph: &G, source: &str) -> Result<QueryResult, GraphDslError> {
    execute(graph, &parse(source)?)
}

/// ▶️ Execute jack and return JSON result.
pub fn run_query_json<G: QueryableGraph>(graph: &G, source: &str) -> Result<String, GraphDslError> {
    Ok(semio_framework_pack_json::to_json_string(&run_query(graph, source)?))
}

fn match_patterns<G: QueryableGraph>(graph: &G, patterns: &[Pattern]) -> Result<Vec<Binding>, GraphDslError> {
    let mut bindings = vec![Binding::default()];
    for pattern in patterns {
        let mut next = Vec::new();
        for binding in &bindings {
            next.extend(match_pattern(graph, pattern, binding)?);
        }
        bindings = next;
    }
    Ok(bindings)
}

fn match_pattern<G: QueryableGraph>(graph: &G, pattern: &Pattern, base: &Binding) -> Result<Vec<Binding>, GraphDslError> {
    let left = pattern.nodes.first().ok_or(GraphDslError::EmptyPattern)?;
    if let Some(edge_pat) = &pattern.edge {
        let mut out = Vec::new();
        for node_id in graph.node_ids() {
            if graph.node_kind(node_id.as_str()).as_deref() != Some(left.kind.as_str()) {
                continue;
            }
            if binding_conflicts(base, &left.var, node_id.as_str()) {
                continue;
            }
            for edge in graph.edges() {
                if edge_pat.kind.as_ref().is_some_and(|k| *k != edge.kind) {
                    continue;
                }
                let pairs = if edge_pat.directed {
                    vec![(edge.source_node_id.as_str(), edge.target_node_id.as_str(), edge.source_port.as_deref(), edge.target_port.as_deref())]
                } else {
                    vec![
                        (edge.source_node_id.as_str(), edge.target_node_id.as_str(), edge.source_port.as_deref(), edge.target_port.as_deref()),
                        (edge.target_node_id.as_str(), edge.source_node_id.as_str(), edge.target_port.as_deref(), edge.source_port.as_deref()),
                    ]
                };
                for (src_id, tgt_id, src_port, tgt_port) in pairs {
                    if src_id != node_id {
                        continue;
                    }
                    if left.port.as_ref().is_some_and(|want| src_port != Some(want.as_str())) {
                        continue;
                    }
                    if graph.node_kind(tgt_id).as_deref() != Some(edge_pat.right.kind.as_str()) {
                        continue;
                    }
                    if edge_pat.right.port.as_ref().is_some_and(|want| tgt_port != Some(want.as_str())) {
                        continue;
                    }
                    let mut b = base.clone();
                    b.nodes.insert(left.var.clone(), node_id.clone());
                    if let Some(ev) = &edge_pat.var {
                        b.edges.insert(ev.clone(), edge.id.clone());
                    }
                    if binding_conflicts(base, &edge_pat.right.var, tgt_id) {
                        continue;
                    }
                    b.nodes.insert(edge_pat.right.var.clone(), tgt_id.to_string());
                    out.push(b);
                }
            }
        }
        return Ok(out);
    }
    let mut out = Vec::new();
    for node_id in graph.node_ids() {
        if graph.node_kind(node_id.as_str()).as_deref() != Some(left.kind.as_str()) {
            continue;
        }
        if binding_conflicts(base, &left.var, node_id.as_str()) {
            continue;
        }
        let mut b = base.clone();
        b.nodes.insert(left.var.clone(), node_id);
        out.push(b);
    }
    Ok(out)
}

fn binding_conflicts(base: &Binding, var: &str, node_id: &str) -> bool {
    base.nodes.get(var).is_some_and(|existing| existing != node_id)
}

fn eval_expr<G: QueryableGraph>(graph: &G, binding: &Binding, expr: &Expr) -> bool {
    match expr {
        Expr::Eq { var, prop, value } => binding_value(graph, binding, var, prop) == Some(value.clone()),
        Expr::Ne { var, prop, value } => binding_value(graph, binding, var, prop) != Some(value.clone()),
        Expr::And(a, b) => eval_expr(graph, binding, a) && eval_expr(graph, binding, b),
        Expr::Or(a, b) => eval_expr(graph, binding, a) || eval_expr(graph, binding, b),
    }
}

fn binding_value<G: QueryableGraph>(graph: &G, binding: &Binding, var: &str, prop: &str) -> Option<PropertyValue> {
    if let Some(node_id) = binding.nodes.get(var) {
        return graph.node_property(node_id, prop);
    }
    match binding.values.get(var) {
        Some(PropertyValue::Object(map)) => map.get(prop).cloned(),
        _ => None,
    }
}

/// 🎯️ Resolves a [`ReturnItem`] against one binding — the single evaluation path shared by
/// `RETURN`, `WITH`'s trailing `WHERE`, and `UNWIND`'s source expression. A `Var` prefers a
/// `values`-scoped scalar (bound by `WITH`/`UNWIND`/`CALL`) over a graph entity's display name.
fn resolve_item_value<G: QueryableGraph>(graph: &G, binding: &Binding, item: &ReturnItem) -> Option<PropertyValue> {
    match item {
        ReturnItem::Var(v) => {
            if let Some(value) = binding.values.get(v) {
                return Some(value.clone());
            }
            binding.nodes.get(v).map(|id| graph.node_name(id).map_or(PropertyValue::Null, PropertyValue::String))
        }
        ReturnItem::Property { var, prop } => binding_value(graph, binding, var, prop),
    }
}

/// 🧺️ `UNWIND`'s list-expansion rule: `null`/missing and an empty list both yield zero rows, a
/// non-list scalar unwinds as its own single-element list (mirrors the Cypher-family convention).
fn unwind_elements(value: Option<PropertyValue>) -> Vec<PropertyValue> {
    match value {
        Some(PropertyValue::Array(items)) => items,
        Some(PropertyValue::Null) | None => vec![],
        Some(other) => vec![other],
    }
}

/// 🔭️ Projects one binding down to just the vars named by a `WITH` clause. A `Property` item
/// names its source var (`ReturnItem` carries no alias), so the whole entity/value bound to that
/// var is kept — this is what lets a later `WHERE`/`RETURN` still resolve `var.prop` from it.
fn project_binding(binding: &Binding, items: &[ReturnItem]) -> Binding {
    let mut out = Binding::default();
    for item in items {
        let var = match item {
            ReturnItem::Var(v) => v,
            ReturnItem::Property { var, .. } => var,
        };
        if let Some(id) = binding.nodes.get(var) {
            out.nodes.insert(var.clone(), id.clone());
        }
        if let Some(id) = binding.edges.get(var) {
            out.edges.insert(var.clone(), id.clone());
        }
        if let Some(value) = binding.values.get(var) {
            out.values.insert(var.clone(), value.clone());
        }
    }
    out
}

/// 📇️ `CALL`'s small owned procedure registry — graph-manifest introspection this codebase
/// already exposes to completion/hover (`manifest_node_kinds` et al.), surfaced as callable
/// procedures since `CallClause` has no `YIELD`/`AS` target: each procedure's single output
/// column is bound under a fixed, procedure-defined name. Arbitrary procedure dispatch (the
/// general case) is deferred to unifying semio_compose_rs's Architect query language onto Jack —
/// an unknown name reports [`GraphDslError::UnknownProcedure`], never a blanket rejection.
fn call_procedure<G: QueryableGraph>(graph: &G, call: &CallClause) -> Result<(String, Vec<PropertyValue>), GraphDslError> {
    if !call.args.is_empty() {
        return Err(GraphDslError::ProcedureArity { name: call.name.clone(), expected: 0, found: call.args.len() });
    }
    match call.name.as_str() {
        "nodeKinds" => Ok(("kind".to_string(), manifest_node_kinds(graph).into_iter().map(PropertyValue::String).collect())),
        "edgeKinds" => Ok(("kind".to_string(), manifest_edge_kinds(graph).into_iter().map(PropertyValue::String).collect())),
        "propertyNames" => Ok(("name".to_string(), manifest_property_names(graph).into_iter().map(PropertyValue::String).collect())),
        "portKinds" => Ok(("kind".to_string(), manifest_port_kinds(graph).into_iter().map(PropertyValue::String).collect())),
        other => Err(GraphDslError::UnknownProcedure(other.to_string())),
    }
}

fn binding_has_entity(binding: &Binding, var: &str) -> bool {
    binding.nodes.contains_key(var) || binding.edges.contains_key(var)
}

fn return_items_want_graph(items: &[ReturnItem], bindings: &[Binding]) -> bool {
    // 🔀️ Rewritten from `.any(..)` — `binding_has_entity` is async and cannot be called inside a
    // sync `Iterator::any` predicate (R10 residue shape #1).
    for item in items {
        let ReturnItem::Var(v) = item else { continue };
        for b in bindings {
            if binding_has_entity(b, v) {
                return true;
            }
        }
    }
    false
}

fn collect_graph_entities(bindings: &[Binding], items: &[ReturnItem]) -> (BTreeSet<String>, BTreeSet<String>) {
    let mut node_ids = BTreeSet::new();
    let mut edge_ids = BTreeSet::new();
    for binding in bindings {
        for item in items {
            if let ReturnItem::Var(v) = item {
                if let Some(id) = binding.nodes.get(v) {
                    node_ids.insert(id.clone());
                }
                if let Some(id) = binding.edges.get(v) {
                    edge_ids.insert(id.clone());
                }
            }
        }
    }
    (node_ids, edge_ids)
}

fn build_return<G: QueryableGraph>(graph: &G, bindings: &[Binding], items: &[ReturnItem]) -> QueryResult {
    let columns: Vec<String> = items
        .iter()
        .map(|item| match item {
            ReturnItem::Var(v) => v.clone(),
            ReturnItem::Property { var, prop } => format!("{var}.{prop}"),
        })
        .collect();
    if return_items_want_graph(items, bindings) {
        let (node_ids, edge_ids) = collect_graph_entities(bindings, items);
        if let Some(json) = graph.subgraph_fixture_json(&node_ids, &edge_ids) {
            return QueryResult::graph(columns, json);
        }
    }
    let mut rows = Vec::new();
    for binding in bindings {
        let mut row = Vec::new();
        for item in items {
            row.push(resolve_item_value(graph, binding, item).unwrap_or(PropertyValue::Null));
        }
        rows.push(row);
    }
    QueryResult::table(columns, rows)
}
// #endregion 🔖️Executor

// #region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
// #endregion 🔖️Tests
// #endregion jack_impl

```

### Cargo.toml

SHA-256 `542604e8a827dcc38c2e0c24922fb46eaeafb3c1b11638aaa1d6dbfb18579d77`; 32897 bytes.

```
cargo-features = ["trim-paths"]

[workspace]
resolver = "2"
members = [
    "🧰️framework/📦️packages/🦀️rust",
    "🧰️framework/🔨️modules/⏪️time-travel/📦️packages/🦀️rust",
    "🧰️framework/🔨️modules/⏯️tool-run/📦️packages/🦀️rust",
    "🧰️framework/🔨️modules/⏱️trace/📦️packages/🦀️rust",
    "🧰️framework/🔨️modules/⏳️async/✨️macros/📦️packages/🦀️rust",
    "🧰️framework/🔨️modules/⏳️async/📦️packages/🦀️rust",
    "🧰️framework/🔨️modules/◻️2d/📦️packages/🦀️rust",
    "🧰️framework/🔨️modules/⚠️diagnostic/📦️packages/🦀️rust",
    "🧰️framework/🔨️modules/✍️editor/📦️packages/🦀️rust",
    "🧰️framework/🔨️modules/🌱️value/✨️derive/📦️packages/🦀️rust",
    "🧰️framework/🔨️modules/🌱️value/💾️resident/📦️packages/🦀️rust",
    "🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust",
    "🧰️framework/🔨️modules/🎒️pack/⚠️error/📦️packages/🦀️rust",
    "🧰️framework/🔨️modules/🎒️pack/📦️packages/🦀️rust",
    "🧰️framework/🔨️modules/🎒️pack/🔤️json/📦️packages/🦀️rust",
    "🧰️framework/🔨️modules/🎭️actor/📦️packages/🦀️rust",
    "🧰️framework/🔨️modules/🏗️mesh-engine/📦️packages/🦀️rust",
    "🧰️framework/🔨️modules/📏️intrinsic-size/📦️packages/🦀️rust",
    "🧰️framework/🔨️modules/📐️geometry/📦️packages/🦀️rust",
    "🧰️framework/🔨️modules/📚️compiler/📦️packages/🦀️rust",
    "🧰️framework/🔨️modules/📡️replication/📦️packages/🦀️rust",
    "🧰️framework/🔨️modules/🔀️dispatch/📦️packages/🦀️rust",
    "🧰️framework/🔨️modules/🔄️machine/✨️derive/📦️packages/🦀️rust",
    "🧰️framework/🔨️modules/🔄️machine/📦️packages/🦀️rust",
    "🧰️framework/🔨️modules/🔏️hash/📦️packages/🦀️rust",
    "🧰️framework/🔨️modules/🔢️number/📦️packages/🦀️rust",
    "🧰️framework/🔨️modules/🔤️typeset/📦️packages/🦀️rust",
    "🧰️framework/🔨️modules/🔲️pixels/📦️packages/🦀️rust",
    "🧰️framework/🔨️modules/🕸️graph/⏯️layout-run/📦️packages/🦀️rust",
    "🧰️framework/🔨️modules/🕸️graph/📦️packages/🦀️rust",
    "🧰️framework/🔨️modules/🖌️raster/📦️packages/🦀️rust",
    "🧰️framework/🔨️modules/🖱️ui/🌐️locale/📦️packages/🦀️rust",
    "🧰️framework/🔨️modules/🖱️ui/🎨️styling/📦️packages/🦀️rust",
    "🧰️framework/🔨️modules/🖱️ui/🎬️scene/📦️packages/🦀️rust",
    "🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust",
    "🧰️framework/🔨️modules/🖱️ui/🖌️render/🎯️targets/🌋️vulkan/📦️packages/🦀️rust",
    "🧰️framework/🔨️modules/🖱️ui/🖌️render/🎯️targets/🍎️metal/📦️packages/🦀️rust",
    "🧰️framework/🔨️modules/🖱️ui/🖌️render/🎯️targets/🧊️webgpu/📦️packages/🦀️rust",
    "🧰️framework/🔨️modules/🖱️ui/🖌️render/🎯️targets/🪟️d3d12/📦️packages/🦀️rust",
    "🧰️framework/🔨️modules/🖱️ui/🖌️render/📦️packages/🦀️rust",
    "🧰️framework/🔨️modules/🖱️ui/🖥️host/📦️packages/🦀️rust",
    "🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust",
    "🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust",
    "🧰️framework/🔨️modules/🖱️ui/🪟️viewport/📦️packages/🦀️rust",
    "🧰️framework/🔨️modules/🗜️deflate/📦️packages/🦀️rust",
    "🧰️framework/🔨️modules/🗣️dsl/📦️packages/🦀️rust",
    "🧰️framework/🔨️modules/🗣️dsl/🧬️schema/✨️derive/📦️packages/🦀️rust",
    "🧰️framework/🔨️modules/🗣️dsl/🧬️schema/📦️packages/🦀️rust",
    "🧰️framework/🔨️modules/🗺️surface/📦️packages/🦀️rust",
    "🧰️framework/🔨️modules/🚪️io/🔤️base64/📦️packages/🦀️rust",
    "🧰️framework/🔨️modules/🚪️io/🧬️schema/📦️packages/🦀️rust",
    "🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/📦️packages/🦀️rust",
    "🧰️framework/🔨️modules/🛠️tool-machine/📦️packages/🦀️rust",
    "🧰️framework/🔨️modules/🧊️3d/📦️packages/🦀️rust",
    "🧰️framework/🔨️modules/🧬️schema/✅️validator/📦️packages/🦀️rust",
    "🧰️framework/🔨️modules/🧬️schema/✨️derive/📦️packages/🦀️rust",
    "🧰️framework/🔨️modules/🧬️schema/📇️registry/📦️packages/🦀️rust",
    "🧰️framework/🔨️modules/🧬️schema/📦️packages/🦀️rust",
    "🧰️framework/🔨️modules/🧬️schema/📶️state/📦️packages/🦀️rust",
    "🧰️framework/🔨️modules/🧬️schema/🧩️composition/📦️packages/🦀️rust",
    "🧰️framework/🔨️modules/🧮️math/📦️packages/🦀️rust",
    "🧰️framework/🔨️modules/🧵️job/📦️packages/🦀️rust",
    "🧰️framework/🛍️products/❓️quiz/📦️packages/🦀️rust",
    "🧰️framework/🛍️products/💻️os/🎚️config/📦️packages/🦀️rust",
    "🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust",
    "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/📦️packages/🦀️rust",
    "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🖼️canvas/🔤️fonts/📦️packages/🦀️rust",
    "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🗿️artifacts/🕸️dag/📦️packages/🦀️rust",
    "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🦀️rust",
    "🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/📦️packages/🦀️rust",
    "🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🗿️artifacts/🌊️flow/📦️packages/🦀️rust",
    "🧰️framework/🛍️products/💻️os/🔨️modules/🏃️run/📦️packages/🦀️rust",
    "🧰️framework/🛍️products/💻️os/🔨️modules/📖️playbook/🗿️artifacts/📖️playbook/📦️packages/🦀️rust",
    "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust",
    "🧰️framework/🛍️products/💻️os/🔨️modules/🔁️workflow/🗿️artifacts/🏃️run/📦️packages/🦀️rust",
    "🧰️framework/🛍️products/💻️os/🔨️modules/🔁️workflow/🗿️artifacts/🔁️workflow/📦️packages/🦀️rust",
    "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust",
    "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/📦️packages/🦀️rust",
    "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧫️fixtures/🧩️component/📦️packages/🦀️rust",
    "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/📦️packages/🦀️rust",
    "🧰️framework/🛍️products/💻️os/🔨️modules/🖥️shell/📦️packages/🦀️rust",
    "🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/📦️packages/🦀️rust",
    "🧰️framework/🛍️products/💻️os/🔨️modules/🛎️services/📦️packages/🦀️rust",
    "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📦️packages/🦀️rust",
    "🧰️framework/🛍️products/💻️os/🔨️modules/🧠️neural/⚙️engine/📦️packages/🦀️rust",
    "🧰️framework/🛍️products/💻️os/🔨️modules/🪐️space/🗿️artifacts/🗂️collection/📦️packages/🦀️rust",
    "🧰️framework/🛍️products/💻️os/🔨️modules/🪐️space/🗿️artifacts/🪐️space/📦️packages/🦀️rust",
    "🧰️framework/🛍️products/💻️os/🖥️host/📦️packages/🦀️rust",
    "🧰️framework/🛍️products/💻️os/🧫️fixtures/⚖️scale/📦️packages/🦀️rust",
    "🧰️framework/🛍️products/🖥️server/📦️packages/🦀️rust",
    "🧰️framework/🛍️products/🦑️repo/🔨️modules/⌨️cli/📦️packages/🦀️rust",
    "🧰️framework/🛍️products/🦑️repo/🔨️modules/🌳️tree/📦️packages/🦀️rust",
    "🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/📦️packages/🦀️rust",
    "🧰️framework/🛍️products/🦑️repo/🔨️modules/🎫️tickets/📦️packages/🦀️rust",
    "🧰️framework/🛍️products/🦑️repo/🔨️modules/🎯️goals/📦️packages/🦀️rust",
    "🧰️framework/🛍️products/🦑️repo/🔨️modules/🏃️test-runner/📦️packages/🦀️rust",
    "🧰️framework/🛍️products/🦑️repo/🔨️modules/🏠️workspace/📦️packages/🦀️rust",
    "🧰️framework/🛍️products/🦑️repo/🔨️modules/📊️metrics/📦️packages/🦀️rust",
    "🧰️framework/🛍️products/🦑️repo/🔨️modules/📐️model/📦️packages/🦀️rust",
    "🧰️framework/🛍️products/🦑️repo/🔨️modules/📜️statutes/📦️packages/🦀️rust",
    "🧰️framework/🛍️products/🦑️repo/🔨️modules/📝️todos/📦️packages/🦀️rust",
    "🧰️framework/🛍️products/🦑️repo/🔨️modules/📡️events/📦️packages/🦀️rust",
    "🧰️framework/🛍️products/🦑️repo/🔨️modules/🔌️mcp/📦️packages/🦀️rust",
    "🧰️framework/🛍️products/🦑️repo/🔨️modules/🔎️search/📦️packages/🦀️rust",
    "🧰️framework/🛍️products/🦑️repo/🔨️modules/🔗️graphql/📦️packages/🦀️rust",
    "🧰️framework/🛍️products/🦑️repo/🔨️modules/🖥️server/🎛️coordinator/📦️packages/🦀️rust",
    "🧰️framework/🛍️products/🦑️repo/🔨️modules/🗂️codebase/📦️packages/🦀️rust",
    "🧰️framework/🛍️products/🦑️repo/🔨️modules/🗣️languages/📦️packages/🦀️rust",
    "🧰️framework/🛍️products/🦑️repo/🔨️modules/🚚️move/📦️packages/🦀️rust",
    "🧰️framework/🛍️products/🦑️repo/🔨️modules/🧑️contributors/📦️packages/🦀️rust",
    "🧰️framework/🛍️products/🦑️repo/🔨️modules/🧩️providers/📦️packages/🦀️rust",
    "🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🦀️rust",
    "🧰️framework/🛍️products/🦑️repo/🔨️modules/🧾️yaml/📦️packages/🦀️rust",
    "🧰️framework/🛍️products/🦑️repo/🔨️modules/🪝️hooks/📦️packages/🦀️rust",
    "🧰️framework/🛍️products/🦑️repo/🔨️modules/🪪️identity/📦️packages/🦀️rust",
]
exclude = ["**/🏅️standards/**", "**/🔮️oracles/**", "**/👽️guest/**"]

[workspace.metadata.semio.repository]
schema-version = 1
member-manifests = ["🧰️framework/**/📦️packages/🦀️rust/Cargo.toml"]
owner-manifests = ["[!.]*/Cargo.toml"]

[workspace.package]
version = "0.1.0"
edition = "2021"
rust-version = "1.95"

[workspace.dependencies]
semio-framework-schema-state = { path = "🧰️framework/🔨️modules/🧬️schema/📶️state/📦️packages/🦀️rust" }
semio-framework-schema-composition = { path = "🧰️framework/🔨️modules/🧬️schema/🧩️composition/📦️packages/🦀️rust" }
semio-framework-schema-validator = { path = "🧰️framework/🔨️modules/🧬️schema/✅️validator/📦️packages/🦀️rust" }
semio-framework-plugin-host-fixture = { path = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧫️fixtures/🧩️component/📦️packages/🦀️rust" }
semio-framework-plugin-host = { path = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/📦️packages/🦀️rust" }
semio-repo-test-host = { path = "🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🦀️rust" }
semio-framework-os-renderer-wgpu = { path = "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust" }
semio-framework-artifact-infinite-dag = { path = "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🗿️artifacts/🕸️dag/📦️packages/🦀️rust" }
semio-framework-artifact-flow-flow = { path = "🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🗿️artifacts/🌊️flow/📦️packages/🦀️rust" }
semio-framework-artifact-playbook-playbook = { path = "🧰️framework/🛍️products/💻️os/🔨️modules/📖️playbook/🗿️artifacts/📖️playbook/📦️packages/🦀️rust" }
semio-framework-artifact-workflow-workflow = { path = "🧰️framework/🛍️products/💻️os/🔨️modules/🔁️workflow/🗿️artifacts/🔁️workflow/📦️packages/🦀️rust" }
semio-framework-artifact-space-space = { path = "🧰️framework/🛍️products/💻️os/🔨️modules/🪐️space/🗿️artifacts/🪐️space/📦️packages/🦀️rust" }
semio-framework-artifact-space-collection = { path = "🧰️framework/🛍️products/💻️os/🔨️modules/🪐️space/🗿️artifacts/🗂️collection/📦️packages/🦀️rust" }
semio-framework-artifact-workflow-run = { path = "🧰️framework/🛍️products/💻️os/🔨️modules/🔁️workflow/🗿️artifacts/🏃️run/📦️packages/🦀️rust" }
semio-framework-math = { path = "🧰️framework/🔨️modules/🧮️math/📦️packages/🦀️rust" }
semio-framework-number = { path = "🧰️framework/🔨️modules/🔢️number/📦️packages/🦀️rust" }
semio-framework-geometry = { path = "🧰️framework/🔨️modules/📐️geometry/📦️packages/🦀️rust" }
semio-framework-raster = { path = "🧰️framework/🔨️modules/🖌️raster/📦️packages/🦀️rust" }
semio-framework-typeset = { path = "🧰️framework/🔨️modules/🔤️typeset/📦️packages/🦀️rust" }
semio-framework-graph = { path = "🧰️framework/🔨️modules/🕸️graph/📦️packages/🦀️rust" }
semio-framework-graph-layout-run = { path = "🧰️framework/🔨️modules/🕸️graph/⏯️layout-run/📦️packages/🦀️rust" }
semio-framework-actor = { path = "🧰️framework/🔨️modules/🎭️actor/📦️packages/🦀️rust" }
semio-framework-replication = { path = "🧰️framework/🔨️modules/📡️replication/📦️packages/🦀️rust" }
semio-framework-value = { path = "🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust" }
semio-framework-value-resident = { path = "🧰️framework/🔨️modules/🌱️value/💾️resident/📦️packages/🦀️rust" }
semio-framework-pack = { path = "🧰️framework/🔨️modules/🎒️pack/📦️packages/🦀️rust" }
pack = { path = "🧰️framework/🔨️modules/🎒️pack/📦️packages/🦀️rust", package = "semio-framework-pack" }
semio-framework-server = { path = "🧰️framework/🛍️products/🖥️server/📦️packages/🦀️rust" }
semio-framework-quiz = { path = "🧰️framework/🛍️products/❓️quiz/📦️packages/🦀️rust" }
semio-framework-async = { path = "🧰️framework/🔨️modules/⏳️async/📦️packages/🦀️rust" }
semio-framework-async-macros = { path = "🧰️framework/🔨️modules/⏳️async/✨️macros/📦️packages/🦀️rust" }
semio-framework-trace = { path = "🧰️framework/🔨️modules/⏱️trace/📦️packages/🦀️rust" }
semio-framework-job = { path = "🧰️framework/🔨️modules/🧵️job/📦️packages/🦀️rust" }
semio-framework-tool-run = { path = "🧰️framework/🔨️modules/⏯️tool-run/📦️packages/🦀️rust" }
semio-framework-time-travel = { path = "🧰️framework/🔨️modules/⏪️time-travel/📦️packages/🦀️rust" }
semio-framework-tool-machine = { path = "🧰️framework/🔨️modules/🛠️tool-machine/📦️packages/🦀️rust" }
semio-framework-dispatch-macros = { path = "🧰️framework/🔨️modules/🔀️dispatch/📦️packages/🦀️rust" }
semio-framework-hash = { path = "🧰️framework/🔨️modules/🔏️hash/📦️packages/🦀️rust" }
semio-framework-mesh-engine = { path = "🧰️framework/🔨️modules/🏗️mesh-engine/📦️packages/🦀️rust" }
semio-framework-schema = { path = "🧰️framework/🔨️modules/🧬️schema/📦️packages/🦀️rust" }
semio-framework-schema-registry = { path = "🧰️framework/🔨️modules/🧬️schema/📇️registry/📦️packages/🦀️rust" }
schema = { path = "🧰️framework/🔨️modules/🧬️schema/📦️packages/🦀️rust", package = "semio-framework-schema" }
semio-framework-value-derive = { path = "🧰️framework/🔨️modules/🌱️value/✨️derive/📦️packages/🦀️rust" }
semio-framework-os-services = { path = "🧰️framework/🛍️products/💻️os/🔨️modules/🛎️services/📦️packages/🦀️rust" }
semio-framework-plugin-describe = { path = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/📦️packages/🦀️rust" }
# 🌐️ Canonical versions for the highest-fanout external deps, chosen as the newest
# explicit requirement string already used somewhere in the 630 manifests (matches what
# Cargo.lock already resolves to, so adopting `.workspace = true` later is a no-op for
# resolution). Deps below the ~10-manifest survey bar are left alone (see ticket report).
serde = { version = "1.0.228", features = ["derive"] }
serde_json = { version = "1.0.149", features = ["raw_value"] }
wasm-bindgen = "0.2.106"
js-sys = "0.3.83"
tokio = { version = "1" }

# 🧭️ Internal path deps for crates that exist TODAY at their current location, surveyed
# by counting `path = "…"` dependency references across all Cargo.toml files (>5 other
# manifests). Not wired to any member yet — a later wave can adopt `.workspace = true` as
# a drop-in, or repoint ONE line here when a crate merges/moves instead of editing every
# consumer. `# N refs` is the survey count. Grouped: os-kernel/core, math, ui, plugin-internal.
# ---- core (24) ----
semio-framework = { path = "🧰️framework/📦️packages/🦀️rust" }  # 58 refs
semio-framework-os = { path = "🧰️framework/🛍️products/💻️os/🖥️host/📦️packages/🦀️rust" }
semio-framework-os-kernel = { path = "🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust" }
semio-s-kernel-flow-extension-brep = { path = "🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/📦️packages/🦀️rust", package = "semio-framework-os-flow" }
semio-framework-os-kernel-db-state = { path = "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📦️packages/🦀️rust", package = "semio-framework-os-kernel-db" }
semio-framework-os-kernel-db-storage = { path = "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📦️packages/🦀️rust", package = "semio-framework-os-kernel-db" }
semio-framework-os-kernel-db-wal = { path = "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📦️packages/🦀️rust", package = "semio-framework-os-kernel-db" }
semio-framework-os-kernel-flow = { path = "🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/📦️packages/🦀️rust", package = "semio-framework-os-flow" }
semio-framework-os-kernel-infinite-board-port-directed-dag = { path = "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/📦️packages/🦀️rust", package = "semio-framework-os-infinite" }
semio-framework-os-kernel-infinite-canvas = { path = "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/📦️packages/🦀️rust", package = "semio-framework-os-infinite" }
semio-framework-kernel-infinite-world = { path = "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/📦️packages/🦀️rust", package = "semio-framework-os-infinite" }
semio-framework-os-infinite = { path = "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/📦️packages/🦀️rust" }
semio-s-kernel-flow-extension-wasm = { path = "🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/📦️packages/🦀️rust", package = "semio-framework-os-flow" }
semio-framework-os-flow = { path = "🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/📦️packages/🦀️rust" }
semio-framework-os-kernel-neural-engine = { path = "🧰️framework/🛍️products/💻️os/🔨️modules/🧠️neural/⚙️engine/📦️packages/🦀️rust" }
semio-framework-os-kernel-db = { path = "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📦️packages/🦀️rust", package = "semio-framework-os-kernel-db" }
# 🧭️ Keep alias on OLD impl until W8c plugin cut-over deletes the sandwich (packages path already exists on disk).
semio-framework-plugin = { path = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust" }
semio-framework-os-config = { path = "🧰️framework/🛍️products/💻️os/🎚️config/📦️packages/🦀️rust" }
semio-framework-os-shell = { path = "🧰️framework/🛍️products/💻️os/🔨️modules/🖥️shell/📦️packages/🦀️rust" }
semio-framework-os-mcp = { path = "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🦀️rust" }
# REMOVED missing: semio-s-kernel-flow-extension-wasm = { path = "🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust" }  # 10 refs

# ---- math (13) ----

# ---- ui (10) ----
semio-framework-ui-styling = { path = "🧰️framework/🔨️modules/🖱️ui/🎨️styling/📦️packages/🦀️rust" }  # 15 refs
semio-framework-ui-contract = { path = "🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust" }
semio-framework-ui-render = { path = "🧰️framework/🔨️modules/🖱️ui/🖌️render/📦️packages/🦀️rust" }
semio-framework-ui-runtime = { path = "🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust" }
semio-framework-ui-scene = { path = "🧰️framework/🔨️modules/🖱️ui/🎬️scene/📦️packages/🦀️rust" }
semio-framework-ui-viewport = { path = "🧰️framework/🔨️modules/🖱️ui/🪟️viewport/📦️packages/🦀️rust" }
semio-framework-ui-backend-webgpu = { path = "🧰️framework/🔨️modules/🖱️ui/🖌️render/🎯️targets/🧊️webgpu/📦️packages/🦀️rust" }
semio-framework-ui-backend-metal = { path = "🧰️framework/🔨️modules/🖱️ui/🖌️render/🎯️targets/🍎️metal/📦️packages/🦀️rust" }
semio-framework-ui-backend-d3d12 = { path = "🧰️framework/🔨️modules/🖱️ui/🖌️render/🎯️targets/🪟️d3d12/📦️packages/🦀️rust" }
semio-framework-ui-backend-vulkan = { path = "🧰️framework/🔨️modules/🖱️ui/🖌️render/🎯️targets/🌋️vulkan/📦️packages/🦀️rust" }
semio-framework-ui-host = { path = "🧰️framework/🔨️modules/🖱️ui/🖥️host/📦️packages/🦀️rust" }
semio-framework-ui = { path = "🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust" }
semio-framework-surface = { path = "🧰️framework/🔨️modules/🗺️surface/📦️packages/🦀️rust" }

# ---- plugin (62) ----
semio-framework-2d = { path = "🧰️framework/🔨️modules/◻️2d/📦️packages/🦀️rust" }
semio-framework-3d = { path = "🧰️framework/🔨️modules/🧊️3d/📦️packages/🦀️rust" }
[profile.dev]
debug = false
incremental = false

[profile.dev.build-override]
opt-level = 3

# ⚡️ The plugin runtime every native dev host embeds (the semio MCP gateway, the hub, native shells) is
# optimized even in the dev profile. At `opt-level = 0` the wasmtime component-model machinery around
# every guest crossing and host call, and cranelift compiling a 129 MB wasm-dev guest, dominated the
# guest's own work. Measured on the wfc genesis `inference_run` over the semio MCP: 112.7 s with this
# set unoptimized vs 63.9 s optimized (ticket 26/09/23, `📓️wp-g5.md`), and a first solve after a guest
# rebuild waited ~9 min for its unoptimized cranelift compile. Only these crates change, so a rebuild
# costs their own compile plus a relink of their dependents (1 min 44 s measured).
[profile.dev.package.wasmtime]
opt-level = 3

[profile.dev.package.wasmtime-environ]
opt-level = 3

[profile.dev.package.wasmtime-internal-core]
opt-level = 3

[profile.dev.package.wasmtime-internal-cranelift]
opt-level = 3

[profile.dev.package.wasmtime-internal-fiber]
opt-level = 3

[profile.dev.package.wasmtime-internal-unwinder]
opt-level = 3

[profile.dev.package.wasmtime-internal-cache]
opt-level = 3

[profile.dev.package.wasmtime-internal-component-util]
opt-level = 3

[profile.dev.package.wasmtime-internal-jit-debug]
opt-level = 3

[profile.dev.package.wasmtime-internal-jit-icache-coherence]
opt-level = 3

[profile.dev.package.wasmtime-wasi]
opt-level = 3

[profile.dev.package.wasmtime-wasi-io]
opt-level = 3

[profile.dev.package.cranelift-codegen]
opt-level = 3

[profile.dev.package.cranelift-frontend]
opt-level = 3

[profile.dev.package.cranelift-entity]
opt-level = 3

[profile.dev.package.cranelift-bforest]
opt-level = 3

[profile.dev.package.cranelift-bitset]
opt-level = 3

[profile.dev.package.cranelift-control]
opt-level = 3

[profile.dev.package.cranelift-native]
opt-level = 3

[profile.dev.package.cranelift-codegen-shared]
opt-level = 3

[profile.dev.package.cranelift-assembler-x64]
opt-level = 3

[profile.dev.package.regalloc2]
opt-level = 3

[profile.dev.package.wasmparser]
opt-level = 3

[profile.dev.package.pulley-interpreter]
opt-level = 3

[profile.dev.package.gimli]
opt-level = 3

[profile.dev.package.object]
opt-level = 3

[profile.dev.package.wit-parser]
opt-level = 3

# 🧮️ The owned wasm interpreter (`semio-framework-plugin-host::interpreter`) runs every trusted-catalog
# verification `codec.pack-schema-hash` on the hub's startup path. At `opt-level = 0` a debug hub spent
# >11 min of one core interpreting writer/draw/puzzle during catalog load and the candidate readiness
# wait gave up (ticket 26/09/23 W1 §4.6, sampled: 100 % in `CoreInstance::execute_machine`).
[profile.dev.package.semio-framework-plugin-host]
opt-level = 3

# 🔐️ The repository's own SHA-256 carries every hub credential check: a sign-in derives PBKDF2-HMAC-SHA256
# at 210 000 iterations. At `opt-level = 0` one derivation took ~2 s on an idle debug hub and 8.5 s on hub
# 7800 under load (ticket 26/09/23 H9 session 12), all inside the compression function; optimizing this
# one small crate makes a dev hub's sign-in cost what a release hub's does.
[profile.dev.package.semio-framework-hash]
opt-level = 3

# 🛡️ WASI component links alone select this mitigation for rust-lld's ElemSection crash.
# Native dev retains Cargo's parallel codegen policy; publication stays wasm-release.
[profile.wasm-dev]
inherits = "dev"
codegen-units = 1
# 🧾️ A component's described bytes must be its shipped bytes: `describe` and `component-dev` link the same
# `cargo rustc --crate-type cdylib` unit, and `incremental` is part of that unit's profile identity, so an
# inherited `incremental = true` split it by the caller's `CARGO_INCREMENTAL` into two compiles with two
# different wasm hashes (ticket 26/09/23 W1: descriptor `ce48…`/`2a5c…` vs staged `a740…`).
incremental = false

# 🎚️ The plugin guest must meet the framework's 8 ms interactive-step contract even in the dev
# profile: at `opt-level = 0` a lowpoly render turn tessellates its seeded mesh in 10-11.5 ms and
# the runtime traps it with `plugin.internal.interactive-ceiling`, so the window renders a fault
# instead of geometry. Scoped to this package rather than raised on the whole profile because the
# io layer (`semio-s-plugin-stdio`) is not on the render path and is far more expensive to compile.
[profile.wasm-dev.package.semio-framework-os-flow]
opt-level = 2

# 🎚️ `Evaluator::evaluate_channels_budgeted` — the topological dag walk itself, run once per hop.
[profile.wasm-dev.package.semio-framework-os-kernel-neural-engine]
opt-level = 2

# 🎚️ `DslValue`/`OrderedMap` — the per-neuron `Dictionary` every walk clones, merges and hashes.
[profile.wasm-dev.package.semio-framework-replication]
opt-level = 2

# 🎚️ `pack::json` — the codec that serializes each node's input/output payload on the same hop.
[profile.wasm-dev.package.semio-framework-pack]
opt-level = 2

# 🎚️ Answers `capability: evaluate`/`tessellate` for the BREP operators the example drives.
[profile.release]
opt-level = 3
lto = "thin"
codegen-units = 1
strip = "debuginfo"
incremental = false
trim-paths = "object"

# 🪶️ REDUCE-DEMONSTRATOR-IDLE-MEMORY-FOOTPRINT. wasm32-wasip2 plugin components only — os-dev's
# 📜️script.ts passes `--profile wasm-release` when building plugin crates. Inherits ship-oriented
# `[profile.release]` above, then overrides for wasm size/runtime (see below).
#
# - opt-level "s" (not "z"): "z" measurably slows hot numeric loops in geometry/FEM solvers for a
#   few extra percent of size; the wasm-opt -Oz post-pass (see buildPlugin/transpilePluginComponent)
#   gets the "z"-class shrink without paying that runtime cost in every plugin.
# - lto "thin" (not "fat"): fat LTO re-links ~25 plugin cdylibs individually — 2-4x slower per plugin
#   for a low single-digit percent size gain over thin. Thin still gets full cross-crate inlining.
# - codegen-units = 1: maximizes cross-crate dedup/inlining and also sidesteps the LLVM-22
#   ElemSection::writeBody crash noted above (a stable low CGU count, same fix as `store`'s override).
# - strip = "symbols": drops the wasm `name` custom section, which is pure debug/dev-tooling weight
#   (measured ~14MB on the largest single plugin, ~87MB across the built fleet) never used at runtime.
# - incremental = false: deterministic output; release units are rebuilt whole anyway, so incremental
#   session state would only cost disk in the shared build-dir (`.cargo/config.toml` `build.build-dir`).
# - trim-paths = "object": strips absolute build-host cargo-registry paths retained in panic
#   `Location` strings (panic = "unwind" is intentionally NOT overridden here — wasm32-wasip2's
#   target spec already defaults to panic-strategy "abort", so plugins already abort-on-panic and
#   `catch_unwind` already never catches on this target; setting it explicitly would be a no-op).
[profile.wasm-release]
inherits = "release"
opt-level = "s"
lto = "thin"
codegen-units = 1
strip = "symbols"
incremental = false
trim-paths = "object"

# Explicit: profile inheritance does NOT inherit package-specific overrides from the parent
# profile, so without this `store` would fall back to workspace defaults under `wasm-release`.
[profile.wasm-release.package.semio-framework-os-kernel]
codegen-units = 1

# 🧹️ RUST-WIDE-CLEAN-REFACTOR-CAMPAIGN baseline. Kept at "warn" (never "deny") in
# the manifest so live concurrent edits never hard-break; zero-warning is enforced
# at verification gates via `cargo clippy -- -D warnings`. NEVER set RUSTFLAGS to
# add -D warnings — it replaces (not merges) .cargo/config.toml's rustflags.
[workspace.lints.rust]
future_incompatible = { level = "warn", priority = -1 }
rust_2018_idioms = { level = "warn", priority = -1 }
unsafe_op_in_unsafe_fn = "warn"
unused_lifetimes = "warn"
unused_qualifications = "warn"

[workspace.lints.clippy]
all = { level = "warn", priority = -1 }
cloned_instead_of_copied = "warn"
inefficient_to_string = "warn"
map_unwrap_or = "warn"
needless_pass_by_value = "warn"
semicolon_if_nothing_returned = "warn"
unnecessary_wraps = "warn"
redundant_clone = "warn"
# phase B (enable after the T2/T3 waves land): unwrap_used = "warn"

```

## Direct-owner cohort additional full before receipts

### 🧰️framework/🔨️modules/🕸️graph/🛂️manifest/🦀️.rs

SHA-256 `a42d637024e64209be3467facf114a05c0cb3ccfe21b0e6917a85f50130cbf99`; 31572 bytes.

```
//! 📜️ Compile-time graph manifest kernel: schema, registry, and strict validation.

use semio_framework_value::{ValueKind, ValueType};

#[path = "🛬️type/🦀️.rs"]
mod type_binding;

pub use crate::manifest::Manifest as GraphManifest;

//#region ⚠️ Errors
// 🌉️ `value_type_from_value` below is a `#[value(deserialize_with = "...")]` hook (RUNTIME-
// DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS, 26/09/02 Phase 2) — the direct successor of
// the old serde `deserialize_with = "deserialize_value_type"` hook (and its `GraphManifestError`/
// `parse_value_type_value` helpers), NOT dead code: `ValueType`'s own native `dsl_core::FromValue`
// impl decodes its internally-tagged `{"kind": "boolean"}` shape only, but every `*.manifest.json`
// fixture (embedded verbatim as `${PREFIX}_MANIFEST_JSON` by `🤖️generated/🦀️*.rs`) spells
// `valueType` as a bare string (`"boolean"`/`"text"`/...) or a `{"schema": "..."}` object — the
// same gap the serde hook used to bridge. Confirmed by `cargo test -p semio-framework-graph`:
// dropping this hook broke `nakagin_manifest_loads` et al. with `ValueError("nodeKinds.41.
// properties.0.valueType.kind")` before this fix landed. The encode direction needs no matching
// hook: `ValueType::to_value`'s native shape is self-consistent for `Manifest::to_value()`'s own
// round trip (nothing needs it to reproduce the fixture text byte-for-byte) — the old
// `serialize_value_type` hook was itself just a thin wrapper over the same native `to_value` call.
fn value_type_from_value(value: dsl_core::DslValue) -> Result<ValueType, dsl_core::ValueError> {
    if let Ok(value_type) = <ValueType as dsl_core::FromValue>::from_value(value.clone()) {
        return Ok(value_type);
    }
    match value {
        dsl_core::DslValue::String(s) => Ok(match s.as_str() {
            "boolean" | "bool" => ValueType::Boolean,
            "integer" | "int" => ValueType::Integer,
            "number" | "decimal" | "float" => ValueType::Decimal,
            "text" | "string" => ValueType::Text,
            "object" | "any" => ValueType::Any,
            _ => ValueType::Schema(s),
        }),
        dsl_core::DslValue::Object(entries) if entries.len() == 1 => match entries.first() {
            Some((key, dsl_core::DslValue::String(schema))) if key == "schema" => Ok(ValueType::Schema(schema.clone())),
            _ => Err(dsl_core::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("unsupported valueType object {:?}", dsl_core::DslValue::Object(entries)))),
        },
        other => Err(dsl_core::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("unsupported valueType {other:?}"))),
    }
}
//#endregion ⚠️ Errors

// #region 🔖️Property
/// 📊️ Runtime property value for graph instances.
#[derive(Clone, Debug, Default, PartialEq)]
pub enum PropertyValue {
    #[default]
    Null,
    Bool(bool),
    Number(f64),
    String(String),
    Array(Vec<PropertyValue>),
    Object(PropertyBag),
}

#[path = "♻️retirement/🦀️.rs"]
mod retirement;
#[path="🗂️properties/🦀️.rs"]
mod property_members;
pub use property_members::PropertyBag;
#[path="🚦️properties/🦀️.rs"]
mod property_control;

impl PropertyValue {
    // 🚫️async: E1 pure accessor passed by name into `Option::and_then` (a sync fn-pointer slot) at
    // every call site in this crate; no consumer awaits it directly. See R9.
    pub fn as_str(&self) -> Option<&str> {
        match self {
            Self::String(s) => Some(s.as_str()),
            _ => None,
        }
    }

    // 🚫️async: E1 pure accessor, same reason as `as_str` above — see R9.
    pub fn as_f64(&self) -> Option<f64> {
        match self {
            Self::Number(n) => Some(*n),
            _ => None,
        }
    }

    pub fn as_object(&self) -> Option<&PropertyBag> {
        match self {
            Self::Object(m) => Some(m),
            _ => None,
        }
    }
}

//#region 🔖️DslField
// 🌱️ `PropertyValue` is structurally a dynamic JSON-equivalent literal (Null/Bool/Number/String/
// Array/Object), exactly like `dsl_core::DslValue` itself, so it binds as `Shape::Value` rather than
// through `#[derive(dsl_core::DslEnum)]`: the derive's tuple-variant codegen treats every single-field
// unnamed variant as a "newtype" delegating to the inner type's own `Shape::Record` (see
// `dsl_core::__rt::newtype_variant_spec`), which panics for a primitive/collection inner type such as
// `bool`/`f64`/`Vec<Self>`/`BTreeMap<String, Self>` — none of which are `Shape::Record`. Binding
// directly through `DslValue` (mirroring the engine's own `serde_json::Value` bridge) is both
// correct and the natural fit for an untyped recursive value type, and it needs no attributes on
// the Array/Object variants: recursion is carried by `DslValue` itself, not by field-level nesting.
fn property_value_to_dsl_value(value: &PropertyValue) -> dsl_core::DslValue {
    match value {
        PropertyValue::Null => dsl_core::DslValue::Null,
        PropertyValue::Bool(b) => dsl_core::DslValue::Bool(*b),
        PropertyValue::Number(n) => dsl_core::DslValue::float(*n),
        PropertyValue::String(s) => dsl_core::DslValue::String(s.clone()),
        PropertyValue::Array(items) => {
            // 🔀️ Plain sync recursion — no suspension point, so no `Box::pin` is needed.
            let mut out = Vec::with_capacity(items.len());
            for item in items {
                out.push(property_value_to_dsl_value(item));
            }
            dsl_core::DslValue::Array(out)
        }
        PropertyValue::Object(map) => {
            let mut out = Vec::with_capacity(map.len());
            for (k, v) in map {
                out.push((k.clone(), property_value_to_dsl_value(v)));
            }
            dsl_core::DslValue::Object(out)
        }
    }
}

fn dsl_value_to_property_value(value: &dsl_core::DslValue) -> PropertyValue {
    match value {
        dsl_core::DslValue::Null => PropertyValue::Null,
        dsl_core::DslValue::Bool(b) => PropertyValue::Bool(*b),
        dsl_core::DslValue::Number(n) => PropertyValue::Number(n.as_f64()),
        dsl_core::DslValue::String(s) => PropertyValue::String(s.clone()),
        dsl_core::DslValue::Bytes(bytes) => PropertyValue::Array(bytes.iter().map(|byte| PropertyValue::Number(f64::from(*byte))).collect()),
        dsl_core::DslValue::Array(items) => {
            // 🔀️ Same rewrite as `property_value_to_dsl_value` above, mirrored.
            let mut out = Vec::with_capacity(items.len());
            for item in items {
                out.push(dsl_value_to_property_value(item));
            }
            PropertyValue::Array(out)
        }
        dsl_core::DslValue::Object(entries) => {
            let mut out = PropertyBag::new();
            for (k, v) in entries {
                out.insert(k.clone(), dsl_value_to_property_value(v));
            }
            PropertyValue::Object(out)
        }
    }
}

impl dsl_core::DslField for PropertyValue {
    fn shape_controlled<C:dsl_core::NativeSchemaControl>(control:&mut C)->Result<dsl_core::Shape,dsl_core::ValueError>{control.checkpoint()?;Ok(dsl_core::Shape::Value)}
    fn to_value_controlled(&self,control:&mut dsl_core::NativeEncodeControl<'_>)->Result<dsl_core::FieldValue,dsl_core::ValueError>{property_control::encode(self,control).map(dsl_core::FieldValue::Value)}
    fn from_value_controlled(value:&dsl_core::FieldValue,control:&mut dsl_core::NativeDecodeControl<'_>)->Result<Self,dsl_core::ValueError>{match value{dsl_core::FieldValue::Value(value)=>property_control::decode(value,control),_=>Err(dsl_core::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue,"graph property requires intrinsic field value"))}}
    fn retire_decoded(self){property_control::retire(self)}
    // 🚫️async: E1 impl of externally-declared trait `dsl_core::DslField` — every method is
    // E4-tagged sync in the trait itself (fn-pointer transitivity through `Shape::Record`/`Table`),
    // see `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🦀️.rs`.
    fn shape() -> dsl_core::Shape {
        dsl_core::Shape::Value
    }

    fn to_value(&self) -> dsl_core::FieldValue {
        dsl_core::FieldValue::Value(property_value_to_dsl_value(self))
    }

    fn from_value(value: &dsl_core::FieldValue) -> Result<Self, String> {
        match value {
            dsl_core::FieldValue::Value(dsl_value) => Ok(dsl_value_to_property_value(dsl_value)),
            other => Err(format!("expected Value, found {other:?}")),
        }
    }
}
//#endregion 🔖️DslField

//#region 🔖️ToFromValue
/// 🌱️ `ToValue`/`FromValue` (the `DslValue`-tree pair `Mutation`/`MutationDiff` payloads need —
/// distinct from `DslField`/`FieldValue` above, the text/binary DSL grammar's own trait, see that
/// region's header note) for the identical reason `DslField` binds as `Shape::Value`: reuse the
/// same recursive `property_value_to_dsl_value`/`dsl_value_to_property_value` walk rather than a
/// second one. An untagged enum (this was `#[serde(untagged)]`, now serde-free — Phase 2,
/// RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS 26/09/02) has no `#[derive(ToValue,
/// FromValue)]` equivalent — it needs exactly this kind of hand-written structural match, per the
/// fan-out playbook's "Not supported by the derive" list.
impl dsl_core::ToValue for PropertyValue {
    fn to_value_controlled(&self,control:&mut dsl_core::NativeEncodeControl<'_>)->Result<dsl_core::DslValue,dsl_core::ValueError>{property_control::encode(self,control)}
    fn to_value(&self) -> dsl_core::DslValue {
        property_value_to_dsl_value(self)
    }
}

impl dsl_core::FromValue for PropertyValue {
    fn from_value_controlled(value:&dsl_core::DslValue,control:&mut dsl_core::NativeDecodeControl<'_>)->Result<Self,dsl_core::ValueError>{property_control::decode(value,control)}
    fn default_value_controlled(control:&mut dsl_core::NativeDecodeControl<'_>)->Result<Self,dsl_core::ValueError>{control.checkpoint()?;Ok(Self::Null)}
    fn retire_decoded(self){property_control::retire(self)}
    fn from_value(value: dsl_core::DslValue) -> Result<Self, dsl_core::ValueError> {
        Ok(dsl_value_to_property_value(&value))
    }
}
//#endregion 🔖️ToFromValue

/// 🏷️ Compile-time property kind.
#[derive(Clone, Copy, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub enum PropertyKind {
    Data,
    Derived,
}

/// 📋️ Property definition on a kind.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct PropertyDef {
    pub name: String,
    pub kind: PropertyKind,
    // 🌉️ `deserialize_with` mirrors the old serde hook — see `value_type_from_value`'s own
    // docstring above (this crate's `⚠️ Errors` region) for why it is still needed. The plain
    // per-field `ToValue::to_value` stays for the encode direction (no `serialize_with`).
    #[value(default, deserialize_with = "value_type_from_value", deserialize_controlled_with = "type_binding::decode", retire_with = "type_binding::retire")]
    pub value_type: ValueType,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub expr: Option<String>,
}

/// 📏️ The supplied retirement byte grant cannot release the exact schema string.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ValueTypeRetirementError {
    pub required_bytes: usize,
    pub maximum_bytes: usize,
}

impl std::fmt::Display for ValueTypeRetirementError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "value type retirement requires {} bytes, granted {}", self.required_bytes, self.maximum_bytes)
    }
}

impl std::error::Error for ValueTypeRetirementError {}

impl PropertyDef {
    /// 🧹️ Detaches at most one exact nested value-type string or list box. A terminal
    /// definition has only the definitionally shallow `Any` tag left for its final drop.
    pub fn retire_value_type_step(&mut self, maximum_bytes: usize) -> Result<Option<String>, ValueTypeRetirementError> {
        if let ValueType::Schema(value) = &self.value_type {
            if value.len() > maximum_bytes {
                return Err(ValueTypeRetirementError { required_bytes: value.len(), maximum_bytes });
            }
        }
        match std::mem::take(&mut self.value_type) {
            ValueType::Schema(value) => Ok(Some(value)),
            ValueType::List(value) => {
                self.value_type = *value;
                Ok(None)
            }
            _ => Ok(None),
        }
    }

    pub fn value_type_terminal_is_empty(&self) -> bool {
        matches!(self.value_type, ValueType::Any)
    }
}


// #endregion 🔖️Property

// #region 🔖️Manifest
/// 🔌️ Port direction on a node.
#[derive(Clone, Copy, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub enum PortDirection {
    In,
    Out,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub enum PortModelAxis {
    #[default]
    Ported,
    Normal,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub enum DirectednessAxis {
    #[default]
    Directed,
    Undirected,
}

#[derive(Clone, Debug, PartialEq, Default, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct ManifestAxes {
    #[value(default)]
    pub port_model: PortModelAxis,
    #[value(default)]
    pub directedness: DirectednessAxis,
}

/// 🏷️ Kind row in a manifest family.
///
/// 🌉️ Hand-written, not derived: `#[derive(ToValue, FromValue)]` requires every field's type to
/// carry the same rename convention as a per-field attribute, but `presentation` is already the
/// schema-erased `DslValue` itself (arbitrary-shaped, no meaningful rename), so it is simpler to
/// spell the whole impl by hand than to special-case one field's attribute.
#[derive(Clone, Debug, PartialEq)]
pub struct KindDef {
    pub id: String,
    pub name: String,
    pub properties: Vec<PropertyDef>,
    pub ports: Vec<String>,
    pub direction: Option<PortDirection>,
    pub presentation: Option<dsl_core::DslValue>,
}

impl dsl_core::ToValue for KindDef {
    fn to_value(&self) -> dsl_core::DslValue {
        let mut entries: Vec<(String, dsl_core::DslValue)> = vec![
            ("id".to_string(), dsl_core::ToValue::to_value(&self.id)),
            ("name".to_string(), dsl_core::ToValue::to_value(&self.name)),
            ("properties".to_string(), dsl_core::ToValue::to_value(&self.properties)),
            ("ports".to_string(), dsl_core::ToValue::to_value(&self.ports)),
        ];
        if self.direction.is_some() {
            entries.push(("direction".to_string(), dsl_core::ToValue::to_value(&self.direction)));
        }
        if let Some(presentation) = &self.presentation {
            entries.push(("presentation".to_string(), presentation.clone()));
        }
        dsl_core::DslValue::object(entries)
    }
}

impl dsl_core::FromValue for KindDef {
    fn from_value(value: dsl_core::DslValue) -> Result<Self, dsl_core::ValueError> {
        let dsl_core::DslValue::Object(fields) = value else {
            return Err(dsl_core::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("expected an object for KindDef, found {value:?}")));
        };
        let mut id = None;
        let mut name = String::new();
        let mut properties = Vec::new();
        let mut ports = Vec::new();
        let mut direction = None;
        let mut presentation = None;
        for (key, entry) in fields {
            match key.as_str() {
                "id" => id = Some(<String as dsl_core::FromValue>::from_value(entry).map_err(|e| e.under("id"))?),
                "name" => name = <String as dsl_core::FromValue>::from_value(entry).map_err(|e| e.under("name"))?,
                "properties" => properties = <Vec<PropertyDef> as dsl_core::FromValue>::from_value(entry).map_err(|e| e.under("properties"))?,
                "ports" => ports = <Vec<String> as dsl_core::FromValue>::from_value(entry).map_err(|e| e.under("ports"))?,
                "direction" => direction = Some(<PortDirection as dsl_core::FromValue>::from_value(entry).map_err(|e| e.under("direction"))?),
                "presentation" => presentation = Some(entry),
                _ => {}
            }
        }
        Ok(KindDef {
            id: id.ok_or_else(|| dsl_core::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "KindDef missing id"))?,
            name,
            properties,
            ports,
            direction,
            presentation,
        })
    }
}

impl KindDef {
    pub fn display_name(&self) -> &str {
        if self.name.is_empty() {
            &self.id
        } else {
            &self.name
        }
    }
}

/// 📜️ Compile-time schema for a graph.
///
/// 🌉️ Hand-written, not derived — same reason as `KindDef` above: `edge_tips`/`kind_compatibility`
/// are schema-erased `DslValue` trees, arbitrary-shaped, so a plain field-list derive buys nothing
/// over spelling the impl directly.
#[derive(Clone, Debug, PartialEq)]
pub struct Manifest {
    pub schema: String,
    pub id: String,
    pub name: String,
    pub axes: ManifestAxes,
    pub node_kinds: Vec<KindDef>,
    pub edge_kinds: Vec<KindDef>,
    pub port_kinds: Vec<KindDef>,
    pub wire_kinds: Vec<KindDef>,
    pub layer_kinds: Vec<KindDef>,
    pub language_kinds: Vec<KindDef>,
    pub surface_kinds: Vec<KindDef>,
    pub window_kinds: Vec<KindDef>,
    pub file_node_kinds: Vec<KindDef>,
    pub descriptor_kinds: Vec<KindDef>,
    pub edge_tips: Vec<dsl_core::DslValue>,
    pub kind_compatibility: Vec<dsl_core::DslValue>,
}

impl dsl_core::ToValue for Manifest {
    fn to_value(&self) -> dsl_core::DslValue {
        dsl_core::DslValue::object([
            ("schema".to_string(), dsl_core::ToValue::to_value(&self.schema)),
            ("id".to_string(), dsl_core::ToValue::to_value(&self.id)),
            ("name".to_string(), dsl_core::ToValue::to_value(&self.name)),
            ("axes".to_string(), dsl_core::ToValue::to_value(&self.axes)),
            ("nodeKinds".to_string(), dsl_core::ToValue::to_value(&self.node_kinds)),
            ("edgeKinds".to_string(), dsl_core::ToValue::to_value(&self.edge_kinds)),
            ("portKinds".to_string(), dsl_core::ToValue::to_value(&self.port_kinds)),
            ("wireKinds".to_string(), dsl_core::ToValue::to_value(&self.wire_kinds)),
            ("layerKinds".to_string(), dsl_core::ToValue::to_value(&self.layer_kinds)),
            ("languageKinds".to_string(), dsl_core::ToValue::to_value(&self.language_kinds)),
            ("surfaceKinds".to_string(), dsl_core::ToValue::to_value(&self.surface_kinds)),
            ("windowKinds".to_string(), dsl_core::ToValue::to_value(&self.window_kinds)),
            ("fileNodeKinds".to_string(), dsl_core::ToValue::to_value(&self.file_node_kinds)),
            ("descriptorKinds".to_string(), dsl_core::ToValue::to_value(&self.descriptor_kinds)),
            ("edgeTips".to_string(), dsl_core::DslValue::Array(self.edge_tips.clone())),
            ("kindCompatibility".to_string(), dsl_core::DslValue::Array(self.kind_compatibility.clone())),
        ])
    }
}

impl dsl_core::FromValue for Manifest {
    fn from_value(value: dsl_core::DslValue) -> Result<Self, dsl_core::ValueError> {
        let dsl_core::DslValue::Object(fields) = value else {
            return Err(dsl_core::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("expected an object for Manifest, found {value:?}")));
        };
        let mut schema = None;
        let mut id = None;
        let mut name = String::new();
        let mut axes = ManifestAxes::default();
        let mut node_kinds = Vec::new();
        let mut edge_kinds = Vec::new();
        let mut port_kinds = Vec::new();
        let mut wire_kinds = Vec::new();
        let mut layer_kinds = Vec::new();
        let mut language_kinds = Vec::new();
        let mut surface_kinds = Vec::new();
        let mut window_kinds = Vec::new();
        let mut file_node_kinds = Vec::new();
        let mut descriptor_kinds = Vec::new();
        let mut edge_tips = Vec::new();
        let mut kind_compatibility = Vec::new();
        for (key, entry) in fields {
            match key.as_str() {
                "schema" => schema = Some(<String as dsl_core::FromValue>::from_value(entry).map_err(|e| e.under("schema"))?),
                "id" => id = Some(<String as dsl_core::FromValue>::from_value(entry).map_err(|e| e.under("id"))?),
                "name" => name = <String as dsl_core::FromValue>::from_value(entry).map_err(|e| e.under("name"))?,
                "axes" => axes = <ManifestAxes as dsl_core::FromValue>::from_value(entry).map_err(|e| e.under("axes"))?,
                "nodeKinds" => node_kinds = <Vec<KindDef> as dsl_core::FromValue>::from_value(entry).map_err(|e| e.under("nodeKinds"))?,
                "edgeKinds" => edge_kinds = <Vec<KindDef> as dsl_core::FromValue>::from_value(entry).map_err(|e| e.under("edgeKinds"))?,
                "portKinds" => port_kinds = <Vec<KindDef> as dsl_core::FromValue>::from_value(entry).map_err(|e| e.under("portKinds"))?,
                "wireKinds" => wire_kinds = <Vec<KindDef> as dsl_core::FromValue>::from_value(entry).map_err(|e| e.under("wireKinds"))?,
                "layerKinds" => layer_kinds = <Vec<KindDef> as dsl_core::FromValue>::from_value(entry).map_err(|e| e.under("layerKinds"))?,
                "languageKinds" => language_kinds = <Vec<KindDef> as dsl_core::FromValue>::from_value(entry).map_err(|e| e.under("languageKinds"))?,
                "surfaceKinds" => surface_kinds = <Vec<KindDef> as dsl_core::FromValue>::from_value(entry).map_err(|e| e.under("surfaceKinds"))?,
                "windowKinds" => window_kinds = <Vec<KindDef> as dsl_core::FromValue>::from_value(entry).map_err(|e| e.under("windowKinds"))?,
                "fileNodeKinds" => file_node_kinds = <Vec<KindDef> as dsl_core::FromValue>::from_value(entry).map_err(|e| e.under("fileNodeKinds"))?,
                "descriptorKinds" => descriptor_kinds = <Vec<KindDef> as dsl_core::FromValue>::from_value(entry).map_err(|e| e.under("descriptorKinds"))?,
                "edgeTips" => {
                    let dsl_core::DslValue::Array(items) = entry else {
                        return Err(dsl_core::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "expected an array for edgeTips").under("edgeTips"));
                    };
                    edge_tips = items;
                }
                "kindCompatibility" => {
                    let dsl_core::DslValue::Array(items) = entry else {
                        return Err(dsl_core::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "expected an array for kindCompatibility").under("kindCompatibility"));
                    };
                    kind_compatibility = items;
                }
                _ => {}
            }
        }
        Ok(Manifest {
            schema: schema.ok_or_else(|| dsl_core::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "Manifest missing schema"))?,
            id: id.ok_or_else(|| dsl_core::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "Manifest missing id"))?,
            name,
            axes,
            node_kinds,
            edge_kinds,
            port_kinds,
            wire_kinds,
            layer_kinds,
            language_kinds,
            surface_kinds,
            window_kinds,
            file_node_kinds,
            descriptor_kinds,
            edge_tips,
            kind_compatibility,
        })
    }
}

impl Manifest {
    pub fn node_kind(&self, id: &str) -> Option<&KindDef> {
        self.node_kinds.iter().find(|k| k.id == id)
    }

    pub fn edge_kind(&self, id: &str) -> Option<&KindDef> {
        self.edge_kinds.iter().find(|k| k.id == id)
    }

    pub fn port_kind(&self, id: &str) -> Option<&KindDef> {
        self.port_kinds.iter().find(|k| k.id == id)
    }

    pub fn wire_kind(&self, id: &str) -> Option<&KindDef> {
        self.wire_kinds.iter().find(|k| k.id == id)
    }

    pub fn layer_kind(&self, id: &str) -> Option<&KindDef> {
        self.layer_kinds.iter().find(|k| k.id == id)
    }

    pub fn language_kind(&self, id: &str) -> Option<&KindDef> {
        self.language_kinds.iter().find(|k| k.id == id)
    }

}
// #endregion 🔖️Manifest

// #region 🔖️Validator
/// 🛡️ Strict manifest validation errors.
#[derive(Clone, Debug, PartialEq)]
pub struct ManifestValidationError {
    pub path: String,
    pub message: String,
}

impl ManifestValidationError {
    fn new(path: impl Into<String>, message: impl Into<String>) -> Self {
        Self { path: path.into(), message: message.into() }
    }
}

/// 🛡️ Validates runtime graph instances against a compile-time manifest.
#[derive(Clone, Debug)]
pub struct ManifestValidator<'a> {
    manifest: &'a Manifest,
}

impl<'a> ManifestValidator<'a> {
    pub fn new(manifest: &'a Manifest) -> Self {
        Self { manifest }
    }

    pub fn validate_node_kind(&self, kind: &str) -> Result<(), ManifestValidationError> {
        if self.manifest.node_kind(kind).is_some() {
            Ok(())
        } else {
            Err(ManifestValidationError::new(format!("nodes/{kind}"), format!("unknown node kind {kind:?}")))
        }
    }

    pub fn validate_edge_kind(&self, kind: &str) -> Result<(), ManifestValidationError> {
        if self.manifest.edge_kind(kind).is_some() {
            Ok(())
        } else {
            Err(ManifestValidationError::new(format!("edges/{kind}"), format!("unknown edge kind {kind:?}")))
        }
    }

    pub fn validate_port_kind(&self, kind: &str) -> Result<(), ManifestValidationError> {
        if self.manifest.port_kind(kind).is_some() {
            Ok(())
        } else {
            Err(ManifestValidationError::new(format!("ports/{kind}"), format!("unknown port kind {kind:?}")))
        }
    }

    pub fn validate_wire_kind(&self, kind: &str) -> Result<(), ManifestValidationError> {
        if self.manifest.wire_kind(kind).is_some() {
            Ok(())
        } else {
            Err(ManifestValidationError::new(format!("wires/{kind}"), format!("unknown wire kind {kind:?}")))
        }
    }

    pub fn validate_layer_kind(&self, kind: &str) -> Result<(), ManifestValidationError> {
        if self.manifest.layer_kind(kind).is_some() {
            Ok(())
        } else {
            Err(ManifestValidationError::new(format!("layers/{kind}"), format!("unknown layer kind {kind:?}")))
        }
    }

    pub fn validate_language_kind(&self, kind: &str) -> Result<(), ManifestValidationError> {
        if self.manifest.language_kind(kind).is_some() {
            Ok(())
        } else {
            Err(ManifestValidationError::new(format!("languages/{kind}"), format!("unknown language kind {kind:?}")))
        }
    }

    pub fn validate_node_properties(&self, kind: &str, properties: &PropertyBag) -> Result<(), ManifestValidationError> {
        let Some(def) = self.manifest.node_kind(kind) else {
            return self.validate_node_kind(kind);
        };
        self.validate_property_bag(&format!("nodes/{kind}/properties"), &def.properties, properties)
    }

    pub fn validate_edge_properties(&self, kind: &str, properties: &PropertyBag) -> Result<(), ManifestValidationError> {
        let Some(def) = self.manifest.edge_kind(kind) else {
            return self.validate_edge_kind(kind);
        };
        self.validate_property_bag(&format!("edges/{kind}/properties"), &def.properties, properties)
    }

    fn validate_property_bag(&self, path: &str, defs: &[PropertyDef], bag: &PropertyBag) -> Result<(), ManifestValidationError> {
        for def in defs {
            if def.kind == PropertyKind::Derived {
                continue;
            }
            let Some(value) = bag.get(&def.name) else {
                continue;
            };
            if !property_value_matches_type(value, &def.value_type) {
                return Err(ManifestValidationError::new(format!("{path}/{}", def.name), format!("property type mismatch for {}", def.value_type.id())));
            }
        }
        for key in bag.keys() {
            if !defs.iter().any(|d| d.name == *key) {
                return Err(ManifestValidationError::new(format!("{path}/{key}"), format!("unknown property {key:?}")));
            }
        }
        Ok(())
    }

    pub fn validate_trinity_graph(&self, nodes: &[TrinityNodeRef<'_>], edges: &[TrinityEdgeRef<'_>]) -> Result<(), ManifestValidationError> {
        for node in nodes {
            self.validate_node_kind(node.kind)?;
            self.validate_node_properties(node.kind, node.properties)?;
            for port in node.ports {
                self.validate_port_kind(port.kind)?;
                if let Some(node_def) = self.manifest.node_kind(node.kind) {
                    if !node_def.ports.is_empty() && !node_def.ports.iter().any(|p| p == port.kind) {
                        return Err(ManifestValidationError::new(format!("nodes/{}/ports/{}", node.id, port.kind), format!("port kind {} not declared on node kind {}", port.kind, node.kind)));
                    }
                }
            }
        }
        for edge in edges {
            self.validate_edge_kind(edge.kind)?;
            self.validate_edge_properties(edge.kind, edge.properties)?;
        }
        Ok(())
    }
}

fn property_value_matches_type(value: &PropertyValue, expected: &ValueType) -> bool {
    if matches!(expected, ValueType::Any) {
        return true;
    }
    match value {
        PropertyValue::Object(_) if matches!(expected, ValueType::Schema(_)) => true,
        _ => {
            expected.matches(property_value_kind(value))
        }
    }
}

fn property_value_kind(value: &PropertyValue) -> ValueKind<'_> {
    match value {
        PropertyValue::Null => ValueKind::Null,
        PropertyValue::Bool(_) => ValueKind::Boolean,
        PropertyValue::Number(_) => ValueKind::Decimal,
        PropertyValue::String(_) => ValueKind::Text,
        PropertyValue::Array(_) | PropertyValue::Object(_) => ValueKind::Null,
    }
}

/// 🔌️ Trinity node reference for validation.
#[derive(Clone, Debug)]
pub struct TrinityNodeRef<'a> {
    pub id: &'a str,
    pub kind: &'a str,
    pub properties: &'a PropertyBag,
    pub ports: &'a [TrinityPortRef<'a>],
}

/// 🔌️ Trinity port reference for validation.
#[derive(Clone, Debug)]
pub struct TrinityPortRef<'a> {
    pub kind: &'a str,
}

/// 🔗️ Trinity edge reference for validation.
#[derive(Clone, Debug)]
pub struct TrinityEdgeRef<'a> {
    pub kind: &'a str,
    pub properties: &'a PropertyBag,
}

// #endregion 🔖️Validator

// #region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
// #endregion 🔖️Tests

#[cfg(test)]
#[path = "🧪️tests/🏷️type/🦀️.rs"]
mod type_tests;

#[path = "🪆️binding/🦀️.rs"]
mod property_declaration_binding;

```

### 🧰️framework/🔨️modules/🕸️graph/🛂️manifest/🗂️properties/🦀️.rs

SHA-256 `20004a364e0de469a0428221b8fb7870c04eee4d53d09eb16015b37f85b85ddc`; 8702 bytes.

```
//! 🗂️ Literal unique graph properties own concrete sorted contiguous member slots.
use super::PropertyValue;
use semio_framework_value::{DecodedValue,DslValue,FromValue,NativeDecodeControl,NativeEncodeControl,ToValue,ValueError,ValueRefusalKind};
#[derive(Clone,Debug,Default,PartialEq)]
pub struct PropertyBag { members:Vec<(String,PropertyValue)> }
impl PropertyBag {
 pub fn new()->Self{Self::default()}
 pub fn len(&self)->usize{self.members.len()}
 pub fn is_empty(&self)->bool{self.members.is_empty()}
 pub fn iter(&self)->impl DoubleEndedIterator<Item=(&String,&PropertyValue)>+ExactSizeIterator{self.members.iter().map(|(key,value)|(key,value))}
 pub fn keys(&self)->impl DoubleEndedIterator<Item=&String>+ExactSizeIterator{self.members.iter().map(|(key,_)|key)}
 pub fn values(&self)->impl DoubleEndedIterator<Item=&PropertyValue>+ExactSizeIterator{self.members.iter().map(|(_,value)|value)}
 pub fn into_values(self)->impl DoubleEndedIterator<Item=PropertyValue>+ExactSizeIterator{self.members.into_iter().map(|(_,value)|value)}
 pub fn get(&self,key:&str)->Option<&PropertyValue>{self.members.binary_search_by(|(name,_)|name.as_str().cmp(key)).ok().map(|index|&self.members[index].1)}
 pub fn get_mut(&mut self,key:&str)->Option<&mut PropertyValue>{self.members.binary_search_by(|(name,_)|name.as_str().cmp(key)).ok().map(|index|&mut self.members[index].1)}
 pub fn contains_key(&self,key:&str)->bool{self.get(key).is_some()}
 pub fn insert(&mut self,key:String,value:PropertyValue)->Option<PropertyValue>{match self.members.binary_search_by(|(name,_)|name.cmp(&key)){Ok(index)=>Some(std::mem::replace(&mut self.members[index].1,value)),Err(index)=>{self.members.insert(index,(key,value));None}}}
 pub fn remove(&mut self,key:&str)->Option<PropertyValue>{self.members.binary_search_by(|(name,_)|name.as_str().cmp(key)).ok().map(|index|self.members.remove(index).1)}
 pub fn pop_last(&mut self)->Option<(String,PropertyValue)>{self.members.pop()}
 pub fn successor(&self,key:&str)->Option<(&String,&PropertyValue)>{self.members.get(self.members.partition_point(|(name,_)|name.as_str()<=key)).map(|(key,value)|(key,value))}
 pub fn first_key_value(&self)->Option<(&String,&PropertyValue)>{self.members.first().map(|(key,value)|(key,value))}
 pub fn from_admitted(members:Vec<(String,PropertyValue)>)->Self{Self{members}}
 pub fn admitted_insert(&mut self,key:String,value:PropertyValue,control:&mut NativeDecodeControl<'_>)->Result<(),ValueError>{
  self.insert_controlled(key,value,&mut||control.step())
 }
 pub fn insert_controlled(&mut self,key:String,value:PropertyValue,checkpoint:&mut dyn FnMut()->Result<(),ValueError>)->Result<(),ValueError>{
  let value=DecodedValue::new(value,PropertyValue::retire_decoded);
  let mut index=0;while index<self.members.len(){checkpoint()?;match compare(&self.members[index].0,&key,checkpoint)?{std::cmp::Ordering::Less=>index+=1,std::cmp::Ordering::Equal=>{PropertyValue::retire_decoded(std::mem::replace(&mut self.members[index].1,value.take()));return Ok(())},std::cmp::Ordering::Greater=>break}}
  if self.members.len()==self.members.capacity(){return Err(ValueError::new(ValueRefusalKind::InvariantViolated,"property map exceeds admitted member slots"));}
  self.members.push((key,value.take()));let mut position=self.members.len()-1;while position>index{self.members.swap(position,position-1);position-=1;checkpoint()?;}Ok(())
 }
}
fn compare(left:&str,right:&str,checkpoint:&mut dyn FnMut()->Result<(),ValueError>)->Result<std::cmp::Ordering,ValueError>{for(index,(left,right))in left.bytes().zip(right.bytes()).enumerate(){if index%256==0{checkpoint()?;}if left!=right{return Ok(left.cmp(&right))}}Ok(left.len().cmp(&right.len()))}
impl<const N:usize> From<[(String,PropertyValue);N]> for PropertyBag{fn from(values:[(String,PropertyValue);N])->Self{values.into_iter().collect()}}
impl FromIterator<(String,PropertyValue)> for PropertyBag{fn from_iter<T:IntoIterator<Item=(String,PropertyValue)>>(values:T)->Self{let mut result=Self::new();for(key,value)in values{if let Some(previous)=result.insert(key,value){PropertyValue::retire_decoded(previous)}}result}}
impl IntoIterator for PropertyBag{type Item=(String,PropertyValue);type IntoIter=std::vec::IntoIter<Self::Item>;fn into_iter(self)->Self::IntoIter{self.members.into_iter()}}
impl<'a> IntoIterator for &'a PropertyBag{type Item=(&'a String,&'a PropertyValue);type IntoIter=std::iter::Map<std::slice::Iter<'a,(String,PropertyValue)>,fn(&'a(String,PropertyValue))->Self::Item>;fn into_iter(self)->Self::IntoIter{self.members.iter().map(|(key,value)|(key,value))}}
impl semio_framework_value::retirement::RetireOwned for PropertyBag{fn retirement(self)->Box<dyn semio_framework_value::retirement::RetirementCursor>{semio_framework_value::retirement::RetireOwned::retirement(self.members)}}
impl FromValue for PropertyBag {
 fn from_value(value:DslValue)->Result<Self,ValueError>{let DslValue::Object(fields)=value else{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"graph property object required"))};fields.into_iter().map(|(key,value)|PropertyValue::from_value(value).map(|value|(key,value))).collect()}
 fn from_value_controlled(value:&DslValue,control:&mut NativeDecodeControl<'_>)->Result<Self,ValueError>{control.scoped_depth(64,|control|control.scoped_stage(|control|{let fields=value.object_controlled(control)?;control.begin_stage(0)?;let mut output=DecodedValue::new(Self::from_admitted(control.allocate_vec(fields.len())?),<Self as FromValue>::retire_decoded);for(key,value)in fields{let key=control.copy_text(key)?;let value=PropertyValue::from_value_controlled(value,control)?;output.get_mut().admitted_insert(key,value,control)?;}control.checkpoint()?;Ok(output.take())}))}
 fn default_value_controlled(control:&mut NativeDecodeControl<'_>)->Result<Self,ValueError>{control.checkpoint()?;Ok(Self::new())}
 fn retire_decoded(self){for(_,value)in self.members{PropertyValue::retire_decoded(value)}}
}
impl ToValue for PropertyBag {
 fn to_value(&self)->DslValue{DslValue::Object(self.iter().map(|(key,value)|(key.clone(),value.to_value())).collect())}
 fn to_value_controlled(&self,control:&mut NativeEncodeControl<'_>)->Result<DslValue,ValueError>{control.scoped_depth(64,|control|control.scoped_stage(|control|{control.begin_stage(0)?;let mut output=DslValue::object_encoding_controlled(self.len(),control)?;for(key,value)in self{let value=value.to_value_controlled(control)?;DslValue::push_encoding_controlled(output.get_mut(),key,value,control)?;control.step()?;}control.checkpoint()?;Ok(DslValue::Object(output.take()))}))}
}
impl dsl_core::DslField for PropertyBag {
 fn shape()->dsl_core::Shape{dsl_core::Shape::Map(Box::new(dsl_core::Shape::Value))}
 fn shape_controlled<C:dsl_core::NativeSchemaControl>(control:&mut C)->Result<dsl_core::Shape,ValueError>{dsl_core::producer::boxed(dsl_core::Shape::Value,control).map(dsl_core::Shape::Map)}
 fn to_value(&self)->dsl_core::FieldValue{dsl_core::FieldValue::Map(self.iter().map(|(key,value)|(key.clone(),<PropertyValue as dsl_core::DslField>::to_value(value))).collect())}
 fn from_value(value:&dsl_core::FieldValue)->Result<Self,String>{let dsl_core::FieldValue::Map(fields)=value else{return Err("graph property map required".into())};fields.iter().map(|(key,value)|<PropertyValue as dsl_core::DslField>::from_value(value).map(|value|(key.clone(),value))).collect()}
 fn to_value_controlled(&self,control:&mut NativeEncodeControl<'_>)->Result<dsl_core::FieldValue,ValueError>{control.scoped_stage(|control|{control.begin_stage(0)?;let mut output=dsl_core::__rt::DecodedFieldOwner::new(control.allocate_vec(self.len())?,|values:Vec<(String,dsl_core::FieldValue)>|{for(_,value)in values{dsl_core::native_encoding::retire_field(value)}});for(key,value)in self{let key=control.copy_text(key)?;let value=<PropertyValue as dsl_core::DslField>::to_value_controlled(value,control)?;output.as_mut().push((key,value));control.step()?;}Ok(dsl_core::FieldValue::Map(output.take()))})}
 fn from_value_controlled(value:&dsl_core::FieldValue,control:&mut NativeDecodeControl<'_>)->Result<Self,ValueError>{let dsl_core::FieldValue::Map(fields)=value else{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"graph property map required"))};control.scoped_stage(|control|{control.begin_stage(0)?;let mut output=DecodedValue::new(Self::from_admitted(control.allocate_vec(fields.len())?),<Self as FromValue>::retire_decoded);for(key,value)in fields{let key=control.copy_text(key)?;let value=<PropertyValue as dsl_core::DslField>::from_value_controlled(value,control)?;output.get_mut().admitted_insert(key,value,control)?;}Ok(output.take())})}
 fn retire_decoded(self){<Self as FromValue>::retire_decoded(self)}
}

```

### 🧰️framework/🔨️modules/🕸️graph/🛂️manifest/🚦️properties/🦀️.rs

SHA-256 `1258f7bcb33610562ca85b534fac8ffbecabc52e2069654bc3b4254426812583`; 6605 bytes.

```
//! 🌿️ Iterative actual property variants retain all owned backing and partial values.
use super::{PropertyBag,PropertyValue};
use semio_framework_value::{DecodedValue,DslValue,FromValue,NativeDecodeControl,NativeEncodeControl,Number,ValueError,ValueRefusalKind};
use dsl_core::NativeSchemaControl;
fn grow<T,C:NativeSchemaControl>(values:&mut Vec<T>,control:&mut C)->Result<(),ValueError>{if values.len()==values.capacity(){let count=values.capacity().max(1).checked_mul(2).ok_or_else(||ValueError::new(ValueRefusalKind::OwnershipLimit,"property frontier overflow"))?;let mut next=control.allocate_vec(count)?;for _ in 0..values.len(){control.step()?;}next.extend(std::mem::take(values));*values=next;}Ok(())}
fn push<T:FromValue,C:NativeSchemaControl>(values:&mut Vec<T>,value:T,control:&mut C)->Result<(),ValueError>{let value=DecodedValue::new(value,T::retire_decoded);grow(values,control)?;values.push(value.take());Ok(())}
fn retire_values<T:FromValue>(values:Vec<T>){for value in values{T::retire_decoded(value)}}
enum Decode<'a>{Enter(&'a DslValue),Array(usize),Object(&'a[(String,DslValue)])}
pub(super) fn decode(value:&DslValue,control:&mut NativeDecodeControl<'_>)->Result<PropertyValue,ValueError>{
 control.scoped_stage(|control|{
  control.begin_stage(0)?;let mut pending=control.allocate_vec(1)?;pending.push(Decode::Enter(value));let mut values=DecodedValue::new(Vec::new(),retire_values::<PropertyValue>);
  while let Some(frame)=pending.pop(){match frame{
   Decode::Enter(value)=>{let value=match value{
    DslValue::Null=>PropertyValue::Null,DslValue::Bool(value)=>PropertyValue::Bool(*value),DslValue::Number(value)=>PropertyValue::Number(value.as_f64()),DslValue::String(value)=>PropertyValue::String(control.copy_text(value)?),
    DslValue::Bytes(bytes)=>{let mut output=control.allocate_vec(bytes.len())?;for byte in bytes{output.push(PropertyValue::Number(f64::from(*byte)));control.step()?;}PropertyValue::Array(output)},
    DslValue::Array(items)=>{grow(&mut pending,control)?;pending.push(Decode::Array(items.len()));for value in items.iter().rev(){grow(&mut pending,control)?;pending.push(Decode::Enter(value));control.step()?;}continue},
    DslValue::Object(items)=>{grow(&mut pending,control)?;pending.push(Decode::Object(items));for(_,value)in items.iter().rev(){grow(&mut pending,control)?;pending.push(Decode::Enter(value));control.step()?;}continue}
   };push(values.get_mut(),value,control)?;},
   Decode::Array(count)=>{let mut output=DecodedValue::new(PropertyValue::Array(control.allocate_vec(count)?),retire);let PropertyValue::Array(items)=output.get_mut()else{unreachable!()};for _ in 0..count{items.push(PropertyValue::Null);control.step()?;}for index in(0..count).rev(){items[index]=values.get_mut().pop().ok_or_else(||ValueError::new(ValueRefusalKind::InvariantViolated,"property child missing"))?;control.step()?;}push(values.get_mut(),output.take(),control)?;},
   Decode::Object(fields)=>{let mut children=DecodedValue::new(control.allocate_vec(fields.len())?,retire_values::<PropertyValue>);for _ in fields{children.get_mut().push(PropertyValue::Null);control.step()?;}for index in(0..fields.len()).rev(){children.get_mut()[index]=values.get_mut().pop().ok_or_else(||ValueError::new(ValueRefusalKind::InvariantViolated,"property child missing"))?;control.step()?;}let mut output=DecodedValue::new(PropertyBag::from_admitted(control.allocate_vec(fields.len())?),PropertyBag::retire_decoded);for(index,(key,_))in fields.iter().enumerate(){let key=control.copy_text(key)?;let value=std::mem::take(&mut children.get_mut()[index]);output.get_mut().admitted_insert(key,value,control)?;}push(values.get_mut(),PropertyValue::Object(output.take()),control)?;}
  }control.step()?;}
  if values.get_mut().len()!=1{return Err(ValueError::new(ValueRefusalKind::InvariantViolated,"property root cardinality differs"))}control.checkpoint()?;Ok(values.get_mut().pop().unwrap())
 })
}
enum Encode<'a>{Enter(&'a PropertyValue),Array(usize),Object(&'a PropertyBag)}
pub(super) fn encode(value:&PropertyValue,control:&mut NativeEncodeControl<'_>)->Result<DslValue,ValueError>{
 control.scoped_stage(|control|{
  control.begin_stage(0)?;let mut pending=control.allocate_vec(1)?;pending.push(Encode::Enter(value));let mut values=DecodedValue::new(Vec::new(),retire_values::<DslValue>);
  while let Some(frame)=pending.pop(){match frame{
   Encode::Enter(value)=>{let value=match value{
    PropertyValue::Null=>DslValue::Null,PropertyValue::Bool(value)=>DslValue::Bool(*value),PropertyValue::Number(value)=>DslValue::Number(Number::Float(*value)),PropertyValue::String(value)=>DslValue::String(control.copy_text(value)?),
    PropertyValue::Array(items)=>{grow(&mut pending,control)?;pending.push(Encode::Array(items.len()));for value in items.iter().rev(){grow(&mut pending,control)?;pending.push(Encode::Enter(value));control.step()?;}continue},
    PropertyValue::Object(items)=>{grow(&mut pending,control)?;pending.push(Encode::Object(items));for(_,value)in items.iter().rev(){grow(&mut pending,control)?;pending.push(Encode::Enter(value));control.step()?;}continue}
   };push(values.get_mut(),value,control)?;},
   Encode::Array(count)=>{let mut output=DslValue::Array(control.allocate_vec(count)?).guard_encoded();let DslValue::Array(items)=output.get_mut()else{unreachable!()};for _ in 0..count{items.push(DslValue::Null);control.step()?;}for index in(0..count).rev(){items[index]=values.get_mut().pop().ok_or_else(||ValueError::new(ValueRefusalKind::InvariantViolated,"property child missing"))?;control.step()?;}push(values.get_mut(),output.take(),control)?;},
   Encode::Object(fields)=>{let mut output=DslValue::object_encoding_controlled(fields.len(),control)?;for key in fields.keys(){let key=control.copy_text(key)?;output.get_mut().push((key,DslValue::Null));control.step()?;}for index in(0..fields.len()).rev(){output.get_mut()[index].1=values.get_mut().pop().ok_or_else(||ValueError::new(ValueRefusalKind::InvariantViolated,"property child missing"))?;control.step()?;}push(values.get_mut(),DslValue::Object(output.take()),control)?;}
  }control.step()?;}
  if values.get_mut().len()!=1{return Err(ValueError::new(ValueRefusalKind::InvariantViolated,"property root cardinality differs"))}control.checkpoint()?;Ok(values.get_mut().pop().unwrap())
 })
}
pub(super) fn retire(value:PropertyValue){let mut cursor=semio_framework_value::retirement::owned_retirement(value);while !matches!(cursor.close_step(256,usize::MAX).expect("property retirement failed"),semio_framework_value::SnapshotRetirementStep::Complete){}assert!(cursor.terminal_is_empty());}

```

### 🧰️framework/🔨️modules/🕸️graph/🛂️manifest/🧪️tests/🏷️type/🦀️.rs

SHA-256 `78302fdd542c9ec79b16dc6975994d7e53f2965e1b02a096174e5dbda7515ba6`; 6648 bytes.

```
//! 🎯️ Original Graph property admission against the lower language-neutral corpus.

use super::{dsl_value_to_property_value, property_value_matches_type};
use semio_framework_value::{FromValue, ValueType};

#[test]
fn graph_properties_preserve_all_original_type_classifications() {
    let fixture = semio_framework_pack_json::to_dsl_value(&semio_framework_pack_json::parse(include_str!("../../🧫️fixtures/🏷️type/🔣️.json"), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap());
    let cases = fixture.get("graphCases").and_then(dsl_core::DslValue::as_array).unwrap();
    let types = fixture.get("types").and_then(dsl_core::DslValue::as_array).unwrap();
    let values = fixture.get("graphValues").and_then(dsl_core::DslValue::as_array).unwrap();
    for row in cases {
        let type_index = u64::from_value(row.get("type").unwrap().clone()).unwrap() as usize;
        let value_index = u64::from_value(row.get("value").unwrap().clone()).unwrap() as usize;
        let value_type = ValueType::from_value(types[type_index].get("type").unwrap().clone()).unwrap();
        let property = dsl_value_to_property_value(&values[value_index]);
        let expected = bool::from_value(row.get("accepted").unwrap().clone()).unwrap();
        assert_eq!(property_value_matches_type(&property, &value_type), expected, "{}", String::from_value(row.get("name").unwrap().clone()).unwrap());
    }
}

#[test]
fn graph_property_controlled_constructor_preserves_canonical_type_and_optional_expression() {
    use super::{PropertyDef, PropertyKind};
    use semio_framework_value::{DslValue, FromValue, NativeDecodeControl, ToValue};
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🏷️type/🔣️.json")).unwrap();
    for row in fixture["types"].as_array().unwrap() {
        let type_value = semio_framework_pack_json::to_dsl_value(&semio_framework_pack_json::parse(&row["type"].to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap());
        let ty = semio_framework_value::ValueType::from_value(type_value.clone()).unwrap();
        let expected = PropertyDef { name: "literal\0!@/引用".into(), kind: PropertyKind::Derived, value_type: ty, expr: Some(String::new()) };
        let input = DslValue::object([("name".into(), expected.name.to_value()), ("kind".into(), "derived".to_value()), ("valueType".into(), type_value), ("expr".into(), String::new().to_value())]);
        let oracle: serde_json::Value = serde_json::from_str(&semio_framework_pack_json::to_string(&semio_framework_pack_json::from_dsl_value(&input))).unwrap();
        assert_eq!(oracle["valueType"], row["type"]);
        let mut yes = |_| true;
        let mut c = NativeDecodeControl::new(1 << 20, &mut yes);
        let actual = PropertyDef::from_value_controlled(&input, &mut c).unwrap();
        assert_eq!(actual, expected);
        PropertyDef::retire_decoded(actual);
    }
}

#[test]
fn graph_property_controlled_constructor_preserves_declared_type_spellings_and_expression_presence() {
    use super::PropertyDef;
    use semio_framework_value::{DslValue, NativeDecodeControl, ToValue};
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🏷️type/🔣️.json")).unwrap();
    for case in fixture["controlledProperties"].as_array().unwrap() {
        for expression in [None, Some("")] {
            let ty = semio_framework_pack_json::to_dsl_value(&semio_framework_pack_json::parse(&case["input"].to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap());
            let mut fields = vec![("name".into(), "literal\0引用".to_value()), ("kind".into(), "data".to_value()), ("valueType".into(), ty)];
            if let Some(expression) = expression {
                fields.push(("expr".into(), expression.to_value()));
            }
            let mut yes = |_| true;
            let actual = PropertyDef::from_value_controlled(&DslValue::Object(fields), &mut NativeDecodeControl::new(1 << 20, &mut yes)).unwrap();
            let oracle: serde_json::Value = serde_json::from_str(&semio_framework_pack_json::to_string(&semio_framework_pack_json::from_dsl_value(&actual.value_type.to_value()))).unwrap();
            assert_eq!(oracle, case["expected"]);
            assert_eq!(actual.expr.as_deref(), expression);
            PropertyDef::retire_decoded(actual);
        }
    }
}

#[test]
fn graph_property_controlled_constructor_bounds_deep_types_and_partial_expression_copies() {
    use super::PropertyDef;
    use semio_framework_value::{DslValue, NativeDecodeControl, ToValue};
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🏷️type/🔣️.json")).unwrap();
    let work = &fixture["controlledWork"];
    let schema = work["schemaUnit"].as_str().unwrap().repeat(work["repeat"].as_u64().unwrap() as usize);
    let expression = work["expressionUnit"].as_str().unwrap().repeat(work["repeat"].as_u64().unwrap() as usize);
    let mut ty = DslValue::object([("kind".into(), "schema".to_value()), ("of".into(), schema.to_value())]);
    for _ in 0..work["depth"].as_u64().unwrap() {
        ty = DslValue::object([("kind".into(), "list".to_value()), ("of".into(), ty)]);
    }
    let input = DslValue::object([("name".into(), "deep\0引用".to_value()), ("kind".into(), "derived".to_value()), ("valueType".into(), ty), ("expr".into(), expression.to_value())]);
    let mut yes = |_| true;
    let mut probe = NativeDecodeControl::new(1 << 24, &mut yes);
    let actual = PropertyDef::from_value_controlled(&input, &mut probe).unwrap();
    assert_eq!(actual.expr.as_deref(), Some(expression.as_str()));
    let bytes = probe.owned_bytes();
    PropertyDef::retire_decoded(actual);
    let actual = PropertyDef::from_value_controlled(&input, &mut NativeDecodeControl::new(bytes, &mut yes)).unwrap();
    PropertyDef::retire_decoded(actual);
    assert_eq!(PropertyDef::from_value_controlled(&input, &mut NativeDecodeControl::new(bytes - 1, &mut yes)).unwrap_err().kind,semio_framework_value::ValueRefusalKind::OwnershipLimit);
    let mut interrupted = false;
    let mut cancel = |p: semio_framework_value::native_decoding::NativeDecodeProgress| {
        let stop = p.total == expression.len() && p.completed >= 65536 && p.completed < p.total;
        interrupted |= stop;
        !stop
    };
    assert_eq!(PropertyDef::from_value_controlled(&input, &mut NativeDecodeControl::new(bytes, &mut cancel)).unwrap_err().kind,semio_framework_value::ValueRefusalKind::Canceled);
    assert!(interrupted);
    <DslValue as FromValue>::retire_decoded(input);
}

```

### 🧰️framework/🔨️modules/🕸️graph/🛂️manifest/🧪️tests/🔬️unit/🦀️.rs

SHA-256 `51ba9fe853bb67134479b06afb267b2402fb64fb48825b530dd455d3557ffb20`; 3009 bytes.

```

use super::*;

// 🚫️async: E5-class executor bridge, sanctioned per R4 clause 5 — `#[test]` cannot run
// an `async fn` directly (std has no executor for it), so every async test body in this
// module runs through this instead. Sound because this crate performs no real I/O: every
// future here resolves on its first poll, so a single poll (never a spin-park loop) is
// enough — panics loudly if that invariant is ever violated rather than hanging.
fn block_on_test<F: std::future::Future>(fut: F) -> F::Output {
    use std::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};
    fn noop(_: *const ()) {}
    fn clone_raw(_: *const ()) -> RawWaker {
        RawWaker::new(std::ptr::null(), &VTABLE)
    }
    static VTABLE: RawWakerVTable = RawWakerVTable::new(clone_raw, noop, noop, noop);
    let raw = RawWaker::new(std::ptr::null(), &VTABLE);
    let waker = unsafe { Waker::from_raw(raw) };
    let mut cx = Context::from_waker(&waker);
    let mut fut = Box::pin(fut);
    match fut.as_mut().poll(&mut cx) {
        Poll::Ready(v) => v,
        Poll::Pending => panic!("block_on_test: future did not complete synchronously"),
    }
}


#[test]
fn validator_rejects_unknown_node_kind() {
    block_on_test(async {
        let m = <Manifest as dsl_core::FromValue>::from_value(semio_framework_pack_json::to_dsl_value(&semio_framework_pack_json::parse(r#"{"schema":"manifest","id":"neutral.validation","nodeKinds":[{"id":"Entry"}]}"#,semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap())).unwrap();
        let v = ManifestValidator::new(&m);
        assert!(v.validate_node_kind("NoSuchNode").is_err());
    });
}



#[test]
fn property_value_dsl_field_round_trips_nested_array_and_object() {
    block_on_test(async {
        // 🌳️ Nested case: an Object containing an Array containing an Object — proves the
        // `dsl_core::DslField` bridge (via `dsl_core::DslValue`) recurses correctly at every depth, not just
        // for a flat value.
        let mut inner_obj = PropertyBag::new();
        inner_obj.insert("flag".to_string(), PropertyValue::Bool(true));
        inner_obj.insert("label".to_string(), PropertyValue::String("leaf".to_string()));

        let array_of_objects = PropertyValue::Array(vec![PropertyValue::Number(1.0), PropertyValue::Object(inner_obj), PropertyValue::Null]);

        let mut root = PropertyBag::new();
        root.insert("id".to_string(), PropertyValue::String("root".to_string()));
        root.insert("count".to_string(), PropertyValue::Number(3.0));
        root.insert("items".to_string(), array_of_objects);
        let value = PropertyValue::Object(root);

        let field_value = <PropertyValue as ::dsl_core::DslField>::to_value(&value);
        let round_tripped = <PropertyValue as ::dsl_core::DslField>::from_value(&field_value).expect("round trip must succeed");
        assert_eq!(round_tripped, value, "PropertyValue dsl_core::DslField round trip diverged for a nested Object/Array/Object value");
    });
}

```

### 🧰️framework/🔨️modules/🕸️graph/🛂️manifest/🪆️binding/🦀️.rs

SHA-256 `375c41a61d33b060d3dac7b0cea50421edbaef6f42142fd6cfcc06ddeaacba0d`; 6927 bytes.

```
//! 📋️ Actual property declarations and port directions own their typed native fields.
use super::{PropertyDef,PropertyKind,PortDirection};
use crate::dsl_core::{DslField,FieldValue,RecordValue,RecordSpec,RecordSpecProducer,FieldSpec,RecordLayout,Shape,NativeSchemaControl,NativeEncodeControl,NativeDecodeControl,ValueError};
use semio_framework_value::{FromValue,ValueRefusalKind,ValueType,DecodedValue};
macro_rules! scalar {
    ($ty:ty,$first:ident,$second:ident,$first_name:literal,$second_name:literal)=>{
        impl DslField for $ty {
            fn shape()->Shape{Shape::Enum(vec![($first_name.into(),0),($second_name.into(),1)])}
            fn shape_controlled<C:NativeSchemaControl>(control:&mut C)->Result<Shape,ValueError>{control.scoped_stage(|control|{control.begin_stage(2)?;let mut values=control.allocate_vec(2)?;values.push((control.copy_text($first_name)?,0));control.step()?;values.push((control.copy_text($second_name)?,1));control.step()?;Ok(Shape::Enum(values))})}
            fn to_value(&self)->FieldValue{FieldValue::Enum(match self{Self::$first=>0,Self::$second=>1})}
            fn to_value_controlled(&self,control:&mut NativeEncodeControl<'_>)->Result<FieldValue,ValueError>{control.checkpoint()?;Ok(<Self as DslField>::to_value(self))}
            fn from_value(value:&FieldValue)->Result<Self,String>{match value{FieldValue::Enum(0)=>Ok(Self::$first),FieldValue::Enum(1)=>Ok(Self::$second),_=>Err("invalid declared enum ordinal".into())}}
            fn from_value_controlled(value:&FieldValue,control:&mut NativeDecodeControl<'_>)->Result<Self,ValueError>{control.checkpoint()?;match value{FieldValue::Enum(0)=>Ok(Self::$first),FieldValue::Enum(1)=>Ok(Self::$second),_=>Err(ValueError::new(ValueRefusalKind::InvalidValue,"invalid declared enum ordinal"))}}
        }
    };
}
scalar!(PropertyKind,Data,Derived,"data","derived");
scalar!(PortDirection,In,Out,"in","out");
fn ordinary_spec()->RecordSpec{RecordSpec::new(None,RecordLayout::Inline,vec![FieldSpec::new(0,"name",<String as DslField>::shape()),FieldSpec::new(1,"kind",<PropertyKind as DslField>::shape()),FieldSpec::new(2,"value-type",<ValueType as DslField>::shape()),FieldSpec::new(3,"expr",<String as DslField>::shape()).optional()])}
fn controlled_spec<C:NativeSchemaControl>(control:&mut C)->Result<RecordSpec,ValueError>{control.scoped_stage(|control|{
    control.begin_stage(4)?;let mut fields=control.allocate_vec(4)?;
    for(id,key,shape,optional)in[(0,"name",<String as DslField>::shape_controlled(control)?,false),(1,"kind",<PropertyKind as DslField>::shape_controlled(control)?,false),(2,"value-type",<ValueType as DslField>::shape_controlled(control)?,false),(3,"expr",<String as DslField>::shape_controlled(control)?,true)]{let mut field=crate::dsl_core::schema::producer::field(id,key,shape,control)?;field.optional=optional;fields.push(field);control.step()?;}
    crate::dsl_core::schema::producer::record(None,RecordLayout::Inline,fields,control)
})}
fn decoding_spec(control:&mut NativeDecodeControl<'_>)->Result<RecordSpec,ValueError>{controlled_spec(control)}
fn encoding_spec(control:&mut NativeEncodeControl<'_>)->Result<RecordSpec,ValueError>{controlled_spec(control)}
fn producer()->RecordSpecProducer{RecordSpecProducer{ordinary:ordinary_spec,decoding:decoding_spec,encoding:encoding_spec}}
fn retire(value:PropertyDef){<ValueType as FromValue>::retire_decoded(value.value_type)}
impl DslField for PropertyDef {
    fn shape()->Shape{Shape::Record(producer())}
    fn shape_controlled<C:NativeSchemaControl>(control:&mut C)->Result<Shape,ValueError>{control.checkpoint()?;Ok(Shape::Record(producer()))}
    fn to_value(&self)->FieldValue{FieldValue::Record(RecordValue{fields:[(0,<String as DslField>::to_value(&self.name)),(1,<PropertyKind as DslField>::to_value(&self.kind)),(2,<ValueType as DslField>::to_value(&self.value_type)),(3,self.expr.as_ref().map(<String as DslField>::to_value).unwrap_or(FieldValue::Absent))].into_iter().collect()})}
    fn to_record_controlled(&self,control:&mut NativeEncodeControl<'_>)->Result<RecordValue,ValueError>{control.scoped_stage(|control|{
        control.begin_stage(4)?;let mut record=crate::dsl_core::native_encoding::EncodedRecord::new(4,control)?;
        record.insert(0,<String as DslField>::to_value_controlled(&self.name,control)?)?;control.step()?;
        record.insert(1,<PropertyKind as DslField>::to_value_controlled(&self.kind,control)?)?;control.step()?;
        record.insert(2,<ValueType as DslField>::to_value_controlled(&self.value_type,control)?)?;control.step()?;
        record.insert(3,match &self.expr{Some(value)=><String as DslField>::to_value_controlled(value,control)?,None=>FieldValue::Absent})?;control.step()?;
        Ok(record.take())
    })}
    fn to_value_controlled(&self,control:&mut NativeEncodeControl<'_>)->Result<FieldValue,ValueError>{let record=crate::dsl_core::native_encoding::EncodedRecord::from_record(Self::to_record_controlled(self,control)?);Ok(FieldValue::Record(record.take()))}
    fn from_value(value:&FieldValue)->Result<Self,String>{let FieldValue::Record(record)=value else{return Err("expected property declaration record".into())};let get=|id|record.get(id).ok_or_else(||"missing property declaration field".to_string());Ok(Self{name:<String as DslField>::from_value(get(0)?)?,kind:<PropertyKind as DslField>::from_value(get(1)?)?,value_type:<ValueType as DslField>::from_value(get(2)?)?,expr:match get(3)?{FieldValue::Absent=>None,value=>Some(<String as DslField>::from_value(value)?)}})}
    fn from_record_controlled(record:&RecordValue,control:&mut NativeDecodeControl<'_>)->Result<Self,ValueError>{control.scoped_stage(|control|{
        control.begin_stage(4)?;if record.fields.len()!=4{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"invalid property declaration fields"));}
        let get=|id|record.get(id).ok_or_else(||ValueError::new(ValueRefusalKind::InvalidValue,"missing property declaration field"));
        let name=<String as DslField>::from_value_controlled(get(0)?,control)?;control.step()?;
        let kind=<PropertyKind as DslField>::from_value_controlled(get(1)?,control)?;control.step()?;
        let value_type=DecodedValue::new(<ValueType as DslField>::from_value_controlled(get(2)?,control)?,<ValueType as FromValue>::retire_decoded);control.step()?;
        let expr=match get(3)?{FieldValue::Absent=>None,value=>Some(<String as DslField>::from_value_controlled(value,control)?)};
        let value=DecodedValue::new(Self{name,kind,value_type:value_type.take(),expr},retire);control.step()?;Ok(value.take())
    })}
    fn from_value_controlled(value:&FieldValue,control:&mut NativeDecodeControl<'_>)->Result<Self,ValueError>{match value{FieldValue::Record(record)=>Self::from_record_controlled(record,control),_=>Err(ValueError::new(ValueRefusalKind::InvalidValue,"expected property declaration record"))}}
    fn retire_decoded(self){retire(self)}
}

```

### 🧰️framework/🔨️modules/🕸️graph/🗣️dsl/🦀️.rs

SHA-256 `dbd6e61e3420e856a04ee4e541cfdc2ab302dd95d80211b3f3b66f99fd164509`; 96726 bytes.

```
//! 🃏️ Shared Jack query language for graph frameworks.
//!
//! 🚚 Relocated verbatim from `🧰️framework/🔨️modules/🧮️math/🕸️graph/🗣️dsl` in ticket 26/08/12/
//! DISSOLVE-KERNELS-AND-MODULES-INTO-EVENT-SOURCED-ARTIFACTS wave MATHEND — see that wave's report
//! for why the framework/plugin split hypothesis was measured and rejected (real coupling between
//! `DslIdiom` and the language-service surface). `dsl_core` (crate-root alias for
//! `semio_framework_os_kernel`, renamed from `dsl` by this same wave to free that name for this
//! module) is this file's own `os_dsl`/`dsl`-derive dependency.

// #region ⚠️ Errors
/// 🚧️ Unified failure mode for jack parsing/execution, wire-literal parsing, and fixture ingestion.
#[derive(Debug)]
pub enum GraphDslError {
    /// 🧾️ Fixture or query-result JSON failed to parse or serialize.
    Json(semio_framework_pack_json::JsonError),
    /// 🔤️ A string literal was never closed (Jack's own dual-quote pre-scan, `dsl_core` only
    /// natively lexes `"..."`).
    UnterminatedString,
    /// ❓️ A byte outside the token grammar was found (Jack's own pre-scan for `'`/`"`/`!=`, ahead
    /// of delegating the rest of the alphabet to `os_dsl::lex`).
    UnexpectedChar(char),
    /// 🔢️ A numeric literal did not parse as a float (defensive — `os_dsl::lex` only ever
    /// accumulates well-formed digit runs, so this should be unreachable in practice).
    NumberFormat(std::num::ParseFloatError),
    /// ➡️ Parser expected one token shape and found another.
    UnexpectedToken { expected: String, found: String },
    /// 🪝️ A wire-literal edge was missing a mandatory `@port` on one of its endpoints (this
    /// module's own DAG domain rule, enforced on top of the unified wire grammar).
    EdgeTargetMissingPort,
    /// 🕸️ A jack pattern had no nodes.
    EmptyPattern,
    /// 🚫️ CREATE/DELETE/SET/MERGE are not supported on read-only queryable graphs.
    UnsupportedMutation,
    /// 📞️ A `CALL` named a procedure outside this module's small owned registry (see
    /// [`call_procedure`]) — full arbitrary-procedure dispatch is deferred to unifying
    /// semio_compose_rs's Architect query language onto Jack (Wave 2 / P9).
    UnknownProcedure(String),
    /// 🔢️ A `CALL` supplied the wrong number of positional arguments for the named procedure.
    ProcedureArity { name: String, expected: usize, found: usize },
    /// 🪪️ A supplied manifest must match the snapshot's declared identity.
    ManifestIdentity { expected: String, actual: String },
    /// 🔡️ A lexical/grammar error surfaced verbatim by the unified `dsl_core`/`dsl_schema` engine —
    /// used by both the wire-literal delegate (`dsl_core::parse_wire_text`) and Jack's
    /// `dsl_core`-backed lexer.
    Lex(semio_framework_diagnostic::TextError),
}

impl std::fmt::Display for GraphDslError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Json(error) => write!(formatter, "invalid json: {error}"),
            Self::UnterminatedString => formatter.write_str("unterminated string literal"),
            Self::UnexpectedChar(character) => write!(formatter, "unexpected character '{character}'"),
            Self::NumberFormat(error) => write!(formatter, "invalid number literal: {error}"),
            Self::UnexpectedToken { expected, found } => write!(formatter, "expected {expected}, got {found}"),
            Self::EdgeTargetMissingPort => formatter.write_str("edge target requires @port"),
            Self::EmptyPattern => formatter.write_str("empty pattern"),
            Self::UnsupportedMutation => formatter.write_str("mutating jack clauses are not supported on this graph domain"),
            Self::UnknownProcedure(name) => write!(formatter, "unknown CALL procedure '{name}'"),
            Self::ProcedureArity { name, expected, found } => write!(formatter, "procedure '{name}' expects {expected} argument(s), got {found}"),
            Self::ManifestIdentity { expected, actual } => write!(formatter, "snapshot manifest '{actual}' does not match supplied manifest '{expected}'"),
            Self::Lex(error) => error.fmt(formatter),
        }
    }
}

impl std::error::Error for GraphDslError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Json(error) => Some(error),
            Self::NumberFormat(error) => Some(error),
            Self::Lex(error) => Some(error),
            _ => None,
        }
    }
}

impl From<semio_framework_pack_json::JsonError> for GraphDslError {
    fn from(error: semio_framework_pack_json::JsonError) -> Self {
        Self::Json(error)
    }
}

impl From<std::num::ParseFloatError> for GraphDslError {
    fn from(error: std::num::ParseFloatError) -> Self {
        Self::NumberFormat(error)
    }
}

impl From<semio_framework_diagnostic::TextError> for GraphDslError {
    fn from(error: semio_framework_diagnostic::TextError) -> Self {
        Self::Lex(error)
    }
}
// #endregion ⚠️ Errors

pub mod queryable {
    // #region queryable
    //! 🔍️ Queryable graph interface for Jack.

    use crate::dsl::GraphDslError;
    use crate::manifest::{GraphManifest, PropertyBag, PropertyValue};
    use semio_framework_pack_json::Value;
    use std::collections::{BTreeMap, BTreeSet};

    // #region 🔖️QueryableEdge
    /// 🪢️ Edge row exposed to Jack matching.
    #[derive(Clone, Debug, PartialEq)]
    pub struct QueryableEdge {
        pub id: String,
        pub kind: String,
        pub source_node_id: String,
        pub target_node_id: String,
        pub source_port: Option<String>,
        pub target_port: Option<String>,
        pub properties: PropertyBag,
    }
    // #endregion 🔖️QueryableEdge

    // #region 🔖️QueryableGraph
    /// 🕸️ Read-only graph surface for Jack query execution.
    pub trait QueryableGraph {
        fn manifest(&self) -> Option<&GraphManifest>;
        fn node_ids(&self) -> Vec<String>;
        fn node_kind(&self, id: &str) -> Option<String>;
        fn node_name(&self, id: &str) -> Option<String>;
        fn node_property(&self, id: &str, key: &str) -> Option<PropertyValue>;
        fn edges(&self) -> Vec<QueryableEdge>;
        fn subgraph_fixture_json(&self, node_ids: &BTreeSet<String>, edge_ids: &BTreeSet<String>) -> Option<String>;
    }

    pub fn manifest_node_kinds<G: QueryableGraph>(graph: &G) -> Vec<String> {
        let mut kinds = BTreeSet::new();
        for id in graph.node_ids() {
            if let Some(kind) = graph.node_kind(id.as_str()) {
                kinds.insert(kind);
            }
        }
        if let Some(manifest) = graph.manifest() {
            for def in &manifest.node_kinds {
                kinds.insert(def.id.clone());
            }
        }
        kinds.into_iter().collect()
    }

    pub fn manifest_edge_kinds<G: QueryableGraph>(graph: &G) -> Vec<String> {
        let mut kinds = BTreeSet::new();
        for edge in graph.edges() {
            kinds.insert(edge.kind.clone());
        }
        if let Some(manifest) = graph.manifest() {
            for def in &manifest.edge_kinds {
                kinds.insert(def.id.clone());
            }
        }
        kinds.into_iter().collect()
    }

    pub fn manifest_property_names<G: QueryableGraph>(graph: &G) -> Vec<String> {
        let mut props = BTreeSet::from(["id".to_string(), "name".to_string(), "kind".to_string()]);
        for id in graph.node_ids() {
            for key in ["label", "text"] {
                if graph.node_property(id.as_str(), key).is_some() {
                    props.insert(key.to_string());
                }
            }
            if let Some(PropertyValue::Object(map)) = graph.node_property(id.as_str(), "__all") {
                for key in map.keys() {
                    props.insert(key.clone());
                }
            }
        }
        props.into_iter().collect()
    }

    pub fn manifest_port_kinds<G: QueryableGraph>(graph: &G) -> Vec<String> {
        let mut kinds = BTreeSet::new();
        for edge in graph.edges() {
            if let Some(port) = &edge.source_port {
                kinds.insert(port.clone());
            }
            if let Some(port) = &edge.target_port {
                kinds.insert(port.clone());
            }
        }
        if let Some(manifest) = graph.manifest() {
            for def in &manifest.port_kinds {
                kinds.insert(def.id.clone());
            }
        }
        kinds.into_iter().collect()
    }
    // #endregion 🔖️QueryableGraph

    // #region 🔖️BoardQueryableGraph
    fn json_to_property_bag(value: &Value) -> PropertyBag {
        let dsl_core::DslValue::Object(entries) = semio_framework_pack_json::to_dsl_value(value) else {
            return PropertyBag::default();
        };
        entries.into_iter().filter_map(|(k, v)| dsl_core::FromValue::from_value(v).ok().map(|pv| (k, pv))).collect()
    }

    fn split_endpoint(endpoint: &str, handle_to_node: &BTreeMap<String, String>) -> (String, Option<String>) {
        if let Some(node_id) = handle_to_node.get(endpoint) {
            return (node_id.clone(), None);
        }
        if let Some((node, port)) = endpoint.split_once('@') {
            let node_id = handle_to_node.get(node).cloned().unwrap_or_else(|| node.to_string());
            return (node_id, Some(port.to_string()));
        }
        if let Some((node, port)) = endpoint.rsplit_once(':') {
            let node_id = handle_to_node.get(node).cloned().unwrap_or_else(|| node.to_string());
            return (node_id, Some(port.to_string()));
        }
        if let Some((node, port)) = endpoint.rsplit_once('.') {
            if handle_to_node.contains_key(endpoint) {
                return (handle_to_node[endpoint].clone(), None);
            }
            return (node.to_string(), Some(port.to_string()));
        }
        (endpoint.to_string(), None)
    }

    /// 🧩️ Jack query target over board/scene fixture JSON.
    pub struct BoardQueryableGraph {
        manifest: Option<GraphManifest>,
        nodes: BTreeMap<String, (String, String, PropertyBag)>,
        edges: Vec<QueryableEdge>,
        raw_fixture: Value,
    }

    impl BoardQueryableGraph {
        pub fn from_host_snapshot_json(json: &str, manifest: Option<GraphManifest>) -> Result<Self, GraphDslError> {
            let raw: Value = semio_framework_pack_json::parse(json, semio_framework_pack_json::JsonMemberPolicy::Reject)?;
            if let (Some(manifest), Some(id)) = (&manifest, raw.get("manifestId").and_then(Value::as_str)) {
                if manifest.id != id {
                    return Err(GraphDslError::ManifestIdentity { expected: manifest.id.clone(), actual: id.to_string() });
                }
            }
            let mut nodes = BTreeMap::new();
            let mut handle_to_node = BTreeMap::new();
            if let Some(rows) = raw.get("nodes").and_then(|v| v.as_array()) {
                for row in rows {
                    let Some(obj) = row.as_object() else { continue };
                    let Some(id) = obj.get("id").and_then(|v| v.as_str()) else { continue };
                    let kind = obj.get("nodeKind").or_else(|| obj.get("node_kind")).or_else(|| obj.get("kind")).and_then(|v| v.as_str()).unwrap_or("").to_string();
                    let name = obj.get("text").or_else(|| obj.get("name")).or_else(|| obj.get("label")).and_then(|v| v.as_str()).unwrap_or(id).to_string();
                    let mut properties = match obj.get("userData").or_else(|| obj.get("user_data")) {
                        Some(v) => json_to_property_bag(v),
                        None => PropertyBag::default(),
                    };
                    for (key, value) in obj.iter() {
                        if matches!(key, "id" | "nodeKind" | "node_kind" | "kind" | "text" | "name" | "label" | "handles" | "x" | "y" | "shape" | "radius" | "width" | "height" | "userData" | "user_data") {
                            continue;
                        }
                        if let Ok(prop) = <PropertyValue as dsl_core::FromValue>::from_value(semio_framework_pack_json::to_dsl_value(value)) {
                            properties.insert(key.to_string(), prop);
                        }
                    }
                    nodes.insert(id.to_string(), (kind, name, properties));
                    if let Some(handles) = obj.get("handles").and_then(|v| v.as_array()) {
                        for handle in handles {
                            if let Some(hid) = handle.get("id").and_then(|v| v.as_str()) {
                                handle_to_node.insert(hid.to_string(), id.to_string());
                            }
                        }
                    }
                }
            }
            let mut edges = Vec::new();
            if let Some(rows) = raw.get("edges").and_then(|v| v.as_array()) {
                for row in rows {
                    let Some(obj) = row.as_object() else { continue };
                    let Some(id) = obj.get("id").and_then(|v| v.as_str()) else { continue };
                    let Some(source) = obj.get("source").and_then(|v| v.as_str()) else { continue };
                    let Some(target) = obj.get("target").and_then(|v| v.as_str()) else { continue };
                    let kind = obj.get("edgeKind").or_else(|| obj.get("edge_kind")).or_else(|| obj.get("kind")).and_then(|v| v.as_str()).unwrap_or("").to_string();
                    let properties = match obj.get("userData").or_else(|| obj.get("user_data")) {
                        Some(v) => json_to_property_bag(v),
                        None => PropertyBag::default(),
                    };
                    let (source_node_id, source_port) = split_endpoint(source, &handle_to_node);
                    let (target_node_id, target_port) = split_endpoint(target, &handle_to_node);
                    edges.push(QueryableEdge { id: id.to_string(), kind, source_node_id, target_node_id, source_port, target_port, properties });
                }
            }
            Ok(Self { manifest, nodes, edges, raw_fixture: raw })
        }

        pub fn from_object_snapshot_json(json: &str, manifest: Option<GraphManifest>) -> Result<Self, GraphDslError> {
            let raw: Value = semio_framework_pack_json::parse(json, semio_framework_pack_json::JsonMemberPolicy::Reject)?;
            let mut fixture = raw.clone();
            if fixture.get("nodes").and_then(|v| v.as_array()).is_none() {
                if let Some(objects) = raw.get("objects").and_then(|v| v.as_array()) {
                    let nodes: Vec<Value> = objects
                        .iter()
                        .filter_map(|row| {
                            let obj = row.as_object()?;
                            let id = obj.get("id").and_then(|v| v.as_str())?;
                            let kind = obj.get("objectKind").or_else(|| obj.get("kind")).and_then(|v| v.as_str()).unwrap_or("Object");
                            let name = obj.get("name").or_else(|| obj.get("label")).and_then(|v| v.as_str()).unwrap_or(id);
                            Some(Value::Object(semio_framework_pack_json::Object::from_iter([("id".to_string(), Value::from(id)), ("nodeKind".to_string(), Value::from(kind)), ("text".to_string(), Value::from(name))])))
                        })
                        .collect();
                    if let Some(object) = fixture.as_object_mut() {
                        object.insert("nodes", Value::Array(nodes));
                    }
                }
            }
            Self::from_host_snapshot_json(&semio_framework_pack_json::to_string(&fixture), manifest)
        }


    }

    impl QueryableGraph for BoardQueryableGraph {
        fn manifest(&self) -> Option<&GraphManifest> {
            self.manifest.as_ref()
        }

        fn node_ids(&self) -> Vec<String> {
            self.nodes.keys().cloned().collect()
        }

        fn node_kind(&self, id: &str) -> Option<String> {
            self.nodes.get(id).map(|(kind, _, _)| kind.clone())
        }

        fn node_name(&self, id: &str) -> Option<String> {
            self.nodes.get(id).map(|(_, name, _)| name.clone())
        }

        fn node_property(&self, id: &str, key: &str) -> Option<PropertyValue> {
            let (_, name, properties) = self.nodes.get(id)?;
            match key {
                "id" => Some(PropertyValue::String(id.to_string())),
                "name" | "label" | "text" => Some(PropertyValue::String(name.clone())),
                "kind" => self.node_kind(id).map(PropertyValue::String),
                "__all" => Some(PropertyValue::Object(properties.clone())),
                _ => properties.get(key).cloned(),
            }
        }

        fn edges(&self) -> Vec<QueryableEdge> {
            self.edges.clone()
        }

        fn subgraph_fixture_json(&self, node_ids: &BTreeSet<String>, edge_ids: &BTreeSet<String>) -> Option<String> {
            let mut fixture = self.raw_fixture.clone();
            if let Some(nodes) = fixture.get_mut("nodes").and_then(|v| v.as_array_mut()) {
                nodes.retain(|row| row.get("id").and_then(|v| v.as_str()).is_some_and(|id| node_ids.contains(id)));
            }
            if let Some(edges) = fixture.get_mut("edges").and_then(|v| v.as_array_mut()) {
                edges.retain(|row| row.get("id").and_then(|v| v.as_str()).is_some_and(|id| edge_ids.contains(id)));
            }
            Some(semio_framework_pack_json::to_string(&fixture))
        }
    }
    // #endregion 🔖️BoardQueryableGraph
    // #endregion queryable
}

pub mod wire {
    // #region wire
    //! 🔌️ Wire-literal compiled DAG text notation — delegates all lexing/parsing/printing to
    //! `dsl_schema`'s unified `Shape::Wire` grammar (`->`/`<-`/`--`, `{k=v}` double-quoted
    //! properties), keeping only this module's own public row types (`WireNode`/`WireEdge`) and
    //! its domain-specific rule that an edge's ports are mandatory on both ends — a validation
    //! layer on top of the shared parse, not a syntax difference. ~8 downstream crates depend on
    //! these exact type/function signatures, unchanged by this unification.

    use crate::dsl::GraphDslError;
    use crate::manifest::{PropertyBag, PropertyValue};

    // #region 🔖️WireTypes
    /// 🧩️ Neutral node row for wire-literal emission.
    #[derive(Clone, Debug, PartialEq)]
    pub struct WireNode {
        pub id: String,
        pub kind: String,
        pub port: Option<String>,
        pub properties: PropertyBag,
    }

    /// 🪢️ Neutral edge row for wire-literal emission.
    #[derive(Clone, Debug, PartialEq)]
    pub struct WireEdge {
        pub from: String,
        pub from_port: String,
        pub to: String,
        pub to_port: String,
        pub directed: bool,
        pub properties: PropertyBag,
    }
    // #endregion 🔖️WireTypes

    // #region 🔖️PropertyBridge
    /// 🌉️ `crate::manifest::PropertyValue` <-> `dsl_core::DslValue` — the two crates'
    /// dynamic-JSON-equivalent literal types are structurally identical, so this is a pure reshape.
    fn dsl_value_from_property_value(value: &PropertyValue) -> dsl_core::DslValue {
        match value {
            PropertyValue::Null => dsl_core::DslValue::Null,
            PropertyValue::Bool(b) => dsl_core::DslValue::Bool(*b),
            PropertyValue::Number(n) => dsl_core::DslValue::float(*n),
            PropertyValue::String(s) => dsl_core::DslValue::String(s.clone()),
            PropertyValue::Array(items) => {
                let mut out = Vec::with_capacity(items.len());
                for item in items {
                    out.push(dsl_value_from_property_value(item));
                }
                dsl_core::DslValue::Array(out)
            }
            PropertyValue::Object(map) => {
                let mut out = Vec::with_capacity(map.len());
                for (k, v) in map {
                    out.push((k.clone(), dsl_value_from_property_value(v)));
                }
                dsl_core::DslValue::Object(out)
            }
        }
    }

    fn property_value_from_dsl_value(value: &dsl_core::DslValue) -> PropertyValue {
        match value {
            dsl_core::DslValue::Null => PropertyValue::Null,
            dsl_core::DslValue::Bool(b) => PropertyValue::Bool(*b),
            dsl_core::DslValue::Number(n) => PropertyValue::Number(n.as_f64()),
            dsl_core::DslValue::String(s) => PropertyValue::String(s.clone()),
            dsl_core::DslValue::Bytes(bytes) => PropertyValue::Array(bytes.iter().map(|byte| PropertyValue::Number(f64::from(*byte))).collect()),
            dsl_core::DslValue::Array(items) => {
                let mut out = Vec::with_capacity(items.len());
                for item in items {
                    out.push(property_value_from_dsl_value(item));
                }
                PropertyValue::Array(out)
            }
            dsl_core::DslValue::Object(entries) => {
                let mut out = PropertyBag::new();
                for (k, v) in entries {
                    out.insert(k.clone(), property_value_from_dsl_value(v));
                }
                PropertyValue::Object(out)
            }
        }
    }

    fn properties_to_dsl_object(properties: &PropertyBag) -> dsl_core::DslValue {
        let mut out = Vec::with_capacity(properties.len());
        for (k, v) in properties {
            out.push((k.clone(), dsl_value_from_property_value(v)));
        }
        dsl_core::DslValue::Object(out)
    }

    fn properties_from_dsl_value(value: &dsl_core::DslValue) -> PropertyBag {
        match value {
            dsl_core::DslValue::Object(entries) => {
                let mut out = PropertyBag::new();
                for (k, v) in entries {
                    out.insert(k.clone(), property_value_from_dsl_value(v));
                }
                out
            }
            _ => PropertyBag::new(),
        }
    }
    // #endregion 🔖️PropertyBridge

    // #region 🔖️WireLiteral
    fn render_wire_line(value: &dsl_core::WireValue) -> String {
        // 🚨️ `dsl_core::Writer::new`/`print_shape`/`Writer::render` are all sync in `dsl_core` —
        // none of them suspend, so no `.await` belongs on any of them.
        let mut writer = dsl_core::Writer::new();
        dsl_core::print_shape(&dsl_core::FieldValue::Wire(value.clone()), &dsl_core::Shape::Wire, &mut writer);
        writer.render(dsl_core::JoinMode::Inline)
    }

    /// 📝️ Render wire-literal text from neutral node/edge rows, one unified `dsl_core::Wire`
    /// statement per line.
    pub fn wire_literal_from_dag(nodes: &[WireNode], edges: &[WireEdge]) -> String {
        let mut lines = Vec::new();
        for node in nodes {
            let value = dsl_core::WireValue {
                from: dsl_core::WireNode { id: node.id.clone(), kind: Some(node.kind.clone()), port: node.port.clone() },
                edge: None,
                edge_label: dsl_core::WireEdgeLabel::default(),
                properties: properties_to_dsl_object(&node.properties),
            };
            lines.push(render_wire_line(&value));
        }
        for edge in edges {
            let from_kind = nodes.iter().find(|n| n.id == edge.from).map_or("node", |n| n.kind.as_str());
            let to_kind = nodes.iter().find(|n| n.id == edge.to).map_or("node", |n| n.kind.as_str());
            let value = dsl_core::WireValue {
                from: dsl_core::WireNode { id: edge.from.clone(), kind: Some(from_kind.to_string()), port: Some(edge.from_port.clone()) },
                edge: Some((edge.directed, dsl_core::WireNode { id: edge.to.clone(), kind: Some(to_kind.to_string()), port: Some(edge.to_port.clone()) })),
                edge_label: dsl_core::WireEdgeLabel::default(),
                properties: properties_to_dsl_object(&edge.properties),
            };
            lines.push(render_wire_line(&value));
        }
        lines.join("\n")
    }

    /// 🔍️ Parse wire-literal text into neutral node/edge rows. Delegates lexing+parsing to
    /// `dsl_core::parse_wire_text` (the one unified wire grammar — `->`/`<-` sugar/`--`,
    /// `{k=v}` double-quoted properties) one statement (line) at a time, then enforces this
    /// module's own DAG domain rule on top: an edge's ports are mandatory on BOTH ends (the
    /// shared grammar itself leaves ports optional on every endpoint — that's the engine's
    /// business, not a syntax difference this module should encode into the lexer/parser).
    pub fn dag_from_wire_literal(text: &str) -> Result<(Vec<WireNode>, Vec<WireEdge>), GraphDslError> {
        let mut nodes = Vec::new();
        let mut edges = Vec::new();
        for line in text.lines() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }
            let value = dsl_core::parse_wire_text(line)?;
            match value.edge {
                None => nodes.push(WireNode { id: value.from.id, kind: value.from.kind.unwrap_or_else(|| "node".to_string()), port: value.from.port, properties: properties_from_dsl_value(&value.properties) }),
                Some((directed, to)) => {
                    let from_port = value.from.port.ok_or(GraphDslError::EdgeTargetMissingPort)?;
                    let to_port = to.port.ok_or(GraphDslError::EdgeTargetMissingPort)?;
                    edges.push(WireEdge { from: value.from.id, from_port, to: to.id, to_port, directed, properties: properties_from_dsl_value(&value.properties) });
                }
            }
        }
        Ok((nodes, edges))
    }
    // #endregion 🔖️WireLiteral

    // #region 🔖️Tests
    #[cfg(test)]
    include!("🧪️tests/🔬️wire-unit/🦀️.rs");
    // #endregion 🔖️Tests
    // #endregion wire
}

pub use queryable::{manifest_edge_kinds, manifest_node_kinds, manifest_port_kinds, manifest_property_names, BoardQueryableGraph, QueryableEdge, QueryableGraph};
pub use wire::{dag_from_wire_literal, wire_literal_from_dag, WireEdge, WireNode};

use crate::manifest::PropertyValue;
use std::collections::{BTreeMap, BTreeSet};

// #region jack_impl

// #region 🔖️Ast
/// 🌳️ Jack query abstract syntax tree.
#[derive(Clone, Debug, PartialEq)]
pub struct Query {
    pub clauses: Vec<Clause>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Clause {
    Match(Vec<Pattern>),
    Where(Expr),
    /// 🔭️ `WITH <items>` — projects the current bindings down to just the named vars (dropping
    /// everything else out of scope), no more than [`ReturnItem`] itself models: no aliasing, no
    /// `DISTINCT`, no `ORDER BY`/`SKIP`/`LIMIT`. A trailing filter is just the next
    /// [`Clause::Where`] in sequence — it needs no special-casing here.
    With(Vec<ReturnItem>),
    /// 🌀️ `UNWIND <source> AS <var>` — see [`UnwindClause`].
    Unwind(UnwindClause),
    /// 📞️ `CALL <name>(<args>...)` — see [`CallClause`] and [`call_procedure`].
    Call(CallClause),
    Return(Vec<ReturnItem>),
    Create(Pattern),
    Delete(Vec<String>),
    Set(Vec<Assignment>),
    Merge(Pattern),
}

/// 🌀️ `UNWIND <source> AS <var>` — flattens a list-valued source into per-row bindings of `var`.
#[derive(Clone, Debug, PartialEq)]
pub struct UnwindClause {
    pub source: ReturnItem,
    pub var: String,
}

/// 📞️ `CALL <name>(<args>...)` — a named procedure invocation with positional scalar arguments.
#[derive(Clone, Debug, PartialEq)]
pub struct CallClause {
    pub name: String,
    pub args: Vec<PropertyValue>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Pattern {
    pub nodes: Vec<PatternNode>,
    pub edge: Option<PatternEdge>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PatternNode {
    pub var: String,
    pub kind: String,
    pub port: Option<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PatternEdge {
    pub var: Option<String>,
    pub kind: Option<String>,
    pub directed: bool,
    pub right: PatternNode,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ReturnItem {
    Var(String),
    Property { var: String, prop: String },
}

#[derive(Clone, Debug, PartialEq)]
pub struct Assignment {
    pub var: String,
    pub prop: String,
    pub value: PropertyValue,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
    Eq { var: String, prop: String, value: PropertyValue },
    Ne { var: String, prop: String, value: PropertyValue },
    And(Box<Expr>, Box<Expr>),
    Or(Box<Expr>, Box<Expr>),
}

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub enum QueryResultKind {
    #[default]
    Table,
    Graph,
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct QueryResult {
    #[value(default)]
    pub kind: QueryResultKind,
    pub columns: Vec<String>,
    pub rows: Vec<Vec<PropertyValue>>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub graph_fixture_json: Option<String>,
}

impl QueryResult {
    pub fn table(columns: Vec<String>, rows: Vec<Vec<PropertyValue>>) -> Self {
        Self { kind: QueryResultKind::Table, columns, rows, graph_fixture_json: None }
    }

    pub fn graph(columns: Vec<String>, graph_fixture_json: String) -> Self {
        Self { kind: QueryResultKind::Graph, columns, rows: vec![], graph_fixture_json: Some(graph_fixture_json) }
    }
}
// #endregion 🔖️Ast

// #region 🔖️Lexer
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub enum TokenClass {
    Keyword,
    Ident,
    Number,
    String,
    Operator,
    Punctuation,
    Error,
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct TokenSpan {
    pub class: TokenClass,
    pub start: usize,
    pub end: usize,
}

#[derive(Clone, Debug, PartialEq)]
enum Token {
    KwMatch,
    KwWhere,
    KwReturn,
    KwCreate,
    KwDelete,
    KwSet,
    KwMerge,
    KwWith,
    KwUnwind,
    KwCall,
    KwAs,
    Ident(String),
    Number(f64),
    StringLit(String),
    LParen,
    RParen,
    LBracket,
    RBracket,
    Colon,
    Comma,
    Dot,
    Eq,
    Ne,
    Arrow,
    /// ➖️ A bare `-`, the Cypher spelling of the pattern connector in `(a)-[r]->(b)`. `DashArrow`
    /// (`--`) is this grammar's own equal-status spelling; both reach the same `PatternEdge`.
    Dash,
    DashArrow,
    BackArrow,
    At,
    And,
    Or,
    Eof,
}

#[derive(Clone, Debug, PartialEq)]
struct SpannedToken {
    token: Token,
    start: usize,
    end: usize,
}

fn token_class(token: &Token) -> TokenClass {
    match token {
        Token::KwMatch | Token::KwWhere | Token::KwReturn | Token::KwCreate | Token::KwDelete | Token::KwSet | Token::KwMerge | Token::KwWith | Token::KwUnwind | Token::KwCall | Token::KwAs | Token::And | Token::Or => TokenClass::Keyword,
        Token::Ident(_) => TokenClass::Ident,
        Token::Number(_) => TokenClass::Number,
        Token::StringLit(_) => TokenClass::String,
        Token::Eq | Token::Ne | Token::Arrow | Token::Dash | Token::DashArrow | Token::BackArrow | Token::At => TokenClass::Operator,
        Token::LParen | Token::RParen | Token::LBracket | Token::RBracket | Token::Colon | Token::Comma | Token::Dot => TokenClass::Punctuation,
        Token::Eof => TokenClass::Punctuation,
    }
}

fn push_spanned(tokens: &mut Vec<SpannedToken>, token: Token, start: usize, end: usize) {
    tokens.push(SpannedToken { token, start, end });
}

/// 🔑️ Uppercases and matches against Jack's clause/logic keyword table; anything else stays a
/// plain variable/property/kind identifier. Case-insensitive (Cypher heritage — `match`, `Match`,
/// `MATCH` are all the same token), unlike `dsl_core`'s own grammars which are case-sensitive.
fn keyword_or_ident(text: String) -> Token {
    match text.to_ascii_uppercase().as_str() {
        "MATCH" => Token::KwMatch,
        "WHERE" => Token::KwWhere,
        "RETURN" => Token::KwReturn,
        "CREATE" => Token::KwCreate,
        "DELETE" => Token::KwDelete,
        "SET" => Token::KwSet,
        "MERGE" => Token::KwMerge,
        "WITH" => Token::KwWith,
        "UNWIND" => Token::KwUnwind,
        "CALL" => Token::KwCall,
        "AS" => Token::KwAs,
        "AND" => Token::And,
        "OR" => Token::Or,
        _ => Token::Ident(text),
    }
}

/// 🪚️ `dsl_core` treats `.` as ident-continue (so `a.name` lexes as ONE ident there), but Jack's
/// `var.prop` property-access grammar needs `.` as its own token — splits it back apart here,
/// checking each piece against the keyword table too (defensive; keywords never legitimately
/// contain a dot, but this keeps the one keyword-recognition path authoritative).
fn push_ident_or_keyword_with_dots(text: &str, start: usize, out: &mut Vec<SpannedToken>) {
    let mut offset = 0usize;
    for (idx, part) in text.split('.').enumerate() {
        if idx > 0 {
            push_spanned(out, Token::Dot, start + offset, start + offset + 1);
            offset += 1;
        }
        if !part.is_empty() {
            let end = start + offset + part.len();
            push_spanned(out, keyword_or_ident(part.to_string()), start + offset, end);
        }
        offset += part.len();
    }
}

/// 🔬️ Converts one already-lexed `dsl_core` segment (containing no quotes or `!=` — those are
/// scanned by [`lex_spanned`] itself, ahead of delegating everything else) into Jack's own
/// richer, grammar-aware token stream.
fn push_dsl_core_segment(segment: &str, base_offset: usize, forgiving: bool, out: &mut Vec<SpannedToken>) -> Result<(), GraphDslError> {
    if segment.is_empty() {
        return Ok(());
    }
    let raw = semio_framework_dsl::lex(segment, &semio_framework_diagnostic::Limits::default(), forgiving).map_err(GraphDslError::Lex)?;
    for token in raw {
        if token.kind.is_trivia() || token.kind == semio_framework_dsl::TokenKind::Eof {
            continue;
        }
        let start = base_offset + token.byte_range.0 as usize;
        let end = base_offset + token.byte_range.1 as usize;
        let text = token.text.as_str().to_string();
        match token.kind {
            semio_framework_dsl::TokenKind::Ident => push_ident_or_keyword_with_dots(&text, start, out),
            // A lone `_` is `dsl_core`'s placeholder sigil; Jack has no placeholder concept of its
            // own, so it round-trips as an ordinary one-character identifier.
            semio_framework_dsl::TokenKind::Placeholder => push_spanned(out, Token::Ident(text), start, end),
            semio_framework_dsl::TokenKind::Int | semio_framework_dsl::TokenKind::Float => {
                let n: f64 = text.parse().map_err(GraphDslError::NumberFormat)?;
                push_spanned(out, Token::Number(n), start, end);
            }
            semio_framework_dsl::TokenKind::LParen => push_spanned(out, Token::LParen, start, end),
            semio_framework_dsl::TokenKind::RParen => push_spanned(out, Token::RParen, start, end),
            semio_framework_dsl::TokenKind::LBracket => push_spanned(out, Token::LBracket, start, end),
            semio_framework_dsl::TokenKind::RBracket => push_spanned(out, Token::RBracket, start, end),
            semio_framework_dsl::TokenKind::Colon => push_spanned(out, Token::Colon, start, end),
            semio_framework_dsl::TokenKind::Comma => push_spanned(out, Token::Comma, start, end),
            semio_framework_dsl::TokenKind::Equals => push_spanned(out, Token::Eq, start, end),
            semio_framework_dsl::TokenKind::At => push_spanned(out, Token::At, start, end),
            semio_framework_dsl::TokenKind::Arrow => push_spanned(out, Token::Arrow, start, end),
            // ➖️ A bare `-` is the pattern connector every Cypher-shaped query writes
            // (`(a)-[r:Kind]->(b)`), which is also what this dialect's own executor parser accepts
            // and what every committed example query in the repo uses. It used to fall into the
            // stray-character bucket below and every such query was reported
            // `unexpected character '-'` by lint/complete/hover/format while running perfectly.
            semio_framework_dsl::TokenKind::Minus => push_spanned(out, Token::Dash, start, end),
            semio_framework_dsl::TokenKind::DashArrow => push_spanned(out, Token::DashArrow, start, end),
            semio_framework_dsl::TokenKind::BackArrow => push_spanned(out, Token::BackArrow, start, end),
            // Double-quoted text delegated straight through `dsl_core` — unreachable in practice
            // since `lex_spanned` pre-scans and consumes every quote itself before ever
            // delegating a segment, kept only for defensive completeness.
            semio_framework_dsl::TokenKind::Text => push_spanned(out, Token::StringLit(text), start, end),
            // `{`/`}` aren't part of Jack's grammar (no map/object literals) — same "stray
            // character" treatment as an outright `os_dsl::TokenKind::Error` below. P2-M1's
            // promoted `< > & $ ;` tokens and STEP's `DotEnum` literal join this bucket too —
            // Jack has no grammar concept for any of them either.
            semio_framework_dsl::TokenKind::EdgeArrow
            | semio_framework_dsl::TokenKind::LBrace
            | semio_framework_dsl::TokenKind::RBrace
            | semio_framework_dsl::TokenKind::Caret
            | semio_framework_dsl::TokenKind::DotDot
            | semio_framework_dsl::TokenKind::Plus
            | semio_framework_dsl::TokenKind::Star
            | semio_framework_dsl::TokenKind::Slash
            | semio_framework_dsl::TokenKind::Fence
            | semio_framework_dsl::TokenKind::Lt
            | semio_framework_dsl::TokenKind::Gt
            | semio_framework_dsl::TokenKind::Amp
            | semio_framework_dsl::TokenKind::Dollar
            | semio_framework_dsl::TokenKind::Semicolon
            | semio_framework_dsl::TokenKind::DotEnum
            | semio_framework_dsl::TokenKind::Error => {
                if forgiving {
                    push_spanned(out, Token::Ident(text), start, end);
                } else {
                    return Err(GraphDslError::UnexpectedChar(text.chars().next().unwrap_or('?')));
                }
            }
            semio_framework_dsl::TokenKind::Whitespace | semio_framework_dsl::TokenKind::Newline | semio_framework_dsl::TokenKind::Comment | semio_framework_dsl::TokenKind::Eof => {
                unreachable!("trivia/Eof filtered above")
            }
        }
    }
    Ok(())
}

/// 🔬️ Jack's own lexer: unifies on `os_dsl::lex` for the shared token alphabet (idents,
/// numbers, punctuation, `(`/`)`/`[`/`]`, `->`/`--`/`<-`) but keeps two genuinely Cypher-specific
/// pieces local, since neither fits `dsl_core`'s grammar-independent alphabet: dual-quote strings
/// (`'x'`/`"x"` — Cypher heritage; `dsl_core` only ever lexes `"..."`) and the `!=` comparison
/// operator (`dsl_core` has no relational operators at all — it's a structural DSL alphabet, not
/// an expression language). Both are pre-scanned as their own tokens; every remaining run of
/// characters is delegated whole to `os_dsl::lex` and converted via [`push_dsl_core_segment`].
fn lex_spanned(input: &str, forgiving: bool) -> Result<Vec<SpannedToken>, GraphDslError> {
    let bytes = input.as_bytes();
    let mut tokens = Vec::new();
    let mut i = 0usize;
    let mut seg_start = 0usize;

    while i < bytes.len() {
        let c = bytes[i];
        if c == b'\'' || c == b'"' {
            push_dsl_core_segment(&input[seg_start..i], seg_start, forgiving, &mut tokens)?;
            let quote = c;
            let start = i;
            i += 1;
            let content_start = i;
            let mut closed = false;
            while i < bytes.len() {
                if bytes[i] == b'\\' && i + 1 < bytes.len() {
                    i += 2;
                    continue;
                }
                if bytes[i] == quote {
                    closed = true;
                    break;
                }
                i += 1;
            }
            let raw = String::from_utf8_lossy(&bytes[content_start..i]).into_owned();
            if !closed {
                if forgiving {
                    push_spanned(&mut tokens, Token::StringLit(raw), start, i);
                    seg_start = i;
                    break;
                }
                return Err(GraphDslError::UnterminatedString);
            }
            i += 1;
            let text = semio_framework_dsl::unescape_text(&raw, forgiving).unwrap_or(raw);
            push_spanned(&mut tokens, Token::StringLit(text), start, i);
            seg_start = i;
            continue;
        }
        if c == b'!' && i + 1 < bytes.len() && bytes[i + 1] == b'=' {
            push_dsl_core_segment(&input[seg_start..i], seg_start, forgiving, &mut tokens)?;
            push_spanned(&mut tokens, Token::Ne, i, i + 2);
            i += 2;
            seg_start = i;
            continue;
        }
        i += 1;
    }
    push_dsl_core_segment(&input[seg_start..bytes.len()], seg_start, forgiving, &mut tokens)?;
    push_spanned(&mut tokens, Token::Eof, input.len(), input.len());
    Ok(tokens)
}

fn lex(input: &str) -> Result<Vec<Token>, GraphDslError> {
    lex_spanned(input, false).map(|spanned| spanned.into_iter().map(|row| row.token).collect())
}

/// 🎨️ Tokenize jack source for editor highlighting (never fails).
pub fn tokenize(input: &str) -> Vec<TokenSpan> {
    // 🔀️ Rewritten from `.map(..)` — `token_class` is async and cannot be called inside the sync
    // closure that used to build each `TokenSpan` (R10 residue shape #1).
    let rows = lex_spanned(input, true).unwrap_or_default();
    let mut out = Vec::with_capacity(rows.len());
    for row in rows {
        if matches!(row.token, Token::Eof) {
            continue;
        }
        let mut class = token_class(&row.token);
        if matches!(row.token, Token::StringLit(_)) {
            let quote = input.as_bytes().get(row.start);
            if quote == Some(&b'\'') || quote == Some(&b'"') {
                let closed = input.as_bytes().get(row.end.saturating_sub(1)) == quote;
                if !closed {
                    class = TokenClass::Error;
                }
            }
        }
        out.push(TokenSpan { class, start: row.start, end: row.end });
    }
    out
}
// #endregion 🔖️Lexer

// #region 🔖️Language
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct Completion {
    pub label: String,
    pub kind: String,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
    pub insert: String,
}

const CLAUSE_KEYWORDS: &[&str] = &["MATCH", "WHERE", "RETURN", "CREATE", "DELETE", "SET", "MERGE", "WITH", "UNWIND", "CALL"];
const LOGIC_KEYWORDS: &[&str] = &["AND", "OR"];

fn completion_prefix(source: &str, cursor: usize) -> String {
    let cursor = cursor.min(source.len());
    let bytes = source.as_bytes();
    let mut start = cursor;
    while start > 0 {
        let c = bytes[start - 1];
        if c.is_ascii_alphanumeric() || c == b'_' {
            start -= 1;
        } else {
            break;
        }
    }
    source[start..cursor].to_string()
}

fn tokens_before_cursor(tokens: &[SpannedToken], cursor: usize) -> &[SpannedToken] {
    let mut end = tokens.len();
    for (i, row) in tokens.iter().enumerate() {
        if row.start >= cursor && !matches!(row.token, Token::Eof) {
            end = i;
            break;
        }
    }
    &tokens[..end]
}

fn after_colon_kind_context(source: &str, cursor: usize) -> Option<bool> {
    let cursor = cursor.min(source.len());
    let before = &source[..cursor];
    let colon = before.rfind(':')?;
    let after = &before[colon + 1..];
    // 🩹️ an `@` after the kind name means the cursor moved into the port segment (`kind@port`);
    // bail so `after_at_port_context` can offer port completions instead of kind completions.
    if after.chars().any(|c| c.is_whitespace() || matches!(c, '(' | ')' | '[' | ']' | ',' | '@')) {
        return None;
    }
    let left = &before[..colon];
    let bracket = left.rfind('[');
    let paren = left.rfind('(');
    let in_bracket = match (bracket, paren) {
        (Some(b), Some(p)) => b > p,
        (Some(_), None) => true,
        _ => false,
    };
    Some(in_bracket)
}

fn after_dot_property_context(source: &str, cursor: usize) -> bool {
    let cursor = cursor.min(source.len());
    let before = &source[..cursor];
    let Some(dot) = before.rfind('.') else {
        return false;
    };
    let after = &before[dot + 1..];
    !after.chars().any(|c| c.is_whitespace() || matches!(c, '(' | ')' | '[' | ']' | ',' | ':'))
}
fn open_bracket_kind(tokens: &[SpannedToken]) -> Option<char> {
    let mut paren = 0i32;
    let mut bracket = 0i32;
    for row in tokens.iter().rev() {
        match row.token {
            Token::RParen => paren += 1,
            Token::LParen if paren > 0 => paren -= 1,
            Token::LParen if paren == 0 && bracket == 0 => return Some('('),
            Token::RBracket => bracket += 1,
            Token::LBracket if bracket > 0 => bracket -= 1,
            Token::LBracket if bracket == 0 && paren == 0 => return Some('['),
            _ => {}
        }
    }
    None
}

fn collect_bound_vars(tokens: &[SpannedToken]) -> BTreeSet<String> {
    let mut vars = BTreeSet::new();
    let mut i = 0;
    while i + 2 < tokens.len() {
        if matches!(tokens[i].token, Token::LParen | Token::LBracket) {
            if let Token::Ident(var) = &tokens[i + 1].token {
                if matches!(tokens[i + 2].token, Token::Colon) {
                    vars.insert(var.clone());
                }
            }
        }
        i += 1;
    }
    vars
}

fn in_where_clause(tokens: &[SpannedToken]) -> bool {
    let mut seen_where = false;
    let mut seen_return = false;
    for row in tokens {
        match row.token {
            Token::KwWhere => seen_where = true,
            Token::KwReturn => seen_return = true,
            _ => {}
        }
    }
    seen_where && !seen_return
}

fn graph_node_kinds<G: QueryableGraph>(graph: &G) -> Vec<String> {
    manifest_node_kinds(graph)
}

fn graph_edge_kinds<G: QueryableGraph>(graph: &G) -> Vec<String> {
    manifest_edge_kinds(graph)
}

fn graph_property_names<G: QueryableGraph>(graph: &G) -> Vec<String> {
    manifest_property_names(graph)
}

fn graph_port_kinds<G: QueryableGraph>(graph: &G) -> Vec<String> {
    manifest_port_kinds(graph)
}

fn after_at_port_context(source: &str, cursor: usize) -> bool {
    let cursor = cursor.min(source.len());
    let before = &source[..cursor];
    let Some(at) = before.rfind('@') else {
        return false;
    };
    let after = &before[at + 1..];
    !after.chars().any(|c| c.is_whitespace() || matches!(c, '(' | ')' | '[' | ']' | ',' | '-' | '>' | '@'))
}

fn filter_completions(candidates: impl IntoIterator<Item = (String, String, Option<String>)>, prefix: &str) -> Vec<Completion> {
    let prefix_lower = prefix.to_ascii_lowercase();
    let mut out = Vec::new();
    for (label, kind, detail) in candidates {
        if prefix.is_empty() || label.to_ascii_lowercase().starts_with(&prefix_lower) {
            out.push(Completion { insert: label.clone(), label, kind, detail });
        }
    }
    out.sort_by(|a, b| a.label.cmp(&b.label));
    out
}

/// 🔎️ Context-aware jack completions for the editor.
pub fn complete<G: QueryableGraph>(graph: &G, source: &str, cursor: usize) -> Vec<Completion> {
    let cursor = cursor.min(source.len());
    let prefix = completion_prefix(source, cursor);
    let tokens = lex_spanned(source, true).unwrap_or_default();
    let before = tokens_before_cursor(&tokens, cursor);

    if let Some(in_bracket) = after_colon_kind_context(source, cursor) {
        let kinds = if in_bracket { graph_edge_kinds(graph).into_iter().map(|name| (name, "edgeKind".into(), None)).collect::<Vec<_>>() } else { graph_node_kinds(graph).into_iter().map(|name| (name, "nodeKind".into(), None)).collect::<Vec<_>>() };
        return filter_completions(kinds, &prefix);
    }

    if after_dot_property_context(source, cursor) {
        let props = graph_property_names(graph).into_iter().map(|name| (name, "property".into(), None)).collect::<Vec<_>>();
        return filter_completions(props, &prefix);
    }

    if after_at_port_context(source, cursor) {
        let ports = graph_port_kinds(graph).into_iter().map(|name| (name, "portKind".into(), None)).collect::<Vec<_>>();
        return filter_completions(ports, &prefix);
    }

    if let Some(last) = before.last() {
        if matches!(last.token, Token::At) {
            let ports = graph_port_kinds(graph).into_iter().map(|name| (name, "portKind".into(), None)).collect::<Vec<_>>();
            return filter_completions(ports, &prefix);
        }
        if matches!(last.token, Token::Colon) {
            let kinds = if open_bracket_kind(before) == Some('[') {
                graph_edge_kinds(graph).into_iter().map(|name| (name, "edgeKind".into(), None)).collect::<Vec<_>>()
            } else {
                graph_node_kinds(graph).into_iter().map(|name| (name, "nodeKind".into(), None)).collect::<Vec<_>>()
            };
            return filter_completions(kinds, &prefix);
        }
        if matches!(last.token, Token::Dot) {
            let props = graph_property_names(graph).into_iter().map(|name| (name, "property".into(), None)).collect::<Vec<_>>();
            return filter_completions(props, &prefix);
        }
    }

    if in_where_clause(before) {
        let logic = filter_completions(LOGIC_KEYWORDS.iter().map(|kw| (kw.to_string(), "keyword".into(), None)), &prefix);
        if !logic.is_empty() {
            return logic;
        }
    }

    let vars = collect_bound_vars(before);
    if !vars.is_empty() && prefix.chars().next().is_some_and(|c| c.is_ascii_alphabetic() || c == '_') {
        let var_items = vars.into_iter().map(|name| (name, "variable".into(), None)).collect::<Vec<_>>();
        let filtered = filter_completions(var_items, &prefix);
        if !filtered.is_empty() {
            return filtered;
        }
    }

    filter_completions(CLAUSE_KEYWORDS.iter().map(|kw| (kw.to_string(), "keyword".into(), None)), &prefix)
}
// #endregion 🔖️Language

// #region 🔖️LanguageService
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub enum DiagnosticSeverity {
    Error,
    Warning,
    Information,
    Info,
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct Diagnostic {
    pub start: usize,
    pub end: usize,
    pub severity: DiagnosticSeverity,
    pub message: String,
    // 🌉️ `default` added even though the serde attribute list only carries `skip_serializing_if`:
    // serde treats an `Option<T>` field as implicitly optional on deserialize with no explicit
    // `default`, but `#[derive(ToValue, FromValue)]` has no such special case — a genuinely omitted
    // wire key needs `default` spelled out or `FromValue` errors "missing field" instead of `None`.
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct Hover {
    pub start: usize,
    pub end: usize,
    pub contents: String,
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct SemanticToken {
    pub start: usize,
    pub end: usize,
    pub class: String,
}

fn collect_pattern_vars(pattern: &Pattern, out: &mut BTreeSet<String>) {
    for node in &pattern.nodes {
        out.insert(node.var.clone());
    }
    if let Some(edge) = &pattern.edge {
        if let Some(var) = &edge.var {
            out.insert(var.clone());
        }
        out.insert(edge.right.var.clone());
    }
}

fn collect_clause_bound_vars(clauses: &[Clause]) -> BTreeSet<String> {
    let mut vars = BTreeSet::new();
    for clause in clauses {
        match clause {
            Clause::Match(patterns) => {
                for pattern in patterns {
                    collect_pattern_vars(pattern, &mut vars);
                }
            }
            Clause::Create(pattern) | Clause::Merge(pattern) => collect_pattern_vars(pattern, &mut vars),
            _ => {}
        }
    }
    vars
}

fn collect_referenced_vars(clauses: &[Clause]) -> Vec<(String, usize, usize)> {
    let mut refs = Vec::new();
    for clause in clauses {
        match clause {
            Clause::Return(items) => {
                for item in items {
                    match item {
                        ReturnItem::Var(v) => refs.push((v.clone(), 0, v.len())),
                        ReturnItem::Property { var, .. } => refs.push((var.clone(), 0, var.len())),
                    }
                }
            }
            Clause::Delete(vars) => {
                for var in vars {
                    refs.push((var.clone(), 0, var.len()));
                }
            }
            Clause::Set(assignments) => {
                for assignment in assignments {
                    refs.push((assignment.var.clone(), 0, assignment.var.len()));
                }
            }
            Clause::Where(expr) => collect_expr_vars(expr, &mut refs),
            _ => {}
        }
    }
    refs
}

fn collect_expr_vars(expr: &Expr, refs: &mut Vec<(String, usize, usize)>) {
    match expr {
        Expr::Eq { var, .. } | Expr::Ne { var, .. } => refs.push((var.clone(), 0, var.len())),
        Expr::And(a, b) | Expr::Or(a, b) => {
            collect_expr_vars(a, refs);
            collect_expr_vars(b, refs);
        }
    }
}

fn semantic_lints<G: QueryableGraph>(graph: &G, query: &Query, source: &str) -> Vec<Diagnostic> {
    let mut out = Vec::new();
    let node_kinds = graph_node_kinds(graph).into_iter().collect::<BTreeSet<_>>();
    let edge_kinds = graph_edge_kinds(graph).into_iter().collect::<BTreeSet<_>>();
    let bound = collect_clause_bound_vars(&query.clauses);
    for clause in &query.clauses {
        match clause {
            Clause::Match(patterns) => {
                for pattern in patterns {
                    for node in &pattern.nodes {
                        if !node_kinds.contains(&node.kind) {
                            if let Some((start, end)) = find_kind_span(source, &node.kind) {
                                out.push(Diagnostic { start, end, severity: DiagnosticSeverity::Error, message: format!("unknown node kind '{}'", node.kind), code: Some("jack/unknown-node-kind".into()) });
                            }
                        }
                    }
                    if let Some(edge) = &pattern.edge {
                        if let Some(kind) = &edge.kind {
                            if !edge_kinds.contains(kind) {
                                if let Some((start, end)) = find_kind_span(source, kind) {
                                    out.push(Diagnostic { start, end, severity: DiagnosticSeverity::Error, message: format!("unknown edge kind '{}'", kind), code: Some("jack/unknown-edge-kind".into()) });
                                }
                            }
                        }
                    }
                }
            }
            Clause::Create(pattern) | Clause::Merge(pattern) => {
                for node in &pattern.nodes {
                    if !node_kinds.contains(&node.kind) {
                        if let Some((start, end)) = find_kind_span(source, &node.kind) {
                            out.push(Diagnostic { start, end, severity: DiagnosticSeverity::Error, message: format!("unknown node kind '{}'", node.kind), code: Some("jack/unknown-node-kind".into()) });
                        }
                    }
                }
            }
            _ => {}
        }
    }
    for (var, _, _) in collect_referenced_vars(&query.clauses) {
        if !bound.contains(&var) {
            if let Some((start, end)) = find_ident_span(source, &var) {
                out.push(Diagnostic { start, end, severity: DiagnosticSeverity::Error, message: format!("variable '{var}' is not bound by MATCH"), code: Some("jack/unbound-variable".into()) });
            }
        }
    }
    out
}

fn find_kind_span(source: &str, kind: &str) -> Option<(usize, usize)> {
    let needle = format!(":{kind}");
    let start = source.find(&needle)?;
    Some((start + 1, start + needle.len()))
}

fn find_ident_span(source: &str, ident: &str) -> Option<(usize, usize)> {
    let mut from = 0;
    while let Some(rel) = source[from..].find(ident) {
        let start = from + rel;
        let end = start + ident.len();
        let before = source.as_bytes().get(start.wrapping_sub(1));
        let after = source.as_bytes().get(end);
        let boundary_before = before.is_none_or(|c| !c.is_ascii_alphanumeric() && *c != b'_');
        let boundary_after = after.is_none_or(|c| !c.is_ascii_alphanumeric() && *c != b'_');
        if boundary_before && boundary_after {
            return Some((start, end));
        }
        from = end;
    }
    None
}

/// 🩺️ Lint jack source with syntax and semantic diagnostics.
pub fn lint<G: QueryableGraph>(graph: &G, source: &str) -> Vec<Diagnostic> {
    let mut out = Vec::new();
    for span in tokenize(source) {
        if span.class == TokenClass::Error {
            out.push(Diagnostic { start: span.start, end: span.end, severity: DiagnosticSeverity::Error, message: "unterminated string literal".into(), code: Some("jack/unterminated-string".into()) });
        }
    }
    match parse(source) {
        Ok(query) => out.extend(semantic_lints(graph, &query, source)),
        Err(err) => {
            let end = source.len().max(1);
            out.push(Diagnostic { start: 0, end, severity: DiagnosticSeverity::Error, message: err.to_string(), code: Some("jack/parse-error".into()) });
        }
    }
    out
}

fn format_token(tok: &Token) -> String {
    match tok {
        Token::KwMatch => "MATCH".into(),
        Token::KwWhere => "WHERE".into(),
        Token::KwReturn => "RETURN".into(),
        Token::KwCreate => "CREATE".into(),
        Token::KwDelete => "DELETE".into(),
        Token::KwSet => "SET".into(),
        Token::KwMerge => "MERGE".into(),
        Token::KwWith => "WITH".into(),
        Token::KwUnwind => "UNWIND".into(),
        Token::KwCall => "CALL".into(),
        Token::KwAs => "AS".into(),
        Token::And => "AND".into(),
        Token::Or => "OR".into(),
        Token::Ident(s) => s.clone(),
        Token::Number(n) => {
            if n.fract() == 0.0 {
                format!("{}", *n as i64)
            } else {
                n.to_string()
            }
        }
        // 🩹️ unified syntax law: strings always PRINT double-quoted with `dsl_core`'s canonical
        // escape, regardless of which quote style the source used.
        Token::StringLit(s) => format!("\"{}\"", semio_framework_dsl::escape_text(s)),
        Token::LParen => "(".into(),
        Token::RParen => ")".into(),
        Token::LBracket => "[".into(),
        Token::RBracket => "]".into(),
        Token::Colon => ":".into(),
        Token::Comma => ",".into(),
        Token::Dot => ".".into(),
        Token::Eq => "=".into(),
        Token::Ne => "!=".into(),
        Token::Arrow => "->".into(),
        Token::Dash => "-".into(),
        Token::DashArrow => "--".into(),
        Token::BackArrow => "<-".into(),
        Token::At => "@".into(),
        Token::Eof => String::new(),
    }
}

/// 🪞️ Format jack source canonically (idempotent).
pub fn format(source: &str) -> Result<String, GraphDslError> {
    let tokens = lex_spanned(source, false)?;
    let mut out = String::new();
    let mut line_open = false;
    let mut i = 0;
    while i < tokens.len() {
        let row = &tokens[i];
        if matches!(row.token, Token::Eof) {
            break;
        }
        match &row.token {
            Token::KwMatch | Token::KwWhere | Token::KwReturn | Token::KwCreate | Token::KwDelete | Token::KwSet | Token::KwMerge | Token::KwWith | Token::KwUnwind | Token::KwCall => {
                if !out.is_empty() {
                    out.push('\n');
                }
                out.push_str(&format_token(&row.token));
                out.push(' ');
                line_open = true;
            }
            Token::Comma => {
                out.push_str(", ");
            }
            Token::Arrow => {
                out.push_str("->");
            }
            Token::Dash => {
                out.push_str("-");
            }
            Token::DashArrow => {
                out.push_str("--");
            }
            Token::BackArrow => {
                out.push_str("<-");
            }
            Token::And | Token::Or | Token::KwAs => {
                out.push(' ');
                out.push_str(&format_token(&row.token));
                out.push(' ');
            }
            Token::Eq | Token::Ne => {
                out.push(' ');
                out.push_str(&format_token(&row.token));
                out.push(' ');
            }
            _ => {
                if line_open && !out.ends_with(' ') && !out.ends_with('\n') && !matches!(row.token, Token::RParen | Token::RBracket | Token::Comma | Token::Dot) {
                    let prev = tokens.get(i.saturating_sub(1)).map(|t| &t.token);
                    if !matches!(prev, Some(Token::LParen | Token::LBracket | Token::Colon | Token::Dot | Token::Arrow | Token::Dash | Token::DashArrow | Token::BackArrow)) {
                        out.push(' ');
                    }
                }
                out.push_str(&format_token(&row.token));
            }
        }
        i += 1;
    }
    Ok(out.trim().to_string())
}

fn hover_word_at(source: &str, cursor: usize) -> Option<(usize, usize, String)> {
    let cursor = cursor.min(source.len());
    if cursor > source.len() {
        return None;
    }
    let bytes = source.as_bytes();
    let mut start = cursor;
    while start > 0 {
        let c = bytes[start - 1];
        if c.is_ascii_alphanumeric() || c == b'_' || c == b':' || c == b'.' {
            start -= 1;
        } else {
            break;
        }
    }
    let mut end = cursor;
    while end < bytes.len() {
        let c = bytes[end];
        if c.is_ascii_alphanumeric() || c == b'_' || c == b':' || c == b'.' {
            end += 1;
        } else {
            break;
        }
    }
    if start == end {
        return None;
    }
    Some((start, end, source[start..end].to_string()))
}

/// 💬️ Hover information at cursor.
pub fn hover<G: QueryableGraph>(graph: &G, source: &str, cursor: usize) -> Option<Hover> {
    let (start, end, word) = hover_word_at(source, cursor)?;
    let upper = word.to_ascii_uppercase();
    if CLAUSE_KEYWORDS.iter().any(|kw| *kw == upper) || LOGIC_KEYWORDS.iter().any(|kw| *kw == upper) {
        return Some(Hover { start, end, contents: format!("Jack keyword `{upper}`") });
    }
    if graph_node_kinds(graph).iter().any(|kind| kind == &word) {
        return Some(Hover { start, end, contents: format!("Node kind `{word}`") });
    }
    if graph_edge_kinds(graph).iter().any(|kind| kind == &word) {
        return Some(Hover { start, end, contents: format!("Edge kind `{word}`") });
    }
    if graph_property_names(graph).iter().any(|prop| prop == &word) {
        return Some(Hover { start, end, contents: format!("Property `{word}`") });
    }
    if collect_bound_vars(&lex_spanned(source, true).unwrap_or_default()).contains(&word) {
        return Some(Hover { start, end, contents: format!("Bound variable `{word}`") });
    }
    None
}

/// 🎨️ Semantic token classes for LSP highlighting.
pub fn semantic_tokens(source: &str) -> Vec<SemanticToken> {
    tokenize(source)
        .into_iter()
        .map(|span| SemanticToken {
            start: span.start,
            end: span.end,
            class: match span.class {
                TokenClass::Keyword => "keyword",
                TokenClass::Ident => "ident",
                TokenClass::Number => "number",
                TokenClass::String => "string",
                TokenClass::Operator => "operator",
                TokenClass::Punctuation => "punctuation",
                TokenClass::Error => "error",
            }
            .into(),
        })
        .collect()
}
// #endregion 🔖️LanguageService

// #region 🔖️DslIdiom
/// 🔌️ Registers Jack as a `dsl_core::IdiomHooks` entry (the `DslIdiom` seam's Route B — an EMBEDDED
/// idiom hosted inside another document's `Shape::Embed("jack")` field) so `canonicalize`
/// normalizes embedded Jack text through this crate's own `format`/`tokenize`. Hand-built rather
/// than `dsl_core::hooks_for::<I: DslIdiom>()`: that helper needs `DslIdiom::print(ast) -> String`, and
/// Jack has no AST-to-text printer (`format` re-derives canonical text token-by-token from SOURCE,
/// not from a `Query`) — `IdiomHooks` itself only needs function pointers, so it's built directly
/// from the language-service surface Jack already has, no printer required.
// 🚫️async: E4 fn-pointer slot — builds an `IdiomHooks` whose fields are plain `fn` pointers; an
// `async fn`'s value cannot coerce to `fn`, so this stays sync. See R2 E4, mirrors
// `dsl_core::hooks_for`/`dsl_core::passthrough_hooks`.
pub fn idiom_hooks() -> semio_framework_dsl::IdiomHooks {
    semio_framework_dsl::IdiomHooks { lang: "jack", canonicalize: idiom_canonicalize, classify: idiom_classify, complete: idiom_complete }
}

// 🚫️async: E4 fn-pointer slot (`IdiomHooks.canonicalize`) — see R2 E4.
fn idiom_canonicalize(text: &str) -> Result<String, semio_framework_diagnostic::TextError> {
    format(text).map_err(|error|match error{GraphDslError::Lex(error)=>error,GraphDslError::Json(error)=>semio_framework_diagnostic::TextError::new(error.kind(),error.to_string(),semio_framework_diagnostic::TextSpan::at(1,1)),GraphDslError::UnsupportedMutation=>semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"mutating jack clauses are not supported on this graph domain",semio_framework_diagnostic::TextSpan::at(1,1)),error @ (GraphDslError::UnterminatedString|GraphDslError::UnexpectedChar(_)|GraphDslError::NumberFormat(_)|GraphDslError::UnexpectedToken{..}|GraphDslError::EdgeTargetMissingPort|GraphDslError::EmptyPattern|GraphDslError::UnknownProcedure(_)|GraphDslError::ProcedureArity{..}|GraphDslError::ManifestIdentity{..})=>semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,error.to_string(),semio_framework_diagnostic::TextSpan::at(1,1))})
}

// 🚫️async: E4 fn-pointer slot (`IdiomHooks.classify`) — see R2 E4.
fn idiom_classify(text: &str) -> Vec<(semio_framework_dsl::TokenClass, semio_framework_diagnostic::TextSpan)> {
    tokenize(text)
        .into_iter()
        .map(|span| {
            let class = match span.class {
                TokenClass::Keyword => semio_framework_dsl::TokenClass::Keyword,
                TokenClass::Ident => semio_framework_dsl::TokenClass::Ident,
                TokenClass::Number => semio_framework_dsl::TokenClass::Number,
                TokenClass::String => semio_framework_dsl::TokenClass::String,
                TokenClass::Operator => semio_framework_dsl::TokenClass::Operator,
                TokenClass::Punctuation => semio_framework_dsl::TokenClass::Punctuation,
                TokenClass::Error => semio_framework_dsl::TokenClass::Error,
            };
            (class, byte_range_to_span(text, span.start, span.end))
        })
        .collect()
}

// 🚫️async: E4 fn-pointer slot (`IdiomHooks.complete`) — see R2 E4.
fn idiom_complete(text: &str, offset: usize) -> Vec<semio_framework_dsl::CompletionItem> {
    // Jack's own `complete` needs a `QueryableGraph`-bounded generic for schema-aware suggestions
    // (node/edge kinds, property names) that the generic `DslIdiom`/embed-host seam has no graph to
    // supply — an empty graph still exercises the syntax-only completions (clause/logic keywords).
    struct EmptyGraph;
    impl QueryableGraph for EmptyGraph {
        fn manifest(&self) -> Option<&crate::manifest::GraphManifest> {
            None
        }
        fn node_ids(&self) -> Vec<String> {
            Vec::new()
        }
        fn node_kind(&self, _id: &str) -> Option<String> {
            None
        }
        fn node_name(&self, _id: &str) -> Option<String> {
            None
        }
        fn node_property(&self, _id: &str, _key: &str) -> Option<PropertyValue> {
            None
        }
        fn edges(&self) -> Vec<QueryableEdge> {
            Vec::new()
        }
        fn subgraph_fixture_json(&self, _node_ids: &BTreeSet<String>, _edge_ids: &BTreeSet<String>) -> Option<String> {
            None
        }
    }
    complete(&EmptyGraph, text, offset).into_iter().map(|c| semio_framework_dsl::CompletionItem { label: c.label, detail: c.detail }).collect()
}

/// 📍️ Converts a byte-offset half-open range into `os_dsl::TextSpan`'s 1-based line/column/
/// length form — Jack's own spans are byte offsets (`TokenSpan`/`SemanticToken`), `dsl_core`'s are
/// line/column, so this is the one place that needs the source text to translate between them.
fn byte_range_to_span(text: &str, start: usize, end: usize) -> semio_framework_diagnostic::TextSpan {
    let mut line = 1u32;
    let mut column = 1u32;
    for (i, ch) in text.char_indices() {
        if i >= start {
            break;
        }
        if ch == '\n' {
            line += 1;
            column = 1;
        } else {
            column += 1;
        }
    }
    let length = text.get(start..end).map_or(0, |s| s.chars().count()) as u32;
    semio_framework_diagnostic::TextSpan::with_length(line, column, length)
}
// #endregion 🔖️DslIdiom

// #region 🔖️Parser
struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, pos: 0 }
    }

    fn peek(&self) -> &Token {
        self.tokens.get(self.pos).unwrap_or(&Token::Eof)
    }

    fn bump(&mut self) -> Token {
        let t = self.peek().clone();
        if !matches!(t, Token::Eof) {
            self.pos += 1;
        }
        t
    }

    fn expect_ident(&mut self) -> Result<String, GraphDslError> {
        match self.bump() {
            Token::Ident(s) => Ok(s),
            other => Err(GraphDslError::UnexpectedToken { expected: "ident".into(), found: format!("{other:?}") }),
        }
    }

    fn parse_query(&mut self) -> Result<Query, GraphDslError> {
        let mut clauses = Vec::new();
        while !matches!(self.peek(), Token::Eof) {
            clauses.push(self.parse_clause()?);
        }
        Ok(Query { clauses })
    }

    fn parse_clause(&mut self) -> Result<Clause, GraphDslError> {
        match self.peek() {
            Token::KwMatch => {
                self.bump();
                let mut patterns = vec![self.parse_pattern()?];
                while matches!(self.peek(), Token::Comma) {
                    self.bump();
                    patterns.push(self.parse_pattern()?);
                }
                Ok(Clause::Match(patterns))
            }
            Token::KwWhere => {
                self.bump();
                Ok(Clause::Where(self.parse_expr()?))
            }
            Token::KwReturn => {
                self.bump();
                let mut items = vec![self.parse_return_item()?];
                while matches!(self.peek(), Token::Comma) {
                    self.bump();
                    items.push(self.parse_return_item()?);
                }
                Ok(Clause::Return(items))
            }
            Token::KwCreate => {
                self.bump();
                Ok(Clause::Create(self.parse_pattern()?))
            }
            Token::KwDelete => {
                self.bump();
                let mut vars = vec![self.expect_ident()?];
                while matches!(self.peek(), Token::Comma) {
                    self.bump();
                    vars.push(self.expect_ident()?);
                }
                Ok(Clause::Delete(vars))
            }
            Token::KwSet => {
                self.bump();
                let mut items = vec![self.parse_assignment()?];
                while matches!(self.peek(), Token::Comma) {
                    self.bump();
                    items.push(self.parse_assignment()?);
                }
                Ok(Clause::Set(items))
            }
            Token::KwMerge => {
                self.bump();
                Ok(Clause::Merge(self.parse_pattern()?))
            }
            Token::KwWith => {
                self.bump();
                let mut items = vec![self.parse_return_item()?];
                while matches!(self.peek(), Token::Comma) {
                    self.bump();
                    items.push(self.parse_return_item()?);
                }
                Ok(Clause::With(items))
            }
            Token::KwUnwind => {
                self.bump();
                let source = self.parse_return_item()?;
                self.expect(&Token::KwAs)?;
                let var = self.expect_ident()?;
                Ok(Clause::Unwind(UnwindClause { source, var }))
            }
            Token::KwCall => {
                self.bump();
                let name = self.expect_ident()?;
                self.expect(&Token::LParen)?;
                let mut args = Vec::new();
                if !matches!(self.peek(), Token::RParen) {
                    args.push(self.parse_value()?);
                    while matches!(self.peek(), Token::Comma) {
                        self.bump();
                        args.push(self.parse_value()?);
                    }
                }
                self.expect(&Token::RParen)?;
                Ok(Clause::Call(CallClause { name, args }))
            }
            other => Err(GraphDslError::UnexpectedToken { expected: "clause start (MATCH/WHERE/RETURN/CREATE/DELETE/SET/MERGE/WITH/UNWIND/CALL)".into(), found: format!("{other:?}") }),
        }
    }

    /// 🕸️ Pattern grammar over the unified token alphabet — `dsl_core` has no standalone `-`
    /// token (only `->`/`--`/`<-`), so the leading connector before a bracketed edge label is
    /// always `--` or `<-`, never a bare dash (real Cypher's `-[r]->`/`<-[r]-` shape, adapted to
    /// this repo's alphabet). `<-` at the front means the edge points INTO `left`; represented by
    /// swapping which parsed node plays "left" so the stored `PatternEdge.right` is always the
    /// forward-direction target, mirroring `dsl_schema`'s own wire `<-` normalization.
    fn parse_pattern(&mut self) -> Result<Pattern, GraphDslError> {
        self.expect(&Token::LParen)?;
        let left = self.parse_pattern_node()?;
        self.expect(&Token::RParen)?;
        match self.peek().clone() {
            Token::Arrow => {
                self.bump();
                let right = self.parse_bracketed_pattern_node()?;
                Ok(Pattern { nodes: vec![left], edge: Some(PatternEdge { var: None, kind: None, directed: true, right }) })
            }
            // ➖️ `-` and `--` are the SAME connector. Cypher (and this dialect's own executor parser,
            // and every committed example query in the repo) writes `(a)-[r:Kind]->(b)`; `--` is this
            // grammar's own equal-status spelling. Accepting only `--` made every real query fail
            // `unexpected character '-'` in lint/complete/hover/format while the executor ran it fine.
            Token::Dash | Token::DashArrow => {
                self.bump();
                if matches!(self.peek(), Token::LBracket) {
                    let (edge_var, edge_kind) = self.parse_edge_label()?;
                    let directed = match self.peek() {
                        Token::Arrow => {
                            self.bump();
                            true
                        }
                        Token::Dash | Token::DashArrow => {
                            self.bump();
                            false
                        }
                        other => return Err(GraphDslError::UnexpectedToken { expected: "->, - or --".into(), found: format!("{other:?}") }),
                    };
                    let right = self.parse_bracketed_pattern_node()?;
                    Ok(Pattern { nodes: vec![left], edge: Some(PatternEdge { var: edge_var, kind: edge_kind, directed, right }) })
                } else {
                    let right = self.parse_bracketed_pattern_node()?;
                    Ok(Pattern { nodes: vec![left], edge: Some(PatternEdge { var: None, kind: None, directed: false, right }) })
                }
            }
            Token::BackArrow => {
                self.bump();
                if matches!(self.peek(), Token::LBracket) {
                    let (edge_var, edge_kind) = self.parse_edge_label()?;
                    match self.peek() {
                        Token::Dash | Token::DashArrow => {
                            self.bump();
                        }
                        other => return Err(GraphDslError::UnexpectedToken { expected: "- or --".into(), found: format!("{other:?}") }),
                    }
                    let right = self.parse_bracketed_pattern_node()?;
                    Ok(Pattern { nodes: vec![right], edge: Some(PatternEdge { var: edge_var, kind: edge_kind, directed: true, right: left }) })
                } else {
                    let right = self.parse_bracketed_pattern_node()?;
                    Ok(Pattern { nodes: vec![right], edge: Some(PatternEdge { var: None, kind: None, directed: true, right: left }) })
                }
            }
            _ => Ok(Pattern { nodes: vec![left], edge: None }),
        }
    }

    fn parse_bracketed_pattern_node(&mut self) -> Result<PatternNode, GraphDslError> {
        self.expect(&Token::LParen)?;
        let node = self.parse_pattern_node()?;
        self.expect(&Token::RParen)?;
        Ok(node)
    }

    fn parse_edge_label(&mut self) -> Result<(Option<String>, Option<String>), GraphDslError> {
        self.expect(&Token::LBracket)?;
        let edge_var = if matches!(self.peek(), Token::Ident(_)) { Some(self.expect_ident()?) } else { None };
        let edge_kind = if matches!(self.peek(), Token::Colon) {
            self.bump();
            Some(self.expect_ident()?)
        } else {
            None
        };
        self.expect(&Token::RBracket)?;
        Ok((edge_var, edge_kind))
    }

    fn parse_pattern_node(&mut self) -> Result<PatternNode, GraphDslError> {
        let var = self.expect_ident()?;
        self.expect(&Token::Colon)?;
        let kind = self.expect_ident()?;
        let port = if matches!(self.peek(), Token::At) {
            self.bump();
            Some(self.expect_ident()?)
        } else {
            None
        };
        Ok(PatternNode { var, kind, port })
    }

    fn parse_return_item(&mut self) -> Result<ReturnItem, GraphDslError> {
        let var = self.expect_ident()?;
        if matches!(self.peek(), Token::Dot) {
            self.bump();
            let prop = self.expect_ident()?;
            Ok(ReturnItem::Property { var, prop })
        } else {
            Ok(ReturnItem::Var(var))
        }
    }

    fn parse_assignment(&mut self) -> Result<Assignment, GraphDslError> {
        let var = self.expect_ident()?;
        self.expect(&Token::Dot)?;
        let prop = self.expect_ident()?;
        self.expect(&Token::Eq)?;
        let value = self.parse_value()?;
        Ok(Assignment { var, prop, value })
    }

    fn parse_expr(&mut self) -> Result<Expr, GraphDslError> {
        self.parse_or_expr()
    }

    fn parse_or_expr(&mut self) -> Result<Expr, GraphDslError> {
        let mut left = self.parse_and_expr()?;
        while matches!(self.peek(), Token::Or) {
            self.bump();
            let right = self.parse_and_expr()?;
            left = Expr::Or(Box::new(left), Box::new(right));
        }
        Ok(left)
    }

    fn parse_and_expr(&mut self) -> Result<Expr, GraphDslError> {
        let mut left = self.parse_cmp_expr()?;
        while matches!(self.peek(), Token::And) {
            self.bump();
            let right = self.parse_cmp_expr()?;
            left = Expr::And(Box::new(left), Box::new(right));
        }
        Ok(left)
    }

    fn parse_cmp_expr(&mut self) -> Result<Expr, GraphDslError> {
        let var = self.expect_ident()?;
        self.expect(&Token::Dot)?;
        let prop = self.expect_ident()?;
        match self.bump() {
            Token::Eq => Ok(Expr::Eq { var, prop, value: self.parse_value()? }),
            Token::Ne => Ok(Expr::Ne { var, prop, value: self.parse_value()? }),
            other => Err(GraphDslError::UnexpectedToken { expected: "= or !=".into(), found: format!("{other:?}") }),
        }
    }

    fn parse_value(&mut self) -> Result<PropertyValue, GraphDslError> {
        match self.bump() {
            Token::Number(n) => Ok(PropertyValue::Number(n)),
            Token::StringLit(s) => Ok(PropertyValue::String(s)),
            Token::Ident(s) if s.eq_ignore_ascii_case("true") => Ok(PropertyValue::Bool(true)),
            Token::Ident(s) if s.eq_ignore_ascii_case("false") => Ok(PropertyValue::Bool(false)),
            Token::Ident(s) if s.eq_ignore_ascii_case("null") => Ok(PropertyValue::Null),
            other => Err(GraphDslError::UnexpectedToken { expected: "value".into(), found: format!("{other:?}") }),
        }
    }

    fn expect(&mut self, want: &Token) -> Result<(), GraphDslError> {
        if std::mem::discriminant(self.peek()) == std::mem::discriminant(want) {
            self.bump();
            Ok(())
        } else {
            Err(GraphDslError::UnexpectedToken { expected: format!("{want:?}"), found: format!("{:?}", self.peek()) })
        }
    }
}

/// 🔍️ Parse a jack query string.
pub fn parse(query: &str) -> Result<Query, GraphDslError> {
    let tokens = lex(query)?;
    Parser::new(tokens).parse_query()
}
// #endregion 🔖️Parser

// #region 🔖️Executor
/// 🎯️ Variable binding in a match row.
#[derive(Clone, Debug, Default)]
pub struct Binding {
    pub nodes: BTreeMap<String, String>,
    pub edges: BTreeMap<String, String>,
    /// 🍇️ Scalar/list/object values bound by `WITH`'s property projection, `UNWIND`, and `CALL` —
    /// entries here have no backing graph entity, unlike `nodes`/`edges`.
    pub values: BTreeMap<String, PropertyValue>,
}

/// ▶️ Execute a read-only jack query against a queryable graph.
pub fn execute<G: QueryableGraph>(graph: &G, query: &Query) -> Result<QueryResult, GraphDslError> {
    let mut bindings: Vec<Binding> = vec![Binding::default()];
    let mut return_items: Option<Vec<ReturnItem>> = None;
    for clause in &query.clauses {
        match clause {
            Clause::Match(patterns) => bindings = match_patterns(graph, patterns)?,
            Clause::Where(expr) => {
                // 🔀️ Rewritten from `.retain(..)` — `eval_expr` is async (it may call the
                // caller-provided `QueryableGraph`, which is not assumed I/O-free) and cannot run
                // inside a sync `retain` predicate (R10 residue shape #1).
                let mut kept = Vec::with_capacity(bindings.len());
                for b in bindings {
                    if eval_expr(graph, &b, expr) {
                        kept.push(b);
                    }
                }
                bindings = kept;
            }
            Clause::Return(items) => return_items = Some(items.clone()),
            Clause::Create(_) | Clause::Delete(_) | Clause::Set(_) | Clause::Merge(_) => {
                return Err(GraphDslError::UnsupportedMutation);
            }
            Clause::With(items) => {
                bindings = bindings.iter().map(|binding| project_binding(binding, items)).collect();
            }
            Clause::Unwind(unwind) => {
                let mut next = Vec::new();
                for binding in &bindings {
                    for element in unwind_elements(resolve_item_value(graph, binding, &unwind.source)) {
                        let mut b = binding.clone();
                        b.values.insert(unwind.var.clone(), element);
                        next.push(b);
                    }
                }
                bindings = next;
            }
            Clause::Call(call) => {
                let (column, results) = call_procedure(graph, call)?;
                let mut next = Vec::with_capacity(bindings.len() * results.len());
                for binding in &bindings {
                    for value in &results {
                        let mut b = binding.clone();
                        b.values.insert(column.clone(), value.clone());
                        next.push(b);
                    }
                }
                bindings = next;
            }
        }
    }
    if let Some(items) = return_items {
        return Ok(build_return(graph, &bindings, &items));
    }
    Ok(QueryResult::table(vec![], vec![]))
}

/// ▶️ Parse and execute jack in one step.
pub fn run_query<G: QueryableGraph>(graph: &G, source: &str) -> Result<QueryResult, GraphDslError> {
    execute(graph, &parse(source)?)
}

/// ▶️ Execute jack and return JSON result.
pub fn run_query_json<G: QueryableGraph>(graph: &G, source: &str) -> Result<String, GraphDslError> {
    Ok(semio_framework_pack_json::to_json_string(&run_query(graph, source)?))
}

fn match_patterns<G: QueryableGraph>(graph: &G, patterns: &[Pattern]) -> Result<Vec<Binding>, GraphDslError> {
    let mut bindings = vec![Binding::default()];
    for pattern in patterns {
        let mut next = Vec::new();
        for binding in &bindings {
            next.extend(match_pattern(graph, pattern, binding)?);
        }
        bindings = next;
    }
    Ok(bindings)
}

fn match_pattern<G: QueryableGraph>(graph: &G, pattern: &Pattern, base: &Binding) -> Result<Vec<Binding>, GraphDslError> {
    let left = pattern.nodes.first().ok_or(GraphDslError::EmptyPattern)?;
    if let Some(edge_pat) = &pattern.edge {
        let mut out = Vec::new();
        for node_id in graph.node_ids() {
            if graph.node_kind(node_id.as_str()).as_deref() != Some(left.kind.as_str()) {
                continue;
            }
            if binding_conflicts(base, &left.var, node_id.as_str()) {
                continue;
            }
            for edge in graph.edges() {
                if edge_pat.kind.as_ref().is_some_and(|k| *k != edge.kind) {
                    continue;
                }
                let pairs = if edge_pat.directed {
                    vec![(edge.source_node_id.as_str(), edge.target_node_id.as_str(), edge.source_port.as_deref(), edge.target_port.as_deref())]
                } else {
                    vec![
                        (edge.source_node_id.as_str(), edge.target_node_id.as_str(), edge.source_port.as_deref(), edge.target_port.as_deref()),
                        (edge.target_node_id.as_str(), edge.source_node_id.as_str(), edge.target_port.as_deref(), edge.source_port.as_deref()),
                    ]
                };
                for (src_id, tgt_id, src_port, tgt_port) in pairs {
                    if src_id != node_id {
                        continue;
                    }
                    if left.port.as_ref().is_some_and(|want| src_port != Some(want.as_str())) {
                        continue;
                    }
                    if graph.node_kind(tgt_id).as_deref() != Some(edge_pat.right.kind.as_str()) {
                        continue;
                    }
                    if edge_pat.right.port.as_ref().is_some_and(|want| tgt_port != Some(want.as_str())) {
                        continue;
                    }
                    let mut b = base.clone();
                    b.nodes.insert(left.var.clone(), node_id.clone());
                    if let Some(ev) = &edge_pat.var {
                        b.edges.insert(ev.clone(), edge.id.clone());
                    }
                    if binding_conflicts(base, &edge_pat.right.var, tgt_id) {
                        continue;
                    }
                    b.nodes.insert(edge_pat.right.var.clone(), tgt_id.to_string());
                    out.push(b);
                }
            }
        }
        return Ok(out);
    }
    let mut out = Vec::new();
    for node_id in graph.node_ids() {
        if graph.node_kind(node_id.as_str()).as_deref() != Some(left.kind.as_str()) {
            continue;
        }
        if binding_conflicts(base, &left.var, node_id.as_str()) {
            continue;
        }
        let mut b = base.clone();
        b.nodes.insert(left.var.clone(), node_id);
        out.push(b);
    }
    Ok(out)
}

fn binding_conflicts(base: &Binding, var: &str, node_id: &str) -> bool {
    base.nodes.get(var).is_some_and(|existing| existing != node_id)
}

fn eval_expr<G: QueryableGraph>(graph: &G, binding: &Binding, expr: &Expr) -> bool {
    match expr {
        Expr::Eq { var, prop, value } => binding_value(graph, binding, var, prop) == Some(value.clone()),
        Expr::Ne { var, prop, value } => binding_value(graph, binding, var, prop) != Some(value.clone()),
        Expr::And(a, b) => eval_expr(graph, binding, a) && eval_expr(graph, binding, b),
        Expr::Or(a, b) => eval_expr(graph, binding, a) || eval_expr(graph, binding, b),
    }
}

fn binding_value<G: QueryableGraph>(graph: &G, binding: &Binding, var: &str, prop: &str) -> Option<PropertyValue> {
    if let Some(node_id) = binding.nodes.get(var) {
        return graph.node_property(node_id, prop);
    }
    match binding.values.get(var) {
        Some(PropertyValue::Object(map)) => map.get(prop).cloned(),
        _ => None,
    }
}

/// 🎯️ Resolves a [`ReturnItem`] against one binding — the single evaluation path shared by
/// `RETURN`, `WITH`'s trailing `WHERE`, and `UNWIND`'s source expression. A `Var` prefers a
/// `values`-scoped scalar (bound by `WITH`/`UNWIND`/`CALL`) over a graph entity's display name.
fn resolve_item_value<G: QueryableGraph>(graph: &G, binding: &Binding, item: &ReturnItem) -> Option<PropertyValue> {
    match item {
        ReturnItem::Var(v) => {
            if let Some(value) = binding.values.get(v) {
                return Some(value.clone());
            }
            binding.nodes.get(v).map(|id| graph.node_name(id).map_or(PropertyValue::Null, PropertyValue::String))
        }
        ReturnItem::Property { var, prop } => binding_value(graph, binding, var, prop),
    }
}

/// 🧺️ `UNWIND`'s list-expansion rule: `null`/missing and an empty list both yield zero rows, a
/// non-list scalar unwinds as its own single-element list (mirrors the Cypher-family convention).
fn unwind_elements(value: Option<PropertyValue>) -> Vec<PropertyValue> {
    match value {
        Some(PropertyValue::Array(items)) => items,
        Some(PropertyValue::Null) | None => vec![],
        Some(other) => vec![other],
    }
}

/// 🔭️ Projects one binding down to just the vars named by a `WITH` clause. A `Property` item
/// names its source var (`ReturnItem` carries no alias), so the whole entity/value bound to that
/// var is kept — this is what lets a later `WHERE`/`RETURN` still resolve `var.prop` from it.
fn project_binding(binding: &Binding, items: &[ReturnItem]) -> Binding {
    let mut out = Binding::default();
    for item in items {
        let var = match item {
            ReturnItem::Var(v) => v,
            ReturnItem::Property { var, .. } => var,
        };
        if let Some(id) = binding.nodes.get(var) {
            out.nodes.insert(var.clone(), id.clone());
        }
        if let Some(id) = binding.edges.get(var) {
            out.edges.insert(var.clone(), id.clone());
        }
        if let Some(value) = binding.values.get(var) {
            out.values.insert(var.clone(), value.clone());
        }
    }
    out
}

/// 📇️ `CALL`'s small owned procedure registry — graph-manifest introspection this codebase
/// already exposes to completion/hover (`manifest_node_kinds` et al.), surfaced as callable
/// procedures since `CallClause` has no `YIELD`/`AS` target: each procedure's single output
/// column is bound under a fixed, procedure-defined name. Arbitrary procedure dispatch (the
/// general case) is deferred to unifying semio_compose_rs's Architect query language onto Jack —
/// an unknown name reports [`GraphDslError::UnknownProcedure`], never a blanket rejection.
fn call_procedure<G: QueryableGraph>(graph: &G, call: &CallClause) -> Result<(String, Vec<PropertyValue>), GraphDslError> {
    if !call.args.is_empty() {
        return Err(GraphDslError::ProcedureArity { name: call.name.clone(), expected: 0, found: call.args.len() });
    }
    match call.name.as_str() {
        "nodeKinds" => Ok(("kind".to_string(), manifest_node_kinds(graph).into_iter().map(PropertyValue::String).collect())),
        "edgeKinds" => Ok(("kind".to_string(), manifest_edge_kinds(graph).into_iter().map(PropertyValue::String).collect())),
        "propertyNames" => Ok(("name".to_string(), manifest_property_names(graph).into_iter().map(PropertyValue::String).collect())),
        "portKinds" => Ok(("kind".to_string(), manifest_port_kinds(graph).into_iter().map(PropertyValue::String).collect())),
        other => Err(GraphDslError::UnknownProcedure(other.to_string())),
    }
}

fn binding_has_entity(binding: &Binding, var: &str) -> bool {
    binding.nodes.contains_key(var) || binding.edges.contains_key(var)
}

fn return_items_want_graph(items: &[ReturnItem], bindings: &[Binding]) -> bool {
    // 🔀️ Rewritten from `.any(..)` — `binding_has_entity` is async and cannot be called inside a
    // sync `Iterator::any` predicate (R10 residue shape #1).
    for item in items {
        let ReturnItem::Var(v) = item else { continue };
        for b in bindings {
            if binding_has_entity(b, v) {
                return true;
            }
        }
    }
    false
}

fn collect_graph_entities(bindings: &[Binding], items: &[ReturnItem]) -> (BTreeSet<String>, BTreeSet<String>) {
    let mut node_ids = BTreeSet::new();
    let mut edge_ids = BTreeSet::new();
    for binding in bindings {
        for item in items {
            if let ReturnItem::Var(v) = item {
                if let Some(id) = binding.nodes.get(v) {
                    node_ids.insert(id.clone());
                }
                if let Some(id) = binding.edges.get(v) {
                    edge_ids.insert(id.clone());
                }
            }
        }
    }
    (node_ids, edge_ids)
}

fn build_return<G: QueryableGraph>(graph: &G, bindings: &[Binding], items: &[ReturnItem]) -> QueryResult {
    let columns: Vec<String> = items
        .iter()
        .map(|item| match item {
            ReturnItem::Var(v) => v.clone(),
            ReturnItem::Property { var, prop } => format!("{var}.{prop}"),
        })
        .collect();
    if return_items_want_graph(items, bindings) {
        let (node_ids, edge_ids) = collect_graph_entities(bindings, items);
        if let Some(json) = graph.subgraph_fixture_json(&node_ids, &edge_ids) {
            return QueryResult::graph(columns, json);
        }
    }
    let mut rows = Vec::new();
    for binding in bindings {
        let mut row = Vec::new();
        for item in items {
            row.push(resolve_item_value(graph, binding, item).unwrap_or(PropertyValue::Null));
        }
        rows.push(row);
    }
    QueryResult::table(columns, rows)
}
// #endregion 🔖️Executor

// #region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
// #endregion 🔖️Tests
// #endregion jack_impl

```

### 🧰️framework/🔨️modules/🕸️graph/🗣️dsl/🧪️tests/🔬️unit/🦀️.rs

SHA-256 `af250f02fb52840afa8006c401ff992c559c1d5ef1c00ca793b7ffc2fe499ccb`; 45034 bytes.

```

use super::*;

// 🚫️async: E5-class executor bridge, sanctioned per R4 clause 5 — `#[test]` cannot run
// an `async fn` directly (std has no executor for it), so every async test body in this
// module runs through this instead. Sound because this crate performs no real I/O: every
// future here resolves on its first poll, so a single poll (never a spin-park loop) is
// enough — panics loudly if that invariant is ever violated rather than hanging.
fn block_on_test<F: std::future::Future>(fut: F) -> F::Output {
    use std::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};
    fn noop(_: *const ()) {}
    fn clone_raw(_: *const ()) -> RawWaker {
        RawWaker::new(std::ptr::null(), &VTABLE)
    }
    static VTABLE: RawWakerVTable = RawWakerVTable::new(clone_raw, noop, noop, noop);
    let raw = RawWaker::new(std::ptr::null(), &VTABLE);
    let waker = unsafe { Waker::from_raw(raw) };
    let mut cx = Context::from_waker(&waker);
    let mut fut = Box::pin(fut);
    match fut.as_mut().poll(&mut cx) {
        Poll::Ready(v) => v,
        Poll::Pending => panic!("block_on_test: future did not complete synchronously"),
    }
}

#[test]
fn parse_match_return() {
    block_on_test(async {
        let q = parse("MATCH (a:computation) RETURN a.name").unwrap();
        assert_eq!(q.clauses.len(), 2);
    });
}

#[test]
fn idiom_hooks_canonicalize_and_classify_through_the_dsl_registry_seam() {
    block_on_test(async {
        let hooks = idiom_hooks();
        assert_eq!(hooks.lang, "jack");
        let canonical = (hooks.canonicalize)("MATCH   (a:computation)   RETURN   a.name").expect("canonicalize");
        assert_eq!(canonical, format("MATCH (a:computation) RETURN a.name").unwrap());
        assert!((hooks.canonicalize)("not jack at all $$$").is_err() || (hooks.canonicalize)("not jack at all $$$").is_ok(), "canonicalize must not panic on malformed input");
        let classes = (hooks.classify)("MATCH (a:computation) RETURN a.name");
        assert!(!classes.is_empty());
        assert!(classes.iter().any(|(class, _)| *class == semio_framework_dsl::TokenClass::Keyword), "MATCH/RETURN must classify as keywords");
        semio_framework_dsl::register_idiom(hooks);
        let resolved = semio_framework_dsl::idiom("jack").expect("jack must be resolvable by lang id after registration");
        assert_eq!((resolved.canonicalize)("MATCH (a:computation) RETURN a.name").unwrap(), format("MATCH (a:computation) RETURN a.name").unwrap());
    });
}

#[test]
fn run_dag_fixture_query() {
    block_on_test(async {
        // 🩹️ Was `include_str!` of the dag technology's example fixture; that technology migrated its
        // fixture to a handcrafted DSL (`store::ArtifactDsl`) — inlined the same dag-fixture JSON this
        // test actually parses (`from_dag_host_snapshot_json`), decoupled from its document format.
        let fixture = r#"{
  "schema": "dag.host_snapshot",
  "camera": { "x": 0, "y": 0, "zoom": 1 },
  "nodes": [
    {
      "id": "slider",
      "name": "Amount",
      "abbreviation": "Amount",
      "icon": "emoji:🎚️",
      "kind": "slider",
      "x": -400,
      "y": -40,
      "width": 70,
      "height": 14,
      "min": 0,
      "max": 10,
      "step": 0.5,
      "value": 5,
      "output": { "id": "out", "label": "value", "cardinality": "!" }
    },
    {
      "id": "mode",
      "name": "Mode",
      "abbreviation": "Mode",
      "icon": "emoji:📋️",
      "kind": "select",
      "x": -400,
      "y": 80,
      "width": 56,
      "height": 28,
      "options": ["Add", "Multiply", "Max"],
      "selected": 0,
      "output": { "id": "out", "label": "mode", "cardinality": "!" }
    },
    {
      "id": "scale",
      "name": "Scale",
      "abbreviation": "Scale",
      "icon": "emoji:📐️",
      "kind": "computation",
      "x": -120,
      "y": -40,
      "width": 104,
      "height": 14,
      "inputs": [{ "id": "in", "label": "value", "cardinality": "!" }],
      "outputs": [{ "id": "out", "label": "scaled", "cardinality": "!" }]
    },
    {
      "id": "combine",
      "name": "Combine",
      "abbreviation": "Combine",
      "icon": "emoji:🔀️",
      "kind": "computation",
      "x": 120,
      "y": 0,
      "width": 104,
      "height": 28,
      "inputs": [
        { "id": "a", "label": "a", "cardinality": "!" },
        { "id": "b", "label": "b", "cardinality": "!" }
      ],
      "outputs": [{ "id": "out", "label": "merged", "cardinality": "!" }]
    },
    {
      "id": "screen",
      "name": "Preview",
      "abbreviation": "Preview",
      "icon": "emoji:🖥️",
      "kind": "screen",
      "x": 400,
      "y": 0,
      "width": 200,
      "height": 140,
      "media": {
        "kind": "svg",
        "src": "data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 100 60'%3E%3Crect fill='%233c78d8' width='100' height='60'/%3E%3Ctext x='50' y='35' text-anchor='middle' fill='white' font-size='12'%3EDAG%3C/text%3E%3C/svg%3E"
      },
      "input": { "id": "in", "label": "result", "cardinality": "!" }
    }
  ],
  "edges": [
    { "id": "e1", "source": "slider:out", "target": "scale:in" },
    { "id": "e2", "source": "scale:out", "target": "combine:a" },
    { "id": "e3", "source": "mode:out", "target": "combine:b" },
    { "id": "e4", "source": "combine:out", "target": "screen:in" }
  ]
}
"#;
        let graph = BoardQueryableGraph::from_host_snapshot_json(fixture, None).unwrap();
        let result = run_query(&graph, "MATCH (n:computation) RETURN n.name").unwrap();
        assert!(!result.rows.is_empty());
    });
}

#[test]
fn parse_match_with_port() {
    block_on_test(async {
        let q = parse("MATCH (a:computation@out) RETURN a.name").unwrap();
        let Clause::Match(patterns) = &q.clauses[0] else { panic!("expected match") };
        assert_eq!(patterns[0].nodes[0].port.as_deref(), Some("out"));
    });
}

#[test]
fn parse_undirected_edge() {
    block_on_test(async {
        // 🩹️ unified undirected sigil is `--`, not the old bare `-` (not even lexable in the
        // shared `dsl_core` alphabet, which has no standalone dash token).
        let q = parse("MATCH (a:computation)--(b:slider) RETURN a.name").unwrap();
        let Clause::Match(patterns) = &q.clauses[0] else { panic!("expected match") };
        let edge = patterns[0].edge.as_ref().expect("edge");
        assert!(!edge.directed);
    });
}

#[test]
fn parse_back_arrow_edge_swaps_left_and_right() {
    block_on_test(async {
        let q = parse("MATCH (a:computation)<-(b:slider) RETURN a.name").unwrap();
        let Clause::Match(patterns) = &q.clauses[0] else { panic!("expected match") };
        // `<-` means the edge points INTO the parenthesized-first node — represented by swapping
        // which parsed node plays "left" so `edge.right` is always the forward-direction target.
        assert_eq!(patterns[0].nodes[0].kind, "slider");
        let edge = patterns[0].edge.as_ref().expect("edge");
        assert!(edge.directed);
        assert_eq!(edge.right.kind, "computation");
    });
}

#[test]
fn parse_labeled_directed_and_undirected_edges_use_double_dash_connector() {
    block_on_test(async {
        let forward = parse("MATCH (a:computation)--[r:wire]->(b:slider) RETURN a.name").unwrap();
        let Clause::Match(patterns) = &forward.clauses[0] else { panic!("expected match") };
        let edge = patterns[0].edge.as_ref().expect("edge");
        assert!(edge.directed);
        assert_eq!(edge.var.as_deref(), Some("r"));
        assert_eq!(edge.kind.as_deref(), Some("wire"));

        let undirected = parse("MATCH (a:computation)--[:wire]--(b:slider) RETURN a.name").unwrap();
        let Clause::Match(patterns) = &undirected.clauses[0] else { panic!("expected match") };
        let edge = patterns[0].edge.as_ref().expect("edge");
        assert!(!edge.directed);
    });
}

#[test]
fn run_port_filtered_query() {
    block_on_test(async {
        // 🩹️ Was `include_str!` of the dag technology's example fixture; that technology migrated its
        // fixture to a handcrafted DSL (`store::ArtifactDsl`) — inlined the same dag-fixture JSON this
        // test actually parses (`from_dag_host_snapshot_json`), decoupled from its document format.
        let fixture = r#"{
  "schema": "dag.host_snapshot",
  "camera": { "x": 0, "y": 0, "zoom": 1 },
  "nodes": [
    {
      "id": "slider",
      "name": "Amount",
      "abbreviation": "Amount",
      "icon": "emoji:🎚️",
      "kind": "slider",
      "x": -400,
      "y": -40,
      "width": 70,
      "height": 14,
      "min": 0,
      "max": 10,
      "step": 0.5,
      "value": 5,
      "output": { "id": "out", "label": "value", "cardinality": "!" }
    },
    {
      "id": "mode",
      "name": "Mode",
      "abbreviation": "Mode",
      "icon": "emoji:📋️",
      "kind": "select",
      "x": -400,
      "y": 80,
      "width": 56,
      "height": 28,
      "options": ["Add", "Multiply", "Max"],
      "selected": 0,
      "output": { "id": "out", "label": "mode", "cardinality": "!" }
    },
    {
      "id": "scale",
      "name": "Scale",
      "abbreviation": "Scale",
      "icon": "emoji:📐️",
      "kind": "computation",
      "x": -120,
      "y": -40,
      "width": 104,
      "height": 14,
      "inputs": [{ "id": "in", "label": "value", "cardinality": "!" }],
      "outputs": [{ "id": "out", "label": "scaled", "cardinality": "!" }]
    },
    {
      "id": "combine",
      "name": "Combine",
      "abbreviation": "Combine",
      "icon": "emoji:🔀️",
      "kind": "computation",
      "x": 120,
      "y": 0,
      "width": 104,
      "height": 28,
      "inputs": [
        { "id": "a", "label": "a", "cardinality": "!" },
        { "id": "b", "label": "b", "cardinality": "!" }
      ],
      "outputs": [{ "id": "out", "label": "merged", "cardinality": "!" }]
    },
    {
      "id": "screen",
      "name": "Preview",
      "abbreviation": "Preview",
      "icon": "emoji:🖥️",
      "kind": "screen",
      "x": 400,
      "y": 0,
      "width": 200,
      "height": 140,
      "media": {
        "kind": "svg",
        "src": "data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 100 60'%3E%3Crect fill='%233c78d8' width='100' height='60'/%3E%3Ctext x='50' y='35' text-anchor='middle' fill='white' font-size='12'%3EDAG%3C/text%3E%3C/svg%3E"
      },
      "input": { "id": "in", "label": "result", "cardinality": "!" }
    }
  ],
  "edges": [
    { "id": "e1", "source": "slider:out", "target": "scale:in" },
    { "id": "e2", "source": "scale:out", "target": "combine:a" },
    { "id": "e3", "source": "mode:out", "target": "combine:b" },
    { "id": "e4", "source": "combine:out", "target": "screen:in" }
  ]
}
"#;
        let graph = BoardQueryableGraph::from_host_snapshot_json(fixture, None).unwrap();
        let result = run_query(&graph, "MATCH (n:computation@out)--[:wire]->(m:slider) RETURN n.name, m.name");
        assert!(result.is_ok());
    });
}

// #region 🔖️Fixtures
/// 🧵️ Small hand-built graph exercising every `split_endpoint` branch: exact handle match,
/// mapped/unmapped `@` and `:` splits, and the plain-id fallback.
fn split_endpoint_fixture() -> &'static str {
    r#"{
  "manifestId": "future.neutral.geometry",
  "nodes": [
    { "id": "a", "nodeKind": "computation", "text": "A", "userData": { "score": 1 }, "handles": [{ "id": "a-out" }] },
    { "id": "b", "nodeKind": "slider", "text": "B" },
    { "id": "c", "nodeKind": "slider", "text": "C" }
  ],
  "edges": [
    { "id": "e1", "edgeKind": "wire", "source": "a-out", "target": "b@in" },
    { "id": "e2", "edgeKind": "wire", "source": "a:out2", "target": "c.in2" },
    { "id": "e3", "edgeKind": "wire", "source": "a-out@x", "target": "a-out:y" },
    { "id": "e4", "edgeKind": "wire", "source": "z", "target": "c" }
  ]
}"#
}

fn find_edge<'a>(edges: &'a [QueryableEdge], id: &str) -> &'a QueryableEdge {
    edges.iter().find(|e| e.id == id).unwrap_or_else(|| panic!("missing edge {id}"))
}

/// 🍇️ Carries list-valued properties (`tags`, `matrix`, `empty`) for `WITH`/`UNWIND` coverage.
/// Deliberately has no `manifestId` so `CALL nodeKinds()` only sees the two node kinds present
/// on the nodes themselves, not any manifest-declared extras.
fn list_property_fixture() -> &'static str {
    r#"{
  "nodes": [
    { "id": "a", "nodeKind": "computation", "text": "A", "userData": { "tags": ["x", "y", "z"], "matrix": [[1, 2], [3, 4]], "empty": [] } },
    { "id": "b", "nodeKind": "slider", "text": "B" }
  ],
  "edges": []
}"#
}
// #endregion 🔖️Fixtures

// #region 🔖️QueryableGraphTests
#[test]
fn split_endpoint_resolves_exact_handle_and_unmapped_at() {
    block_on_test(async {
        let graph = BoardQueryableGraph::from_host_snapshot_json(split_endpoint_fixture(), None).unwrap();
        let edges = graph.edges();
        let e1 = find_edge(&edges, "e1");
        assert_eq!(e1.source_node_id, "a");
        assert_eq!(e1.source_port, None);
        assert_eq!(e1.target_node_id, "b");
        assert_eq!(e1.target_port.as_deref(), Some("in"));
    });
}

#[test]
fn split_endpoint_resolves_unmapped_colon_and_dot() {
    block_on_test(async {
        let graph = BoardQueryableGraph::from_host_snapshot_json(split_endpoint_fixture(), None).unwrap();
        let edges = graph.edges();
        let e2 = find_edge(&edges, "e2");
        assert_eq!(e2.source_node_id, "a");
        assert_eq!(e2.source_port.as_deref(), Some("out2"));
        assert_eq!(e2.target_node_id, "c");
        assert_eq!(e2.target_port.as_deref(), Some("in2"));
    });
}

#[test]
fn split_endpoint_resolves_handle_mapped_at_and_colon() {
    block_on_test(async {
        let graph = BoardQueryableGraph::from_host_snapshot_json(split_endpoint_fixture(), None).unwrap();
        let edges = graph.edges();
        let e3 = find_edge(&edges, "e3");
        assert_eq!(e3.source_node_id, "a");
        assert_eq!(e3.source_port.as_deref(), Some("x"));
        assert_eq!(e3.target_node_id, "a");
        assert_eq!(e3.target_port.as_deref(), Some("y"));
    });
}

#[test]
fn split_endpoint_falls_back_to_plain_id() {
    block_on_test(async {
        let graph = BoardQueryableGraph::from_host_snapshot_json(split_endpoint_fixture(), None).unwrap();
        let edges = graph.edges();
        let e4 = find_edge(&edges, "e4");
        assert_eq!(e4.source_node_id, "z");
        assert_eq!(e4.source_port, None);
        assert_eq!(e4.target_node_id, "c");
        assert_eq!(e4.target_port, None);
    });
}

#[test]
fn board_graph_node_property_id_kind_all_and_missing() {
    block_on_test(async {
        let graph = BoardQueryableGraph::from_host_snapshot_json(split_endpoint_fixture(), None).unwrap();
        assert_eq!(graph.node_property("a", "id"), Some(PropertyValue::String("a".into())));
        assert_eq!(graph.node_property("a", "kind"), Some(PropertyValue::String("computation".into())));
        let all = graph.node_property("a", "__all").unwrap();
        assert!(matches!(all, PropertyValue::Object(ref map) if map.get("score") == Some(&PropertyValue::Number(1.0))));
        assert_eq!(graph.node_property("a", "score"), Some(PropertyValue::Number(1.0)));
        assert_eq!(graph.node_property("a", "nonexistent"), None);
        assert_eq!(graph.node_property("missing-node", "id"), None);
    });
}

#[test]
fn manifest_helpers_merge_graph_and_manifest_kinds() {
    block_on_test(async {
        let manifest = <crate::manifest::Manifest as dsl_core::FromValue>::from_value(semio_framework_pack_json::to_dsl_value(&semio_framework_pack_json::parse(r#"{"schema":"manifest","id":"future.neutral.geometry","nodeKinds":[{"id":"Declared"}]}"#,semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap())).unwrap();
        let graph = BoardQueryableGraph::from_host_snapshot_json(split_endpoint_fixture(), Some(manifest)).unwrap();
        assert_eq!(graph.manifest().map(|m| m.id.as_str()), Some("future.neutral.geometry"));
        let node_kinds = manifest_node_kinds(&graph);
        assert!(node_kinds.iter().any(|k| k == "computation"));
        assert!(node_kinds.iter().any(|k| k == "Declared"), "manifest-only kind should be included");
        let edge_kinds = manifest_edge_kinds(&graph);
        assert!(edge_kinds.iter().any(|k| k == "wire"));
        let port_kinds = manifest_port_kinds(&graph);
        assert!(port_kinds.iter().any(|k| k == "in"));
        let props = manifest_property_names(&graph);
        for expected in ["id", "name", "kind", "label", "text", "score"] {
            assert!(props.iter().any(|p| p == expected), "missing property {expected}");
        }
    });
}

#[test]
fn subgraph_fixture_json_filters_to_requested_ids() {
    block_on_test(async {
        let graph = BoardQueryableGraph::from_host_snapshot_json(split_endpoint_fixture(), None).unwrap();
        let node_ids = BTreeSet::from(["a".to_string(), "b".to_string()]);
        let edge_ids = BTreeSet::from(["e1".to_string()]);
        let json = graph.subgraph_fixture_json(&node_ids, &edge_ids).unwrap();
        let value: semio_framework_pack_json::Value = semio_framework_pack_json::parse(&json,semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
        assert_eq!(value["nodes"].as_array().unwrap().len(), 2);
        assert_eq!(value["edges"].as_array().unwrap().len(), 1);
    });
}

#[test]
fn from_host_snapshot_json_rejects_invalid_json() {
    block_on_test(async {
        let Err(err) = BoardQueryableGraph::from_host_snapshot_json("not json", None) else { panic!("expected error") };
        assert!(matches!(err, GraphDslError::Json(_)));
    });
}

#[test]
fn from_object_snapshot_json_converts_objects_array() {
    block_on_test(async {
        let fixture = r#"{"objects": [{"id": "o1", "objectKind": "Cube", "name": "Box"}]}"#;
        let graph = BoardQueryableGraph::from_object_snapshot_json(fixture, None).unwrap();
        assert_eq!(graph.node_kind("o1").as_deref(), Some("Cube"));
        assert_eq!(graph.node_name("o1").as_deref(), Some("Box"));
        assert_eq!(graph.manifest().map(|m| m.id.as_str()), None);
    });
}

#[test]
fn from_object_snapshot_json_passes_through_existing_nodes() {
    block_on_test(async {
        let fixture = r#"{"nodes": [{"id": "n1", "nodeKind": "Widget", "text": "N1"}]}"#;
        let graph = BoardQueryableGraph::from_object_snapshot_json(fixture, None).unwrap();
        assert_eq!(graph.node_kind("n1").as_deref(), Some("Widget"));
    });
}

#[test]
fn explicit_manifest_consumption_follows_portable_owner_corpus() {
    let corpus = semio_framework_pack_json::parse(include_str!("../../../🛂️manifest/🧫️fixtures/🧩️consumption/🔣️.json"),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    for row in corpus["cases"].as_array().unwrap() {
        let manifest = if row["manifest"].is_null() { None } else { Some(<crate::manifest::Manifest as dsl_core::FromValue>::from_value(semio_framework_pack_json::to_dsl_value(&row["manifest"])).unwrap()) };
        let actual = BoardQueryableGraph::from_host_snapshot_json(&semio_framework_pack_json::to_string(&row["snapshot"]), manifest);
        assert_eq!(actual.is_ok(), row["expected"]["accepted"].as_bool().unwrap(), "{}", row["id"]);
        if let Ok(graph) = actual {
            assert_eq!(graph.manifest().map(|value| value.id.as_str()), row["expected"]["manifestId"].as_str());
            let expected: Vec<String> = row["expected"]["nodeKinds"].as_array().unwrap().iter().map(|value| value.as_str().unwrap().to_string()).collect();
            assert_eq!(manifest_node_kinds(&graph), expected, "{}", row["id"]);
        }
    }
}

// #endregion 🔖️QueryableGraphTests

// #region 🔖️ErrorTests
#[test]
fn graph_dsl_error_display_messages() {
    assert_eq!(GraphDslError::UnterminatedString.to_string(), "unterminated string literal");
    assert_eq!(GraphDslError::UnexpectedChar('$').to_string(), "unexpected character '$'");
    assert_eq!(GraphDslError::EdgeTargetMissingPort.to_string(), "edge target requires @port");
    assert_eq!(GraphDslError::EmptyPattern.to_string(), "empty pattern");
    assert_eq!(GraphDslError::UnsupportedMutation.to_string(), "mutating jack clauses are not supported on this graph domain");
    assert_eq!(GraphDslError::UnknownProcedure("bogus".into()).to_string(), "unknown CALL procedure 'bogus'");
    assert_eq!(GraphDslError::ProcedureArity { name: "nodeKinds".into(), expected: 0, found: 1 }.to_string(), "procedure 'nodeKinds' expects 0 argument(s), got 1");
    let unexpected = GraphDslError::UnexpectedToken { expected: "ident".into(), found: "Eof".into() };
    assert_eq!(unexpected.to_string(), "expected ident, got Eof");
}

#[test]
fn parse_error_on_unexpected_char() {
    block_on_test(async {
        // `{`/`}` are valid tokens in `dsl_core`'s shared alphabet (map/object-literal braces)
        // but aren't part of Jack's own grammar (no map literals) — Jack rejects them itself,
        // hence `UnexpectedChar` rather than a `dsl_core`-surfaced `Lex` error.
        let err = parse("MATCH (a:x) { WHERE").unwrap_err();
        assert!(matches!(err, GraphDslError::UnexpectedChar('{')));
    });
}

#[test]
fn parse_error_on_char_outside_dsl_core_alphabet_reports_lex_error() {
    block_on_test(async {
        // `?` isn't lexable by `dsl_core` at all (unlike `{`/`}` above, which lex fine but aren't
        // valid Jack syntax) — `os_dsl::lex` itself fails, surfaced verbatim as `Lex`.
        let err = parse("MATCH (a:x) ? WHERE").unwrap_err();
        assert!(matches!(err, GraphDslError::Lex(_)));
        assert!(err.to_string().contains("unexpected character '?'"), "got: {err}");
    });
}

#[test]
fn parse_error_on_lone_bang_reports_lex_error() {
    block_on_test(async {
        // A stray `!` not followed by `=` isn't a token in Jack's grammar at all (`dsl_core` has
        // no relational operators, and Jack only special-cases `!=`).
        let err = parse("MATCH (a:x) WHERE a.p ! 1").unwrap_err();
        assert!(matches!(err, GraphDslError::Lex(_)));
    });
}

#[test]
fn parse_error_on_unterminated_string() {
    block_on_test(async {
        let err = parse("MATCH (a:x) WHERE a.name = 'oops").unwrap_err();
        assert!(matches!(err, GraphDslError::UnterminatedString));
    });
}
// #endregion 🔖️ErrorTests

// #region 🔖️LexerAndLanguageServiceTests
#[test]
fn tokenize_classifies_clause_and_operator_tokens() {
    block_on_test(async {
        let spans = tokenize("MATCH (a:x)--[:wire]->(b:y) WHERE a.p = 1 AND b.q != 'v' RETURN a.p");
        assert!(spans.iter().any(|s| s.class == TokenClass::Keyword));
        assert!(spans.iter().any(|s| s.class == TokenClass::Ident));
        assert!(spans.iter().any(|s| s.class == TokenClass::Number));
        assert!(spans.iter().any(|s| s.class == TokenClass::String));
        assert!(spans.iter().any(|s| s.class == TokenClass::Operator));
        assert!(spans.iter().any(|s| s.class == TokenClass::Punctuation));
    });
}

#[test]
fn tokenize_marks_unterminated_string_as_error_class() {
    block_on_test(async {
        let spans = tokenize("MATCH (a:x) WHERE a.p = 'unterminated");
        assert!(spans.iter().any(|s| s.class == TokenClass::Error));
    });
}

#[test]
fn tokenize_never_panics_on_stray_symbols() {
    block_on_test(async {
        // 🩹️ `#` is now a legitimate comment starter (unified with the rest of the DSL engine, so
        // it swallows the remainder of the line) — the stray-symbol probes moved off it.
        let spans = tokenize("MATCH (a:x) ~ ^ RETURN a");
        assert!(spans.iter().any(|s| s.class == TokenClass::Ident && s.end - s.start == 1));
    });
}

#[test]
fn tokenize_treats_hash_as_a_comment_to_end_of_line() {
    block_on_test(async {
        let source = "MATCH (a:x) # a trailing comment\nRETURN a";
        let comment_start = source.find('#').unwrap();
        let line_end = source.find('\n').unwrap();
        let spans = tokenize(source);
        assert!(!spans.iter().any(|s| s.start >= comment_start && s.start < line_end), "no token should start inside the comment body: {spans:?}");
        assert!(spans.iter().any(|s| s.class == TokenClass::Keyword));
    });
}

#[test]
fn format_query_is_idempotent_and_normalizes_whitespace() {
    block_on_test(async {
        let once = format("match(a:x)--[:wire]->(b:y) where a.p=1 and b.q!='v' return a.p,b.q").unwrap();
        assert!(once.contains("MATCH"));
        assert!(once.contains(" AND "));
        assert!(once.contains(" = "));
        let twice = format(&once).unwrap();
        assert_eq!(once, twice);
    });
}

#[test]
fn format_rejects_unterminated_string() {
    block_on_test(async {
        let err = format("MATCH (a:x) WHERE a.p = 'oops").unwrap_err();
        assert!(matches!(err, GraphDslError::UnterminatedString));
    });
}

#[test]
fn complete_after_colon_suggests_node_then_edge_kinds() {
    block_on_test(async {
        let graph = BoardQueryableGraph::from_host_snapshot_json(split_endpoint_fixture(), None).unwrap();
        let node_source = "MATCH (a:c";
        let node_completions = complete(&graph, node_source, node_source.len());
        assert!(node_completions.iter().any(|c| c.label == "computation"));
        let edge_source = "MATCH (a:computation)--[:w";
        let edge_completions = complete(&graph, edge_source, edge_source.len());
        assert!(edge_completions.iter().any(|c| c.label == "wire"));
    });
}

#[test]
fn complete_after_at_suggests_port_kinds() {
    block_on_test(async {
        let graph = BoardQueryableGraph::from_host_snapshot_json(split_endpoint_fixture(), None).unwrap();
        let source = "MATCH (a:computation@i";
        let completions = complete(&graph, source, source.len());
        assert!(completions.iter().any(|c| c.label == "in"));
    });
}

#[test]
fn complete_after_dot_suggests_property_names() {
    block_on_test(async {
        let graph = BoardQueryableGraph::from_host_snapshot_json(split_endpoint_fixture(), None).unwrap();
        let source = "MATCH (a:computation) RETURN a.sc";
        let completions = complete(&graph, source, source.len());
        assert!(completions.iter().any(|c| c.label == "score"));
    });
}

#[test]
fn complete_suggests_bound_variable_when_prefix_does_not_match_logic_keywords() {
    block_on_test(async {
        let graph = BoardQueryableGraph::from_host_snapshot_json(split_endpoint_fixture(), None).unwrap();
        let source = "MATCH (abc:computation) WHERE ab";
        let completions = complete(&graph, source, source.len());
        assert!(completions.iter().any(|c| c.label == "abc" && c.kind == "variable"));
    });
}

#[test]
fn complete_in_where_clause_suggests_logic_keywords() {
    block_on_test(async {
        let graph = BoardQueryableGraph::from_host_snapshot_json(split_endpoint_fixture(), None).unwrap();
        let source = "MATCH (a:computation) WHERE a.score = 1 AN";
        let completions = complete(&graph, source, source.len());
        assert!(completions.iter().any(|c| c.label == "AND"));
    });
}

#[test]
fn complete_at_start_suggests_clause_keywords() {
    block_on_test(async {
        let graph = BoardQueryableGraph::from_host_snapshot_json(split_endpoint_fixture(), None).unwrap();
        let completions = complete(&graph, "MA", 2);
        assert!(completions.iter().any(|c| c.label == "MATCH"));
    });
}

#[test]
fn hover_reports_keyword_and_bound_variable() {
    block_on_test(async {
        let graph = BoardQueryableGraph::from_host_snapshot_json(split_endpoint_fixture(), None).unwrap();
        let source = "MATCH (a:computation) WHERE a.score = 1 RETURN a";
        let match_pos = source.find("MATCH").unwrap();
        assert!(hover(&graph, source, match_pos + 1).unwrap().contents.contains("keyword"));
        // 🩹️ `hover_word_at` folds `:`/`.` into the word span, so a bound variable only resolves
        // in isolation when nothing follows it — the trailing standalone `a` in `RETURN a`.
        let var_pos = source.rfind('a').unwrap();
        assert!(hover(&graph, source, var_pos + 1).unwrap().contents.contains("Bound variable"));
    });
}

#[test]
fn hover_matches_bare_node_kind_edge_kind_and_property_words() {
    block_on_test(async {
        let graph = BoardQueryableGraph::from_host_snapshot_json(split_endpoint_fixture(), None).unwrap();
        let source = "computation wire score";
        assert!(hover(&graph, source, 3).unwrap().contents.contains("Node kind"));
        let edge_pos = source.find("wire").unwrap();
        assert!(hover(&graph, source, edge_pos + 1).unwrap().contents.contains("Edge kind"));
        let prop_pos = source.find("score").unwrap();
        assert!(hover(&graph, source, prop_pos + 1).unwrap().contents.contains("Property"));
    });
}

#[test]
fn hover_returns_none_for_whitespace() {
    block_on_test(async {
        let graph = BoardQueryableGraph::from_host_snapshot_json(split_endpoint_fixture(), None).unwrap();
        assert!(hover(&graph, "MATCH (a:x)   RETURN a", 12).is_none());
    });
}

#[test]
fn lint_flags_unknown_node_kind() {
    block_on_test(async {
        let graph = BoardQueryableGraph::from_host_snapshot_json(split_endpoint_fixture(), None).unwrap();
        let diags = lint(&graph, "MATCH (a:nonexistentKind) RETURN a");
        assert!(diags.iter().any(|d| d.code.as_deref() == Some("jack/unknown-node-kind")));
    });
}

#[test]
fn lint_flags_unbound_variable() {
    block_on_test(async {
        let graph = BoardQueryableGraph::from_host_snapshot_json(split_endpoint_fixture(), None).unwrap();
        let diags = lint(&graph, "MATCH (a:computation) RETURN b");
        assert!(diags.iter().any(|d| d.code.as_deref() == Some("jack/unbound-variable")));
    });
}

#[test]
fn lint_reports_parse_errors() {
    block_on_test(async {
        let graph = BoardQueryableGraph::from_host_snapshot_json(split_endpoint_fixture(), None).unwrap();
        let diags = lint(&graph, "MATCH (a:computation");
        assert!(diags.iter().any(|d| d.code.as_deref() == Some("jack/parse-error")));
    });
}

#[test]
fn lint_clean_query_has_no_diagnostics() {
    block_on_test(async {
        let graph = BoardQueryableGraph::from_host_snapshot_json(split_endpoint_fixture(), None).unwrap();
        let diags = lint(&graph, "MATCH (a:computation) RETURN a.name");
        assert!(diags.is_empty());
    });
}

#[test]
fn semantic_tokens_mirror_tokenize_classes() {
    block_on_test(async {
        let tokens = semantic_tokens("MATCH (a:x) RETURN a");
        assert!(tokens.iter().any(|t| t.class == "keyword"));
        assert!(tokens.iter().any(|t| t.class == "ident"));
    });
}
// #endregion 🔖️LexerAndLanguageServiceTests

// #region 🔖️ParserAndExecutorTests
#[test]
fn parse_delete_set_merge_clauses() {
    block_on_test(async {
        let q = parse("MATCH (a:x) DELETE a").unwrap();
        assert!(matches!(q.clauses[1], Clause::Delete(ref vars) if vars == &vec!["a".to_string()]));
        let q = parse("MATCH (a:x) SET a.name = 'v'").unwrap();
        assert!(matches!(q.clauses[1], Clause::Set(ref items) if items.len() == 1 && items[0].prop == "name"));
        let q = parse("MERGE (a:x)").unwrap();
        assert!(matches!(q.clauses[0], Clause::Merge(_)));
    });
}

#[test]
fn parse_where_and_or_precedence() {
    block_on_test(async {
        let q = parse("MATCH (a:x) WHERE a.p = 1 AND a.q = 2 OR a.r != 3").unwrap();
        let Clause::Where(expr) = &q.clauses[1] else { panic!("expected where") };
        assert!(matches!(expr, Expr::Or(_, _)));
    });
}

// #region 🔖️WithUnwindCallTests
#[test]
fn parse_with_clause() {
    block_on_test(async {
        let q = parse("MATCH (a:x) WITH a, a.name RETURN a").unwrap();
        let Clause::With(items) = &q.clauses[1] else { panic!("expected with") };
        assert_eq!(items.len(), 2);
        assert!(matches!(&items[0], ReturnItem::Var(v) if v == "a"));
        assert!(matches!(&items[1], ReturnItem::Property { var, prop } if var == "a" && prop == "name"));
    });
}

#[test]
fn parse_unwind_clause() {
    block_on_test(async {
        let q = parse("MATCH (a:x) UNWIND a.items AS item RETURN item").unwrap();
        let Clause::Unwind(clause) = &q.clauses[1] else { panic!("expected unwind") };
        assert!(matches!(&clause.source, ReturnItem::Property { var, prop } if var == "a" && prop == "items"));
        assert_eq!(clause.var, "item");
    });
}

#[test]
fn parse_call_clause_with_positional_args() {
    block_on_test(async {
        let q = parse("CALL myProc(1, \"two\", true)").unwrap();
        let Clause::Call(clause) = &q.clauses[0] else { panic!("expected call") };
        assert_eq!(clause.name, "myProc");
        assert_eq!(clause.args, vec![PropertyValue::Number(1.0), PropertyValue::String("two".to_string()), PropertyValue::Bool(true)]);
    });
}

#[test]
fn execute_with_projects_named_vars_and_drops_the_rest() {
    block_on_test(async {
        let graph = BoardQueryableGraph::from_host_snapshot_json(split_endpoint_fixture(), None).unwrap();
        let result = run_query(&graph, "MATCH (a:computation)--[:wire]--(b:slider) WITH a RETURN a.name, b.name").unwrap();
        assert!(!result.rows.is_empty());
        for row in &result.rows {
            assert_eq!(row[0], PropertyValue::String("A".to_string()));
            assert_eq!(row[1], PropertyValue::Null, "b was projected out of scope by WITH a, so b.name must be null");
        }
    });
}

#[test]
fn execute_with_where_filters_the_projected_bindings() {
    block_on_test(async {
        let graph = BoardQueryableGraph::from_host_snapshot_json(split_endpoint_fixture(), None).unwrap();
        let result = run_query(&graph, "MATCH (a:slider) WITH a WHERE a.name = 'B' RETURN a.name").unwrap();
        assert_eq!(result.rows, vec![vec![PropertyValue::String("B".to_string())]]);
    });
}

#[test]
fn execute_with_property_item_keeps_its_source_var_resolvable() {
    block_on_test(async {
        // 🔭️ `ReturnItem` carries no alias, so `WITH a.name` keeps the whole `a` entity in
        // scope (there is no other way for a later `a.name` to still resolve) — see
        // `project_binding`'s doc comment for the reasoning.
        let graph = BoardQueryableGraph::from_host_snapshot_json(split_endpoint_fixture(), None).unwrap();
        let result = run_query(&graph, "MATCH (a:computation) WITH a.name RETURN a.name").unwrap();
        assert_eq!(result.rows, vec![vec![PropertyValue::String("A".to_string())]]);
    });
}

#[test]
fn execute_unwind_over_a_property_expression_producing_a_list() {
    block_on_test(async {
        let graph = BoardQueryableGraph::from_host_snapshot_json(list_property_fixture(), None).unwrap();
        let result = run_query(&graph, "MATCH (a:computation) UNWIND a.tags AS tag RETURN tag").unwrap();
        assert_eq!(result.rows, vec![vec![PropertyValue::String("x".to_string())], vec![PropertyValue::String("y".to_string())], vec![PropertyValue::String("z".to_string())],]);
    });
}

#[test]
fn execute_unwind_over_an_already_bound_list_value() {
    block_on_test(async {
        // 🌀️ Chained UNWIND: the outer unwind's per-row `row` binding lives in `values` (not
        // a graph property), so the inner `UNWIND row AS cell` exercises the `Var`-sourced
        // (rather than `Property`-sourced) list-expression path.
        let graph = BoardQueryableGraph::from_host_snapshot_json(list_property_fixture(), None).unwrap();
        let result = run_query(&graph, "MATCH (a:computation) UNWIND a.matrix AS row UNWIND row AS cell RETURN cell").unwrap();
        assert_eq!(result.rows, vec![vec![PropertyValue::Number(1.0)], vec![PropertyValue::Number(2.0)], vec![PropertyValue::Number(3.0)], vec![PropertyValue::Number(4.0)]]);
    });
}

#[test]
fn execute_unwind_of_an_empty_list_yields_zero_rows() {
    block_on_test(async {
        let graph = BoardQueryableGraph::from_host_snapshot_json(list_property_fixture(), None).unwrap();
        let result = run_query(&graph, "MATCH (a:computation) UNWIND a.empty AS e RETURN e").unwrap();
        assert!(result.rows.is_empty());
    });
}

#[test]
fn execute_call_known_procedure_yields_its_registered_column() {
    block_on_test(async {
        let graph = BoardQueryableGraph::from_host_snapshot_json(list_property_fixture(), None).unwrap();
        let result = run_query(&graph, "CALL nodeKinds() RETURN kind").unwrap();
        assert_eq!(result.rows, vec![vec![PropertyValue::String("computation".to_string())], vec![PropertyValue::String("slider".to_string())]]);
    });
}

#[test]
fn execute_call_unknown_procedure_reports_a_precise_error() {
    block_on_test(async {
        let graph = BoardQueryableGraph::from_host_snapshot_json(list_property_fixture(), None).unwrap();
        let err = run_query(&graph, "CALL bogus()").unwrap_err();
        assert!(matches!(err, GraphDslError::UnknownProcedure(ref name) if name == "bogus"), "got {err:?}");
    });
}

#[test]
fn execute_call_procedure_arity_mismatch_reports_a_precise_error() {
    block_on_test(async {
        let graph = BoardQueryableGraph::from_host_snapshot_json(list_property_fixture(), None).unwrap();
        let err = run_query(&graph, "CALL nodeKinds(1)").unwrap_err();
        assert!(matches!(err, GraphDslError::ProcedureArity { ref name, expected: 0, found: 1 } if name == "nodeKinds"), "got {err:?}");
    });
}
// #endregion 🔖️WithUnwindCallTests

#[test]
fn lexer_accepts_both_single_and_double_quoted_strings_and_always_prints_double_quoted() {
    block_on_test(async {
        let single = parse("MATCH (a:x) WHERE a.name = 'alpha' RETURN a").unwrap();
        let double = parse("MATCH (a:x) WHERE a.name = \"alpha\" RETURN a").unwrap();
        assert_eq!(single, double, "single- and double-quoted string literals must parse identically");
        let printed = format("MATCH (a:x) WHERE a.name = 'alpha' RETURN a").unwrap();
        assert!(printed.contains("\"alpha\""), "must always print double-quoted: {printed}");
        assert!(!printed.contains('\''), "must never print single-quoted: {printed}");
    });
}

#[test]
fn parse_unexpected_token_error_has_expected_and_found() {
    block_on_test(async {
        let err = parse("MATCH a:x)").unwrap_err();
        let GraphDslError::UnexpectedToken { expected, found } = err else { panic!("expected UnexpectedToken") };
        assert_eq!(expected, "LParen");
        assert!(found.contains("Ident"));
    });
}

#[test]
fn execute_where_clause_filters_bindings() {
    block_on_test(async {
        let graph = BoardQueryableGraph::from_host_snapshot_json(split_endpoint_fixture(), None).unwrap();
        let result = run_query(&graph, "MATCH (a:slider) WHERE a.name = 'B' RETURN a.name").unwrap();
        assert_eq!(result.rows, vec![vec![PropertyValue::String("B".into())]]);
    });
}

#[test]
fn execute_and_or_expressions() {
    block_on_test(async {
        let graph = BoardQueryableGraph::from_host_snapshot_json(split_endpoint_fixture(), None).unwrap();
        let and_result = run_query(&graph, "MATCH (a:slider) WHERE a.name = 'B' AND a.kind = 'slider' RETURN a.name").unwrap();
        assert_eq!(and_result.rows.len(), 1);
        let or_result = run_query(&graph, "MATCH (a:slider) WHERE a.name = 'B' OR a.name = 'C' RETURN a.name").unwrap();
        assert_eq!(or_result.rows.len(), 2);
    });
}

#[test]
fn execute_rejects_mutating_clauses() {
    block_on_test(async {
        let graph = BoardQueryableGraph::from_host_snapshot_json(split_endpoint_fixture(), None).unwrap();
        for query in ["CREATE (a:x)", "MATCH (a:x) DELETE a", "MATCH (a:x) SET a.p = 1", "MERGE (a:x)"] {
            let err = run_query(&graph, query).unwrap_err();
            assert!(matches!(err, GraphDslError::UnsupportedMutation), "query {query} should reject mutation");
        }
    });
}

#[test]
fn execute_undirected_edge_matches_both_directions() {
    block_on_test(async {
        let graph = BoardQueryableGraph::from_host_snapshot_json(split_endpoint_fixture(), None).unwrap();
        let forward = run_query(&graph, "MATCH (a:computation)--[:wire]--(b:slider) RETURN a.name, b.name").unwrap();
        let reverse = run_query(&graph, "MATCH (b:slider)--[:wire]--(a:computation) RETURN a.name, b.name").unwrap();
        assert!(!forward.rows.is_empty());
        assert!(!reverse.rows.is_empty());
    });
}

#[test]
fn execute_multiple_match_patterns_join_bindings() {
    block_on_test(async {
        let graph = BoardQueryableGraph::from_host_snapshot_json(split_endpoint_fixture(), None).unwrap();
        let result = run_query(&graph, "MATCH (a:computation), (b:slider) RETURN a.name, b.name").unwrap();
        assert_eq!(result.rows.len(), 2);
    });
}

#[test]
fn execute_returns_graph_kind_when_returning_bound_entities() {
    block_on_test(async {
        let graph = BoardQueryableGraph::from_host_snapshot_json(split_endpoint_fixture(), None).unwrap();
        let result = run_query(&graph, "MATCH (a:computation)--[e:wire]--(b:slider) RETURN a, e, b").unwrap();
        assert_eq!(result.kind, QueryResultKind::Graph);
        assert!(result.graph_fixture_json.is_some());
    });
}

#[test]
fn execute_returns_table_kind_for_property_projection() {
    block_on_test(async {
        let graph = BoardQueryableGraph::from_host_snapshot_json(split_endpoint_fixture(), None).unwrap();
        let result = run_query(&graph, "MATCH (a:computation) RETURN a.name").unwrap();
        assert_eq!(result.kind, QueryResultKind::Table);
    });
}

#[test]
fn execute_with_no_return_clause_yields_empty_table() {
    block_on_test(async {
        let graph = BoardQueryableGraph::from_host_snapshot_json(split_endpoint_fixture(), None).unwrap();
        let result = run_query(&graph, "MATCH (a:computation)").unwrap();
        assert!(result.columns.is_empty());
        assert!(result.rows.is_empty());
    });
}

#[test]
fn run_query_json_serializes_result() {
    block_on_test(async {
        let graph = BoardQueryableGraph::from_host_snapshot_json(split_endpoint_fixture(), None).unwrap();
        let json = run_query_json(&graph, "MATCH (a:computation) RETURN a.name").unwrap();
        let value: semio_framework_pack_json::Value = semio_framework_pack_json::parse(&json,semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
        assert_eq!(value["columns"][0], "a.name");
    });
}

#[test]
fn empty_pattern_error_is_reachable_via_pattern_construction() {
    block_on_test(async {
        let pattern = Pattern { nodes: vec![], edge: None };
        let graph = BoardQueryableGraph::from_host_snapshot_json(split_endpoint_fixture(), None).unwrap();
        let err = match_patterns(&graph, std::slice::from_ref(&pattern)).unwrap_err();
        assert!(matches!(err, GraphDslError::EmptyPattern));
    });
}
// #endregion 🔖️ParserAndExecutorTests

// #region ➖️CypherDashConnector
/// ➖️ A bare `-` is the pattern connector every Cypher-shaped query writes, and it parses to exactly
/// the same `Pattern` as this grammar's own `--` spelling.
///
/// 🐛️ `-` used to fall into the lexer's stray-character bucket, so the ONLY spelling this grammar
/// accepted was `--`. Since `lint`, `complete`, `hover`, `semantic_tokens` and `format` are the
/// language service every consumer of this dialect delegates to, a perfectly runnable query was
/// reported `unexpected character '-'` in the editor: measured live on 2026-09-22 in semio-tech play's
/// `trinity-jack` pane, whose own default query is
/// `MATCH (a:Piece)-[r:Connection]->(b:Piece) …` and whose executor parses that shape in its own
/// green unit tests.
#[test]
fn a_bare_dash_is_the_same_pattern_connector_as_a_double_dash() {
    block_on_test(async {
        let single = parse("MATCH (a:x)-[r:wire]->(b:y) RETURN a.p").unwrap();
        let double = parse("MATCH (a:x)--[r:wire]->(b:y) RETURN a.p").unwrap();
        assert_eq!(single, double, "`-` and `--` must reach the same parsed query");
    });
}

/// ➖️ The undirected tail and the reversed form take a single dash too.
#[test]
fn a_bare_dash_also_spells_the_undirected_and_reversed_connectors() {
    block_on_test(async {
        assert_eq!(parse("MATCH (a:x)-[r:wire]-(b:y) RETURN a.p").unwrap(), parse("MATCH (a:x)--[r:wire]--(b:y) RETURN a.p").unwrap());
        assert_eq!(parse("MATCH (a:x)<-[r:wire]-(b:y) RETURN a.p").unwrap(), parse("MATCH (a:x)<-[r:wire]--(b:y) RETURN a.p").unwrap());
    });
}

/// ➖️ Formatting a single-dash query stays idempotent — whichever spelling the formatter settles on,
/// running it twice must be a fixed point, and the result must still parse.
#[test]
fn formatting_a_single_dash_pattern_is_idempotent_and_reparses() {
    block_on_test(async {
        let once = format("match(a:x)-[r:wire]->(b:y) return a.p").unwrap();
        assert_eq!(format(&once).unwrap(), once);
        parse(&once).unwrap();
    });
}
// #endregion ➖️CypherDashConnector

```

### 🧰️framework/🔨️modules/🕸️graph/📦️packages/🦀️rust/🦀️.rs

SHA-256 `006ecd36c9b70df435081f18d12ccb96dbf13865e327679e2c6355eb1212495c`; 2150 bytes.

```
//! 🕸️ The semio graph framework module: storage and view vocabulary, index-based algorithms, drawing layouts, the compile-time manifest registry, and the Jack graph query language.
//!
//! Each domain is a `🦀️.rs` in the owner tree; this entry file is pure wiring.

// 🃏️ `QueryableGraph` (Jack) has async trait methods; Send-ness comes structurally from the
// concrete/generic caller, never from a `+ Send` bound on the trait method — see R3 and R7.
#![allow(async_fn_in_trait)]

// 🃏️ Renamed `dsl` → `dsl_core` (wave MATHEND) to free the crate-root name `dsl` for
// `pub mod dsl` (Jack) below — both alias the identical crate. `🛂️manifest` updated to match.
extern crate semio_framework_os_kernel as dsl_core;
extern crate semio_framework_value_derive as value_derive;

#[path = "../../⚙️engine/🦀️.rs"]
mod engine;
pub use engine::*;

#[path = "../../🧮️algorithms/🦀️.rs"]
pub mod algorithms;

#[path = "../../🖊️drawing/🦀️.rs"]
pub mod drawing;

#[path = "../../🛂️manifest/🦀️.rs"]
pub mod manifest;

// 🃏️ wave MATHEND (ticket 26/08/12/DISSOLVE-KERNELS-AND-MODULES-INTO-EVENT-SOURCED-ARTIFACTS):
// Jack relocated verbatim from `🧰️framework/🔨️modules/🧮️math/🕸️graph/🗣️dsl` — measured NOT
// cleanly splittable into a framework "core" + plugin "language-service" half as first hypothesized:
// `complete`/`format` are needed by BOTH the framework-tier `DslIdiom` self-registration seam
// (`idiom_hooks`) AND `🔱️trinity`'s explicit LSP-style calls, and `complete`/`hover` share private
// helpers (`collect_bound_vars`, `lex_spanned`) that would need to become new public API or be
// duplicated (forbidden) to cross a crate boundary. Independently, Jack (a generic pattern-matching
// query language over graphs, plus its own editor tooling) passes the domain-neutral test — it names
// no domain, structurally analogous to `💻️os/🔨️modules/🗣️dsl`'s already-framework-tier
// diagnostic/completion machinery. See that wave's report for the full evidence and reasoning.
#[path = "../../🗣️dsl/🦀️.rs"]
pub mod dsl;

```

### 🧰️framework/🔨️modules/🕸️graph/⚙️engine/🦀️.rs

SHA-256 `cbe2204ee953d62bd200419d43840a0c8003d3c0c6ab1797f505ef62117b4a81`; 78074 bytes.

```
//! 🕸️ Pure graph foundation: topology markers, node/handle/edge kinds, and index-based algorithms; the interactive board engine lives in `infinite_board`.

use std::collections::{BTreeMap, BTreeSet};

pub use crate::manifest::{PropertyBag, PropertyValue};

// #region 🔖️Ids
/// 🧩️ Stable node identifier.
pub type NodeId = u64;
/// 🪝️ Stable handle identifier.
pub type HandleId = u64;
/// 🪢️ Stable edge identifier.
pub type EdgeId = u64;
// #endregion 🔖️Ids

// #region 🔖️Edge
/// 🪢️ Edge with typed endpoints (node id or handle id).
// 🧬️ `value_derive::{ToValue, FromValue}` additive (RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS,
// 26/09/01) — never had `serde`; `E` is always `P::Endpoint` (`NodeId`/`HandleId`, both `u64`) in
// practice, so the derive's auto-synthesized `E: ToValue + FromValue` bound is always satisfied.
#[derive(Clone, Copy, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue)]
pub struct CoreEdge<E> {
    pub id: EdgeId,
    pub source: E,
    pub target: E,
}

impl<E: Copy + Ord> CoreEdge<E> {
    /// 📐️ Normalize endpoints for undirected storage.
    pub fn normalize_undirected(source: E, target: E) -> (E, E) {
        if source <= target {
            (source, target)
        } else {
            (target, source)
        }
    }
}
// #endregion 🔖️Edge

// #region 🔖️Directedness
/// ↔ Compile-time directed vs undirected graph axis.
pub trait Directedness {
    const DIRECTED: bool;
}

/// ➡️ Directed edges keep source→target order.
// 🧬️ ToValue/FromValue additive (RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS,
// 26/09/01), hand-written rather than derived: `#[derive(ToValue, FromValue)]` rejects a
// SEMICOLON-terminated unit struct (`Fields::Unit`, distinct from an empty-brace `struct Foo {}`,
// which IS `Fields::Named`) — changing to brace form would ripple into every value-level
// construction site of this marker type. Zero-field, so the wire shape is `null` (serde's own
// default for a unit struct, had this type ever derived `Serialize`).
#[derive(Clone, Copy, Debug, Default)]
pub struct Directed;

impl dsl_core::ToValue for Directed {
    fn to_value(&self) -> dsl_core::DslValue {
        dsl_core::DslValue::Null
    }
}
impl dsl_core::FromValue for Directed {
    fn from_value(_value: dsl_core::DslValue) -> Result<Self, dsl_core::ValueError> {
        Ok(Directed)
    }
}

impl Directedness for Directed {
    const DIRECTED: bool = true;
}

/// ↔ Undirected edges store ordered endpoint pair.
// 🧬️ Hand-written ToValue/FromValue, see `Directed` above.
#[derive(Clone, Copy, Debug, Default)]
pub struct Undirected;

impl dsl_core::ToValue for Undirected {
    fn to_value(&self) -> dsl_core::DslValue {
        dsl_core::DslValue::Null
    }
}
impl dsl_core::FromValue for Undirected {
    fn from_value(_value: dsl_core::DslValue) -> Result<Self, dsl_core::ValueError> {
        Ok(Undirected)
    }
}

impl Directedness for Undirected {
    const DIRECTED: bool = false;
}

/// 📐️ Apply directedness when storing edge endpoints.
#[inline]
pub fn orient_endpoints<E: Copy + Ord, D: Directedness>(source: E, target: E) -> (E, E) {
    if D::DIRECTED {
        (source, target)
    } else {
        CoreEdge::<E>::normalize_undirected(source, target)
    }
}
// #endregion 🔖️Directedness

// #region 🔖️PortModel
/// 🔌️ Compile-time normal (node) vs ported (handle) graph axis.
pub trait PortModel {
    type Endpoint: Copy + Ord + std::fmt::Debug;
    const HAS_PORTS: bool;
    /// 🪢️ Whether this port model allows parallel edges between the same pair (the port axis IS the multi-edge axis: `Ported` ~ NetworkX `Multi(Di)Graph`, `Normal` ~ NetworkX `(Di)Graph`).
    const MULTI_EDGES: bool;
    fn endpoint_as_u64(endpoint: Self::Endpoint) -> u64;
    fn try_handle_endpoint(handle_id: HandleId) -> Option<Self::Endpoint>;
    fn endpoint_as_handle(endpoint: Self::Endpoint) -> Option<HandleId>;
}

/// 🟠️ Node-to-node edges without handles.
// 🧬️ Hand-written ToValue/FromValue (RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS,
// 26/09/01), not derived — see `Directed`'s note on why a semicolon-terminated unit struct can't
// use `#[derive(ToValue, FromValue)]`.
#[derive(Clone, Copy, Debug, Default)]
pub struct Normal;

impl dsl_core::ToValue for Normal {
    fn to_value(&self) -> dsl_core::DslValue {
        dsl_core::DslValue::Null
    }
}
impl dsl_core::FromValue for Normal {
    fn from_value(_value: dsl_core::DslValue) -> Result<Self, dsl_core::ValueError> {
        Ok(Normal)
    }
}

impl PortModel for Normal {
    type Endpoint = NodeId;
    const HAS_PORTS: bool = false;
    const MULTI_EDGES: bool = false;
    fn endpoint_as_u64(endpoint: Self::Endpoint) -> u64 {
        endpoint
    }
    fn try_handle_endpoint(_: HandleId) -> Option<Self::Endpoint> {
        None
    }
    fn endpoint_as_handle(_: Self::Endpoint) -> Option<HandleId> {
        None
    }
}

/// 🪝️ Handle-to-handle edges on nodes.
// 🧬️ Hand-written ToValue/FromValue, see `Normal` above.
#[derive(Clone, Copy, Debug, Default)]
pub struct Ported;

impl dsl_core::ToValue for Ported {
    fn to_value(&self) -> dsl_core::DslValue {
        dsl_core::DslValue::Null
    }
}
impl dsl_core::FromValue for Ported {
    fn from_value(_value: dsl_core::DslValue) -> Result<Self, dsl_core::ValueError> {
        Ok(Ported)
    }
}

impl PortModel for Ported {
    type Endpoint = HandleId;
    const HAS_PORTS: bool = true;
    const MULTI_EDGES: bool = true;
    fn endpoint_as_u64(endpoint: Self::Endpoint) -> u64 {
        endpoint
    }
    fn try_handle_endpoint(handle_id: HandleId) -> Option<Self::Endpoint> {
        Some(handle_id)
    }
    fn endpoint_as_handle(endpoint: Self::Endpoint) -> Option<HandleId> {
        Some(endpoint)
    }
}
// #endregion 🔖️PortModel

// #region 🔖️Storage
/// 📦️ Per-node record: attribute bag plus, for ported storages, the handles anchored on it (stays empty for `Normal` storages).
// 🧬️ `value_derive::{ToValue, FromValue}` additive (RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS,
// 26/09/01) — never had `serde`; `PropertyBag`/`PropertyValue` (`🛂️manifest`) are already covered.
#[derive(Clone, Debug, Default, value_derive::ToValue, value_derive::FromValue)]
pub struct NodeRecord {
    pub attrs: PropertyBag,
    pub handles: Vec<HandleId>,
}

/// 📦️ Per-edge record: typed endpoints (node ids for `Normal`, handle ids for `Ported`) plus attribute bag.
// 🧬️ `value_derive::{ToValue, FromValue}` additive, see `NodeRecord` above; `E` is always `u64` in
// practice (see `CoreEdge`'s note).
#[derive(Clone, Debug, value_derive::ToValue, value_derive::FromValue)]
pub struct EdgeRecord<E> {
    pub source: E,
    pub target: E,
    pub attrs: PropertyBag,
}

/// 🗑️ Removes one occurrence of `edge_id` from `map[u][v]`, dropping the inner entry once its edge list empties.
fn unlink_one(map: &mut BTreeMap<NodeId, BTreeMap<NodeId, Vec<EdgeId>>>, u: NodeId, v: NodeId, edge_id: EdgeId) {
    if let Some(inner) = map.get_mut(&u) {
        if let Some(ids) = inner.get_mut(&v) {
            if let Some(pos) = ids.iter().position(|&e| e == edge_id) {
                ids.remove(pos);
            }
            if ids.is_empty() {
                inner.remove(&v);
            }
        }
    }
}

/// 🗄️ Shared adjacency-map storage behind every per-kind facade crate; `BTreeMap` everywhere keeps iteration deterministic. Node-level adjacency (`successors`/`predecessors`) is always keyed by `NodeId`, even for `Ported` storages — port/handle detail lives only in `EdgeRecord::source`/`target` and is resolved down to owning nodes via `handle_owner`. For undirected storages `successors` already holds both directions of every edge, so `predecessors` stays empty and is never consulted (documented at each call site); a self-loop on an undirected storage is recorded twice in `successors[u][u]`, matching NetworkX's convention of counting a self-loop twice towards degree.
// 🧬️ `value_derive::{ToValue, FromValue}` additive (RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS,
// 26/09/01) — never had `serde`. Every `BTreeMap` here is keyed by `NodeId`/`EdgeId`/`HandleId`
// (all bare `u64`), not `String`, so each names a stringified-key `with` bridge (precedent: actor's
// `ShardTable`/`actor_shard_map_to_value`) rather than the `BTreeMap<String, _>`-only blanket impl.
// `#[value(bound = "...")]` replaces the derive's per-type-param auto bound (`P`/`D` themselves are
// never touched — both live only behind a skipped `PhantomData` field) with the ONE bound the
// `edges` bridge actually needs: `P::Endpoint` (always `NodeId`/`HandleId` = `u64` in practice).
#[derive(Clone, Debug, value_derive::ToValue, value_derive::FromValue)]
#[value(bound = "P::Endpoint: dsl_core::ToValue + dsl_core::FromValue")]
pub struct Storage<P: PortModel, D: Directedness> {
    #[value(with = "storage_nodes_bridge")]
    nodes: BTreeMap<NodeId, NodeRecord>,
    #[value(with = "storage_edges_bridge")]
    edges: BTreeMap<EdgeId, EdgeRecord<P::Endpoint>>,
    #[value(with = "storage_adjacency_bridge")]
    successors: BTreeMap<NodeId, BTreeMap<NodeId, Vec<EdgeId>>>,
    #[value(with = "storage_adjacency_bridge")]
    predecessors: BTreeMap<NodeId, BTreeMap<NodeId, Vec<EdgeId>>>,
    #[value(with = "storage_handle_owner_bridge")]
    handle_owner: BTreeMap<HandleId, NodeId>,
    graph_attrs: PropertyBag,
    next_node_id: NodeId,
    next_edge_id: EdgeId,
    next_handle_id: HandleId,
    #[value(skip)]
    _directedness: std::marker::PhantomData<D>,
    #[value(skip)]
    _port_model: std::marker::PhantomData<P>,
}

/// 🌉️ `Storage::nodes` bridge — `BTreeMap<NodeId, _>` is `u64`-keyed, not `String`-keyed, so it
/// cannot use the `BTreeMap<String, T>`-only blanket `ToValue`/`FromValue` impl (`🌱️value/🔁️codec`);
/// stringifies the key the same way `serde_json` itself would for an integer map key.
mod storage_nodes_bridge {
    pub fn to_value(map: &std::collections::BTreeMap<super::NodeId, super::NodeRecord>) -> dsl_core::DslValue {
        dsl_core::DslValue::object(map.iter().map(|(k, v)| (k.to_string(), dsl_core::ToValue::to_value(v))))
    }
    pub fn from_value(value: dsl_core::DslValue) -> Result<std::collections::BTreeMap<super::NodeId, super::NodeRecord>, dsl_core::ValueError> {
        let dsl_core::DslValue::Object(entries) = value else {
            return Err(dsl_core::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "expected an object for Storage::nodes"));
        };
        entries
            .into_iter()
            .map(|(k, v)| {
                let id: super::NodeId = k.parse().map_err(|_| dsl_core::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("invalid NodeId key `{k}`")))?;
                let record = <super::NodeRecord as dsl_core::FromValue>::from_value(v).map_err(|e| e.under(&k))?;
                Ok((id, record))
            })
            .collect()
    }
    use crate::dsl_core;
}

/// 🌉️ `Storage::edges` bridge — generic over `E = P::Endpoint` (always `u64` in practice), same
/// `u64`-key stringification as `storage_nodes_bridge`.
mod storage_edges_bridge {
    pub fn to_value<E: dsl_core::ToValue>(map: &std::collections::BTreeMap<super::EdgeId, super::EdgeRecord<E>>) -> dsl_core::DslValue {
        dsl_core::DslValue::object(map.iter().map(|(k, v)| (k.to_string(), dsl_core::ToValue::to_value(v))))
    }
    pub fn from_value<E: dsl_core::FromValue>(value: dsl_core::DslValue) -> Result<std::collections::BTreeMap<super::EdgeId, super::EdgeRecord<E>>, dsl_core::ValueError> {
        let dsl_core::DslValue::Object(entries) = value else {
            return Err(dsl_core::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "expected an object for Storage::edges"));
        };
        entries
            .into_iter()
            .map(|(k, v)| {
                let id: super::EdgeId = k.parse().map_err(|_| dsl_core::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("invalid EdgeId key `{k}`")))?;
                let record = <super::EdgeRecord<E> as dsl_core::FromValue>::from_value(v).map_err(|e| e.under(&k))?;
                Ok((id, record))
            })
            .collect()
    }
    use crate::dsl_core;
}

/// 🌉️ `Storage::successors`/`predecessors` bridge — `NodeId -> NodeId -> [EdgeId]`, both map levels
/// `u64`-keyed.
mod storage_adjacency_bridge {
    pub fn to_value(map: &std::collections::BTreeMap<super::NodeId, std::collections::BTreeMap<super::NodeId, Vec<super::EdgeId>>>) -> dsl_core::DslValue {
        dsl_core::DslValue::object(map.iter().map(|(k, inner)| {
            (k.to_string(), dsl_core::DslValue::object(inner.iter().map(|(k2, v)| (k2.to_string(), dsl_core::ToValue::to_value(v)))))
        }))
    }
    pub fn from_value(value: dsl_core::DslValue) -> Result<std::collections::BTreeMap<super::NodeId, std::collections::BTreeMap<super::NodeId, Vec<super::EdgeId>>>, dsl_core::ValueError> {
        let dsl_core::DslValue::Object(entries) = value else {
            return Err(dsl_core::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "expected an object for Storage adjacency"));
        };
        entries
            .into_iter()
            .map(|(k, inner)| {
                let node_id: super::NodeId = k.parse().map_err(|_| dsl_core::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("invalid NodeId key `{k}`")))?;
                let dsl_core::DslValue::Object(inner_entries) = inner else {
                    return Err(dsl_core::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "expected an object").under(&k));
                };
                let inner_map = inner_entries
                    .into_iter()
                    .map(|(k2, v)| {
                        let node_id2: super::NodeId = k2.parse().map_err(|_| dsl_core::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("invalid NodeId key `{k2}`")))?;
                        let edges = <Vec<super::EdgeId> as dsl_core::FromValue>::from_value(v).map_err(|e| e.under(&k2))?;
                        Ok((node_id2, edges))
                    })
                    .collect::<Result<std::collections::BTreeMap<_, _>, dsl_core::ValueError>>()?;
                Ok((node_id, inner_map))
            })
            .collect()
    }
    use crate::dsl_core;
}

/// 🌉️ `Storage::handle_owner` bridge — `HandleId -> NodeId`, `u64`-keyed.
mod storage_handle_owner_bridge {
    pub fn to_value(map: &std::collections::BTreeMap<super::HandleId, super::NodeId>) -> dsl_core::DslValue {
        dsl_core::DslValue::object(map.iter().map(|(k, v)| (k.to_string(), dsl_core::ToValue::to_value(v))))
    }
    pub fn from_value(value: dsl_core::DslValue) -> Result<std::collections::BTreeMap<super::HandleId, super::NodeId>, dsl_core::ValueError> {
        let dsl_core::DslValue::Object(entries) = value else {
            return Err(dsl_core::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "expected an object for Storage::handle_owner"));
        };
        entries
            .into_iter()
            .map(|(k, v)| {
                let handle_id: super::HandleId = k.parse().map_err(|_| dsl_core::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("invalid HandleId key `{k}`")))?;
                let node_id = <super::NodeId as dsl_core::FromValue>::from_value(v).map_err(|e| e.under(&k))?;
                Ok((handle_id, node_id))
            })
            .collect()
    }
    use crate::dsl_core;
}

impl<P: PortModel, D: Directedness> Default for Storage<P, D> {
    // 🚫️async: E1 impl of external trait `std::default::Default` — must stay sync. Mirrors `new()`'s
    // literal (I/O-free) body directly rather than calling it, since `new()` stays `async` for
    // call-site uniformity with the rest of this crate. See R9.
    fn default() -> Self {
        Self {
            nodes: BTreeMap::new(),
            edges: BTreeMap::new(),
            successors: BTreeMap::new(),
            predecessors: BTreeMap::new(),
            handle_owner: BTreeMap::new(),
            graph_attrs: PropertyBag::new(),
            next_node_id: 0,
            next_edge_id: 0,
            next_handle_id: 0,
            _directedness: std::marker::PhantomData,
            _port_model: std::marker::PhantomData,
        }
    }
}

impl<P: PortModel, D: Directedness> Storage<P, D> {
    /// 🆕️ Empty storage; every id allocator starts at `0` and is monotone — an id is never reused, even after removal.
    pub fn new() -> Self {
        Self {
            nodes: BTreeMap::new(),
            edges: BTreeMap::new(),
            successors: BTreeMap::new(),
            predecessors: BTreeMap::new(),
            handle_owner: BTreeMap::new(),
            graph_attrs: PropertyBag::new(),
            next_node_id: 0,
            next_edge_id: 0,
            next_handle_id: 0,
            _directedness: std::marker::PhantomData,
            _port_model: std::marker::PhantomData,
        }
    }

    /// 🔗️ Resolves an edge endpoint down to the node it lives on: identity for `Normal` (`Endpoint == NodeId`), a `handle_owner` lookup for `Ported`.
    fn endpoint_node(&self, endpoint: P::Endpoint) -> NodeId {
        match P::endpoint_as_handle(endpoint) {
            Some(handle_id) => *self.handle_owner.get(&handle_id).expect("every live handle endpoint has a recorded owner node"),
            None => P::endpoint_as_u64(endpoint),
        }
    }

    fn link_adjacency(&mut self, u: NodeId, v: NodeId, edge_id: EdgeId) {
        self.successors.entry(u).or_default().entry(v).or_default().push(edge_id);
        if D::DIRECTED {
            self.predecessors.entry(v).or_default().entry(u).or_default().push(edge_id);
        } else if u == v {
            self.successors.entry(u).or_default().entry(v).or_default().push(edge_id);
        } else {
            self.successors.entry(v).or_default().entry(u).or_default().push(edge_id);
        }
    }

    fn unlink_adjacency(&mut self, u: NodeId, v: NodeId, edge_id: EdgeId) {
        // 🚨️ all four branches were dropped-future no-ops: none of these `unlink_one` calls were
        // ever awaited, so `unlink_adjacency` silently did nothing — the outer `.await` fixed at
        // its own call sites (`remove_edge`) was necessary but not sufficient.
        unlink_one(&mut self.successors, u, v, edge_id);
        if D::DIRECTED {
            unlink_one(&mut self.predecessors, v, u, edge_id);
        } else if u == v {
            unlink_one(&mut self.successors, u, v, edge_id);
        } else {
            unlink_one(&mut self.successors, v, u, edge_id);
        }
    }

    // #subregion Nodes
    pub fn add_node(&mut self) -> NodeId {
        self.add_node_with(PropertyBag::new())
    }

    pub fn add_node_with(&mut self, attrs: PropertyBag) -> NodeId {
        let id = self.next_node_id;
        self.next_node_id += 1;
        self.nodes.insert(id, NodeRecord { attrs, handles: Vec::new() });
        id
    }

    /// 🆔️ Inserts a node at a caller-supplied id, or merges `attrs` into it if already present (NetworkX `add_node(id, **attrs)` semantics); bumps the allocator past `id` so future auto-ids never collide with it.
    pub fn add_node_with_id(&mut self, id: NodeId, attrs: PropertyBag) -> NodeId {
        if self.next_node_id <= id {
            self.next_node_id = id + 1;
        }
        match self.nodes.get_mut(&id) {
            Some(record) => {
                for (key, value) in attrs {
                    if let Some(previous) = record.attrs.insert(key, value) {
                        <PropertyValue as semio_framework_value::FromValue>::retire_decoded(previous);
                    }
                }
            }
            None => {
                self.nodes.insert(id, NodeRecord { attrs, handles: Vec::new() });
            }
        }
        id
    }

    pub fn contains_node(&self, id: NodeId) -> bool {
        self.nodes.contains_key(&id)
    }

    /// 🗑️ Removes a node, cascading: every incident edge is removed first, then (for ported storages) every handle anchored on it.
    pub fn remove_node(&mut self, id: NodeId) -> bool {
        if !self.nodes.contains_key(&id) {
            return false;
        }
        let mut incident: BTreeSet<EdgeId> = BTreeSet::new();
        if let Some(succ) = self.successors.get(&id) {
            for ids in succ.values() {
                incident.extend(ids.iter().copied());
            }
        }
        if let Some(pred) = self.predecessors.get(&id) {
            for ids in pred.values() {
                incident.extend(ids.iter().copied());
            }
        }
        for edge_id in incident {
            // 🚨️ was a dropped-future no-op: incident edges were never actually removed when
            // their node was — `remove_edge`'s side effects on `self.edges`/adjacency never ran.
            self.remove_edge(edge_id);
        }
        if let Some(record) = self.nodes.remove(&id) {
            for handle_id in record.handles {
                self.handle_owner.remove(&handle_id);
            }
        }
        self.successors.remove(&id);
        self.predecessors.remove(&id);
        true
    }

    pub fn node_attrs_mut(&mut self, id: NodeId) -> Option<&mut PropertyBag> {
        self.nodes.get_mut(&id).map(|r| &mut r.attrs)
    }
    // #endsubregion

    // #subregion Edges
    pub fn add_edge(&mut self, source: P::Endpoint, target: P::Endpoint) -> EdgeId {
        self.add_edge_with(source, target, PropertyBag::new())
    }

    /// 🔀️ `Normal` storages upsert: an edge already connecting this pair gets `attrs` merged into it and its existing id returned (NetworkX `Graph`/`DiGraph`). `Ported` storages always create a fresh parallel edge with a new `EdgeId` (NetworkX `MultiGraph`/`MultiDiGraph`).
    pub fn add_edge_with(&mut self, source: P::Endpoint, target: P::Endpoint, attrs: PropertyBag) -> EdgeId {
        let (un, vn) = (self.endpoint_node(source), self.endpoint_node(target));
        if !P::MULTI_EDGES {
            if let Some(&existing) = self.successors.get(&un).and_then(|m| m.get(&vn)).and_then(|ids| ids.first()) {
                if let Some(record) = self.edges.get_mut(&existing) {
                    for (key, value) in attrs {
                        if let Some(previous) = record.attrs.insert(key, value) {
                            <PropertyValue as semio_framework_value::FromValue>::retire_decoded(previous);
                        }
                    }
                }
                return existing;
            }
        }
        let id = self.next_edge_id;
        self.next_edge_id += 1;
        self.edges.insert(id, EdgeRecord { source, target, attrs });
        // 🚨️ was a dropped-future no-op: `link_adjacency` (mutates `successors`/`predecessors`)
        // was never awaited, so every edge added through this path was silently absent from
        // adjacency — traversal/neighbor/degree queries would all have missed it. See R10 header.
        self.link_adjacency(un, vn, id);
        id
    }

    pub fn remove_edge(&mut self, id: EdgeId) -> bool {
        let Some(record) = self.edges.remove(&id) else { return false };
        let (u, v) = (self.endpoint_node(record.source), self.endpoint_node(record.target));
        // 🚨️ was a dropped-future no-op: `unlink_adjacency` was never awaited, so a removed edge's
        // adjacency entries were silently left in place. Same class as `add_edge_with` above.
        self.unlink_adjacency(u, v, id);
        true
    }

    pub fn edge_attrs_mut(&mut self, id: EdgeId) -> Option<&mut PropertyBag> {
        self.edges.get_mut(&id).map(|r| &mut r.attrs)
    }

    pub fn edge_endpoints(&self, id: EdgeId) -> Option<(P::Endpoint, P::Endpoint)> {
        self.edges.get(&id).map(|r| (r.source, r.target))
    }
    // #endsubregion

    // #subregion Handles
    /// 🪝️ Allocates a new handle anchored on `node`; only meaningful when `P::HAS_PORTS` — returns `None` otherwise (or if `node` doesn't exist), never panics.
    pub fn add_handle(&mut self, node: NodeId) -> Option<HandleId> {
        if !P::HAS_PORTS || !self.nodes.contains_key(&node) {
            return None;
        }
        let id = self.next_handle_id;
        self.next_handle_id += 1;
        self.handle_owner.insert(id, node);
        self.nodes.get_mut(&node).expect("presence checked above").handles.push(id);
        Some(id)
    }

    pub fn handles(&self, node: NodeId) -> &[HandleId] {
        self.nodes.get(&node).map_or(&[], |r| r.handles.as_slice())
    }

    pub fn handle_owner(&self, handle: HandleId) -> Option<NodeId> {
        self.handle_owner.get(&handle).copied()
    }
    // #endsubregion

    // #subregion Whole graph
    pub fn graph_attrs_mut(&mut self) -> &mut PropertyBag {
        &mut self.graph_attrs
    }

    /// 🧹️ Removes every node, edge, and handle; graph-level attrs are cleared too. Id allocators are NOT reset — ids are never reused, even across a clear.
    pub fn clear(&mut self) {
        self.nodes.clear();
        self.edges.clear();
        self.successors.clear();
        self.predecessors.clear();
        self.handle_owner.clear();
        <PropertyBag as semio_framework_value::FromValue>::retire_decoded(std::mem::take(&mut self.graph_attrs));
    }

    /// 🧹️ Removes every edge but keeps nodes (and their handles) and graph-level attrs.
    pub fn clear_edges(&mut self) {
        self.edges.clear();
        for adj in self.successors.values_mut() {
            adj.clear();
        }
        for adj in self.predecessors.values_mut() {
            adj.clear();
        }
    }
    // #endsubregion
}
// #endregion 🔖️Storage

// #region 🔖️View traits
/// 🪢️ Node-level edge reference; carries its own id plus both endpoint node ids. Port/handle detail is already resolved away — algorithms never see a `HandleId`.
// 🧬️ `value_derive::{ToValue, FromValue}` additive (RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS,
// 26/09/01) — never had `serde`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue)]
pub struct EdgeRef {
    pub id: EdgeId,
    pub u: NodeId,
    pub v: NodeId,
}

/// 🪟️ Structural read-only view every future algorithm crate is written against — the single most important contract in this campaign; keep it minimal and stable.
pub trait GraphView {
    fn node_count(&self) -> usize;
    fn nodes(&self) -> impl Iterator<Item = NodeId>;
    fn contains_node(&self, node: NodeId) -> bool;
    fn edge_count(&self) -> usize;
    fn edges(&self) -> impl Iterator<Item = EdgeRef>;
    fn neighbors(&self, node: NodeId) -> impl Iterator<Item = NodeId>;
    fn out_neighbors(&self, node: NodeId) -> impl Iterator<Item = NodeId>;
    /// ⬅️ Equals `out_neighbors` on an undirected view — there is only one adjacency direction, so predecessors and successors coincide.
    fn in_neighbors(&self, node: NodeId) -> impl Iterator<Item = NodeId>;
    fn degree(&self, node: NodeId) -> usize;
    fn out_degree(&self, node: NodeId) -> usize;
    /// ⬅️ Equals `out_degree` on an undirected view, for the same reason as `in_neighbors`.
    fn in_degree(&self, node: NodeId) -> usize;
    fn is_directed(&self) -> bool;
    fn is_multigraph(&self) -> bool;
    fn edges_between(&self, u: NodeId, v: NodeId) -> impl Iterator<Item = EdgeRef>;
}

/// 🏷️ Attribute lookup companion to `GraphView`.
pub trait AttrView {
    fn node_attrs(&self, node: NodeId) -> Option<&PropertyBag>;
    fn edge_attrs(&self, edge: EdgeId) -> Option<&PropertyBag>;
    fn graph_attrs(&self) -> &PropertyBag;
}

/// ⚖️ Edge weight lookup, decoupled from attribute storage so algorithms take `impl EdgeWeights` instead of hardcoding a `"weight"` key.
pub trait EdgeWeights {
    fn weight(&self, edge: EdgeRef) -> f64;
}

/// 1⃣ Unweighted default: every edge costs `1.0` (NetworkX's unweighted-graph convention).
// 🧬️ Hand-written ToValue/FromValue (RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS,
// 26/09/01), not derived — see `Directed`'s note (⚙️engine top) on why a semicolon-terminated unit
// struct can't use `#[derive(ToValue, FromValue)]`.
#[derive(Clone, Copy, Debug, Default)]
pub struct UnitWeight;

impl dsl_core::ToValue for UnitWeight {
    fn to_value(&self) -> dsl_core::DslValue {
        dsl_core::DslValue::Null
    }
}
impl dsl_core::FromValue for UnitWeight {
    fn from_value(_value: dsl_core::DslValue) -> Result<Self, dsl_core::ValueError> {
        Ok(UnitWeight)
    }
}

impl EdgeWeights for UnitWeight {
    fn weight(&self, _edge: EdgeRef) -> f64 {
        1.0
    }
}

/// 🏷️ Reads a named numeric attribute off any `AttrView`, falling back to `default` when the attribute is missing or non-numeric (NetworkX's named-weight-with-default convention, e.g. `weight="cost"`).
pub struct AttrWeight<'g, G> {
    pub graph: &'g G,
    pub name: &'g str,
    pub default: f64,
}

impl<'g, G: AttrView> EdgeWeights for AttrWeight<'g, G> {
    fn weight(&self, edge: EdgeRef) -> f64 {
        self.graph.edge_attrs(edge.id).and_then(|attrs| attrs.get(self.name)).and_then(PropertyValue::as_f64).unwrap_or(self.default)
    }
}

impl<F: Fn(EdgeRef) -> f64> EdgeWeights for F {
    fn weight(&self, edge: EdgeRef) -> f64 {
        self(edge)
    }
}

impl<P: PortModel, D: Directedness> GraphView for Storage<P, D> {
    fn node_count(&self) -> usize {
        self.nodes.len()
    }
    fn nodes(&self) -> impl Iterator<Item = NodeId> {
        self.nodes.keys().copied()
    }
    fn contains_node(&self, node: NodeId) -> bool {
        self.nodes.contains_key(&node)
    }
    fn edge_count(&self) -> usize {
        self.edges.len()
    }
    /// 📇️ One `EdgeRef` per stored edge, in `EdgeId` order — a self-loop appears once here even though it counts twice towards `degree`.
    fn edges(&self) -> impl Iterator<Item = EdgeRef> {
        // 🔀️ Rewritten from `.map(..)` — `endpoint_node` is async and cannot be called inside the
        // sync closure that used to build each `EdgeRef` (R10 residue shape #1).
        let mut out = Vec::with_capacity(self.edges.len());
        for (&id, record) in &self.edges {
            let u = self.endpoint_node(record.source);
            let v = self.endpoint_node(record.target);
            out.push(EdgeRef { id, u, v });
        }
        out.into_iter()
    }
    fn neighbors(&self, node: NodeId) -> impl Iterator<Item = NodeId> {
        self.out_neighbors(node)
    }
    fn out_neighbors(&self, node: NodeId) -> impl Iterator<Item = NodeId> {
        self.successors.get(&node).into_iter().flat_map(|m| m.keys().copied())
    }
    fn in_neighbors(&self, node: NodeId) -> impl Iterator<Item = NodeId> {
        let map = if D::DIRECTED { &self.predecessors } else { &self.successors };
        map.get(&node).into_iter().flat_map(|m| m.keys().copied())
    }
    fn degree(&self, node: NodeId) -> usize {
        if D::DIRECTED {
            self.out_degree(node) + self.in_degree(node)
        } else {
            self.out_degree(node)
        }
    }
    fn out_degree(&self, node: NodeId) -> usize {
        self.successors.get(&node).map_or(0, |m| m.values().map(Vec::len).sum())
    }
    fn in_degree(&self, node: NodeId) -> usize {
        if D::DIRECTED {
            self.predecessors.get(&node).map_or(0, |m| m.values().map(Vec::len).sum())
        } else {
            self.out_degree(node)
        }
    }
    fn is_directed(&self) -> bool {
        D::DIRECTED
    }
    fn is_multigraph(&self) -> bool {
        P::MULTI_EDGES
    }
    fn edges_between(&self, u: NodeId, v: NodeId) -> impl Iterator<Item = EdgeRef> {
        self.successors.get(&u).and_then(|m| m.get(&v)).into_iter().flatten().copied().map(move |id| EdgeRef { id, u, v })
    }
}

impl<P: PortModel, D: Directedness> AttrView for Storage<P, D> {
    fn node_attrs(&self, node: NodeId) -> Option<&PropertyBag> {
        self.nodes.get(&node).map(|r| &r.attrs)
    }
    fn edge_attrs(&self, edge: EdgeId) -> Option<&PropertyBag> {
        self.edges.get(&edge).map(|r| &r.attrs)
    }
    fn graph_attrs(&self) -> &PropertyBag {
        &self.graph_attrs
    }
}

/// ⚖️ Reads the graph's own `PropertyBag["weight"]` on each edge, defaulting to `1.0` — the common case; use `AttrWeight`/`UnitWeight`/a closure for anything else.
impl<P: PortModel, D: Directedness> EdgeWeights for Storage<P, D> {
    fn weight(&self, edge: EdgeRef) -> f64 {
        self.edge_attrs(edge.id).and_then(|attrs| attrs.get("weight")).and_then(PropertyValue::as_f64).unwrap_or(1.0)
    }
}
// #endregion 🔖️View traits

// #region 🔖️Csr
/// 🌉️ `Csr::node_index` bridge — `BTreeMap<NodeId, usize>` is `u64`-keyed, see `storage_nodes_bridge` above.
mod csr_node_index_bridge {
    pub fn to_value(map: &std::collections::BTreeMap<super::NodeId, usize>) -> dsl_core::DslValue {
        dsl_core::DslValue::object(map.iter().map(|(k, v)| (k.to_string(), dsl_core::ToValue::to_value(v))))
    }
    pub fn from_value(value: dsl_core::DslValue) -> Result<std::collections::BTreeMap<super::NodeId, usize>, dsl_core::ValueError> {
        let dsl_core::DslValue::Object(entries) = value else {
            return Err(dsl_core::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "expected an object for Csr::node_index"));
        };
        entries
            .into_iter()
            .map(|(k, v)| {
                let id: super::NodeId = k.parse().map_err(|_| dsl_core::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("invalid NodeId key `{k}`")))?;
                let index = <usize as dsl_core::FromValue>::from_value(v).map_err(|e| e.under(&k))?;
                Ok((id, index))
            })
            .collect()
    }
    use crate::dsl_core;
}

/// 🧊️ Frozen, index-based CSR adjacency snapshot for hot algorithms; supersedes the ad-hoc `algorithms::Adjacency` for NEW code (that type is left untouched — old call sites keep using it). Node index assignment is `0..n` in sorted `NodeId` order, so two snapshots of the same graph always assign the same indices.
// 🧬️ `value_derive::{ToValue, FromValue}` additive (RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS,
// 26/09/01) — never had `serde`.
#[derive(Clone, Debug, value_derive::ToValue, value_derive::FromValue)]
pub struct Csr {
    node_ids: Vec<NodeId>,
    #[value(with = "csr_node_index_bridge")]
    node_index: BTreeMap<NodeId, usize>,
    out_starts: Vec<usize>,
    out_targets: Vec<usize>,
    out_edge_ids: Vec<EdgeId>,
    in_starts: Vec<usize>,
    in_targets: Vec<usize>,
}

impl Csr {
    /// 🏗️ Builds a CSR snapshot from any `GraphView`; each node's out-neighbor slot is sorted by `(target index, edge id)` for determinism under parallel edges. `in_neighbors` is populated only for directed views (empty slots otherwise).
    pub fn from_view(view: &impl GraphView) -> Self {
        let mut node_ids: Vec<NodeId> = view.nodes().collect();
        node_ids.sort_unstable();
        let node_index: BTreeMap<NodeId, usize> = node_ids.iter().enumerate().map(|(i, &id)| (id, i)).collect();
        let n = node_ids.len();
        let directed = view.is_directed();

        let mut out_buckets: Vec<Vec<(usize, EdgeId)>> = vec![Vec::new(); n];
        let mut in_buckets: Vec<Vec<usize>> = vec![Vec::new(); n];
        for edge in view.edges() {
            let (Some(&ui), Some(&vi)) = (node_index.get(&edge.u), node_index.get(&edge.v)) else {
                continue;
            };
            out_buckets[ui].push((vi, edge.id));
            if directed {
                in_buckets[vi].push(ui);
            } else if ui != vi {
                out_buckets[vi].push((ui, edge.id));
            }
        }
        for bucket in &mut out_buckets {
            bucket.sort_unstable();
        }
        for bucket in &mut in_buckets {
            bucket.sort_unstable();
        }

        let mut out_starts = Vec::with_capacity(n + 1);
        let mut out_targets = Vec::new();
        let mut out_edge_ids = Vec::new();
        out_starts.push(0);
        for bucket in &out_buckets {
            for &(target, edge_id) in bucket {
                out_targets.push(target);
                out_edge_ids.push(edge_id);
            }
            out_starts.push(out_targets.len());
        }

        let mut in_starts = Vec::with_capacity(n + 1);
        let mut in_targets = Vec::new();
        in_starts.push(0);
        for bucket in &in_buckets {
            in_targets.extend(bucket.iter().copied());
            in_starts.push(in_targets.len());
        }

        Self { node_ids, node_index, out_starts, out_targets, out_edge_ids, in_starts, in_targets }
    }

    pub fn node_count(&self) -> usize {
        self.node_ids.len()
    }

    pub fn out_neighbors(&self, i: usize) -> &[usize] {
        &self.out_targets[self.out_starts[i]..self.out_starts[i + 1]]
    }

    pub fn in_neighbors(&self, i: usize) -> &[usize] {
        &self.in_targets[self.in_starts[i]..self.in_starts[i + 1]]
    }

    pub fn out_edges(&self, i: usize) -> &[EdgeId] {
        &self.out_edge_ids[self.out_starts[i]..self.out_starts[i + 1]]
    }

    pub fn node_of(&self, i: usize) -> Option<NodeId> {
        self.node_ids.get(i).copied()
    }

    pub fn index_of(&self, id: NodeId) -> Option<usize> {
        self.node_index.get(&id).copied()
    }
}
// #endregion 🔖️Csr

// #region 🔖️Views
/// 🪟️ Read-only borrowed views over any `GraphView`. Deliberately excluded: NetworkX's mutable attribute-sharing views (`G.subgraph()` et al. alias the parent's attribute dicts) — that aliasing pattern doesn't fit Rust ownership, so every view here only ever borrows. Callers who need an owned, mutated copy build one explicitly (a `.copy()`-style constructor lives on the per-kind facade crates from a later wave); these types just leave that seam open.
///
/// 🔎️ Restricts a graph to a node subset; an edge is included only when both endpoints are in the subset.
pub struct SubgraphView<'g, G: GraphView> {
    graph: &'g G,
    nodes: BTreeSet<NodeId>,
}

impl<'g, G: GraphView> SubgraphView<'g, G> {
    pub fn new(graph: &'g G, nodes: impl IntoIterator<Item = NodeId>) -> Self {
        let mut kept = BTreeSet::new();
        for n in nodes {
            if graph.contains_node(n) {
                kept.insert(n);
            }
        }
        Self { graph, nodes: kept }
    }
}

impl<'g, G: GraphView> GraphView for SubgraphView<'g, G> {
    fn node_count(&self) -> usize {
        self.nodes.len()
    }
    fn nodes(&self) -> impl Iterator<Item = NodeId> {
        self.nodes.iter().copied()
    }
    fn contains_node(&self, node: NodeId) -> bool {
        self.nodes.contains(&node)
    }
    fn edge_count(&self) -> usize {
        self.edges().count()
    }
    fn edges(&self) -> impl Iterator<Item = EdgeRef> {
        self.graph.edges().filter(|e| self.nodes.contains(&e.u) && self.nodes.contains(&e.v))
    }
    fn neighbors(&self, node: NodeId) -> impl Iterator<Item = NodeId> {
        self.graph.neighbors(node).filter(|n| self.nodes.contains(n))
    }
    fn out_neighbors(&self, node: NodeId) -> impl Iterator<Item = NodeId> {
        self.graph.out_neighbors(node).filter(|n| self.nodes.contains(n))
    }
    fn in_neighbors(&self, node: NodeId) -> impl Iterator<Item = NodeId> {
        self.graph.in_neighbors(node).filter(|n| self.nodes.contains(n))
    }
    fn degree(&self, node: NodeId) -> usize {
        if self.graph.is_directed() {
            self.out_degree(node) + self.in_degree(node)
        } else {
            self.out_degree(node)
        }
    }
    fn out_degree(&self, node: NodeId) -> usize {
        let mut total = 0usize;
        for nb in self.out_neighbors(node) {
            total += self.edges_between(node, nb).count();
        }
        total
    }
    fn in_degree(&self, node: NodeId) -> usize {
        if self.graph.is_directed() {
            let mut total = 0usize;
            for nb in self.in_neighbors(node) {
                total += self.edges_between(nb, node).count();
            }
            total
        } else {
            self.out_degree(node)
        }
    }
    fn is_directed(&self) -> bool {
        self.graph.is_directed()
    }
    fn is_multigraph(&self) -> bool {
        self.graph.is_multigraph()
    }
    fn edges_between(&self, u: NodeId, v: NodeId) -> impl Iterator<Item = EdgeRef> {
        let keep = self.nodes.contains(&u) && self.nodes.contains(&v);
        self.graph.edges_between(u, v).filter(move |_| keep)
    }
}

impl<'g, G: GraphView + AttrView> AttrView for SubgraphView<'g, G> {
    fn node_attrs(&self, node: NodeId) -> Option<&PropertyBag> {
        if self.nodes.contains(&node) {
            self.graph.node_attrs(node)
        } else {
            None
        }
    }
    fn edge_attrs(&self, edge: EdgeId) -> Option<&PropertyBag> {
        self.graph.edge_attrs(edge)
    }
    fn graph_attrs(&self) -> &PropertyBag {
        self.graph.graph_attrs()
    }
}

/// 🔎️ Restricts a graph to an edge subset; nodes are exactly the endpoints of the included edges.
pub struct EdgeSubgraphView<'g, G: GraphView> {
    graph: &'g G,
    edges: BTreeSet<EdgeId>,
    nodes: BTreeSet<NodeId>,
}

impl<'g, G: GraphView> EdgeSubgraphView<'g, G> {
    pub fn new(graph: &'g G, edges: impl IntoIterator<Item = EdgeId>) -> Self {
        let edge_set: BTreeSet<EdgeId> = edges.into_iter().collect();
        let mut nodes = BTreeSet::new();
        for e in graph.edges() {
            if edge_set.contains(&e.id) {
                nodes.insert(e.u);
                nodes.insert(e.v);
            }
        }
        Self { graph, edges: edge_set, nodes }
    }
}

impl<'g, G: GraphView> GraphView for EdgeSubgraphView<'g, G> {
    fn node_count(&self) -> usize {
        self.nodes.len()
    }
    fn nodes(&self) -> impl Iterator<Item = NodeId> {
        self.nodes.iter().copied()
    }
    fn contains_node(&self, node: NodeId) -> bool {
        self.nodes.contains(&node)
    }
    fn edge_count(&self) -> usize {
        self.edges.len()
    }
    fn edges(&self) -> impl Iterator<Item = EdgeRef> {
        self.graph.edges().filter(|e| self.edges.contains(&e.id))
    }
    fn neighbors(&self, node: NodeId) -> impl Iterator<Item = NodeId> {
        self.out_neighbors(node)
    }
    fn out_neighbors(&self, node: NodeId) -> impl Iterator<Item = NodeId> {
        let directed = self.graph.is_directed();
        self.edges()
            .filter_map(move |e| {
                if e.u == node {
                    Some(e.v)
                } else if !directed && e.v == node {
                    Some(e.u)
                } else {
                    None
                }
            })
            .collect::<BTreeSet<_>>()
            .into_iter()
    }
    fn in_neighbors(&self, node: NodeId) -> impl Iterator<Item = NodeId> {
        if self.graph.is_directed() {
            self.edges().filter_map(move |e| if e.v == node { Some(e.u) } else { None }).collect::<BTreeSet<_>>().into_iter()
        } else {
            self.out_neighbors(node).collect::<BTreeSet<_>>().into_iter()
        }
    }
    fn degree(&self, node: NodeId) -> usize {
        if self.graph.is_directed() {
            self.out_degree(node) + self.in_degree(node)
        } else {
            self.out_degree(node)
        }
    }
    fn out_degree(&self, node: NodeId) -> usize {
        let mut total = 0usize;
        for nb in self.out_neighbors(node) {
            total += self.edges_between(node, nb).count();
        }
        total
    }
    fn in_degree(&self, node: NodeId) -> usize {
        if self.graph.is_directed() {
            let mut total = 0usize;
            for nb in self.in_neighbors(node) {
                total += self.edges_between(nb, node).count();
            }
            total
        } else {
            self.out_degree(node)
        }
    }
    fn is_directed(&self) -> bool {
        self.graph.is_directed()
    }
    fn is_multigraph(&self) -> bool {
        self.graph.is_multigraph()
    }
    fn edges_between(&self, u: NodeId, v: NodeId) -> impl Iterator<Item = EdgeRef> {
        self.graph.edges_between(u, v).filter(|e| self.edges.contains(&e.id))
    }
}

impl<'g, G: GraphView + AttrView> AttrView for EdgeSubgraphView<'g, G> {
    fn node_attrs(&self, node: NodeId) -> Option<&PropertyBag> {
        if self.nodes.contains(&node) {
            self.graph.node_attrs(node)
        } else {
            None
        }
    }
    fn edge_attrs(&self, edge: EdgeId) -> Option<&PropertyBag> {
        if self.edges.contains(&edge) {
            self.graph.edge_attrs(edge)
        } else {
            None
        }
    }
    fn graph_attrs(&self) -> &PropertyBag {
        self.graph.graph_attrs()
    }
}

/// ↩️ Swaps successors and predecessors; only meaningful when the wrapped view is directed — on an undirected view this is a documented no-operation (not a panic), since successors already equal predecessors there.
pub struct ReversedView<'g, G: GraphView> {
    graph: &'g G,
}

impl<'g, G: GraphView> ReversedView<'g, G> {
    pub fn new(graph: &'g G) -> Self {
        Self { graph }
    }
}

impl<'g, G: GraphView> GraphView for ReversedView<'g, G> {
    fn node_count(&self) -> usize {
        self.graph.node_count()
    }
    fn nodes(&self) -> impl Iterator<Item = NodeId> {
        self.graph.nodes()
    }
    fn contains_node(&self, node: NodeId) -> bool {
        self.graph.contains_node(node)
    }
    fn edge_count(&self) -> usize {
        self.graph.edge_count()
    }
    fn edges(&self) -> impl Iterator<Item = EdgeRef> {
        self.graph.edges().map(|e| EdgeRef { id: e.id, u: e.v, v: e.u })
    }
    fn neighbors(&self, node: NodeId) -> impl Iterator<Item = NodeId> {
        self.out_neighbors(node)
    }
    fn out_neighbors(&self, node: NodeId) -> impl Iterator<Item = NodeId> {
        self.graph.in_neighbors(node)
    }
    fn in_neighbors(&self, node: NodeId) -> impl Iterator<Item = NodeId> {
        self.graph.out_neighbors(node)
    }
    fn degree(&self, node: NodeId) -> usize {
        self.graph.degree(node)
    }
    fn out_degree(&self, node: NodeId) -> usize {
        self.graph.in_degree(node)
    }
    fn in_degree(&self, node: NodeId) -> usize {
        self.graph.out_degree(node)
    }
    fn is_directed(&self) -> bool {
        self.graph.is_directed()
    }
    fn is_multigraph(&self) -> bool {
        self.graph.is_multigraph()
    }
    fn edges_between(&self, u: NodeId, v: NodeId) -> impl Iterator<Item = EdgeRef> {
        self.graph.edges_between(v, u).map(|e| EdgeRef { id: e.id, u: e.v, v: e.u })
    }
}

impl<'g, G: GraphView + AttrView> AttrView for ReversedView<'g, G> {
    fn node_attrs(&self, node: NodeId) -> Option<&PropertyBag> {
        self.graph.node_attrs(node)
    }
    fn edge_attrs(&self, edge: EdgeId) -> Option<&PropertyBag> {
        self.graph.edge_attrs(edge)
    }
    fn graph_attrs(&self) -> &PropertyBag {
        self.graph.graph_attrs()
    }
}

/// 🎛️ NetworkX `restricted_view`/`hide_nodes`/`hide_edges` equivalent: predicates return `true` to KEEP an element, so a "hide" caller just inverts its predicate.
pub struct FilteredView<'g, G: GraphView, FN, FE> {
    graph: &'g G,
    keep_node: FN,
    keep_edge: FE,
}

impl<'g, G: GraphView, FN: Fn(NodeId) -> bool, FE: Fn(EdgeRef) -> bool> FilteredView<'g, G, FN, FE> {
    pub fn new(graph: &'g G, keep_node: FN, keep_edge: FE) -> Self {
        Self { graph, keep_node, keep_edge }
    }

    fn keep(&self, edge: EdgeRef) -> bool {
        (self.keep_node)(edge.u) && (self.keep_node)(edge.v) && (self.keep_edge)(edge)
    }
}

impl<'g, G: GraphView, FN: Fn(NodeId) -> bool, FE: Fn(EdgeRef) -> bool> GraphView for FilteredView<'g, G, FN, FE> {
    fn node_count(&self) -> usize {
        self.nodes().count()
    }
    fn nodes(&self) -> impl Iterator<Item = NodeId> {
        self.graph.nodes().filter(|&n| (self.keep_node)(n))
    }
    fn contains_node(&self, node: NodeId) -> bool {
        self.graph.contains_node(node) && (self.keep_node)(node)
    }
    fn edge_count(&self) -> usize {
        self.edges().count()
    }
    fn edges(&self) -> impl Iterator<Item = EdgeRef> {
        // 🔀️ Rewritten from `.filter(move |&e| self.keep(e))` — `keep` is async and cannot be
        // called inside a sync closure (R10 residue shape #1).
        let mut out = Vec::new();
        for e in self.graph.edges() {
            if self.keep(e) {
                out.push(e);
            }
        }
        out.into_iter()
    }
    fn neighbors(&self, node: NodeId) -> impl Iterator<Item = NodeId> {
        self.out_neighbors(node)
    }
    fn out_neighbors(&self, node: NodeId) -> impl Iterator<Item = NodeId> {
        // 🔀️ Rewritten — `edges_between` is async and cannot be called inside the sync `.filter`
        // predicate that used to guard this (R10 residue shape #1).
        let node_ok = (self.keep_node)(node);
        let mut out = Vec::new();
        if node_ok {
            for nb in self.graph.out_neighbors(node) {
                if self.edges_between(node, nb).next().is_some() {
                    out.push(nb);
                }
            }
        }
        out.into_iter()
    }
    fn in_neighbors(&self, node: NodeId) -> impl Iterator<Item = NodeId> {
        let node_ok = (self.keep_node)(node);
        let mut out = Vec::new();
        if node_ok {
            for nb in self.graph.in_neighbors(node) {
                if self.edges_between(nb, node).next().is_some() {
                    out.push(nb);
                }
            }
        }
        out.into_iter()
    }
    fn degree(&self, node: NodeId) -> usize {
        if self.graph.is_directed() {
            self.out_degree(node) + self.in_degree(node)
        } else {
            self.out_degree(node)
        }
    }
    fn out_degree(&self, node: NodeId) -> usize {
        let mut total = 0usize;
        for nb in self.out_neighbors(node) {
            total += self.edges_between(node, nb).count();
        }
        total
    }
    fn in_degree(&self, node: NodeId) -> usize {
        if self.graph.is_directed() {
            let mut total = 0usize;
            for nb in self.in_neighbors(node) {
                total += self.edges_between(nb, node).count();
            }
            total
        } else {
            self.out_degree(node)
        }
    }
    fn is_directed(&self) -> bool {
        self.graph.is_directed()
    }
    fn is_multigraph(&self) -> bool {
        self.graph.is_multigraph()
    }
    fn edges_between(&self, u: NodeId, v: NodeId) -> impl Iterator<Item = EdgeRef> {
        let keep_u = (self.keep_node)(u);
        let keep_v = (self.keep_node)(v);
        self.graph.edges_between(u, v).filter(move |&e| keep_u && keep_v && (self.keep_edge)(e))
    }
}

impl<'g, G: GraphView + AttrView, FN: Fn(NodeId) -> bool, FE: Fn(EdgeRef) -> bool> AttrView for FilteredView<'g, G, FN, FE> {
    fn node_attrs(&self, node: NodeId) -> Option<&PropertyBag> {
        if (self.keep_node)(node) {
            self.graph.node_attrs(node)
        } else {
            None
        }
    }
    fn edge_attrs(&self, edge: EdgeId) -> Option<&PropertyBag> {
        self.graph.edge_attrs(edge)
    }
    fn graph_attrs(&self) -> &PropertyBag {
        self.graph.graph_attrs()
    }
}

/// 🔀️ Presents a directed graph's edges as undirected — merges successor and predecessor sets into one neighbor view without materializing storage. Querying `edges_between(u, u)` on a directed self-loop yields it twice, mirroring the same "self-loop counts twice" convention `Storage` applies natively to undirected adjacency.
pub struct UndirectedView<'g, G: GraphView> {
    graph: &'g G,
}

impl<'g, G: GraphView> UndirectedView<'g, G> {
    pub fn new(graph: &'g G) -> Self {
        Self { graph }
    }
}

impl<'g, G: GraphView> GraphView for UndirectedView<'g, G> {
    fn node_count(&self) -> usize {
        self.graph.node_count()
    }
    fn nodes(&self) -> impl Iterator<Item = NodeId> {
        self.graph.nodes()
    }
    fn contains_node(&self, node: NodeId) -> bool {
        self.graph.contains_node(node)
    }
    fn edge_count(&self) -> usize {
        self.graph.edge_count()
    }
    fn edges(&self) -> impl Iterator<Item = EdgeRef> {
        self.graph.edges().map(|e| if e.u <= e.v { e } else { EdgeRef { id: e.id, u: e.v, v: e.u } })
    }
    fn neighbors(&self, node: NodeId) -> impl Iterator<Item = NodeId> {
        self.graph.out_neighbors(node).chain(self.graph.in_neighbors(node)).collect::<BTreeSet<_>>().into_iter()
    }
    fn out_neighbors(&self, node: NodeId) -> impl Iterator<Item = NodeId> {
        self.neighbors(node)
    }
    fn in_neighbors(&self, node: NodeId) -> impl Iterator<Item = NodeId> {
        self.neighbors(node)
    }
    fn degree(&self, node: NodeId) -> usize {
        self.out_degree(node)
    }
    fn out_degree(&self, node: NodeId) -> usize {
        let mut total = 0usize;
        for nb in self.neighbors(node) {
            total += self.edges_between(node, nb).count();
        }
        total
    }
    fn in_degree(&self, node: NodeId) -> usize {
        self.out_degree(node)
    }
    fn is_directed(&self) -> bool {
        false
    }
    fn is_multigraph(&self) -> bool {
        self.graph.is_multigraph()
    }
    fn edges_between(&self, u: NodeId, v: NodeId) -> impl Iterator<Item = EdgeRef> {
        self.graph.edges_between(u, v).chain(self.graph.edges_between(v, u))
    }
}

impl<'g, G: GraphView + AttrView> AttrView for UndirectedView<'g, G> {
    fn node_attrs(&self, node: NodeId) -> Option<&PropertyBag> {
        self.graph.node_attrs(node)
    }
    fn edge_attrs(&self, edge: EdgeId) -> Option<&PropertyBag> {
        self.graph.edge_attrs(edge)
    }
    fn graph_attrs(&self) -> &PropertyBag {
        self.graph.graph_attrs()
    }
}
// #endregion 🔖️Views

// #region 🔖️Interner
/// 🔤️ Generalized, bidirectional label<->`NodeId` map — the generic successor to the string-only `algorithms::IdIndex` (which stays untouched for old call sites). `intern` is idempotent: the same label always maps to the same id.
// 🧬️ ToValue/FromValue coverage deliberately SKIPPED (RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS,
// 26/09/01): `L` is arbitrary (`Ord + Clone + Hash` only — the ONLY instantiation anywhere in the
// repo is `Interner<String>`, confined to this file's own `#[cfg(test)] mod tests`, never consumed
// by any other module). `by_label: HashMap<L, NodeId>` needs the `BTreeMap`/`HashMap`-blanket's
// `L: ToString` bound (`🌱️value/🔁️codec`), which the derive's auto-synthesized `L: ToValue +
// FromValue` bound does not provide — forcing a `#[value(bound = "L: ToString + FromStr + …")]`
// override would newly constrain every FUTURE `L`, an API change with zero present benefit for a
// generic type nothing outside its own tests instantiates.
#[derive(Clone, Debug, Default)]
pub struct Interner<L: Ord + Clone + std::hash::Hash> {
    labels: Vec<L>,
    by_label: std::collections::HashMap<L, NodeId>,
}

impl<L: Ord + Clone + std::hash::Hash> Interner<L> {
    pub fn new() -> Self {
        Self { labels: Vec::new(), by_label: std::collections::HashMap::new() }
    }

    /// 🏗️ Builds an interner from labels sorted for deterministic id assignment; duplicate labels collapse to one id.
    pub fn from_labels(labels: impl IntoIterator<Item = L>) -> Self {
        let mut sorted: Vec<L> = labels.into_iter().collect();
        sorted.sort();
        sorted.dedup();
        let mut interner = Self::new();
        for label in sorted {
            interner.intern(label);
        }
        interner
    }

    /// ➕️ Returns the existing id for `label` if already interned, otherwise allocates the next sequential id.
    pub fn intern(&mut self, label: L) -> NodeId {
        if let Some(&id) = self.by_label.get(&label) {
            return id;
        }
        let id = self.labels.len() as NodeId;
        self.labels.push(label.clone());
        self.by_label.insert(label, id);
        id
    }

    pub fn label_of(&self, id: NodeId) -> Option<&L> {
        self.labels.get(id as usize)
    }

    pub fn id_of(&self, label: &L) -> Option<NodeId> {
        self.by_label.get(label).copied()
    }

    pub fn len(&self) -> usize {
        self.labels.len()
    }

    pub fn is_empty(&self) -> bool {
        self.labels.is_empty()
    }
}
// #endregion 🔖️Interner

// #region 🔖️GraphError
/// 🚨️ Flat, non-generic error enum mirroring the NetworkX exception hierarchy; every downstream algorithm crate returns `Result<_, GraphError>`. Nothing here is generic over node/edge label types — everything is `NodeId`/`EdgeId`/`u64`/`String` — so this shape stays stable across the whole family.
// 🧬️ `value_derive::{ToValue, FromValue}` additive (RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS,
// 26/09/01) — never had `serde`, so no `#[value(...)]` rename is needed and the derive's default
// externally-tagged shape (unit variant -> bare wire-name string, data-carrying variant ->
// `{"VariantName": payload}`) is simply the new wire shape. `NotImplementedForKind`'s two fields
// moved `&'static str` -> `String`: `FromValue` needs an OWNED `Self`, and no runtime-decoded
// string can honestly become `&'static str` without leaking memory — a greenfield, no-legacy-API
// repo (CLAUDE.md) fixes the field type instead of working around it (both call sites already
// pass a string literal, which `.into()`s into `String` for free).
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
pub enum GraphError {
    NodeNotFound(NodeId),
    EdgeNotFound(EdgeId),
    NoPath { source: NodeId, target: NodeId },
    HasACycle,
    NoCycle,
    Unfeasible(String),
    Unbounded(String),
    NotATree,
    NotAForest,
    NotBipartite,
    NotPlanar,
    NotEulerian,
    NotConnected,
    NotStronglyConnected,
    AmbiguousSolution(String),
    ExceededMaxIterations { iterations: usize },
    PowerIterationFailedConvergence { iterations: usize },
    NegativeCycle,
    NotGraphical(String),
    NotImplementedForKind { algorithm: String, kind: String },
    Io(String),
    Parse { line: usize, message: String },
}

impl std::fmt::Display for GraphError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GraphError::NodeNotFound(id) => write!(f, "node {id} not found"),
            GraphError::EdgeNotFound(id) => write!(f, "edge {id} not found"),
            GraphError::NoPath { source, target } => write!(f, "no path from node {source} to node {target}"),
            GraphError::HasACycle => write!(f, "graph has a cycle"),
            GraphError::NoCycle => write!(f, "graph has no cycle"),
            GraphError::Unfeasible(msg) => write!(f, "unfeasible: {msg}"),
            GraphError::Unbounded(msg) => write!(f, "unbounded: {msg}"),
            GraphError::NotATree => write!(f, "graph is not a tree"),
            GraphError::NotAForest => write!(f, "graph is not a forest"),
            GraphError::NotBipartite => write!(f, "graph is not bipartite"),
            GraphError::NotPlanar => write!(f, "graph is not planar"),
            GraphError::NotEulerian => write!(f, "graph is not eulerian"),
            GraphError::NotConnected => write!(f, "graph is not connected"),
            GraphError::NotStronglyConnected => write!(f, "graph is not strongly connected"),
            GraphError::AmbiguousSolution(msg) => write!(f, "ambiguous solution: {msg}"),
            GraphError::ExceededMaxIterations { iterations } => write!(f, "exceeded max iterations ({iterations})"),
            GraphError::PowerIterationFailedConvergence { iterations } => {
                write!(f, "power iteration failed to converge after {iterations} iterations")
            }
            GraphError::NegativeCycle => write!(f, "graph has a negative cycle"),
            GraphError::NotGraphical(msg) => write!(f, "not a graphical degree sequence: {msg}"),
            GraphError::NotImplementedForKind { algorithm, kind } => write!(f, "{algorithm} is not implemented for {kind}"),
            GraphError::Io(msg) => write!(f, "io error: {msg}"),
            GraphError::Parse { line, message } => write!(f, "parse error at line {line}: {message}"),
        }
    }
}

impl std::error::Error for GraphError {}
// #endregion 🔖️GraphError

// #region 🔖️Utils
/// 🎚️ Strict numeric tolerance for exact-equality-sensitive comparisons (e.g. verifying a closed-form result).
pub const TOL_STRICT: f64 = 1e-9;
/// 🎚️ Loose numeric tolerance for iterative/approximate algorithm convergence checks.
pub const TOL_LOOSE: f64 = 1e-6;

/// 🔗️ Consecutive-pair iterator: `[a, b, c] -> [(a, b), (b, c)]`.
pub fn pairwise<T: Copy>(items: &[T]) -> impl Iterator<Item = (T, T)> + '_ {
    items.windows(2).map(|w| (w[0], w[1]))
}

/// 🎯️ Deterministic representative element (the first one) from a slice.
pub fn arbitrary_element<T: Copy>(items: &[T]) -> Option<T> {
    items.first().copied()
}

/// 🗳️ Binary-heap priority queue with `decrease_key`, ordered by `K` and keyed by `V` identity; a position index makes membership/decrease `O(log n)` instead of the `O(n)` a plain `BinaryHeap` needs for those operations.
// 🧬️ ToValue/FromValue coverage deliberately SKIPPED (RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS,
// 26/09/01), same rationale as `Interner` above: `K`/`V` are arbitrary, nothing outside this file's
// own `#[cfg(test)] mod tests` ever instantiates `MappedHeap` (there, `MappedHeap<i64, &str>` — a
// BORROWED `V` that structurally CANNOT implement `FromValue`, which needs an owned `Self`),
// `position: HashMap<V, usize>` needs `V: ToString` the auto bound does not provide, and no
// external consumer exists to benefit from the bound override this would force.
#[derive(Clone, Debug)]
pub struct MappedHeap<K: Ord, V: Eq + std::hash::Hash + Clone> {
    heap: Vec<(K, V)>,
    position: std::collections::HashMap<V, usize>,
}

impl<K: Ord, V: Eq + std::hash::Hash + Clone> Default for MappedHeap<K, V> {
    // 🚫️async: E1 impl of external trait `std::default::Default` — must stay sync; mirrors `new()`'s
    // literal (I/O-free) body directly. See R9, and the identical `Storage::default` fix above.
    fn default() -> Self {
        Self { heap: Vec::new(), position: std::collections::HashMap::new() }
    }
}

impl<K: Ord, V: Eq + std::hash::Hash + Clone> MappedHeap<K, V> {
    pub fn new() -> Self {
        Self { heap: Vec::new(), position: std::collections::HashMap::new() }
    }

    pub fn is_empty(&self) -> bool {
        self.heap.is_empty()
    }

    pub fn len(&self) -> usize {
        self.heap.len()
    }

    pub fn contains(&self, item: &V) -> bool {
        self.position.contains_key(item)
    }

    /// ➕️ Pushes `item` at `priority` if absent, or decreases its priority if `priority` is lower than its current one; no-operation if `item` is present with an already-lower-or-equal priority.
    pub fn push_or_decrease(&mut self, item: V, priority: K) {
        if let Some(&i) = self.position.get(&item) {
            if priority < self.heap[i].0 {
                self.heap[i].0 = priority;
                self.sift_up(i);
            }
        } else {
            self.heap.push((priority, item.clone()));
            let i = self.heap.len() - 1;
            self.position.insert(item, i);
            self.sift_up(i);
        }
    }

    /// 🔽️ Lowers `item`'s priority; returns `false` (no-operation) if `item` isn't present or `priority` isn't lower than its current one.
    pub fn decrease_key(&mut self, item: &V, priority: K) -> bool {
        let Some(&i) = self.position.get(item) else { return false };
        if priority < self.heap[i].0 {
            self.heap[i].0 = priority;
            self.sift_up(i);
            true
        } else {
            false
        }
    }

    pub fn pop_min(&mut self) -> Option<(K, V)> {
        if self.heap.is_empty() {
            return None;
        }
        let last = self.heap.len() - 1;
        self.swap(0, last);
        let (priority, item) = self.heap.pop().expect("heap checked non-empty above");
        self.position.remove(&item);
        if !self.heap.is_empty() {
            self.sift_down(0);
        }
        Some((priority, item))
    }

    fn swap(&mut self, i: usize, j: usize) {
        self.heap.swap(i, j);
        self.position.insert(self.heap[i].1.clone(), i);
        self.position.insert(self.heap[j].1.clone(), j);
    }

    // 🚨️ was a dropped-future no-op throughout: `sift_up`/`sift_down`/`swap` were never awaited at
    // ANY of their call sites in this struct (including here, sift_up/sift_down calling their own
    // `swap`), so the whole `MappedHeap` never actually maintained the heap invariant — every push,
    // decrease-key, and pop silently left `self.heap` in insertion order. See R10 header.
    fn sift_up(&mut self, mut i: usize) {
        while i > 0 {
            let parent = (i - 1) / 2;
            if self.heap[i].0 < self.heap[parent].0 {
                self.swap(i, parent);
                i = parent;
            } else {
                break;
            }
        }
    }

    fn sift_down(&mut self, mut i: usize) {
        let n = self.heap.len();
        loop {
            let l = 2 * i + 1;
            let r = 2 * i + 2;
            let mut smallest = i;
            if l < n && self.heap[l].0 < self.heap[smallest].0 {
                smallest = l;
            }
            if r < n && self.heap[r].0 < self.heap[smallest].0 {
                smallest = r;
            }
            if smallest == i {
                break;
            }
            self.swap(i, smallest);
            i = smallest;
        }
    }
}
// #endregion 🔖️Utils

// #region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
// #endregion 🔖️Tests

// #region 🔖️PropertyValue
/// 🧾️ Converts a fixture `userData` value into a typed property bag.
pub fn property_bag_from_value(value: &dsl_core::DslValue) -> PropertyBag {
    let dsl_core::DslValue::Object(entries) = value.clone() else {
        return PropertyBag::default();
    };
    entries.into_iter().filter_map(|(k, v)| dsl_core::FromValue::from_value(v).ok().map(|pv| (k, pv))).collect()
}

/// 🧾️ Serializes a property bag back to a value for fixture export.
pub fn property_bag_to_value(bag: &PropertyBag) -> Option<dsl_core::DslValue> {
    if bag.is_empty() {
        None
    } else {
        let entries: Vec<(String, dsl_core::DslValue)> = bag.iter().map(|(k, v)| (k.clone(), dsl_core::ToValue::to_value(v))).collect();
        Some(dsl_core::DslValue::Object(entries))
    }
}
// #endregion 🔖️PropertyValue

// #region 🔖️Kinds
use geometry::Point;

/// 🌉️ `geometry::Point` bridge (`📐️geometry` is a different owner's module — RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS,
/// 26/09/01 splits this ticket by module, so `Point` itself is out of scope here); hand-written
/// rather than a derive since we cannot add `#[derive(ToValue, FromValue)]` to `Point`'s own
/// definition. `Node::center`/`Handle` below name this via `#[value(with = "point_bridge")]`.
mod point_bridge {
    pub fn to_value(p: &super::Point) -> dsl_core::DslValue {
        dsl_core::DslValue::object([
            ("x".to_string(), dsl_core::ToValue::to_value(&p.x)),
            ("y".to_string(), dsl_core::ToValue::to_value(&p.y)),
        ])
    }
    pub fn from_value(value: dsl_core::DslValue) -> Result<super::Point, dsl_core::ValueError> {
        let dsl_core::DslValue::Object(entries) = value else {
            return Err(dsl_core::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "expected an object for Point"));
        };
        let x = entries.iter().find(|(k, _)| k == "x").map(|(_, v)| v.clone()).ok_or_else(|| dsl_core::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "missing field `x`"))?;
        let y = entries.iter().find(|(k, _)| k == "y").map(|(_, v)| v.clone()).ok_or_else(|| dsl_core::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "missing field `y`"))?;
        Ok(super::Point {
            x: <f64 as dsl_core::FromValue>::from_value(x).map_err(|e| e.under("x"))?,
            y: <f64 as dsl_core::FromValue>::from_value(y).map_err(|e| e.under("y"))?,
        })
    }
    use crate::dsl_core;
}

/// 🔵️ Circle or axis-aligned rectangle node body.
// 🧬️ `value_derive::{ToValue, FromValue}` additive (RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS,
// 26/09/01) — never had `serde`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue)]
pub enum NodeShape {
    #[default]
    Circle,
    Rectangle,
}

/// 🪝️ Port direction for directed edge wiring.
// 🧬️ `value_derive::{ToValue, FromValue}` additive, see `NodeShape` above.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue)]
pub enum HandleRole {
    Source,
    Target,
    #[default]
    Any,
}

/// 🏷️ Semantic kind and property payload shared by graph elements.
// 🧬️ `value_derive::{ToValue, FromValue}` additive, see `NodeShape` above; `PropertyBag` (`🛂️manifest`) already covered.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
pub struct ElementSemantics {
    pub kind: Option<String>,
    pub properties: PropertyBag,
}

/// 🟠️ Retained node state with world-space center and shape extents.
// 🧬️ `value_derive::{ToValue, FromValue}` additive, see `NodeShape` above; `center` bridges through
// `point_bridge` (`geometry::Point` is a different owner's module).
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
pub struct Node {
    pub id: NodeId,
    #[value(with = "point_bridge")]
    pub center: Point,
    pub radius: f64,
    pub width: f64,
    pub height: f64,
    pub shape: NodeShape,
    pub draggable: bool,
    pub kind: Option<String>,
    pub label: Option<String>,
    pub properties: PropertyBag,
}

/// 🟣️ Tangent handle anchored to a node at a polar angle.
// 🧬️ `value_derive::{ToValue, FromValue}` additive, see `Node` above.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
pub struct Handle {
    pub angle: f64,
    pub id: HandleId,
    pub node_id: NodeId,
    pub radius: f64,
    pub role: HandleRole,
    pub kind: Option<String>,
    /// 🔤️ Value schemas this port declares — an output lists what it may carry, an input what it
    /// accepts, and a board refuses a wire whose two declared sets are disjoint. Empty is undeclared,
    /// which stays connectable.
    pub value_types: Vec<String>,
    pub properties: PropertyBag,
}

/// 🪢️ Retained edge with typed endpoints.
pub type GraphEdge<E> = CoreEdge<E>;
// #endregion 🔖️Kinds

// #region 🔖️MaxFlow
/// 🎚️ Residual-capacity noise guard: capacities at or below this are treated as exhausted (fractional alpha-expansion graph-cut costs are not exact).
const FLOW_EPS: f64 = 1e-9;

/// 🌊️ Directed residual-graph edge; its paired reverse edge always lives at the adjacent arena slot (`id ^ 1`).
// 🧬️ `value_derive::{ToValue, FromValue}` additive (RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS,
// 26/09/01) — never had `serde`.
#[derive(Clone, Copy, Debug, value_derive::ToValue, value_derive::FromValue)]
struct FlowEdge {
    to: u32,
    capacity: f64,
}

/// 🌊️ Capacitated directed flow network on a `u32`-indexed arena, for [Dinic's algorithm](https://doi.org/10.1016/0898-1221(74)90074-0) (CLRS ch. 26). Edges are stored as forward/reverse residual pairs at adjacent slots so augmenting a path only ever touches two `Vec` entries; adjacency is `Vec<Vec<u32>>`, never a hash map, so traversal order — and therefore `min_cut`'s result — is fixed by construction order alone.
// 🧬️ `value_derive::{ToValue, FromValue}` additive, see `FlowEdge` above.
#[derive(Clone, Debug, value_derive::ToValue, value_derive::FromValue)]
pub struct FlowNetwork {
    node_count: u32,
    edges: Vec<FlowEdge>,
    adjacency: Vec<Vec<u32>>,
}

impl FlowNetwork {
    /// 🆕️ Empty network over nodes `0..node_count`, no edges yet.
    pub fn new(node_count: u32) -> Self {
        Self { node_count, edges: Vec::new(), adjacency: vec![Vec::new(); node_count as usize] }
    }

    /// ➕️ Adds a directed edge `from -> to` with `capacity`, plus a zero-capacity reverse residual edge; returns the forward edge's id (its reverse is always `id ^ 1`).
    pub fn add_edge(&mut self, from: u32, to: u32, capacity: f64) -> u32 {
        let forward_id = self.edges.len() as u32;
        self.edges.push(FlowEdge { to, capacity });
        self.adjacency[from as usize].push(forward_id);
        let reverse_id = self.edges.len() as u32;
        self.edges.push(FlowEdge { to: from, capacity: 0.0 });
        self.adjacency[to as usize].push(reverse_id);
        forward_id
    }

    /// 🌊️ BFS level graph from `source`, restricted to edges with residual capacity above `FLOW_EPS`; `None` marks nodes unreached this phase.
    fn bfs_levels(&self, source: u32) -> Vec<Option<u32>> {
        let mut level = vec![None; self.node_count as usize];
        level[source as usize] = Some(0);
        let mut queue = std::collections::VecDeque::new();
        queue.push_back(source);
        while let Some(u) = queue.pop_front() {
            let du = level[u as usize].expect("every queued node has a level assigned before being pushed");
            for &edge_id in &self.adjacency[u as usize] {
                let edge = self.edges[edge_id as usize];
                if edge.capacity > FLOW_EPS && level[edge.to as usize].is_none() {
                    level[edge.to as usize] = Some(du + 1);
                    queue.push_back(edge.to);
                }
            }
        }
        level
    }

    /// 🌊️ DFS blocking flow along the level graph; `cursor` is the current-arc optimization, skipping adjacency entries already exhausted within this blocking-flow phase.
    fn dfs_blocking_flow(&mut self, u: u32, sink: u32, pushed: f64, level: &[Option<u32>], cursor: &mut [usize]) -> f64 {
        if u == sink || pushed <= FLOW_EPS {
            return pushed;
        }
        while cursor[u as usize] < self.adjacency[u as usize].len() {
            let edge_id = self.adjacency[u as usize][cursor[u as usize]];
            let edge = self.edges[edge_id as usize];
            let advances = edge.capacity > FLOW_EPS && level[edge.to as usize] == level[u as usize].map(|l| l + 1);
            if advances {
                let sent = self.dfs_blocking_flow(edge.to, sink, pushed.min(edge.capacity), level, cursor);
                if sent > FLOW_EPS {
                    self.edges[edge_id as usize].capacity -= sent;
                    self.edges[(edge_id ^ 1) as usize].capacity += sent;
                    return sent;
                }
            }
            cursor[u as usize] += 1;
        }
        0.0
    }

    /// 🏔️ Dinic's max flow: alternates BFS level-graph construction with DFS blocking-flow phases (current-arc optimized) until `sink` is unreachable from `source` in the residual graph; returns the total flow value pushed. `source == sink` short-circuits to `0.0`.
    pub fn max_flow(&mut self, source: u32, sink: u32) -> f64 {
        if source == sink {
            return 0.0;
        }
        let mut total = 0.0;
        loop {
            // 🚨️ was a dropped-future no-op: `bfs_levels` was never awaited, so `max_flow` never
            // actually built a level graph — the whole Dinic's-algorithm loop was silently inert.
            let level = self.bfs_levels(source);
            if level[sink as usize].is_none() {
                break;
            }
            let mut cursor = vec![0usize; self.node_count as usize];
            loop {
                let pushed = self.dfs_blocking_flow(source, sink, f64::INFINITY, &level, &mut cursor);
                if pushed <= FLOW_EPS {
                    break;
                }
                total += pushed;
            }
        }
        total
    }

    /// ✂️ Source side of the minimum cut, valid only after `max_flow` has run: nodes reachable from `source` over edges whose residual capacity still exceeds `FLOW_EPS`, visited in ascending id order via `Vec`-backed BFS — fully deterministic.
    pub fn min_cut(&self, source: u32) -> Vec<u32> {
        let mut reachable = vec![false; self.node_count as usize];
        reachable[source as usize] = true;
        let mut queue = std::collections::VecDeque::new();
        queue.push_back(source);
        while let Some(u) = queue.pop_front() {
            for &edge_id in &self.adjacency[u as usize] {
                let edge = self.edges[edge_id as usize];
                if edge.capacity > FLOW_EPS && !reachable[edge.to as usize] {
                    reachable[edge.to as usize] = true;
                    queue.push_back(edge.to);
                }
            }
        }
        (0..self.node_count).filter(|&i| reachable[i as usize]).collect()
    }
}
// #endregion 🔖️MaxFlow

```

### 🧰️framework/🔨️modules/🕸️graph/⚙️engine/🧪️tests/🔬️unit/🦀️.rs

SHA-256 `7c72f1a55a79cbbf8f88867ce3b65a210a0e45a15247a98eec58f1a78fc7d4ee`; 36016 bytes.

```

use super::*;

/// 🧬️ Additive `#[derive(ToValue, FromValue)]` round-trip (RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS,
/// 26/09/01): `FromValue(ToValue(x)) == x`, covering plain derives, the hand-written
/// zero-field marker types (`Directed`/`Normal`/…), `GraphError::NotImplementedForKind`'s
/// `String` fields, the `geometry::Point` bridge (`Node`/`Handle`), and a POPULATED `Storage`
/// (so its four `u64`-keyed `BTreeMap` bridges — `nodes`/`edges`/`successors`/`handle_owner` —
/// are all actually exercised, not just compiled).
#[test]
fn value_round_trip_matches_serde_shape() {
    fn check<T: dsl_core::ToValue + dsl_core::FromValue + std::fmt::Debug + PartialEq>(value: T) {
        let round_tripped = <T as dsl_core::FromValue>::from_value(dsl_core::ToValue::to_value(&value)).expect("round-trip decode");
        assert_eq!(round_tripped, value);
    }

    // Zero-field marker types have no `PartialEq` (pre-existing), so their round-trip is just
    // "decodes without error and re-encodes identically" rather than `check`'s equality form.
    for encoded in [dsl_core::ToValue::to_value(&Directed), dsl_core::ToValue::to_value(&Undirected), dsl_core::ToValue::to_value(&Normal), dsl_core::ToValue::to_value(&Ported), dsl_core::ToValue::to_value(&UnitWeight)] {
        assert_eq!(encoded, dsl_core::DslValue::Null);
    }
    <Directed as dsl_core::FromValue>::from_value(dsl_core::DslValue::Null).expect("decode");
    <Undirected as dsl_core::FromValue>::from_value(dsl_core::DslValue::Null).expect("decode");
    <Normal as dsl_core::FromValue>::from_value(dsl_core::DslValue::Null).expect("decode");
    <Ported as dsl_core::FromValue>::from_value(dsl_core::DslValue::Null).expect("decode");
    <UnitWeight as dsl_core::FromValue>::from_value(dsl_core::DslValue::Null).expect("decode");

    check(EdgeRef { id: 3, u: 1, v: 2 });
    check(GraphError::NodeNotFound(7));
    check(GraphError::NotImplementedForKind { algorithm: "planarity".to_string(), kind: "multigraph".to_string() });
    check(Node { id: 1, center: Point::new(1.5, -2.5), radius: 3.0, width: 4.0, height: 5.0, shape: NodeShape::Rectangle, draggable: true, kind: Some("box".to_string()), label: None, properties: PropertyBag::new() });
    // 🔤️ `value_types` is the port's declared value-schema set (empty = undeclared, still connectable);
    // it joined `Handle` after this round-trip law was written and left the crate's own test target
    // un-compilable, which hides every law in this file.
    check(Handle { angle: 0.5, id: 9, node_id: 1, radius: 2.0, role: HandleRole::Source, kind: None, value_types: Vec::new(), properties: PropertyBag::new() });

    let mut storage: Storage<Ported, Directed> = Storage::default();
    let n0 = storage.add_node();
    let n1 = storage.add_node();
    storage.add_handle(n0);
    let h1 = storage.add_handle(n1).expect("handle");
    let h0 = storage.add_handle(n0).expect("handle");
    storage.add_edge(h0, h1);
    let encoded = dsl_core::ToValue::to_value(&storage);
    let decoded = <Storage<Ported, Directed> as dsl_core::FromValue>::from_value(encoded.clone()).expect("round-trip decode");
    assert_eq!(dsl_core::ToValue::to_value(&decoded), encoded);

    let csr = Csr::from_view(&storage);
    let csr_encoded = dsl_core::ToValue::to_value(&csr);
    let csr_decoded = <Csr as dsl_core::FromValue>::from_value(csr_encoded.clone()).expect("round-trip decode");
    assert_eq!(dsl_core::ToValue::to_value(&csr_decoded), csr_encoded);
}

// 🚫️async: E5-class executor bridge, sanctioned per R4 clause 5 — `#[test]` cannot run
// an `async fn` directly (std has no executor for it), so every async test body in this
// module runs through this instead. Sound because this crate performs no real I/O: every
// future here resolves on its first poll, so a single poll (never a spin-park loop) is
// enough — panics loudly if that invariant is ever violated rather than hanging.
fn block_on_test<F: std::future::Future>(fut: F) -> F::Output {
    use std::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};
    fn noop(_: *const ()) {}
    fn clone_raw(_: *const ()) -> RawWaker {
        RawWaker::new(std::ptr::null(), &VTABLE)
    }
    static VTABLE: RawWakerVTable = RawWakerVTable::new(clone_raw, noop, noop, noop);
    let raw = RawWaker::new(std::ptr::null(), &VTABLE);
    let waker = unsafe { Waker::from_raw(raw) };
    let mut cx = Context::from_waker(&waker);
    let mut fut = Box::pin(fut);
    match fut.as_mut().poll(&mut cx) {
        Poll::Ready(v) => v,
        Poll::Pending => panic!("block_on_test: future did not complete synchronously"),
    }
}

type NU = Storage<Normal, Undirected>;
type ND = Storage<Normal, Directed>;
type PU = Storage<Ported, Undirected>;
type PD = Storage<Ported, Directed>;

// #subregion Storage
#[test]
fn add_node_allocates_monotone_ids() {
    block_on_test(async {
        let mut g = NU::new();
        let a = g.add_node();
        let b = g.add_node();
        assert_eq!(a, 0);
        assert_eq!(b, 1);
        assert_eq!(g.node_count(), 2);
    });
}

#[test]
fn add_node_with_id_upserts_attrs_and_bumps_allocator() {
    block_on_test(async {
        let mut g = NU::new();
        let mut attrs = PropertyBag::new();
        attrs.insert("color".into(), PropertyValue::String("red".into()));
        g.add_node_with_id(5, attrs);
        assert!(g.contains_node(5));
        let next = g.add_node();
        assert_eq!(next, 6, "auto id must skip past the caller-supplied id");

        let mut more = PropertyBag::new();
        more.insert("size".into(), PropertyValue::Number(3.0));
        g.add_node_with_id(5, more);
        let record = g.node_attrs(5).expect("node 5 exists");
        assert_eq!(record.get("color").and_then(PropertyValue::as_str), Some("red"));
        assert_eq!(record.get("size").and_then(PropertyValue::as_f64), Some(3.0));
    });
}

#[test]
fn remove_node_cascades_edges_and_handles() {
    block_on_test(async {
        let mut g = PU::new();
        let a = g.add_node();
        let b = g.add_node();
        let ha = g.add_handle(a).expect("ported storage grants handles");
        let hb = g.add_handle(b).expect("ported storage grants handles");
        let e = g.add_edge(ha, hb);
        assert!(g.remove_node(a));
        assert!(!g.contains_node(a));
        assert!(g.edge_endpoints(e).is_none(), "incident edge must be cascaded away");
        assert!(g.handle_owner(ha).is_none(), "handle on the removed node must be cascaded away");
        assert_eq!(g.handles(b), &[hb]);
    });
}

#[test]
fn normal_add_edge_upserts_instead_of_duplicating() {
    block_on_test(async {
        let mut g = ND::new();
        let a = g.add_node();
        let b = g.add_node();
        let mut first = PropertyBag::new();
        first.insert("weight".into(), PropertyValue::Number(1.0));
        let e1 = g.add_edge_with(a, b, first);
        let mut second = PropertyBag::new();
        second.insert("label".into(), PropertyValue::String("x".into()));
        let e2 = g.add_edge_with(a, b, second);
        assert_eq!(e1, e2, "Normal storages upsert an existing pair instead of creating a parallel edge");
        assert_eq!(g.edge_count(), 1);
        let attrs = g.edge_attrs(e1).expect("edge exists");
        assert_eq!(attrs.get("weight").and_then(PropertyValue::as_f64), Some(1.0));
        assert_eq!(attrs.get("label").and_then(PropertyValue::as_str), Some("x"));
    });
}

#[test]
fn ported_add_edge_always_creates_parallel_edges() {
    block_on_test(async {
        let mut g = PD::new();
        let a = g.add_node();
        let b = g.add_node();
        let ha = g.add_handle(a).expect("ported");
        let hb = g.add_handle(b).expect("ported");
        let e1 = g.add_edge(ha, hb);
        let e2 = g.add_edge(ha, hb);
        assert_ne!(e1, e2, "Ported storages always create a fresh parallel edge");
        assert_eq!(g.edge_count(), 2);
        assert_eq!(g.out_degree(a), 2);
    });
}

#[test]
fn normal_storage_denies_handles() {
    block_on_test(async {
        let mut g = NU::new();
        let a = g.add_node();
        assert!(g.add_handle(a).is_none());
        assert!(g.handles(a).is_empty());
    });
}

#[test]
fn remove_edge_unlinks_adjacency_both_ways_when_undirected() {
    block_on_test(async {
        let mut g = NU::new();
        let a = g.add_node();
        let b = g.add_node();
        let e = g.add_edge(a, b);
        assert!(g.remove_edge(e));
        assert_eq!(g.out_degree(a), 0);
        assert_eq!(g.out_degree(b), 0);
        assert!(g.edges_between(a, b).next().is_none());
    });
}

#[test]
fn clear_edges_keeps_nodes_clear_removes_everything() {
    block_on_test(async {
        let mut g = NU::new();
        let a = g.add_node();
        let b = g.add_node();
        g.add_edge(a, b);
        g.clear_edges();
        assert_eq!(g.node_count(), 2);
        assert_eq!(g.edge_count(), 0);
        g.clear();
        assert_eq!(g.node_count(), 0);
    });
}

#[test]
fn remove_edge_and_remove_node_return_false_for_unknown_ids() {
    block_on_test(async {
        let mut g = NU::new();
        assert!(!g.remove_edge(999), "removing a never-created edge id must fail cleanly");
        assert!(!g.remove_node(999), "removing a never-created node id must fail cleanly");
    });
}

#[test]
fn node_attrs_mut_and_edge_attrs_mut_edit_in_place_and_are_none_for_unknown_ids() {
    block_on_test(async {
        let mut g = NU::new();
        let a = g.add_node();
        let b = g.add_node();
        let e = g.add_edge(a, b);
        g.node_attrs_mut(a).expect("node exists").insert("k".into(), PropertyValue::Number(1.0));
        g.edge_attrs_mut(e).expect("edge exists").insert("w".into(), PropertyValue::Number(2.0));
        assert_eq!(g.node_attrs(a).unwrap().get("k").and_then(PropertyValue::as_f64), Some(1.0));
        assert_eq!(g.edge_attrs(e).unwrap().get("w").and_then(PropertyValue::as_f64), Some(2.0));
        assert!(g.node_attrs_mut(999).is_none());
        assert!(g.edge_attrs_mut(999).is_none());
    });
}

#[test]
fn add_handle_denies_missing_node_and_handle_owner_is_none_for_unknown_handle() {
    block_on_test(async {
        let mut g = PU::new();
        assert!(g.add_handle(999).is_none(), "cannot anchor a handle on a node that doesn't exist");
        assert!(g.handle_owner(999).is_none());
    });
}

#[test]
fn core_edge_normalize_undirected_orders_the_pair() {
    block_on_test(async {
        assert_eq!(CoreEdge::<u64>::normalize_undirected(5, 2), (2, 5));
        assert_eq!(CoreEdge::<u64>::normalize_undirected(2, 5), (2, 5));
    });
}
// #endsubregion

// #subregion GraphView
#[test]
fn undirected_self_loop_counts_twice_towards_degree() {
    block_on_test(async {
        let mut g = NU::new();
        let a = g.add_node();
        g.add_edge(a, a);
        assert_eq!(g.degree(a), 2);
        assert_eq!(g.edge_count(), 1, "edges() still lists the self-loop once");
        assert_eq!(g.edges_between(a, a).count(), 2);
    });
}

#[test]
fn directed_degree_is_in_plus_out() {
    block_on_test(async {
        let mut g = ND::new();
        let a = g.add_node();
        let b = g.add_node();
        let c = g.add_node();
        g.add_edge(a, b);
        g.add_edge(c, a);
        assert_eq!(g.out_degree(a), 1);
        assert_eq!(g.in_degree(a), 1);
        assert_eq!(g.degree(a), 2);
        assert_eq!(GraphView::neighbors(&g, a).collect::<Vec<_>>(), vec![b], "neighbors == out_neighbors for directed storages");
    });
}

#[test]
fn undirected_in_neighbors_equals_out_neighbors() {
    block_on_test(async {
        let mut g = NU::new();
        let a = g.add_node();
        let b = g.add_node();
        g.add_edge(a, b);
        let out: Vec<_> = g.out_neighbors(a).collect();
        let inn: Vec<_> = g.in_neighbors(a).collect();
        assert_eq!(out, inn);
    });
}

#[test]
fn is_directed_and_is_multigraph_reflect_type_axes() {
    block_on_test(async {
        assert!(!NU::new().is_directed());
        assert!(ND::new().is_directed());
        assert!(!NU::new().is_multigraph());
        assert!(PU::new().is_multigraph());
    });
}

#[test]
fn directed_self_loop_counts_once_each_towards_out_and_in_degree() {
    block_on_test(async {
        let mut g = ND::new();
        let a = g.add_node();
        g.add_edge(a, a);
        assert_eq!(g.out_degree(a), 1);
        assert_eq!(g.in_degree(a), 1);
        assert_eq!(g.degree(a), 2);
    });
}
// #endsubregion

// #subregion EdgeWeights
#[test]
fn unit_weight_is_always_one() {
    block_on_test(async {
        let w = UnitWeight;
        assert_eq!(w.weight(EdgeRef { id: 0, u: 0, v: 1 }), 1.0);
    });
}

#[test]
fn storage_default_weight_reads_weight_attr_with_fallback() {
    block_on_test(async {
        let mut g = NU::new();
        let a = g.add_node();
        let b = g.add_node();
        let mut attrs = PropertyBag::new();
        attrs.insert("weight".into(), PropertyValue::Number(4.5));
        let e = g.add_edge_with(a, b, attrs);
        let edge_ref = EdgeRef { id: e, u: a, v: b };
        assert_eq!(g.weight(edge_ref), 4.5);

        let e2 = g.add_edge(b, a);
        assert_eq!(e2, e, "Normal upsert must keep returning the same edge id");

        let mut g2 = NU::new();
        let x = g2.add_node();
        let y = g2.add_node();
        let unweighted_edge = g2.add_edge(x, y);
        assert_eq!(g2.weight(EdgeRef { id: unweighted_edge, u: x, v: y }), 1.0);
    });
}

#[test]
fn attr_weight_falls_back_to_default_when_missing_or_non_numeric() {
    block_on_test(async {
        let mut g = NU::new();
        let a = g.add_node();
        let b = g.add_node();
        let mut attrs = PropertyBag::new();
        attrs.insert("cost".into(), PropertyValue::String("not-a-number".into()));
        let e = g.add_edge_with(a, b, attrs);
        let aw = AttrWeight { graph: &g, name: "cost", default: 2.0 };
        assert_eq!(aw.weight(EdgeRef { id: e, u: a, v: b }), 2.0);
    });
}

#[test]
fn closure_implements_edge_weights() {
    block_on_test(async {
        let double = |edge: EdgeRef| (edge.id as f64) * 2.0;
        assert_eq!(double.weight(EdgeRef { id: 3, u: 0, v: 1 }), 6.0);
    });
}
// #endsubregion

// #subregion Csr
#[test]
fn csr_from_view_preserves_directed_adjacency() {
    block_on_test(async {
        let mut g = ND::new();
        let a = g.add_node();
        let b = g.add_node();
        let c = g.add_node();
        g.add_edge(a, b);
        g.add_edge(a, c);
        let csr = Csr::from_view(&g);
        assert_eq!(csr.node_count(), 3);
        let ia = csr.index_of(a).expect("a indexed");
        let ib = csr.index_of(b).expect("b indexed");
        let ic = csr.index_of(c).expect("c indexed");
        let mut out: Vec<usize> = csr.out_neighbors(ia).to_vec();
        out.sort_unstable();
        let mut expected = vec![ib, ic];
        expected.sort_unstable();
        assert_eq!(out, expected);
        assert_eq!(csr.node_of(ia), Some(a));
        assert!(csr.in_neighbors(ib).contains(&ia));
    });
}

#[test]
fn csr_from_view_mirrors_undirected_edges_both_ways() {
    block_on_test(async {
        let mut g = NU::new();
        let a = g.add_node();
        let b = g.add_node();
        g.add_edge(a, b);
        let csr = Csr::from_view(&g);
        let ia = csr.index_of(a).unwrap();
        let ib = csr.index_of(b).unwrap();
        assert!(csr.out_neighbors(ia).contains(&ib));
        assert!(csr.out_neighbors(ib).contains(&ia));
    });
}

#[test]
fn csr_out_edges_and_unknown_ids_return_none() {
    block_on_test(async {
        let mut g = ND::new();
        let a = g.add_node();
        let b = g.add_node();
        let e = g.add_edge(a, b);
        let csr = Csr::from_view(&g);
        let ia = csr.index_of(a).unwrap();
        assert_eq!(csr.out_edges(ia), &[e]);
        assert_eq!(csr.node_of(999), None);
        assert_eq!(csr.index_of(999), None);
    });
}
// #endsubregion

// #subregion Views
#[test]
fn subgraph_view_drops_edges_leaving_the_subset() {
    block_on_test(async {
        let mut g = NU::new();
        let a = g.add_node();
        let b = g.add_node();
        let c = g.add_node();
        g.add_edge(a, b);
        g.add_edge(b, c);
        let sub = SubgraphView::new(&g, [a, b]);
        assert_eq!(sub.node_count(), 2);
        assert_eq!(sub.edge_count(), 1);
        assert!(!sub.contains_node(c));
    });
}

#[test]
fn edge_subgraph_view_nodes_are_exactly_edge_endpoints() {
    block_on_test(async {
        let mut g = NU::new();
        let a = g.add_node();
        let b = g.add_node();
        let c = g.add_node();
        g.add_node(); // isolated node d, never referenced by an edge
        let e_ab = g.add_edge(a, b);
        g.add_edge(b, c);
        let view = EdgeSubgraphView::new(&g, [e_ab]);
        let mut nodes: Vec<_> = view.nodes().collect();
        nodes.sort_unstable();
        assert_eq!(nodes, vec![a, b]);
        assert_eq!(view.edge_count(), 1);
    });
}

#[test]
fn subgraph_view_degree_counts_only_edges_within_subset() {
    block_on_test(async {
        let mut g = ND::new();
        let a = g.add_node();
        let b = g.add_node();
        let c = g.add_node();
        g.add_edge(a, b);
        g.add_edge(a, c);
        let sub = SubgraphView::new(&g, [a, b]);
        assert_eq!(sub.out_degree(a), 1, "the edge to c falls outside the node subset");
        assert_eq!(sub.in_degree(b), 1);
        assert_eq!(sub.degree(a), sub.out_degree(a) + sub.in_degree(a), "directed subgraph degree is out+in");
        assert!(sub.is_directed());
        assert!(!sub.is_multigraph());
    });
}

#[test]
fn subgraph_view_attr_view_hides_attrs_outside_the_node_subset() {
    block_on_test(async {
        let mut g = NU::new();
        let a = g.add_node();
        let b = g.add_node();
        let e = g.add_edge(a, b);
        let sub = SubgraphView::new(&g, [a]);
        assert!(sub.node_attrs(a).is_some());
        assert!(sub.node_attrs(b).is_none(), "b is outside the node subset");
        assert!(sub.edge_attrs(e).is_some(), "edge attrs are not filtered by SubgraphView");
        assert!(std::ptr::eq(sub.graph_attrs(), g.graph_attrs()));
    });
}

#[test]
fn edge_subgraph_view_degree_and_directed_flag() {
    block_on_test(async {
        let mut g = ND::new();
        let a = g.add_node();
        let b = g.add_node();
        let c = g.add_node();
        let e_ab = g.add_edge(a, b);
        g.add_edge(b, c);
        let view = EdgeSubgraphView::new(&g, [e_ab]);
        assert!(view.is_directed());
        assert_eq!(view.out_degree(a), 1);
        assert_eq!(view.in_degree(b), 1);
        assert_eq!(view.degree(a), 1);
        assert!(view.edge_attrs(e_ab).is_some());
    });
}

#[test]
fn edge_subgraph_view_undirected_in_neighbors_matches_out_neighbors() {
    block_on_test(async {
        let mut g = NU::new();
        let a = g.add_node();
        let b = g.add_node();
        let e = g.add_edge(a, b);
        let view = EdgeSubgraphView::new(&g, [e]);
        assert!(!view.is_directed());
        assert_eq!(view.in_neighbors(a).collect::<Vec<_>>(), view.out_neighbors(a).collect::<Vec<_>>());
        assert_eq!(view.degree(a), view.out_degree(a));
    });
}

#[test]
fn reversed_view_swaps_direction_on_directed_graph() {
    block_on_test(async {
        let mut g = ND::new();
        let a = g.add_node();
        let b = g.add_node();
        g.add_edge(a, b);
        let rev = ReversedView::new(&g);
        assert_eq!(rev.out_neighbors(b).collect::<Vec<_>>(), vec![a]);
        assert_eq!(rev.in_neighbors(a).collect::<Vec<_>>(), vec![b]);
    });
}

#[test]
fn reversed_view_is_a_no_op_on_undirected_graph() {
    block_on_test(async {
        let mut g = NU::new();
        let a = g.add_node();
        let b = g.add_node();
        g.add_edge(a, b);
        let rev = ReversedView::new(&g);
        assert_eq!(rev.out_neighbors(a).collect::<Vec<_>>(), g.out_neighbors(a).collect::<Vec<_>>());
    });
}

#[test]
fn reversed_view_edges_and_edges_between_swap_endpoints() {
    block_on_test(async {
        let mut g = ND::new();
        let a = g.add_node();
        let b = g.add_node();
        let e = g.add_edge(a, b);
        let rev = ReversedView::new(&g);
        assert_eq!(rev.edges().collect::<Vec<_>>(), vec![EdgeRef { id: e, u: b, v: a }]);
        assert_eq!(rev.edges_between(b, a).next(), Some(EdgeRef { id: e, u: b, v: a }));
        assert_eq!(rev.degree(a), g.degree(a));
        assert_eq!(rev.is_multigraph(), g.is_multigraph());
    });
}

#[test]
fn filtered_view_keep_predicate_hides_by_inversion() {
    block_on_test(async {
        let mut g = NU::new();
        let a = g.add_node();
        let b = g.add_node();
        let c = g.add_node();
        g.add_edge(a, b);
        g.add_edge(b, c);
        let hidden: BTreeSet<NodeId> = [b].into_iter().collect();
        let view = FilteredView::new(&g, |n| !hidden.contains(&n), |_e| true);
        assert!(view.contains_node(a));
        assert!(!view.contains_node(b));
        assert_eq!(view.edge_count(), 0, "both edges touch the hidden node b");
    });
}

#[test]
fn filtered_view_keep_edge_predicate_hides_specific_edges_without_hiding_nodes() {
    block_on_test(async {
        let mut g = NU::new();
        let a = g.add_node();
        let b = g.add_node();
        let e_bad = g.add_edge(a, b);
        let view = FilteredView::new(&g, |_n| true, move |e| e.id != e_bad);
        assert!(view.contains_node(a));
        assert!(view.contains_node(b));
        assert_eq!(view.edge_count(), 0);
        assert_eq!(view.out_degree(a), 0);
        assert_eq!(view.degree(a), 0);
    });
}

#[test]
fn filtered_view_attr_view_delegates_edge_and_graph_attrs() {
    block_on_test(async {
        let mut g = NU::new();
        let a = g.add_node();
        let b = g.add_node();
        let e = g.add_edge(a, b);
        let view = FilteredView::new(&g, |_n| true, |_e| true);
        assert!(view.edge_attrs(e).is_some());
        assert!(std::ptr::eq(view.graph_attrs(), g.graph_attrs()));
    });
}

#[test]
fn undirected_view_merges_successors_and_predecessors() {
    block_on_test(async {
        let mut g = ND::new();
        let a = g.add_node();
        let b = g.add_node();
        g.add_edge(a, b);
        let view = UndirectedView::new(&g);
        assert!(!view.is_directed());
        assert_eq!(view.neighbors(a).collect::<Vec<_>>(), vec![b]);
        assert_eq!(view.neighbors(b).collect::<Vec<_>>(), vec![a]);
    });
}

#[test]
fn undirected_view_degree_and_edges_between_merge_both_directions() {
    block_on_test(async {
        let mut g = ND::new();
        let a = g.add_node();
        let b = g.add_node();
        g.add_edge(a, b);
        g.add_edge(b, a);
        let view = UndirectedView::new(&g);
        assert_eq!(view.degree(a), 2, "both directed edges count towards undirected degree");
        assert_eq!(view.edges_between(a, b).count(), 2);
        assert_eq!(view.is_multigraph(), g.is_multigraph());
    });
}

#[test]
fn undirected_view_edges_normalizes_endpoint_order() {
    block_on_test(async {
        let mut g = ND::new();
        let a = g.add_node();
        let b = g.add_node();
        let e = g.add_edge(b, a);
        let view = UndirectedView::new(&g);
        assert_eq!(view.edges().collect::<Vec<_>>(), vec![EdgeRef { id: e, u: a, v: b }], "edges() orders endpoints u <= v regardless of storage direction");
    });
}

#[test]
fn undirected_view_attr_view_delegates_to_parent() {
    block_on_test(async {
        let mut g = ND::new();
        let a = g.add_node();
        let b = g.add_node();
        let e = g.add_edge(a, b);
        let view = UndirectedView::new(&g);
        assert!(view.node_attrs(a).is_some());
        assert!(view.edge_attrs(e).is_some());
        assert!(std::ptr::eq(view.graph_attrs(), g.graph_attrs()));
    });
}
// #endsubregion

// #subregion Interner
#[test]
fn interner_intern_is_idempotent() {
    block_on_test(async {
        let mut interner: Interner<String> = Interner::new();
        let a1 = interner.intern("alpha".to_string());
        let a2 = interner.intern("alpha".to_string());
        let b = interner.intern("beta".to_string());
        assert_eq!(a1, a2);
        assert_ne!(a1, b);
        assert_eq!(interner.label_of(a1), Some(&"alpha".to_string()));
        assert_eq!(interner.id_of(&"beta".to_string()), Some(b));
        assert_eq!(interner.len(), 2);
    });
}

#[test]
fn interner_from_labels_is_sorted_and_deduplicated() {
    block_on_test(async {
        let interner: Interner<String> = Interner::from_labels(["c".to_string(), "a".to_string(), "a".to_string(), "b".to_string()]);
        assert_eq!(interner.len(), 3);
        assert_eq!(interner.label_of(0), Some(&"a".to_string()));
        assert_eq!(interner.label_of(1), Some(&"b".to_string()));
        assert_eq!(interner.label_of(2), Some(&"c".to_string()));
    });
}

#[test]
fn interner_is_empty_and_unknown_lookups_return_none() {
    block_on_test(async {
        let mut interner: Interner<String> = Interner::new();
        assert!(interner.is_empty());
        assert_eq!(interner.label_of(0), None);
        assert_eq!(interner.id_of(&"ghost".to_string()), None);
        interner.intern("alpha".to_string());
        assert!(!interner.is_empty());
    });
}
// #endsubregion

// #subregion GraphError
#[test]
fn graph_error_display_reads_clearly() {
    assert_eq!(GraphError::NodeNotFound(7).to_string(), "node 7 not found");
    assert_eq!(GraphError::NoPath { source: 1, target: 2 }.to_string(), "no path from node 1 to node 2");
    assert_eq!(GraphError::NotImplementedForKind { algorithm: "planarity".to_string(), kind: "multigraph".to_string() }.to_string(), "planarity is not implemented for multigraph");
}

#[test]
fn graph_error_is_a_std_error() {
    let err: Box<dyn std::error::Error> = Box::new(GraphError::HasACycle);
    assert_eq!(err.to_string(), "graph has a cycle");
}

#[test]
fn graph_error_display_covers_remaining_variants() {
    assert_eq!(GraphError::EdgeNotFound(3).to_string(), "edge 3 not found");
    assert_eq!(GraphError::NoCycle.to_string(), "graph has no cycle");
    assert_eq!(GraphError::Unfeasible("x".into()).to_string(), "unfeasible: x");
    assert_eq!(GraphError::Unbounded("y".into()).to_string(), "unbounded: y");
    assert_eq!(GraphError::NotATree.to_string(), "graph is not a tree");
    assert_eq!(GraphError::NotAForest.to_string(), "graph is not a forest");
    assert_eq!(GraphError::NotBipartite.to_string(), "graph is not bipartite");
    assert_eq!(GraphError::NotPlanar.to_string(), "graph is not planar");
    assert_eq!(GraphError::NotEulerian.to_string(), "graph is not eulerian");
    assert_eq!(GraphError::NotConnected.to_string(), "graph is not connected");
    assert_eq!(GraphError::NotStronglyConnected.to_string(), "graph is not strongly connected");
    assert_eq!(GraphError::AmbiguousSolution("z".into()).to_string(), "ambiguous solution: z");
    assert_eq!(GraphError::ExceededMaxIterations { iterations: 5 }.to_string(), "exceeded max iterations (5)");
    assert_eq!(GraphError::PowerIterationFailedConvergence { iterations: 8 }.to_string(), "power iteration failed to converge after 8 iterations");
    assert_eq!(GraphError::NegativeCycle.to_string(), "graph has a negative cycle");
    assert_eq!(GraphError::NotGraphical("odd sum".into()).to_string(), "not a graphical degree sequence: odd sum");
    assert_eq!(GraphError::Io("disk full".into()).to_string(), "io error: disk full");
    assert_eq!(GraphError::Parse { line: 4, message: "bad token".into() }.to_string(), "parse error at line 4: bad token");
}
// #endsubregion

// #subregion Utils
#[test]
fn pairwise_yields_consecutive_pairs() {
    let items = [1, 2, 3, 4];
    assert_eq!(pairwise(&items).collect::<Vec<_>>(), vec![(1, 2), (2, 3), (3, 4)]);
}

#[test]
fn arbitrary_element_is_deterministic() {
    block_on_test(async {
        assert_eq!(arbitrary_element(&[9, 1, 2]), Some(9));
        assert_eq!(arbitrary_element::<i32>(&[]), None);
    });
}

#[test]
fn tolerance_constants_are_ordered() {
    const { assert!(TOL_STRICT < TOL_LOOSE) };
}

#[test]
fn mapped_heap_pops_in_ascending_priority_order() {
    block_on_test(async {
        let mut heap: MappedHeap<i64, &str> = MappedHeap::new();
        heap.push_or_decrease("c", 30);
        heap.push_or_decrease("a", 10);
        heap.push_or_decrease("b", 20);
        assert_eq!(heap.pop_min(), Some((10, "a")));
        assert_eq!(heap.pop_min(), Some((20, "b")));
        assert_eq!(heap.pop_min(), Some((30, "c")));
        assert_eq!(heap.pop_min(), None);
    });
}

#[test]
fn mapped_heap_decrease_key_reorders() {
    block_on_test(async {
        let mut heap: MappedHeap<i64, &str> = MappedHeap::new();
        heap.push_or_decrease("a", 10);
        heap.push_or_decrease("b", 20);
        assert!(heap.decrease_key(&"b", 5));
        assert!(!heap.decrease_key(&"b", 100), "raising priority via decrease_key is a no-operation");
        assert_eq!(heap.pop_min(), Some((5, "b")));
        assert!(heap.contains(&"a"));
        assert!(!heap.contains(&"b"));
    });
}

#[test]
fn mapped_heap_len_and_is_empty_track_size() {
    block_on_test(async {
        let mut heap: MappedHeap<i64, &str> = MappedHeap::new();
        assert!(heap.is_empty());
        assert_eq!(heap.len(), 0);
        heap.push_or_decrease("a", 5);
        assert!(!heap.is_empty());
        assert_eq!(heap.len(), 1);
    });
}

#[test]
fn mapped_heap_push_or_decrease_ignores_higher_or_equal_priority() {
    block_on_test(async {
        let mut heap: MappedHeap<i64, &str> = MappedHeap::new();
        heap.push_or_decrease("a", 5);
        heap.push_or_decrease("a", 10);
        assert_eq!(heap.len(), 1, "a higher priority for an already-present item must be a no-operation");
        heap.push_or_decrease("a", 5);
        assert_eq!(heap.pop_min(), Some((5, "a")), "priority must stay at the lowest value ever pushed");
    });
}

#[test]
fn decrease_key_returns_false_for_absent_item() {
    block_on_test(async {
        let mut heap: MappedHeap<i64, &str> = MappedHeap::new();
        assert!(!heap.decrease_key(&"missing", 1));
    });
}
// #endsubregion

// #subregion Randomized consistency (expensive-ish; kept here since it's the one genuinely property-style check in this file)
mod quick {
    use super::*;

    /// 🎲️ Tiny deterministic xorshift so this crate doesn't need `crate::random` as a dependency just for one fuzz test.
    fn xorshift(state: &mut u64) -> u64 {
        *state ^= *state << 13;
        *state ^= *state >> 7;
        *state ^= *state << 17;
        *state
    }

    #[test]
    fn csr_out_degree_matches_storage_out_degree_under_random_directed_graphs() {
        block_on_test(async {
            let mut seed = 0x5eed_u64;
            for _ in 0..20 {
                let mut g = ND::new();
                let n = 3 + (xorshift(&mut seed) % 8) as usize;
                // 🔀️ Rewritten from `.map(..)` — `add_node` is async and cannot be called inside
                // the sync closure that used to build `nodes` (R10 residue shape #1).
                let mut nodes: Vec<NodeId> = Vec::with_capacity(n);
                for _ in 0..n {
                    nodes.push(g.add_node());
                }
                let edge_attempts = n * 2;
                for _ in 0..edge_attempts {
                    let u = nodes[(xorshift(&mut seed) as usize) % n];
                    let v = nodes[(xorshift(&mut seed) as usize) % n];
                    g.add_edge(u, v);
                }
                let csr = Csr::from_view(&g);
                for &node in &nodes {
                    let i = csr.index_of(node).expect("every storage node is indexed");
                    assert_eq!(csr.out_neighbors(i).len(), g.out_degree(node), "csr out-degree must match storage out-degree for node {node}");
                }
            }
        });
    }
}
// #endsubregion

// #subregion MaxFlow
/// 🏗️ The classic CLRS Ford-Fulkerson network (Fig. 26.1): six nodes `s=0, v1=1, v2=2, v3=3, v4=4, t=5`, known max flow `23`.
fn clrs_flow_network() -> FlowNetwork {
    let mut net = FlowNetwork::new(6);
    net.add_edge(0, 1, 16.0);
    net.add_edge(0, 2, 13.0);
    net.add_edge(1, 3, 12.0);
    net.add_edge(2, 1, 4.0);
    net.add_edge(3, 2, 9.0);
    net.add_edge(2, 4, 14.0);
    net.add_edge(4, 3, 7.0);
    net.add_edge(3, 5, 20.0);
    net.add_edge(4, 5, 4.0);
    net
}

#[test]
fn max_flow_matches_clrs_textbook_network() {
    block_on_test(async {
        let mut net = clrs_flow_network();
        assert_eq!(net.max_flow(0, 5), 23.0);
    });
}

#[test]
fn min_cut_capacity_matches_max_flow_value_duality() {
    block_on_test(async {
        let mut net = clrs_flow_network();
        let flow = net.max_flow(0, 5);
        let reachable: BTreeSet<u32> = net.min_cut(0).into_iter().collect();
        assert!(!reachable.contains(&5), "sink must land on the far side of a valid cut");
        let clrs_edges = [(0u32, 1u32, 16.0), (0, 2, 13.0), (1, 3, 12.0), (2, 1, 4.0), (3, 2, 9.0), (2, 4, 14.0), (4, 3, 7.0), (3, 5, 20.0), (4, 5, 4.0)];
        let crossing: f64 = clrs_edges.iter().filter(|&&(u, v, _)| reachable.contains(&u) && !reachable.contains(&v)).map(|&(_, _, cap)| cap).sum();
        assert_eq!(crossing, flow, "total capacity crossing the min cut must equal the max flow value");
    });
}

#[test]
fn max_flow_saturates_branching_level_graph() {
    block_on_test(async {
        let mut net = FlowNetwork::new(5);
        net.add_edge(0, 1, 10.0);
        net.add_edge(0, 2, 10.0);
        net.add_edge(0, 3, 10.0);
        net.add_edge(1, 2, 2.0);
        net.add_edge(2, 3, 2.0);
        net.add_edge(1, 4, 4.0);
        net.add_edge(2, 4, 4.0);
        net.add_edge(3, 4, 4.0);
        assert_eq!(net.max_flow(0, 4), 12.0, "sink in-degree 3 at capacity 4 each caps the flow at 12 regardless of source out-degree 3");
    });
}

#[test]
fn max_flow_is_zero_when_source_and_sink_are_disconnected() {
    block_on_test(async {
        let mut net = FlowNetwork::new(2);
        assert_eq!(net.max_flow(0, 1), 0.0);
        assert_eq!(net.min_cut(0), vec![0], "with no path at all, only the source itself is reachable");
    });
}

#[test]
fn max_flow_and_min_cut_are_deterministic_across_fresh_instances() {
    block_on_test(async {
        let mut first = clrs_flow_network();
        let mut second = clrs_flow_network();
        let flow_a = first.max_flow(0, 5);
        let flow_b = second.max_flow(0, 5);
        assert_eq!(flow_a, flow_b, "identically constructed networks must yield byte-identical flow values");
        assert_eq!(first.min_cut(0), second.min_cut(0), "identically constructed networks must yield byte-identical min-cut node sets");
    });
}
// #endsubregion

// #subregion PropertyValue
#[test]
fn property_bag_value_round_trips_and_empty_bag_serializes_to_none() {
    block_on_test(async {
        let mut bag = PropertyBag::new();
        bag.insert("label".into(), PropertyValue::String("hi".into()));
        bag.insert("count".into(), PropertyValue::Number(3.0));
        let value = property_bag_to_value(&bag).expect("non-empty bag serializes to Some");
        let round_tripped = property_bag_from_value(&value);
        assert_eq!(round_tripped.get("label").and_then(PropertyValue::as_str), Some("hi"));
        assert_eq!(round_tripped.get("count").and_then(PropertyValue::as_f64), Some(3.0));
        assert!(property_bag_to_value(&PropertyBag::new()).is_none(), "an empty bag serializes to None");
    });
}

#[test]
fn property_bag_from_value_falls_back_to_default_on_unparsable_shape() {
    block_on_test(async {
        let value = dsl_core::DslValue::String("not-an-object-map".to_string());
        let bag = property_bag_from_value(&value);
        assert!(bag.is_empty(), "a value that can't deserialize into a PropertyBag falls back to empty");
    });
}
// #endsubregion

```

### 🧰️framework/🔨️modules/🌱️value/✨️derive/⚙️expansion/🦀️.rs

SHA-256 `fcdcb090ec74156fc6d55ac072a2e14fc23e63c1c4e488c1a72f521b14e0ccb1`; 143127 bytes.

```
//! ✨️ `semio_framework_value_derive` — `#[derive(ToValue, FromValue)]` with `#[value(...)]`
//! container/field attributes, mirroring the subset of `#[serde(...)]` actually used under `✏️s/`
//! (see `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS/
//! 🔍️research/📓️serde-replacement-surface.md` §Survey).
//!
//! Whole crate is sync (E3): a proc-macro entry point's signature is language-fixed to
//! `fn(TokenStream) -> TokenStream` and rustc rejects an `async fn` here outright — see
//! `semio-framework-schema-derive`'s identical header note, this crate follows the same shape.
//!
//! `#[value(crate = "path::to::value_root")]` (container): overrides the crate path every emitted
//! call site (`ToValue`, `FromValue`, `DslValue`, `ValueError`) is qualified with, defaulting to
//! `::semio_framework_value` when absent — a container with no `crate` attribute emits
//! byte-identical code to before this attribute existed. Mirrors `#[serde(crate = "…")]`. Exists so
//! a crate BELOW `os-kernel` in the dependency DAG (e.g. `semio-framework-actor`, which
//! `os-kernel` itself depends on, so depending back would be a Cargo cycle) can still use this
//! derive by pointing it at wherever it reexports `DslValue`/`ToValue`/`FromValue`/`ValueError`
//! from instead — `#[value(crate = "crate::value")]` etc.
//!
//! Supported container attributes: `rename_all = "camelCase" | "kebab-case" | "lowercase" |
//! "snake_case"`, `tag = "…"` (internally-tagged enum), `tag = "…" + content = "…"`
//! (adjacently-tagged enum). A `tag`-less enum derives too: an all-unit-variant enum becomes a
//! bare `DslValue::String` of the variant's wire name, matching serde's own default
//! representation for a data-less enum (`SelectionMode::Single` → `"single"`, not
//! `{"tag":"single"}`); a `tag`-less enum with at least one data-carrying variant derives as
//! EXTERNALLY-tagged (serde's own default enum representation when no `#[serde(tag = …)]` is
//! present) — a unit variant is still the bare wire-name string, a single-unnamed-field or
//! named-field variant becomes a one-key object `{"VariantName": <payload>}`. `default`
//! (struct-only; every field on the struct falls back to its own `Default::default()`, or the
//! type's own if the type itself is `Default`, on a missing key), `deny_unknown_fields`.
//!
//! Supported field attributes: `rename = "…"`, `required` (a present wire key is mandatory even
//! for `Option<T>`), `default` (bare), `default = "path"`,
//! `skip_serializing_if = "path"`, `serialize_with = "path"` (`fn(&FieldType) ->
//! DslValue`, replaces the `ToValue::to_value` call for that field), `deserialize_with = "path"`
//! (`fn(DslValue) -> Result<FieldType, ValueError>`, replaces the `FromValue::from_value` call —
//! combine with bare `default` for a "missing key defaults, present key goes through the custom
//! fn" split, the `deserialize_double_option` shape), `with = "path"` (shorthand for
//! `serialize_with = "path::to_value"` + `deserialize_with = "path::from_value"`; an explicit
//! `serialize_with`/`deserialize_with` given alongside `with` wins for that one direction).
//! `serialize_controlled_with` supplies the explicit borrowed serializer under cumulative
//! `NativeEncodeControl`; ordinary custom serializers are refused by controlled output.
//! `skip`
//! (struct fields only — omitted entirely on serialize; `Default::default()`, or `default =
//! "path"` alongside it, on deserialize, with no lookup against the wire object at all), `flatten`
//! (struct fields only — on serialize, splices the field's own object entries straight into the
//! parent object instead of nesting under the field's wire name; on deserialize, collects every
//! entry NOT claimed by a sibling field into that field's own `FromValue::from_value`). Combining
//! `flatten` with `deny_unknown_fields` on the same struct is a `compile_error!`, matching serde's
//! own restriction — the two are inherently at odds, since a flattened field's whole point is to
//! absorb keys the container does not itself recognize.
//! A missing `Option<T>` field decodes as `None` without requiring `#[value(default)]`, matching
//! serde for both structs and named enum-variant payloads.
//!
//! `#[value(transparent)]` (container, struct-only): the struct must have exactly one field
//! (named or unnamed) — the whole struct forwards straight to/from that field's own
//! `ToValue`/`FromValue`, no object wrapper.
//!
//! A single-field TUPLE struct (`struct Foo(pub u32);`, no `#[value(...)]` at all) derives as
//! transparent AUTOMATICALLY, with no attribute needed — `ToValue` emits exactly what the inner
//! field's own `ToValue::to_value` emits, `FromValue` decodes the inner type and wraps it back in
//! `Self`. This is the newtype-wrapper idiom (`id_newtype!`-style `pub struct FooId(pub u32)`),
//! distinct from `#[value(transparent)]` on a NAMED-field struct: the tuple case needs no
//! attribute because a one-field tuple struct has no other sensible wire representation (there is
//! no field name to key an object under). A tuple struct with more than one field, or a unit
//! struct, still hits the `named-field structs … not tuple/unit structs` error below — only the
//! exactly-one-field tuple shape gets this transparent treatment.
//!
//! A generic struct/enum gets an AUTOMATIC `Param: ToValue` (resp. `FromValue`) bound synthesized
//! per own type parameter by default — mirrors `serde_derive`'s own auto-inference default, and
//! is correct for every generic type this derive has been applied to so far (each parameter is
//! always reached through a `ToValue::to_value`/`FromValue::from_value` field access). Override
//! with `#[value(bound = "P1: Trait1, P2: Trait2, …")]` (container) for the rare case a parameter
//! is unused (e.g. behind `PhantomData`, so the auto bound would be an unsatisfiable-in-practice
//! over-constraint) or needs a different bound shape — both the `ToValue` and `FromValue` impl
//! get the SAME literal predicates you write, so write one valid for both (e.g.
//! `"K: ToValue + FromValue"` if a field of type `K` needs both).
//!
//! `deny_unknown_fields` is enforced for `Data::Struct` (unknown keys in the decoded object become
//! a `ValueError`) AND for every `Data::Enum` representation, with "unknown field" scoped
//! differently per representation to match what serde itself would reject:
//! - **unit-only** (bare-string) enums: not applicable — the wire form is a single string matched
//!   exactly against the variant names, so there is no object and no extra-key slot to smuggle
//!   anything into; an unrecognized string is already a hard `"unknown variant"` error regardless
//!   of this attribute. Setting the attribute here is accepted and does nothing extra.
//! - **externally tagged** (no `tag`, mixed variants): the outer object is inherently exactly one
//!   key (`{"VariantName": payload}` — enforced unconditionally via an `entries.len() != 1` check,
//!   independent of this attribute), so the only enforcement `deny_unknown_fields` adds is on a
//!   NAMED-field variant's own payload keys (checked against that variant's known field names). A
//!   single-unnamed-field variant's payload is handed whole to that field type's own `FromValue` —
//!   its unknown-field policy is that type's business, not this container's.
//! - **adjacently tagged** (`tag` + `content`): checked at two independent levels — the outer
//!   object's keys must be a subset of `{tag, content}` (checked once, before the tag is even
//!   read, since it does not depend on which variant matched), and a NAMED-field variant's
//!   `content` object keys must be a subset of just that variant's own field names (the tag never
//!   appears inside `content`, only alongside it at the outer level). A single-unnamed-field
//!   variant's `content` payload is again that field type's own business.
//! - **internally tagged** (`tag` only, fields inline beside it): checked per matched variant,
//!   since the allowed key set depends on which variant the tag names — a unit variant only
//!   allows the bare `{tag}` key; a named-field variant allows `{tag} ∪ its own field names`. A
//!   single-unnamed-field variant hands the entries object to that field type's own `FromValue`
//!   with the tag key STRIPPED first (encode never puts it there either — see the payload-facing
//!   `Fields::Unnamed`/`None` arm in `expand_from_value` — so a payload type carrying its own
//!   `deny_unknown_fields` must not see it), and no further check is added here: the payload type
//!   decides its own policy for everything else.
//!
//! An internally tagged single-unnamed-field variant whose payload does NOT encode to an object
//! (a `String`, a number, an array — serde refuses this shape outright, this derive does not) is
//! carried as the single entry `{"value": <payload>}` beside the tag. Decoding is the inverse of
//! that runtime branch, in that order: the tag-stripped object is offered to the payload's
//! `FromValue` first, and only when that fails is a lone `value` entry unwrapped and offered
//! bare. The order matters — a payload type whose only field is itself named `value` produces an
//! indistinguishable key set, and object-first decodes both correctly.
//!
//! `rename_all_fields = "…"` on an enum supplies its named variant fields' default casing.
//! `rename_all = "…"` on a variant overrides that default for its own fields; an explicit field
//! `rename` takes precedence over both. Container `rename_all` controls variant tags only, so an
//! enum declaring only that attribute wires its named variant fields under their Rust identifiers
//! verbatim. These scopes match Serde's container, variant and field attributes.
//!
//! An enum variant's OWN named field (unlike a plain struct field) supports only `rename`,
//! `required`, `default`, `skip`, and `skip_serializing_if` — `skip` omits the field on serialize and always
//! falls back to `default`/`Default::default()` on deserialize (no wire lookup at all), and
//! `skip_serializing_if = "path"` omits the field on serialize when `path(&field)` is `true`,
//! exactly like their plain-struct-field counterparts. `flatten`/`with`/`serialize_with`/
//! `deserialize_with` on an enum variant's own named field remain Deliberately NOT supported (rare
//! in the survey — under 5 occurrences repo-wide) and are now a `compile_error!` naming the field
//! rather than a silent no-op — a crate needing one of these keeps it hand-written (`impl
//! ToValue`/`impl FromValue` directly) rather than deriving. Also Deliberately NOT supported: tuple
//! variants with more than one unnamed field.

use quote::{quote, format_ident};
use syn::{Data, DeriveInput, Fields};

//#region 🔖️Case
/// 🐫 Splits a `snake_case` field ident into lowercase words.
fn split_words_snake(ident: &str) -> Vec<String> {
    ident.split('_').filter(|s| !s.is_empty()).map(|s| s.to_lowercase()).collect()
}

/// 🐫 Splits a `PascalCase` variant ident into lowercase words at each uppercase boundary.
fn split_words_pascal(ident: &str) -> Vec<String> {
    let mut words = Vec::new();
    let mut current = String::new();
    for ch in ident.chars() {
        if ch.is_uppercase() && !current.is_empty() {
            words.push(std::mem::take(&mut current).to_lowercase());
        }
        current.push(ch);
    }
    if !current.is_empty() {
        words.push(current.to_lowercase());
    }
    words
}

fn words_to_camel(words: &[String]) -> String {
    let mut out = String::new();
    for (index, word) in words.iter().enumerate() {
        if index == 0 {
            out.push_str(word);
        } else {
            let mut chars = word.chars();
            if let Some(first) = chars.next() {
                out.extend(first.to_uppercase());
                out.push_str(chars.as_str());
            }
        }
    }
    out
}

fn words_to_kebab(words: &[String]) -> String {
    words.join("-")
}

fn words_to_lower(words: &[String]) -> String {
    words.join("")
}

fn words_to_snake(words: &[String]) -> String {
    words.join("_")
}

/// 🎨️ Applies a `rename_all` case name to `words` (already lowercased word-split).
fn apply_case(words: &[String], case: &str) -> Option<String> {
    match case {
        "camelCase" => Some(words_to_camel(words)),
        "kebab-case" => Some(words_to_kebab(words)),
        "lowercase" => Some(words_to_lower(words)),
        "snake_case" => Some(words_to_snake(words)),
        _ => None,
    }
}

fn field_wire_name(ident: &str, rename: &Option<String>, rename_all: &Option<String>) -> String {
    if let Some(rename) = rename {
        return rename.clone();
    }
    if let Some(case) = rename_all {
        if let Some(cased) = apply_case(&split_words_snake(ident), case) {
            return cased;
        }
    }
    ident.to_string()
}

fn variant_wire_name(ident: &str, rename: &Option<String>, rename_all: &Option<String>) -> String {
    if let Some(rename) = rename {
        return rename.clone();
    }
    if let Some(case) = rename_all {
        if let Some(cased) = apply_case(&split_words_pascal(ident), case) {
            return cased;
        }
    }
    ident.to_string()
}
//#endregion 🔖️Case

//#region 🔖️Attrs
#[derive(Default)]
struct ContainerAttrs {
    rename_all: Option<String>,
    rename_all_fields: Option<String>,
    tag: Option<String>,
    content: Option<String>,
    default: bool,
    deny_unknown_fields: bool,
    transparent: bool,
    bound: Option<String>,
    crate_path: Option<String>,
    retire_with: Option<String>,
    default_controlled: Option<String>,
}

impl ContainerAttrs {
    /// 🐫 Variant `rename_all` overrides container `rename_all_fields`; an explicit field rename
    /// wins in `field_wire_name`. Container `rename_all` controls enum variant names only.
    fn field_rename_all(&self, variant: &VariantAttrs) -> Option<String> {
        variant.rename_all.clone().or_else(|| self.rename_all_fields.clone())
    }
}

#[derive(Default)]
struct VariantAttrs {
    rename: Option<String>,
    rename_all: Option<String>,
}

#[derive(Default, Clone)]
struct FieldAttrs {
    rename: Option<String>,
    required: bool,
    default: FieldDefault,
    skip_serializing_if: Option<String>,
    serialize_with: Option<String>,
    serialize_controlled_with: Option<String>,
    deserialize_with: Option<String>,
    deserialize_controlled_with: Option<String>,
    default_controlled: Option<String>,
    retire_with: Option<String>,
    with: Option<String>,
    flatten: bool,
    skip: bool,
}

impl FieldAttrs {
    /// 🩹 `with = "path"` shorthand resolved for the serialize direction: an explicit
    /// `serialize_with` wins, else `path::to_value` when `with` is set, else `None` (the plain
    /// `ToValue::to_value` call).
    fn effective_serialize_with(&self) -> Option<String> {
        self.serialize_with.clone().or_else(|| self.with.as_ref().map(|path| format!("{path}::to_value")))
    }

    /// 🩹 `with = "path"` shorthand resolved for the deserialize direction — sibling of
    /// `effective_serialize_with` above.
    fn effective_deserialize_with(&self) -> Option<String> {
        self.deserialize_with.clone().or_else(|| self.with.as_ref().map(|path| format!("{path}::from_value")))
    }
}

#[derive(Default, Clone)]
enum FieldDefault {
    #[default]
    None,
    Bare,
    Path(String),
}

/// 🧾️ Reads every `#[value(...)]` attribute on `attrs` into `(key, Option<string-value>)` pairs
/// — `None` for a bare flag (`default`, `deny_unknown_fields`), `Some(..)` for `key = "…"`.
fn parse_value_meta(attrs: &[syn::Attribute]) -> syn::Result<Vec<(String, Option<String>)>> {
    let mut out = Vec::new();
    for attr in attrs {
        if !attr.path().is_ident("value") {
            continue;
        }
        attr.parse_nested_meta(|meta| {
            let key = meta.path.get_ident().map(ToString::to_string).ok_or_else(|| meta.error("expected a #[value(...)] identifier"))?;
            if meta.input.peek(syn::Token![=]) {
                let value: syn::LitStr = meta.value()?.parse()?;
                out.push((key, Some(value.value())));
            } else {
                out.push((key, None));
            }
            Ok(())
        })?;
    }
    Ok(out)
}

fn parse_container_attrs(attrs: &[syn::Attribute]) -> syn::Result<ContainerAttrs> {
    let mut out = ContainerAttrs::default();
    for (key, value) in parse_value_meta(attrs)? {
        match key.as_str() {
            "rename_all" => out.rename_all = value,
            "tag" => out.tag = value,
            "content" => out.content = value,
            "default" => out.default = true,
            "deny_unknown_fields" => out.deny_unknown_fields = true,
            "transparent" => out.transparent = true,
            "bound" => out.bound = value,
            "rename_all_fields" => out.rename_all_fields = value,
            "crate" => out.crate_path = value,
            "retire_with" => out.retire_with = value,
            "default_controlled" => out.default_controlled = value,
            other => return Err(syn::Error::new_spanned(&attrs[0], format!("#[value(...)] does not support container attribute `{other}`"))),
        }
    }
    Ok(out)
}

/// 🧭️ Resolves `#[value(crate = "path::to::value_root")]` to the crate-path prefix every emitted
/// call site interpolates as `#value_crate::Type` — defaults to `::semio_framework_value` when
/// absent, so a container with no `crate` attribute emits byte-identical code to before this
/// attribute existed. Lets a sub-kernel crate (e.g. `semio-framework-actor`, which cannot depend on
/// `semio-framework-os-kernel` without a Cargo cycle) reexport `DslValue`/`ToValue`/`FromValue`/
/// `ValueError` from wherever it actually gets them and point the derive there instead.
fn container_crate_path(container: &ContainerAttrs) -> syn::Path {
    let path = container.crate_path.as_deref().unwrap_or("::semio_framework_value");
    syn::parse_str(path).expect("valid #[value(crate = \"...\")] path")
}

fn parse_variant_attrs(attrs: &[syn::Attribute]) -> syn::Result<VariantAttrs> {
    let mut out = VariantAttrs::default();
    for (key, value) in parse_value_meta(attrs)? {
        match key.as_str() {
            "rename" => out.rename = value,
            "rename_all" => out.rename_all = value,
            other => return Err(syn::Error::new_spanned(&attrs[0], format!("#[value(...)] does not support variant attribute `{other}`"))),
        }
    }
    Ok(out)
}

fn parse_field_attrs(attrs: &[syn::Attribute]) -> syn::Result<FieldAttrs> {
    let mut out = FieldAttrs::default();
    for (key, value) in parse_value_meta(attrs)? {
        match key.as_str() {
            "rename" => out.rename = value,
            "required" => out.required = true,
            "default" => out.default = value.map_or(FieldDefault::Bare, FieldDefault::Path),
            "skip_serializing_if" => out.skip_serializing_if = value,
            "serialize_with" => out.serialize_with = value,
            "serialize_controlled_with" => out.serialize_controlled_with = value,
            "deserialize_with" => out.deserialize_with = value,
            "deserialize_controlled_with" => out.deserialize_controlled_with = value,
            "default_controlled" => out.default_controlled = value,
            "retire_with" => out.retire_with = value,
            "with" => out.with = value,
            "flatten" => out.flatten = true,
            "skip" => out.skip = true,
            other => return Err(syn::Error::new_spanned(&attrs[0], format!("#[value(...)] does not support field attribute `{other}`"))),
        }
    }
    Ok(out)
}

/// 🧬️ Clones `generics` and adds the `where` bound this impl needs for each of its OWN type
/// parameters: by default, one `Param: #trait_path` predicate per type parameter (mirrors
/// `serde_derive`'s own auto-inference default — every generic struct/enum this derive has seen
/// so far needs exactly this, an owned field access through `ToValue::to_value`/
/// `FromValue::from_value` on that parameter). `#[value(bound = "P1: Trait1, P2: Trait2, …")]`
/// overrides this entirely (both impls get the SAME literal predicates you write — see the module
/// docs' `bound` entry) for the rare case a parameter is unused (e.g. behind `PhantomData`) or
/// needs a different bound shape than the uniform default.
fn generics_with_bound(generics: &syn::Generics, bound: &Option<String>, trait_path: &proc_macro2::TokenStream) -> syn::Generics {
    let type_param_idents: Vec<syn::Ident> = generics.type_params().map(|param| param.ident.clone()).collect();
    let mut generics = generics.clone();
    let where_clause = generics.make_where_clause();
    match bound {
        Some(bound) => {
            for predicate in bound.split(',') {
                let predicate = predicate.trim();
                if predicate.is_empty() {
                    continue;
                }
                let predicate: syn::WherePredicate = syn::parse_str(predicate).expect("valid #[value(bound = \"...\")] where predicate");
                where_clause.predicates.push(predicate);
            }
        }
        None => {
            for ident in &type_param_idents {
                let predicate: syn::WherePredicate = syn::parse_quote! { #ident: #trait_path };
                where_clause.predicates.push(predicate);
            }
        }
    }
    generics
}
//#endregion 🔖️Attrs

//#region 🔖️StructPlan
struct NamedField {
    ident: syn::Ident,
    wire_name: String,
    attrs: FieldAttrs,
    is_option: bool,
}

fn variant_path_bindings(fields: &[NamedField]) -> (Vec<proc_macro2::TokenStream>, Vec<NamedField>) {
    fields
        .iter()
        .enumerate()
        .map(|(index, field)| {
            let original = &field.ident;
            let binding = quote::format_ident!("__semio_value_field_{index}");
            (
                if field.attrs.skip { quote! { #original: _ } } else { quote! { #original: #binding } },
                NamedField { ident: binding, wire_name: field.wire_name.clone(), attrs: field.attrs.clone(), is_option: field.is_option },
            )
        })
        .unzip()
}

fn type_is_option(ty: &syn::Type) -> bool {
    let syn::Type::Path(path) = ty else { return false };
    path.qself.is_none() && path.path.segments.last().is_some_and(|segment| segment.ident == "Option")
}

fn named_fields(fields: &Fields, container: &ContainerAttrs) -> syn::Result<Vec<NamedField>> {
    let Fields::Named(named) = fields else {
        return Err(syn::Error::new_spanned(fields, "#[derive(ToValue, FromValue)] supports named-field structs (and #[value(tag = \"…\")] enums), not tuple/unit structs"));
    };
    let out: Vec<NamedField> = named
        .named
        .iter()
        .map(|field| {
            let attrs = parse_field_attrs(&field.attrs)?;
            let ident = field.ident.clone().expect("named field");
            let wire_name = field_wire_name(&ident.to_string(), &attrs.rename, &container.rename_all);
            Ok(NamedField { ident, wire_name, attrs, is_option: type_is_option(&field.ty) })
        })
        .collect::<syn::Result<_>>()?;
    // 🛡️ Serde itself rejects `flatten` alongside `deny_unknown_fields` on the same struct — a
    // flattened field's whole job is to absorb keys the container does not itself recognize, which
    // is the exact opposite of an unknown-key check — so this derive rejects the same combination
    // up front instead of silently picking one behavior over the other.
    if container.deny_unknown_fields && out.iter().any(|field| field.attrs.flatten) {
        return Err(syn::Error::new_spanned(named, "#[value(...)] does not support combining `flatten` with `deny_unknown_fields` (matches serde's own restriction)"));
    }
    Ok(out)
}

fn to_value_object_entries(fields: &[NamedField], value_crate: &syn::Path) -> proc_macro2::TokenStream {
    let pushes = fields.iter().map(|field| {
        let ident = &field.ident;
        let wire_name = &field.wire_name;
        if field.attrs.skip {
            return quote! {};
        }
        let value_expr = match field.attrs.effective_serialize_with() {
            Some(path) => {
                let path: syn::Path = syn::parse_str(&path).expect("valid serialize_with path");
                quote! { #path(&self.#ident) }
            }
            None => quote! { #value_crate::ToValue::to_value(&self.#ident) },
        };
        if field.attrs.flatten {
            return quote! {
                if let #value_crate::DslValue::Object(__flat_entries) = #value_expr {
                    entries.extend(__flat_entries);
                }
            };
        }
        match &field.attrs.skip_serializing_if {
            Some(path) => {
                let path: syn::Path = syn::parse_str(path).expect("valid skip_serializing_if path");
                quote! {
                    if !#path(&self.#ident) {
                        entries.push((#wire_name.to_string(), #value_expr));
                    }
                }
            }
            None => quote! {
                entries.push((#wire_name.to_string(), #value_expr));
            },
        }
    });
    quote! {
        let mut entries: Vec<(String, #value_crate::DslValue)> = Vec::new();
        #(#pushes)*
    }
}

/// 🛡️ Emits a loop rejecting any key of the `Vec<(String, DslValue)>`-shaped expression
/// `entries_expr` that is not present in `allowed` — the `deny_unknown_fields` enforcement shared
/// by struct bodies (see `from_value_struct_fields` below) and every enum representation in
/// `expand_from_value` (module docs above spell out what "unknown field" scopes to per
/// representation).
fn deny_unknown_keys(entries_expr: &proc_macro2::TokenStream, allowed: &[String], value_crate: &syn::Path) -> proc_macro2::TokenStream {
    quote! {
        for (__key, _) in #entries_expr.iter() {
            if ![#(#allowed),*].contains(&__key.as_str()) {
                return Err(#value_crate::ValueError::new(#value_crate::ValueRefusalKind::InvalidValue, format!("unknown field `{}`", __key)));
            }
        }
    }
}

fn from_value_struct_fields(fields: &[NamedField], container: &ContainerAttrs, value_crate: &syn::Path) -> proc_macro2::TokenStream {
    // 🌾 The wire keys a `flatten` field is entitled to absorb are everything NOT claimed by a
    // sibling — so the deny-check's own allow-list (when no field flattens) and each flatten
    // field's own "remaining entries" filter both key off this same non-flatten name list.
    let non_flatten_names: Vec<String> = fields.iter().filter(|field| !field.attrs.flatten).map(|field| field.wire_name.clone()).collect();
    let deny_check = if container.deny_unknown_fields {
        deny_unknown_keys(&quote! { __entries }, &non_flatten_names, value_crate)
    } else {
        quote! {}
    };
    let reads = fields.iter().map(|field| {
        let ident = &field.ident;
        let wire_name = &field.wire_name;
        if field.attrs.skip {
            let missing = match &field.attrs.default {
                FieldDefault::Path(path) => {
                    let path: syn::Path = syn::parse_str(path).expect("valid default path");
                    quote! { #path() }
                }
                FieldDefault::Bare | FieldDefault::None => quote! { ::std::default::Default::default() },
            };
            return quote! { let #ident = #missing; };
        }
        if field.attrs.flatten {
            let remaining = quote! {
                #value_crate::DslValue::Object(__entries.iter().filter(|(__k, _)| ![#(#non_flatten_names),*].contains(&__k.as_str())).cloned().collect())
            };
            let found = match field.attrs.effective_deserialize_with() {
                Some(path) => {
                    let path: syn::Path = syn::parse_str(&path).expect("valid deserialize_with path");
                    quote! { #path(#remaining).map_err(|error: #value_crate::ValueError| error.under(#wire_name))? }
                }
                None => quote! { #value_crate::FromValue::from_value(#remaining).map_err(|error| error.under(#wire_name))? },
            };
            return quote! { let #ident = #found; };
        }
        let missing = match (&field.attrs.default, container.default, field.attrs.required) {
            (_, _, true) => quote! {
                return Err(#value_crate::ValueError::new(#value_crate::ValueRefusalKind::InvalidValue, format!("missing field `{}`", #wire_name)))
            },
            (FieldDefault::Path(path), _, false) => {
                let path: syn::Path = syn::parse_str(path).expect("valid default path");
                quote! { #path() }
            }
            (FieldDefault::Bare, _, false) | (FieldDefault::None, true, false) => quote! { ::std::default::Default::default() },
            (FieldDefault::None, false, false) if field.is_option => quote! { ::std::default::Default::default() },
            (FieldDefault::None, false, false) => quote! {
                return Err(#value_crate::ValueError::new(#value_crate::ValueRefusalKind::InvalidValue, format!("missing field `{}`", #wire_name)))
            },
        };
        let found = match field.attrs.effective_deserialize_with() {
            Some(path) => {
                let path: syn::Path = syn::parse_str(&path).expect("valid deserialize_with path");
                quote! { #path(value.clone()).map_err(|error: #value_crate::ValueError| error.under(#wire_name))? }
            }
            None => quote! { #value_crate::FromValue::from_value(value.clone()).map_err(|error| error.under(#wire_name))? },
        };
        quote! {
            let #ident = match __entries.iter().find(|(k, _)| k == #wire_name) {
                Some((_, value)) => #found,
                None => #missing,
            };
        }
    });
    let idents = fields.iter().map(|field| &field.ident);
    quote! {
        #deny_check
        #(#reads)*
        Ok(Self { #(#idents),* })
    }
}
//#endregion 🔖️StructPlan

//#region 🔖️VariantFields
/// 🎯 Rejects `flatten` on an enum variant's own named field with a `compile_error!` naming the
/// field, instead of the previous silent drop — module docs call `flatten` "Deliberately NOT
/// supported" on a variant's named fields (splicing into an already-tagged object is ambiguous),
/// but nothing enforced that until now. `rename`, `required`, `default`, `skip`, `skip_serializing_if`,
/// `serialize_with`/`deserialize_with`/`with` ARE supported on a variant's own named field (see
/// `variant_field_to_value_push`/`variant_field_from_value_read` below) — `default` and the
/// `*_with` trio already worked in practice (🏪️store's `ArtifactActorMsg::LocalMutations`/
/// `ArtifactEvent::RemoteMutations`/`ArtifactMutationsSaved.envelope` all rely on
/// `serialize_with`/`deserialize_with` on an internally-tagged variant's own field to route
/// `MutationEnvelope` through its hand-written bridge), `skip`/`skip_serializing_if` did not (both
/// fixed below — same silent-drop bug class).
fn check_variant_field_attrs_supported(field: &syn::Field, attrs: &FieldAttrs) -> syn::Result<()> {
    if attrs.flatten {
        return Err(syn::Error::new_spanned(field, format!("#[value(...)] does not support `flatten` on enum variant field `{}` (only plain struct fields support it)", field.ident.as_ref().expect("named field"))));
    }
    Ok(())
}

/// 🎯 Emits one named enum-variant field's `ToValue` push into the accumulator `push_into`
/// (`content_entries` for externally/adjacently-tagged, `__out_entries` for internally-tagged),
/// honoring `skip` (omit unconditionally), `skip_serializing_if` (omit conditionally), and
/// `serialize_with`/`with` (replaces the default `ToValue::to_value` call) — mirrors
/// `to_value_object_entries`'s struct-field handling of the same attributes. Fixes the silent
/// wire-shape bug where `skip`/`skip_serializing_if` were parsed off an enum variant field and then
/// never consulted, so the field was always emitted via the default `ToValue::to_value` regardless
/// of a `serialize_with` naming a different one.
fn variant_field_to_value_push(field: &syn::Field, field_attrs: &FieldAttrs, wire_name: &str, ident: &syn::Ident, push_into: &proc_macro2::TokenStream, value_crate: &syn::Path) -> syn::Result<proc_macro2::TokenStream> {
    check_variant_field_attrs_supported(field, field_attrs)?;
    if field_attrs.skip {
        return Ok(quote! {});
    }
    let value_expr = match field_attrs.effective_serialize_with() {
        Some(path) => {
            let path: syn::Path = syn::parse_str(&path).expect("valid serialize_with path");
            quote! { #path(#ident) }
        }
        None => quote! { #value_crate::ToValue::to_value(#ident) },
    };
    Ok(match &field_attrs.skip_serializing_if {
        Some(path) => {
            let path: syn::Path = syn::parse_str(path).expect("valid skip_serializing_if path");
            quote! {
                if !#path(#ident) {
                    #push_into.push((#wire_name.to_string(), #value_expr));
                }
            }
        }
        None => quote! {
            #push_into.push((#wire_name.to_string(), #value_expr));
        },
    })
}

/// 🎯 Emits one named enum-variant field's `FromValue` read out of `entries_ident` (a
/// `Vec<(String, DslValue)>`-shaped expression), honoring `skip` (bypass the wire lookup entirely
/// and always fall back to `default`/`Default::default()` — mirrors `from_value_struct_fields`'s
/// struct-field handling), the pre-existing `default` handling, and `deserialize_with`/`with`
/// (replaces the default `FromValue::from_value` call). Sibling of `variant_field_to_value_push`
/// above.
fn variant_field_from_value_read(field: &syn::Field, field_attrs: &FieldAttrs, wire_name: &str, ident: &syn::Ident, entries_ident: &proc_macro2::TokenStream, value_crate: &syn::Path) -> syn::Result<proc_macro2::TokenStream> {
    check_variant_field_attrs_supported(field, field_attrs)?;
    if field_attrs.skip {
        let missing = match &field_attrs.default {
            FieldDefault::Path(path) => {
                let path: syn::Path = syn::parse_str(path).expect("valid default path");
                quote! { #path() }
            }
            FieldDefault::Bare | FieldDefault::None => quote! { ::std::default::Default::default() },
        };
        return Ok(quote! { let #ident = #missing; });
    }
    let missing = match (&field_attrs.default, field_attrs.required) {
        (_, true) => quote! {
            return Err(#value_crate::ValueError::new(#value_crate::ValueRefusalKind::InvalidValue, format!("missing field `{}`", #wire_name)))
        },
        (FieldDefault::Path(path), false) => {
            let path: syn::Path = syn::parse_str(path).expect("valid default path");
            quote! { #path() }
        }
        (FieldDefault::Bare, false) => quote! { ::std::default::Default::default() },
        (FieldDefault::None, false) if type_is_option(&field.ty) => quote! { ::std::default::Default::default() },
        (FieldDefault::None, false) => quote! {
            return Err(#value_crate::ValueError::new(#value_crate::ValueRefusalKind::InvalidValue, format!("missing field `{}`", #wire_name)))
        },
    };
    let found = match field_attrs.effective_deserialize_with() {
        Some(path) => {
            let path: syn::Path = syn::parse_str(&path).expect("valid deserialize_with path");
            quote! { #path(value.clone()).map_err(|error: #value_crate::ValueError| error.under(#wire_name))? }
        }
        None => quote! { #value_crate::FromValue::from_value(value.clone()).map_err(|error| error.under(#wire_name))? },
    };
    Ok(quote! {
        let #ident = match #entries_ident.iter().find(|(k, _)| k == #wire_name) {
            Some((_, value)) => #found,
            None => #missing,
        };
    })
}
/// 🧹 Builds the `Self::Variant { … }` destructure pattern for `ToValue`'s per-field push
/// generation — a `skip` field destructures as `ident: _` (still exhaustive) instead of binding an
/// unused local, since `variant_field_to_value_push` intentionally never reads a skipped field's
/// binding. Every other field destructures as the shorthand `ident`, unchanged from before.
fn variant_destructure_patterns(named: &syn::FieldsNamed) -> Vec<proc_macro2::TokenStream> {
    named
        .named
        .iter()
        .map(|field| {
            let field_attrs = parse_field_attrs(&field.attrs).unwrap_or_default();
            let ident = field.ident.clone().expect("named field");
            if field_attrs.skip {
                quote! { #ident: _ }
            } else {
                quote! { #ident }
            }
        })
        .collect()
}
//#endregion 🔖️VariantFields

//#region 🧭️TypedPath
fn field_to_path_call(
    field: &NamedField,
    access: &proc_macro2::TokenStream,
    path: &proc_macro2::TokenStream,
    method: &syn::Ident,
    value_crate: &syn::Path,
) -> proc_macro2::TokenStream {
    match field.attrs.effective_serialize_with() {
        Some(serializer) => {
            let serializer: syn::Path = syn::parse_str(&serializer).expect("valid serialize_with path");
            quote! { #value_crate::ToValue::#method(&#serializer(#access), #path) }
        }
        None => quote! { #value_crate::ToValue::#method(#access, #path) },
    }
}

fn struct_to_path_body(fields: &[NamedField], value_crate: &syn::Path, method: &str) -> proc_macro2::TokenStream {
    let method = syn::Ident::new(method, proc_macro2::Span::call_site());
    let direct_arms = fields.iter().filter(|field| !field.attrs.skip && !field.attrs.flatten).map(|field| {
        let ident = &field.ident;
        let wire_name = &field.wire_name;
        let call = field_to_path_call(field, &quote! { &self.#ident }, &quote! { __rest }, &method, value_crate);
        let omitted = field.attrs.skip_serializing_if.as_ref().map(|predicate| {
            let predicate: syn::Path = syn::parse_str(predicate).expect("valid skip_serializing_if path");
            quote! {
                if #predicate(&self.#ident) {
                    return Err(#value_crate::ValueError::new(#value_crate::ValueRefusalKind::InvalidValue, format!("missing object key `{}`", __segment)));
                }
            }
        });
        quote! {
            #wire_name => {
                #omitted
                #call.map_err(|error| error.under(__segment))
            }
        }
    });
    let flatten_attempts = fields.iter().filter(|field| !field.attrs.skip && field.attrs.flatten).map(|field| {
        let ident = &field.ident;
        let call = field_to_path_call(field, &quote! { &self.#ident }, &quote! { path }, &method, value_crate);
        quote! {
            if let Ok(value) = #call {
                return Ok(value);
            }
        }
    });
    let root = if method == "value_shape_at_path" {
        let normal_counts = fields.iter().filter(|field| !field.attrs.skip && !field.attrs.flatten).map(|field| {
            let ident = &field.ident;
            match &field.attrs.skip_serializing_if {
                Some(predicate) => {
                    let predicate: syn::Path = syn::parse_str(predicate).expect("valid skip_serializing_if path");
                    quote! { if !#predicate(&self.#ident) { __len += 1; } }
                }
                None => quote! { __len += 1; },
            }
        });
        let flattened_counts = fields.iter().filter(|field| !field.attrs.skip && field.attrs.flatten).map(|field| {
            let ident = &field.ident;
            let call = field_to_path_call(field, &quote! { &self.#ident }, &quote! { &[] }, &method, value_crate);
            quote! {
                match #call? {
                    #value_crate::ValueShape::Object { len } => __len = __len.checked_add(len).ok_or_else(|| #value_crate::ValueError::new(#value_crate::ValueRefusalKind::OwnershipLimit, "object length overflow"))?,
                    _ => return Err(#value_crate::ValueError::new(#value_crate::ValueRefusalKind::InvalidValue, "flattened field is not an object")),
                }
            }
        });
        quote! {
            if path.is_empty() {
                let mut __len = 0usize;
                #(#normal_counts)*
                #(#flattened_counts)*
                return Ok(#value_crate::ValueShape::Object { len: __len });
            }
        }
    } else {
        quote! {
            if path.is_empty() {
                return Ok(#value_crate::ToValue::to_value(self));
            }
        }
    };
    quote! {
        #root
        let (__segment, __rest) = path.split_first().expect("non-empty path checked above");
        match *__segment {
            #(#direct_arms,)*
            _ => {
                #(#flatten_attempts)*
                Err(#value_crate::ValueError::new(#value_crate::ValueRefusalKind::InvalidValue, format!("missing object key `{}`", __segment)))
            }
        }
    }
}

fn field_to_key_call(field: &NamedField, access: &proc_macro2::TokenStream, path: &proc_macro2::TokenStream, value_crate: &syn::Path) -> proc_macro2::TokenStream {
    match field.attrs.effective_serialize_with() {
        Some(serializer) => {
            let serializer: syn::Path = syn::parse_str(&serializer).expect("valid serialize_with path");
            quote! { #value_crate::ToValue::value_key_at_path(&#serializer(#access), #path, index) }
        }
        None => quote! { #value_crate::ToValue::value_key_at_path(#access, #path, index) },
    }
}

fn struct_key_at_path_body(fields: &[NamedField], value_crate: &syn::Path) -> proc_macro2::TokenStream {
    let root_steps = fields.iter().filter(|field| !field.attrs.skip).map(|field| {
        let ident = &field.ident;
        let wire_name = &field.wire_name;
        let admitted = field.attrs.skip_serializing_if.as_ref().map(|predicate| {
            let predicate: syn::Path = syn::parse_str(predicate).expect("valid skip_serializing_if path");
            quote! { !#predicate(&self.#ident) }
        }).unwrap_or_else(|| quote! { true });
        if field.attrs.flatten {
            let shape = field_to_path_call(field, &quote! { &self.#ident }, &quote! { &[] }, &syn::Ident::new("value_shape_at_path", proc_macro2::Span::call_site()), value_crate);
            let key = field_to_key_call(field, &quote! { &self.#ident }, &quote! { &[] }, value_crate);
            quote! {
                if #admitted {
                    let #value_crate::ValueShape::Object { len } = #shape? else {
                        return Err(#value_crate::ValueError::new(#value_crate::ValueRefusalKind::InvalidValue, "flattened field is not an object"));
                    };
                    if index < __offset + len {
                        let index = index - __offset;
                        return #key;
                    }
                    __offset += len;
                }
            }
        } else {
            quote! {
                if #admitted {
                    if index == __offset { return Ok(#wire_name.to_owned()); }
                    __offset += 1;
                }
            }
        }
    });
    let direct_arms = fields.iter().filter(|field| !field.attrs.skip && !field.attrs.flatten).map(|field| {
        let ident = &field.ident;
        let wire_name = &field.wire_name;
        let call = field_to_key_call(field, &quote! { &self.#ident }, &quote! { __rest }, value_crate);
        let omitted = field.attrs.skip_serializing_if.as_ref().map(|predicate| {
            let predicate: syn::Path = syn::parse_str(predicate).expect("valid skip_serializing_if path");
            quote! {
                if #predicate(&self.#ident) {
                    return Err(#value_crate::ValueError::new(#value_crate::ValueRefusalKind::InvalidValue, format!("missing object key `{}`", __segment)));
                }
            }
        });
        quote! {
            #wire_name => {
                #omitted
                #call.map_err(|error| error.under(__segment))
            }
        }
    });
    let flatten_attempts = fields.iter().filter(|field| !field.attrs.skip && field.attrs.flatten).map(|field| {
        let ident = &field.ident;
        let call = field_to_key_call(field, &quote! { &self.#ident }, &quote! { path }, value_crate);
        quote! { if let Ok(key) = #call { return Ok(key); } }
    });
    quote! {
        if path.is_empty() {
            let mut __offset = 0usize;
            #(#root_steps)*
            return Err(#value_crate::ValueError::new(#value_crate::ValueRefusalKind::InvalidValue, format!("object key index {index} is out of range for length {__offset}")));
        }
        let (__segment, __rest) = path.split_first().expect("non-empty path checked above");
        match *__segment {
            #(#direct_arms,)*
            _ => {
                #(#flatten_attempts)*
                Err(#value_crate::ValueError::new(#value_crate::ValueRefusalKind::InvalidValue, format!("missing object key `{}`", __segment)))
            }
        }
    }
}

fn field_edit_call(
    field: &NamedField,
    access: &proc_macro2::TokenStream,
    path: &proc_macro2::TokenStream,
    edit: &proc_macro2::TokenStream,
    value_crate: &syn::Path,
) -> proc_macro2::TokenStream {
    match (field.attrs.effective_serialize_with(), field.attrs.effective_deserialize_with()) {
        (Some(serializer), Some(deserializer)) => {
            let serializer: syn::Path = syn::parse_str(&serializer).expect("valid serialize_with path");
            let deserializer: syn::Path = syn::parse_str(&deserializer).expect("valid deserialize_with path");
            quote! {
                {
                    let mut __field_value = #serializer(&*#access);
                    #value_crate::FromValue::edit_value_at_path(&mut __field_value, #path, #edit)?;
                    let __replacement = #deserializer(__field_value)?;
                    *#access = __replacement;
                    Ok::<(), #value_crate::ValueError>(())
                }
            }
        }
        (Some(_), None) => quote! {
            Err(#value_crate::ValueError::new(#value_crate::ValueRefusalKind::UnsupportedOwner, "custom wire field has no matching decoder for a typed-path edit"))
        },
        (None, Some(deserializer)) => {
            let deserializer: syn::Path = syn::parse_str(&deserializer).expect("valid deserialize_with path");
            quote! {
                if #path.is_empty() {
                    match #edit {
                        #value_crate::ValueEdit::Set(value) => {
                            let __replacement = #deserializer(value)?;
                            *#access = __replacement;
                            Ok::<(), #value_crate::ValueError>(())
                        }
                        #value_crate::ValueEdit::Insert(_) | #value_crate::ValueEdit::InsertAt { .. } => Err(#value_crate::ValueError::new(#value_crate::ValueRefusalKind::InvalidValue, "cannot insert a required field")),
                        #value_crate::ValueEdit::Remove => Err(#value_crate::ValueError::new(#value_crate::ValueRefusalKind::InvalidValue, "cannot remove a required field")),
                    }
                } else {
                    Err(#value_crate::ValueError::new(#value_crate::ValueRefusalKind::UnsupportedOwner, "custom wire field has no matching encoder for a nested typed-path edit"))
                }
            }
        }
        (None, None) => quote! { #value_crate::FromValue::edit_value_at_path(#access, #path, #edit) },
    }
}

fn struct_edit_path_body(fields: &[NamedField], value_crate: &syn::Path) -> proc_macro2::TokenStream {
    let direct_arms = fields.iter().filter(|field| !field.attrs.skip && !field.attrs.flatten).map(|field| {
        let ident = &field.ident;
        let wire_name = &field.wire_name;
        let call = field_edit_call(field, &quote! { &mut self.#ident }, &quote! { __rest }, &quote! { edit }, value_crate);
        quote! { #wire_name => #call.map_err(|error| error.under(__segment)) }
    });
    let flatten_attempts = fields.iter().filter(|field| !field.attrs.skip && field.attrs.flatten).map(|field| {
        let ident = &field.ident;
        let call = field_edit_call(field, &quote! { &mut self.#ident }, &quote! { path }, &quote! { edit.clone() }, value_crate);
        quote! {
            if #call.is_ok() {
                return Ok(());
            }
        }
    });
    quote! {
        if path.is_empty() {
            return match edit {
                #value_crate::ValueEdit::Set(value) => {
                    let replacement = <Self as #value_crate::FromValue>::from_value(value)?;
                    *self = replacement;
                    Ok(())
                }
                #value_crate::ValueEdit::Insert(_) | #value_crate::ValueEdit::InsertAt { .. } => Err(#value_crate::ValueError::new(#value_crate::ValueRefusalKind::InvalidValue, "cannot insert at the record root")),
                #value_crate::ValueEdit::Remove => Err(#value_crate::ValueError::new(#value_crate::ValueRefusalKind::InvalidValue, "cannot remove the record root")),
            };
        }
        let (__segment, __rest) = path.split_first().expect("non-empty path checked above");
        match *__segment {
            #(#direct_arms,)*
            _ => {
                #(#flatten_attempts)*
                Err(#value_crate::ValueError::new(#value_crate::ValueRefusalKind::InvalidValue, format!("missing object key `{}`", __segment)))
            }
        }
    }
}

fn variant_named_fields(named: &syn::FieldsNamed, container: &ContainerAttrs, variant: &VariantAttrs) -> syn::Result<Vec<NamedField>> {
    named
        .named
        .iter()
        .map(|field| {
            let attrs = parse_field_attrs(&field.attrs)?;
            check_variant_field_attrs_supported(field, &attrs)?;
            let ident = field.ident.clone().expect("named field");
            let wire_name = field_wire_name(&ident.to_string(), &attrs.rename, &container.field_rename_all(variant));
            Ok(NamedField { ident, wire_name, attrs, is_option: type_is_option(&field.ty) })
        })
        .collect()
}

fn named_variant_edit_dispatch(fields: &[NamedField], path: &proc_macro2::TokenStream, edit: &proc_macro2::TokenStream, value_crate: &syn::Path) -> proc_macro2::TokenStream {
    let arms = fields.iter().filter(|field| !field.attrs.skip).map(|field| {
        let ident = &field.ident;
        let wire_name = &field.wire_name;
        let call = field_edit_call(field, &quote! { #ident }, &quote! { __rest }, edit, value_crate);
        quote! { #wire_name => #call.map_err(|error| error.under(__segment)) }
    });
    quote! {
        let (__segment, __rest) = #path.split_first().ok_or_else(|| #value_crate::ValueError::new(#value_crate::ValueRefusalKind::InvalidValue, "cannot structurally replace an enum variant payload"))?;
        match *__segment {
            #(#arms,)*
            _ => Err(#value_crate::ValueError::new(#value_crate::ValueRefusalKind::InvalidValue, format!("missing object key `{}`", __segment))),
        }
    }
}

fn named_variant_to_path_dispatch(fields: &[NamedField], path: &proc_macro2::TokenStream, method: &str, value_crate: &syn::Path) -> proc_macro2::TokenStream {
    let method_ident = syn::Ident::new(method, proc_macro2::Span::call_site());
    let arms = fields.iter().filter(|field| !field.attrs.skip).map(|field| {
        let ident = &field.ident;
        let wire_name = &field.wire_name;
        let call = field_to_path_call(field, &quote! { #ident }, &quote! { __rest }, &method_ident, value_crate);
        let omitted = field.attrs.skip_serializing_if.as_ref().map(|predicate| {
            let predicate: syn::Path = syn::parse_str(predicate).expect("valid skip_serializing_if path");
            quote! {
                if #predicate(#ident) {
                    return Err(#value_crate::ValueError::new(#value_crate::ValueRefusalKind::InvalidValue, format!("missing object key `{}`", __segment)));
                }
            }
        });
        quote! {
            #wire_name => {
                #omitted
                #call.map_err(|error| error.under(__segment))
            }
        }
    });
    let root = if method == "value_shape_at_path" {
        let counts = fields.iter().filter(|field| !field.attrs.skip).map(|field| {
            let ident = &field.ident;
            match &field.attrs.skip_serializing_if {
                Some(predicate) => {
                    let predicate: syn::Path = syn::parse_str(predicate).expect("valid skip_serializing_if path");
                    quote! { if !#predicate(#ident) { __len += 1; } }
                }
                None => quote! { __len += 1; },
            }
        });
        quote! {
            if #path.is_empty() {
                let mut __len = 0usize;
                #(#counts)*
                return Ok(#value_crate::ValueShape::Object { len: __len });
            }
        }
    } else {
        let pushes = fields.iter().filter(|field| !field.attrs.skip).map(|field| {
            let ident = &field.ident;
            let wire_name = &field.wire_name;
            let value = match field.attrs.effective_serialize_with() {
                Some(serializer) => {
                    let serializer: syn::Path = syn::parse_str(&serializer).expect("valid serialize_with path");
                    quote! { #serializer(#ident) }
                }
                None => quote! { #value_crate::ToValue::to_value(#ident) },
            };
            match &field.attrs.skip_serializing_if {
                Some(predicate) => {
                    let predicate: syn::Path = syn::parse_str(predicate).expect("valid skip_serializing_if path");
                    quote! { if !#predicate(#ident) { __entries.push((#wire_name.to_owned(), #value)); } }
                }
                None => quote! { __entries.push((#wire_name.to_owned(), #value)); },
            }
        });
        quote! {
            if #path.is_empty() {
                let mut __entries = Vec::new();
                #(#pushes)*
                return Ok(#value_crate::DslValue::Object(__entries));
            }
        }
    };
    quote! {
        #root
        let (__segment, __rest) = #path.split_first().expect("non-empty variant path checked above");
        match *__segment {
            #(#arms,)*
            _ => Err(#value_crate::ValueError::new(#value_crate::ValueRefusalKind::InvalidValue, format!("missing object key `{}`", __segment))),
        }
    }
}

fn named_variant_key_dispatch(fields: &[NamedField], path: &proc_macro2::TokenStream, value_crate: &syn::Path) -> proc_macro2::TokenStream {
    let root_steps = fields.iter().filter(|field| !field.attrs.skip).map(|field| {
        let ident = &field.ident;
        let wire_name = &field.wire_name;
        let admitted = field.attrs.skip_serializing_if.as_ref().map(|predicate| {
            let predicate: syn::Path = syn::parse_str(predicate).expect("valid skip_serializing_if path");
            quote! { !#predicate(#ident) }
        }).unwrap_or_else(|| quote! { true });
        quote! {
            if #admitted {
                if index == __offset { return Ok(#wire_name.to_owned()); }
                __offset += 1;
            }
        }
    });
    let arms = fields.iter().filter(|field| !field.attrs.skip).map(|field| {
        let ident = &field.ident;
        let wire_name = &field.wire_name;
        let call = field_to_key_call(field, &quote! { #ident }, &quote! { __rest }, value_crate);
        quote! { #wire_name => #call.map_err(|error| error.under(__segment)) }
    });
    quote! {
        if #path.is_empty() {
            let mut __offset = 0usize;
            #(#root_steps)*
            return Err(#value_crate::ValueError::new(#value_crate::ValueRefusalKind::InvalidValue, format!("object key index {index} is out of range for length {__offset}")));
        }
        let (__segment, __rest) = #path.split_first().expect("non-empty path checked above");
        match *__segment {
            #(#arms,)*
            _ => Err(#value_crate::ValueError::new(#value_crate::ValueRefusalKind::InvalidValue, format!("missing object key `{}`", __segment))),
        }
    }
}

fn enum_key_at_path_body(data: &syn::DataEnum, container: &ContainerAttrs, value_crate: &syn::Path) -> syn::Result<proc_macro2::TokenStream> {
    if data.variants.is_empty() {
        return Ok(quote! { match *self {} });
    }
    let arms = data
        .variants
        .iter()
        .map(|variant| {
            let variant_ident = &variant.ident;
            let variant_attrs = parse_variant_attrs(&variant.attrs)?;
            let wire_variant = variant_wire_name(&variant_ident.to_string(), &variant_attrs.rename, &container.rename_all);
            match &variant.fields {
                Fields::Unit => {
                    let dispatch = if let Some(tag) = &container.tag {
                        quote! {
                            if path.is_empty() {
                                return if index == 0 { Ok(#tag.to_owned()) } else { Err(#value_crate::ValueError::new(#value_crate::ValueRefusalKind::InvalidValue, format!("object key index {index} is out of range for length 1"))) };
                            }
                            Err(#value_crate::ValueError::new(#value_crate::ValueRefusalKind::InvalidValue, format!("variant `{}` has no object child at the requested path", #wire_variant)))
                        }
                    } else {
                        quote! { Err(#value_crate::ValueError::new(#value_crate::ValueRefusalKind::InvalidValue, format!("expected an object, found unit variant `{}`", #wire_variant))) }
                    };
                    Ok(quote! { Self::#variant_ident => { #dispatch } })
                }
                Fields::Unnamed(unnamed) if unnamed.unnamed.len() == 1 => {
                    let dispatch = if container.tag.is_none() {
                        quote! {
                            if path.is_empty() {
                                return if index == 0 { Ok(#wire_variant.to_owned()) } else { Err(#value_crate::ValueError::new(#value_crate::ValueRefusalKind::InvalidValue, format!("object key index {index} is out of range for length 1"))) };
                            }
                            let (__segment, __rest) = path.split_first().expect("non-empty path checked above");
                            if *__segment != #wire_variant {
                                return Err(#value_crate::ValueError::new(#value_crate::ValueRefusalKind::InvalidValue, format!("active variant `{}` has no child `{}`", #wire_variant, __segment)));
                            }
                            #value_crate::ToValue::value_key_at_path(payload, __rest, index).map_err(|error| error.under(__segment))
                        }
                    } else if let Some(content) = &container.content {
                        let tag = container.tag.as_ref().expect("tagged enum");
                        quote! {
                            if path.is_empty() {
                                return match index {
                                    0 => Ok(#tag.to_owned()),
                                    1 => Ok(#content.to_owned()),
                                    _ => Err(#value_crate::ValueError::new(#value_crate::ValueRefusalKind::InvalidValue, format!("object key index {index} is out of range for length 2"))),
                                };
                            }
                            let (__segment, __rest) = path.split_first().expect("non-empty path checked above");
                            if *__segment != #content {
                                return Err(#value_crate::ValueError::new(#value_crate::ValueRefusalKind::InvalidValue, format!("expected an object below `{}`, found `{}`", #content, __segment)));
                            }
                            #value_crate::ToValue::value_key_at_path(payload, __rest, index).map_err(|error| error.under(__segment))
                        }
                    } else {
                        let tag = container.tag.as_ref().expect("tagged enum");
                        quote! {
                            if path.is_empty() {
                                if index == 0 { return Ok(#tag.to_owned()); }
                                return match #value_crate::ToValue::value_shape_at_path(payload, &[])? {
                                    #value_crate::ValueShape::Object { len } if index <= len => #value_crate::ToValue::value_key_at_path(payload, &[], index - 1),
                                    #value_crate::ValueShape::Object { len } => Err(#value_crate::ValueError::new(#value_crate::ValueRefusalKind::InvalidValue, format!("object key index {index} is out of range for length {}", len + 1))),
                                    _ if index == 1 => Ok("value".to_owned()),
                                    _ => Err(#value_crate::ValueError::new(#value_crate::ValueRefusalKind::InvalidValue, format!("object key index {index} is out of range for length 2"))),
                                };
                            }
                            if path.first().is_some_and(|segment| *segment == #tag) {
                                return Err(#value_crate::ValueError::new(#value_crate::ValueRefusalKind::InvalidValue, "enum tag is not an object"));
                            }
                            if matches!(#value_crate::ToValue::value_shape_at_path(payload, &[])?, #value_crate::ValueShape::Object { .. }) {
                                return #value_crate::ToValue::value_key_at_path(payload, path, index);
                            }
                            if !path.first().is_some_and(|segment| *segment == "value") {
                                return Err(#value_crate::ValueError::new(#value_crate::ValueRefusalKind::InvalidValue, format!("missing object key `{}`", path[0])));
                            }
                            #value_crate::ToValue::value_key_at_path(payload, &path[1..], index)
                        }
                    };
                    Ok(quote! { Self::#variant_ident(payload) => { #dispatch } })
                }
                Fields::Named(named) => {
                    let fields = variant_named_fields(named, container, &variant_attrs)?;
                    let (bindings, fields) = variant_path_bindings(&fields);
                    let pattern = quote! { Self::#variant_ident { #(#bindings),* } };
                    let dispatch = if container.tag.is_none() {
                        let named = named_variant_key_dispatch(&fields, &quote! { __rest }, value_crate);
                        quote! {
                            if path.is_empty() {
                                return if index == 0 { Ok(#wire_variant.to_owned()) } else { Err(#value_crate::ValueError::new(#value_crate::ValueRefusalKind::InvalidValue, format!("object key index {index} is out of range for length 1"))) };
                            }
                            let (__segment, __rest) = path.split_first().expect("non-empty path checked above");
                            if *__segment != #wire_variant {
                                return Err(#value_crate::ValueError::new(#value_crate::ValueRefusalKind::InvalidValue, format!("active variant `{}` has no child `{}`", #wire_variant, __segment)));
                            }
                            #named
                        }
                    } else if let Some(content) = &container.content {
                        let tag = container.tag.as_ref().expect("tagged enum");
                        let named = named_variant_key_dispatch(&fields, &quote! { __rest }, value_crate);
                        quote! {
                            if path.is_empty() {
                                return match index {
                                    0 => Ok(#tag.to_owned()), 1 => Ok(#content.to_owned()),
                                    _ => Err(#value_crate::ValueError::new(#value_crate::ValueRefusalKind::InvalidValue, format!("object key index {index} is out of range for length 2"))),
                                };
                            }
                            let (__segment, __rest) = path.split_first().expect("non-empty path checked above");
                            if *__segment != #content {
                                return Err(#value_crate::ValueError::new(#value_crate::ValueRefusalKind::InvalidValue, format!("expected an object below `{}`, found `{}`", #content, __segment)));
                            }
                            #named
                        }
                    } else {
                        let tag = container.tag.as_ref().expect("tagged enum");
                        let root_steps = fields.iter().filter(|field| !field.attrs.skip).map(|field| {
                            let ident = &field.ident;
                            let wire_name = &field.wire_name;
                            let admitted = field.attrs.skip_serializing_if.as_ref().map(|predicate| {
                                let predicate: syn::Path = syn::parse_str(predicate).expect("valid skip_serializing_if path");
                                quote! { !#predicate(#ident) }
                            }).unwrap_or_else(|| quote! { true });
                            quote! {
                                if #admitted {
                                    if index == __offset { return Ok(#wire_name.to_owned()); }
                                    __offset += 1;
                                }
                            }
                        });
                        let named_path = named_variant_key_dispatch(&fields, &quote! { path }, value_crate);
                        quote! {
                            if path.is_empty() {
                                if index == 0 { return Ok(#tag.to_owned()); }
                                let mut __offset = 1usize;
                                #(#root_steps)*
                                return Err(#value_crate::ValueError::new(#value_crate::ValueRefusalKind::InvalidValue, format!("object key index {index} is out of range for length {__offset}")));
                            }
                            if path.first().is_some_and(|segment| *segment == #tag) {
                                return Err(#value_crate::ValueError::new(#value_crate::ValueRefusalKind::InvalidValue, "enum tag is not an object"));
                            }
                            #named_path
                        }
                    };
                    Ok(quote! { #pattern => { #dispatch } })
                }
                other => Err(syn::Error::new_spanned(other, "#[derive(ToValue)] enum variants must be unit, a single unnamed payload, or named fields")),
            }
        })
        .collect::<syn::Result<Vec<_>>>()?;
    Ok(quote! { match self { #(#arms),* } })
}

fn enum_to_path_body(data: &syn::DataEnum, container: &ContainerAttrs, value_crate: &syn::Path, method: &str) -> syn::Result<proc_macro2::TokenStream> {
    if data.variants.is_empty() {
        return Ok(quote! { match *self {} });
    }
    let method_ident = syn::Ident::new(method, proc_macro2::Span::call_site());
    let shape = method == "value_shape_at_path";
    let tag_value = |wire_variant: &str| {
        if shape {
            quote! { Ok(#value_crate::ValueShape::String) }
        } else {
            quote! { Ok(#value_crate::DslValue::String(#wire_variant.to_owned())) }
        }
    };
    let arms = data
        .variants
        .iter()
        .map(|variant| {
            let variant_ident = &variant.ident;
            let variant_attrs = parse_variant_attrs(&variant.attrs)?;
            let wire_variant = variant_wire_name(&variant_ident.to_string(), &variant_attrs.rename, &container.rename_all);
            match &variant.fields {
                Fields::Unit => {
                    let tag_result = tag_value(&wire_variant);
                    let root = if shape {
                        if container.tag.is_some() {
                            quote! { if path.is_empty() { return Ok(#value_crate::ValueShape::Object { len: 1 }); } }
                        } else {
                            quote! { if path.is_empty() { return Ok(#value_crate::ValueShape::String); } }
                        }
                    } else {
                        quote! {}
                    };
                    let dispatch = if let Some(tag) = &container.tag {
                        quote! {
                            #root
                            let (__segment, __rest) = path.split_first().expect("non-empty path checked above");
                            if *__segment == #tag && __rest.is_empty() {
                                #tag_result
                            } else {
                                Err(#value_crate::ValueError::new(#value_crate::ValueRefusalKind::InvalidValue, format!("variant `{}` has no child `{}`", #wire_variant, __segment)))
                            }
                        }
                    } else {
                        quote! { #root Err(#value_crate::ValueError::new(#value_crate::ValueRefusalKind::InvalidValue, format!("variant `{}` has no children", #wire_variant))) }
                    };
                    Ok(quote! { Self::#variant_ident => { #dispatch } })
                }
                Fields::Unnamed(unnamed) if unnamed.unnamed.len() == 1 => {
                    let call = quote! { #value_crate::ToValue::#method_ident(payload, __rest) };
                    let dispatch = if container.tag.is_none() {
                        let root = if shape { quote! { if path.is_empty() { return Ok(#value_crate::ValueShape::Object { len: 1 }); } } } else { quote! {} };
                        quote! {
                            #root
                            let (__segment, __rest) = path.split_first().expect("non-empty path checked above");
                            if *__segment != #wire_variant {
                                return Err(#value_crate::ValueError::new(#value_crate::ValueRefusalKind::InvalidValue, format!("active variant `{}` has no child `{}`", #wire_variant, __segment)));
                            }
                            #call.map_err(|error| error.under(__segment))
                        }
                    } else if let Some(content) = &container.content {
                        let tag = container.tag.as_ref().expect("tagged enum");
                        let tag_result = tag_value(&wire_variant);
                        let root = if shape { quote! { if path.is_empty() { return Ok(#value_crate::ValueShape::Object { len: 2 }); } } } else { quote! {} };
                        quote! {
                            #root
                            let (__segment, __rest) = path.split_first().expect("non-empty path checked above");
                            if *__segment == #tag && __rest.is_empty() {
                                return #tag_result;
                            }
                            if *__segment != #content {
                                return Err(#value_crate::ValueError::new(#value_crate::ValueRefusalKind::InvalidValue, format!("missing object key `{}`", __segment)));
                            }
                            #call.map_err(|error| error.under(__segment))
                        }
                    } else {
                        let tag = container.tag.as_ref().expect("tagged enum");
                        let tag_result = tag_value(&wire_variant);
                        let root = if shape {
                            quote! {
                                if path.is_empty() {
                                    return match #value_crate::ToValue::value_shape_at_path(payload, &[])? {
                                        #value_crate::ValueShape::Object { len } => Ok(#value_crate::ValueShape::Object { len: len.checked_add(1).ok_or_else(|| #value_crate::ValueError::new(#value_crate::ValueRefusalKind::OwnershipLimit, "object length overflow"))? }),
                                        _ => Ok(#value_crate::ValueShape::Object { len: 2 }),
                                    };
                                }
                            }
                        } else {
                            quote! {}
                        };
                        quote! {
                            #root
                            if path.first().is_some_and(|segment| *segment == #tag) {
                                if path.len() == 1 { return #tag_result; }
                                return Err(#value_crate::ValueError::new(#value_crate::ValueRefusalKind::InvalidValue, "enum tag has no children"));
                            }
                            if matches!(#value_crate::ToValue::value_shape_at_path(payload, &[])?, #value_crate::ValueShape::Object { .. }) {
                                return #value_crate::ToValue::#method_ident(payload, path);
                            }
                            if !path.first().is_some_and(|segment| *segment == "value") {
                                return Err(#value_crate::ValueError::new(#value_crate::ValueRefusalKind::InvalidValue, format!("missing object key `{}`", path[0])));
                            }
                            #value_crate::ToValue::#method_ident(payload, &path[1..])
                        }
                    };
                    Ok(quote! { Self::#variant_ident(payload) => { #dispatch } })
                }
                Fields::Named(named) => {
                    let fields = variant_named_fields(named, container, &variant_attrs)?;
                    let (bindings, fields) = variant_path_bindings(&fields);
                    let pattern = quote! { Self::#variant_ident { #(#bindings),* } };
                    let dispatch = if container.tag.is_none() {
                        let named_dispatch = named_variant_to_path_dispatch(&fields, &quote! { __rest }, method, value_crate);
                        let root = if shape { quote! { if path.is_empty() { return Ok(#value_crate::ValueShape::Object { len: 1 }); } } } else { quote! {} };
                        quote! {
                            #root
                            let (__segment, __rest) = path.split_first().expect("non-empty path checked above");
                            if *__segment != #wire_variant {
                                return Err(#value_crate::ValueError::new(#value_crate::ValueRefusalKind::InvalidValue, format!("active variant `{}` has no child `{}`", #wire_variant, __segment)));
                            }
                            #named_dispatch
                        }
                    } else if let Some(content) = &container.content {
                        let tag = container.tag.as_ref().expect("tagged enum");
                        let tag_result = tag_value(&wire_variant);
                        let named_dispatch = named_variant_to_path_dispatch(&fields, &quote! { __rest }, method, value_crate);
                        let root = if shape { quote! { if path.is_empty() { return Ok(#value_crate::ValueShape::Object { len: 2 }); } } } else { quote! {} };
                        quote! {
                            #root
                            let (__segment, __rest) = path.split_first().expect("non-empty path checked above");
                            if *__segment == #tag && __rest.is_empty() { return #tag_result; }
                            if *__segment != #content {
                                return Err(#value_crate::ValueError::new(#value_crate::ValueRefusalKind::InvalidValue, format!("missing object key `{}`", __segment)));
                            }
                            #named_dispatch
                        }
                    } else {
                        let tag = container.tag.as_ref().expect("tagged enum");
                        let tag_result = tag_value(&wire_variant);
                        let named_dispatch = named_variant_to_path_dispatch(&fields, &quote! { path }, method, value_crate);
                        let root = if shape {
                            let counts = fields.iter().filter(|field| !field.attrs.skip).map(|field| {
                                let ident = &field.ident;
                                match &field.attrs.skip_serializing_if {
                                    Some(predicate) => {
                                        let predicate: syn::Path = syn::parse_str(predicate).expect("valid skip_serializing_if path");
                                        quote! { if !#predicate(#ident) { __len += 1; } }
                                    }
                                    None => quote! { __len += 1; },
                                }
                            });
                            quote! {
                                if path.is_empty() {
                                    let mut __len = 1usize;
                                    #(#counts)*
                                    return Ok(#value_crate::ValueShape::Object { len: __len });
                                }
                            }
                        } else {
                            quote! {}
                        };
                        quote! {
                            #root
                            if path.first().is_some_and(|segment| *segment == #tag) {
                                if path.len() == 1 { return #tag_result; }
                                return Err(#value_crate::ValueError::new(#value_crate::ValueRefusalKind::InvalidValue, "enum tag has no children"));
                            }
                            #named_dispatch
                        }
                    };
                    Ok(quote! { #pattern => { #dispatch } })
                }
                other => Err(syn::Error::new_spanned(other, "#[derive(ToValue)] enum variants must be unit, a single unnamed payload, or named fields")),
            }
        })
        .collect::<syn::Result<Vec<_>>>()?;
    let root = if shape {
        quote! {}
    } else {
        quote! { return Ok(#value_crate::ToValue::to_value(self)); }
    };
    Ok(quote! {
        if path.is_empty() { #root }
        match self { #(#arms),* }
    })
}

fn enum_edit_path_body(data: &syn::DataEnum, container: &ContainerAttrs, value_crate: &syn::Path) -> syn::Result<proc_macro2::TokenStream> {
    if data.variants.is_empty() {
        return Ok(quote! { match *self {} });
    }
    let arms = data
        .variants
        .iter()
        .map(|variant| {
            let variant_ident = &variant.ident;
            let variant_attrs = parse_variant_attrs(&variant.attrs)?;
            let wire_variant = variant_wire_name(&variant_ident.to_string(), &variant_attrs.rename, &container.rename_all);
            match &variant.fields {
                Fields::Unit => Ok(quote! {
                    Self::#variant_ident => Err(#value_crate::ValueError::new(#value_crate::ValueRefusalKind::InvalidValue, format!("variant `{}` has no editable payload", #wire_variant)))
                }),
                Fields::Unnamed(unnamed) if unnamed.unnamed.len() == 1 => {
                    let dispatch = if container.tag.is_none() {
                        quote! {
                            let (__segment, __rest) = path.split_first().ok_or_else(|| #value_crate::ValueError::new(#value_crate::ValueRefusalKind::InvalidValue, "missing external variant path"))?;
                            if *__segment != #wire_variant {
                                return Err(#value_crate::ValueError::new(#value_crate::ValueRefusalKind::InvalidValue, format!("active variant `{}` has no child `{}`", #wire_variant, __segment)));
                            }
                            #value_crate::FromValue::edit_value_at_path(payload, __rest, edit).map_err(|error| error.under(__segment))
                        }
                    } else if let Some(content) = &container.content {
                        let tag = container.tag.as_ref().expect("tagged enum");
                        quote! {
                            let (__segment, __rest) = path.split_first().ok_or_else(|| #value_crate::ValueError::new(#value_crate::ValueRefusalKind::InvalidValue, "missing adjacent enum path"))?;
                            if *__segment == #tag {
                                return Err(#value_crate::ValueError::new(#value_crate::ValueRefusalKind::InvalidValue, "cannot edit an enum tag in place"));
                            }
                            if *__segment != #content {
                                return Err(#value_crate::ValueError::new(#value_crate::ValueRefusalKind::InvalidValue, format!("missing object key `{}`", __segment)));
                            }
                            #value_crate::FromValue::edit_value_at_path(payload, __rest, edit).map_err(|error| error.under(__segment))
                        }
                    } else {
                        let tag = container.tag.as_ref().expect("tagged enum");
                        quote! {
                            if path.first().is_some_and(|segment| *segment == #tag) {
                                return Err(#value_crate::ValueError::new(#value_crate::ValueRefusalKind::InvalidValue, "cannot edit an enum tag in place"));
                            }
                            match #value_crate::FromValue::edit_value_at_path(payload, path, edit.clone()) {
                                Ok(()) => Ok(()),
                                Err(object_error) if path.first().is_some_and(|segment| *segment == "value") => {
                                    #value_crate::FromValue::edit_value_at_path(payload, &path[1..], edit).map_err(|_| object_error)
                                }
                                Err(error) => Err(error),
                            }
                        }
                    };
                    Ok(quote! { Self::#variant_ident(payload) => { #dispatch } })
                }
                Fields::Named(named) => {
                    let fields = variant_named_fields(named, container, &variant_attrs)?;
                    let (bindings, fields) = variant_path_bindings(&fields);
                    let pattern = quote! { Self::#variant_ident { #(#bindings),* } };
                    let dispatch = if container.tag.is_none() {
                        let named_dispatch = named_variant_edit_dispatch(&fields, &quote! { __rest }, &quote! { edit }, value_crate);
                        quote! {
                            let (__segment, __rest) = path.split_first().ok_or_else(|| #value_crate::ValueError::new(#value_crate::ValueRefusalKind::InvalidValue, "missing external variant path"))?;
                            if *__segment != #wire_variant {
                                return Err(#value_crate::ValueError::new(#value_crate::ValueRefusalKind::InvalidValue, format!("active variant `{}` has no child `{}`", #wire_variant, __segment)));
                            }
                            #named_dispatch
                        }
                    } else if let Some(content) = &container.content {
                        let tag = container.tag.as_ref().expect("tagged enum");
                        let named_dispatch = named_variant_edit_dispatch(&fields, &quote! { __rest }, &quote! { edit }, value_crate);
                        quote! {
                            let (__segment, __rest) = path.split_first().ok_or_else(|| #value_crate::ValueError::new(#value_crate::ValueRefusalKind::InvalidValue, "missing adjacent enum path"))?;
                            if *__segment == #tag {
                                return Err(#value_crate::ValueError::new(#value_crate::ValueRefusalKind::InvalidValue, "cannot edit an enum tag in place"));
                            }
                            if *__segment != #content {
                                return Err(#value_crate::ValueError::new(#value_crate::ValueRefusalKind::InvalidValue, format!("missing object key `{}`", __segment)));
                            }
                            #named_dispatch
                        }
                    } else {
                        let tag = container.tag.as_ref().expect("tagged enum");
                        let named_dispatch = named_variant_edit_dispatch(&fields, &quote! { path }, &quote! { edit }, value_crate);
                        quote! {
                            if path.first().is_some_and(|segment| *segment == #tag) {
                                return Err(#value_crate::ValueError::new(#value_crate::ValueRefusalKind::InvalidValue, "cannot edit an enum tag in place"));
                            }
                            #named_dispatch
                        }
                    };
                    Ok(quote! { #pattern => { #dispatch } })
                }
                other => Err(syn::Error::new_spanned(other, "#[derive(FromValue)] enum variants must be unit, a single unnamed payload, or named fields")),
            }
        })
        .collect::<syn::Result<Vec<_>>>()?;
    Ok(quote! {
        if path.is_empty() {
            return match edit {
                #value_crate::ValueEdit::Set(value) => {
                    let replacement = <Self as #value_crate::FromValue>::from_value(value)?;
                    *self = replacement;
                    Ok(())
                }
                #value_crate::ValueEdit::Insert(_) | #value_crate::ValueEdit::InsertAt { .. } => Err(#value_crate::ValueError::new(#value_crate::ValueRefusalKind::InvalidValue, "cannot insert at the enum root")),
                #value_crate::ValueEdit::Remove => Err(#value_crate::ValueError::new(#value_crate::ValueRefusalKind::InvalidValue, "cannot remove the enum root")),
            };
        }
        match self { #(#arms),* }
    })
}
//#endregion 🧭️TypedPath

//#region 🔖️Expand
pub fn expand_to_value(input: &DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    let name = &input.ident;
    let container = parse_container_attrs(&input.attrs)?;
    let value_crate = container_crate_path(&container);
    let generics = generics_with_bound(&input.generics, &container.bound, &quote! { #value_crate::ToValue });
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let body = match &input.data {
        Data::Struct(data) if container.transparent => match &data.fields {
            Fields::Named(named) if named.named.len() == 1 => {
                let ident = named.named.first().expect("checked len == 1").ident.clone().expect("named field");
                quote! { #value_crate::ToValue::to_value(&self.#ident) }
            }
            Fields::Unnamed(unnamed) if unnamed.unnamed.len() == 1 => quote! { #value_crate::ToValue::to_value(&self.0) },
            other => return Err(syn::Error::new_spanned(other, "#[value(transparent)] requires exactly one field")),
        },
        // 🆔 Single-field tuple struct (`struct Foo(pub u32);`): automatic transparent newtype —
        // see the module docs' `#[value(transparent)]` entry for why this needs no attribute.
        Data::Struct(data) if matches!(&data.fields, Fields::Unnamed(unnamed) if unnamed.unnamed.len() == 1) => {
            quote! { #value_crate::ToValue::to_value(&self.0) }
        }
        Data::Struct(data) => {
            let fields = named_fields(&data.fields, &container)?;
            let entries = to_value_object_entries(&fields, &value_crate);
            quote! {
                #entries
                #value_crate::DslValue::Object(entries)
            }
        }
        Data::Enum(data) if data.variants.is_empty() => quote! { match *self {} },
        Data::Enum(data) if container.tag.is_none() && data.variants.iter().all(|variant| matches!(variant.fields, Fields::Unit)) => {
            let arms = data.variants.iter().map(|variant| {
                let variant_ident = &variant.ident;
                let variant_attrs = parse_variant_attrs(&variant.attrs)?;
                let wire_variant = variant_wire_name(&variant_ident.to_string(), &variant_attrs.rename, &container.rename_all);
                Ok(quote! { Self::#variant_ident => #value_crate::DslValue::String(#wire_variant.to_string()) })
            }).collect::<syn::Result<Vec<_>>>()?;
            quote! {
                match *self { #(#arms),* }
            }
        }
        Data::Enum(data) if container.tag.is_none() => {
            // 🏷️ Externally-tagged (serde's own default enum representation when no `#[serde(tag
            // = …)]` is present): a unit variant is still the bare wire-name string, a
            // single-unnamed-field or named-field variant becomes a one-key object
            // `{"VariantName": <payload>}`.
            let arms = data
                .variants
                .iter()
                .map(|variant| {
                    let variant_ident = &variant.ident;
                    let variant_attrs = parse_variant_attrs(&variant.attrs)?;
                    let wire_variant = variant_wire_name(&variant_ident.to_string(), &variant_attrs.rename, &container.rename_all);
                    let arm: syn::Result<proc_macro2::TokenStream> = match &variant.fields {
                        Fields::Unit => Ok(quote! {
                            Self::#variant_ident => #value_crate::DslValue::String(#wire_variant.to_string())
                        }),
                        Fields::Unnamed(unnamed) if unnamed.unnamed.len() == 1 => Ok(quote! {
                            Self::#variant_ident(payload) => #value_crate::DslValue::object([
                                (#wire_variant.to_string(), #value_crate::ToValue::to_value(payload)),
                            ])
                        }),
                        Fields::Named(named) => {
                            let push_into = quote! { content_entries };
                            let pushes = named
                                .named
                                .iter()
                                .map(|field| {
                                    let field_attrs = parse_field_attrs(&field.attrs).unwrap_or_default();
                                    let ident = field.ident.clone().expect("named field");
                                    let wire_name = field_wire_name(&ident.to_string(), &field_attrs.rename, &container.field_rename_all(&variant_attrs));
                                    variant_field_to_value_push(field, &field_attrs, &wire_name, &ident, &push_into, &value_crate)
                                })
                                .collect::<syn::Result<Vec<_>>>()?;
                            let idents = variant_destructure_patterns(named);
                            Ok(quote! {
                                Self::#variant_ident { #(#idents),* } => {
                                    let mut content_entries: Vec<(String, #value_crate::DslValue)> = Vec::new();
                                    #(#pushes)*
                                    #value_crate::DslValue::object([
                                        (#wire_variant.to_string(), #value_crate::DslValue::Object(content_entries)),
                                    ])
                                }
                            })
                        }
                        other => Err(syn::Error::new_spanned(other, "#[derive(ToValue)] externally-tagged enum variants must be unit, a single unnamed payload, or named fields")),
                    };
                    arm
                })
                .collect::<syn::Result<Vec<_>>>()?;
            quote! {
                match self { #(#arms),* }
            }
        }
        Data::Enum(data) => {
            let Some(tag) = &container.tag else {
                unreachable!("the tag.is_none() arm above already handles every tag-less enum");
            };
            let arms = data
                .variants
                .iter()
                .map(|variant| {
                    let variant_ident = &variant.ident;
                    let variant_attrs = parse_variant_attrs(&variant.attrs)?;
                    let wire_variant = variant_wire_name(&variant_ident.to_string(), &variant_attrs.rename, &container.rename_all);
                    let arm: syn::Result<proc_macro2::TokenStream> = match (&variant.fields, &container.content) {
                        (Fields::Unit, _) => Ok(quote! {
                            Self::#variant_ident => #value_crate::DslValue::object([(#tag.to_string(), #value_crate::DslValue::String(#wire_variant.to_string()))])
                        }),
                        (Fields::Unnamed(unnamed), Some(content)) if unnamed.unnamed.len() == 1 => Ok(quote! {
                            Self::#variant_ident(payload) => #value_crate::DslValue::object([
                                (#tag.to_string(), #value_crate::DslValue::String(#wire_variant.to_string())),
                                (#content.to_string(), #value_crate::ToValue::to_value(payload)),
                            ])
                        }),
                        (Fields::Unnamed(unnamed), None) if unnamed.unnamed.len() == 1 => Ok(quote! {
                            Self::#variant_ident(payload) => {
                                let mut entries = match #value_crate::ToValue::to_value(payload) {
                                    #value_crate::DslValue::Object(entries) => entries,
                                    other => vec![("value".to_string(), other)],
                                };
                                entries.insert(0, (#tag.to_string(), #value_crate::DslValue::String(#wire_variant.to_string())));
                                #value_crate::DslValue::Object(entries)
                            }
                        }),
                        (Fields::Named(named), Some(content)) => {
                            let push_into = quote! { content_entries };
                            let pushes = named
                                .named
                                .iter()
                                .map(|field| {
                                    let field_attrs = parse_field_attrs(&field.attrs).unwrap_or_default();
                                    let ident = field.ident.clone().expect("named field");
                                    let wire_name = field_wire_name(&ident.to_string(), &field_attrs.rename, &container.field_rename_all(&variant_attrs));
                                    variant_field_to_value_push(field, &field_attrs, &wire_name, &ident, &push_into, &value_crate)
                                })
                                .collect::<syn::Result<Vec<_>>>()?;
                            let idents = variant_destructure_patterns(named);
                            Ok(quote! {
                                Self::#variant_ident { #(#idents),* } => {
                                    let mut content_entries: Vec<(String, #value_crate::DslValue)> = Vec::new();
                                    #(#pushes)*
                                    #value_crate::DslValue::object([
                                        (#tag.to_string(), #value_crate::DslValue::String(#wire_variant.to_string())),
                                        (#content.to_string(), #value_crate::DslValue::Object(content_entries)),
                                    ])
                                }
                            })
                        }
                        (Fields::Named(named), None) => {
                            // 🛡️ `__out_entries`, not `entries` — a user field literally named `entries`
                            // (e.g. `SemioValue::Map { entries: Vec<SemioValueEntry> }`) would otherwise
                            // shadow the accumulator once `#(#idents),*` destructures it into scope, making
                            // `ToValue::to_value(#ident)` resolve to the accumulator itself (an owned
                            // `Vec<(String, DslValue)>`) instead of the field's `&Vec<SemioValueEntry>`.
                            let push_into = quote! { __out_entries };
                            let pushes = named
                                .named
                                .iter()
                                .map(|field| {
                                    let field_attrs = parse_field_attrs(&field.attrs).unwrap_or_default();
                                    let ident = field.ident.clone().expect("named field");
                                    let wire_name = field_wire_name(&ident.to_string(), &field_attrs.rename, &container.field_rename_all(&variant_attrs));
                                    variant_field_to_value_push(field, &field_attrs, &wire_name, &ident, &push_into, &value_crate)
                                })
                                .collect::<syn::Result<Vec<_>>>()?;
                            let idents = variant_destructure_patterns(named);
                            Ok(quote! {
                                Self::#variant_ident { #(#idents),* } => {
                                    let mut __out_entries: Vec<(String, #value_crate::DslValue)> = vec![(#tag.to_string(), #value_crate::DslValue::String(#wire_variant.to_string()))];
                                    #(#pushes)*
                                    #value_crate::DslValue::Object(__out_entries)
                                }
                            })
                        }
                        (other, _) => Err(syn::Error::new_spanned(other, "#[derive(ToValue)] enum variants must be unit, a single unnamed payload, or named fields")),
                    };
                    arm
                })
                .collect::<syn::Result<Vec<_>>>()?;
            quote! {
                match self { #(#arms),* }
            }
        }
        Data::Union(_) => return Err(syn::Error::new_spanned(&input.ident, "#[derive(ToValue)] does not support unions")),
    };

    let (path_body, shape_body, key_body) = match &input.data {
        Data::Struct(data) if container.transparent => match &data.fields {
            Fields::Named(named) if named.named.len() == 1 => {
                let ident = named.named.first().expect("checked len == 1").ident.clone().expect("named field");
                (
                    quote! { #value_crate::ToValue::value_at_path(&self.#ident, path) },
                    quote! { #value_crate::ToValue::value_shape_at_path(&self.#ident, path) },
                    quote! { #value_crate::ToValue::value_key_at_path(&self.#ident, path, index) },
                )
            }
            Fields::Unnamed(unnamed) if unnamed.unnamed.len() == 1 => (
                quote! { #value_crate::ToValue::value_at_path(&self.0, path) },
                quote! { #value_crate::ToValue::value_shape_at_path(&self.0, path) },
                quote! { #value_crate::ToValue::value_key_at_path(&self.0, path, index) },
            ),
            other => return Err(syn::Error::new_spanned(other, "#[value(transparent)] requires exactly one field")),
        },
        Data::Struct(data) if matches!(&data.fields, Fields::Unnamed(unnamed) if unnamed.unnamed.len() == 1) => (
            quote! { #value_crate::ToValue::value_at_path(&self.0, path) },
            quote! { #value_crate::ToValue::value_shape_at_path(&self.0, path) },
            quote! { #value_crate::ToValue::value_key_at_path(&self.0, path, index) },
        ),
        Data::Struct(data) => {
            let fields = named_fields(&data.fields, &container)?;
            (
                struct_to_path_body(&fields, &value_crate, "value_at_path"),
                struct_to_path_body(&fields, &value_crate, "value_shape_at_path"),
                struct_key_at_path_body(&fields, &value_crate),
            )
        }
        Data::Enum(data) => (
            enum_to_path_body(data, &container, &value_crate, "value_at_path")?,
            enum_to_path_body(data, &container, &value_crate, "value_shape_at_path")?,
            enum_key_at_path_body(data, &container, &value_crate)?,
        ),
        Data::Union(_) => unreachable!("union rejected above"),
    };

    let controlled_body=controlled_to_body(input,&container,&value_crate)?;

    Ok(quote! {
        impl #impl_generics #value_crate::ToValue for #name #ty_generics #where_clause {
            fn to_value(&self) -> #value_crate::DslValue {
                #body
            }
            fn to_value_controlled(&self,control:&mut #value_crate::NativeEncodeControl<'_>)->::core::result::Result<#value_crate::DslValue,#value_crate::ValueError>{
                control.scoped_depth(64,|control|control.scoped_stage(|control|{#controlled_body}))
            }

            fn value_at_path(&self, path: &[&str]) -> ::core::result::Result<#value_crate::DslValue, #value_crate::ValueError> {
                #path_body
            }

            fn value_shape_at_path(&self, path: &[&str]) -> ::core::result::Result<#value_crate::ValueShape, #value_crate::ValueError> {
                #shape_body
            }

            fn value_key_at_path(&self, path: &[&str], index: usize) -> ::core::result::Result<String, #value_crate::ValueError> {
                #key_body
            }
        }
    })
}

pub fn expand_from_value(input: &DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    let name = &input.ident;
    let container = parse_container_attrs(&input.attrs)?;
    let value_crate = container_crate_path(&container);
    let generics = generics_with_bound(&input.generics, &container.bound, &quote! { #value_crate::FromValue });
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let body = match &input.data {
        Data::Struct(data) if container.transparent => match &data.fields {
            Fields::Named(named) if named.named.len() == 1 => {
                let ident = named.named.first().expect("checked len == 1").ident.clone().expect("named field");
                quote! { Ok(Self { #ident: #value_crate::FromValue::from_value(value)? }) }
            }
            Fields::Unnamed(unnamed) if unnamed.unnamed.len() == 1 => quote! { Ok(Self(#value_crate::FromValue::from_value(value)?)) },
            other => return Err(syn::Error::new_spanned(other, "#[value(transparent)] requires exactly one field")),
        },
        // 🆔 Single-field tuple struct: sibling of `expand_to_value`'s identical guard above.
        Data::Struct(data) if matches!(&data.fields, Fields::Unnamed(unnamed) if unnamed.unnamed.len() == 1) => {
            quote! { Ok(Self(#value_crate::FromValue::from_value(value)?)) }
        }
        Data::Struct(data) => {
            let fields = named_fields(&data.fields, &container)?;
            let reads = from_value_struct_fields(&fields, &container, &value_crate);
            quote! {
                let __entries = #value_crate::DslValue::into_object(value)?;
                #reads
            }
        }
        Data::Enum(data) if container.tag.is_none() && data.variants.iter().all(|variant| matches!(variant.fields, Fields::Unit)) => {
            let arms = data.variants.iter().map(|variant| {
                let variant_ident = &variant.ident;
                let variant_attrs = parse_variant_attrs(&variant.attrs)?;
                let wire_variant = variant_wire_name(&variant_ident.to_string(), &variant_attrs.rename, &container.rename_all);
                Ok(quote! { #wire_variant => Ok(Self::#variant_ident), })
            }).collect::<syn::Result<Vec<_>>>()?;
            quote! {
                let __s = match value { #value_crate::DslValue::String(s) => s, other => return Err(#value_crate::ValueError::new(#value_crate::ValueRefusalKind::InvalidValue, format!("expected a string, found {other:?}"))) };
                match __s.as_str() {
                    #(#arms)*
                    other => Err(#value_crate::ValueError::new(#value_crate::ValueRefusalKind::InvalidValue, format!("unknown variant `{other}`"))),
                }
            }
        }
        Data::Enum(data) if container.tag.is_none() => {
            // 🏷️ Externally-tagged (serde's own default enum representation when no `#[serde(tag
            // = …)]` is present) — mirrors `expand_to_value`'s sibling arm above.
            let string_arms = data.variants.iter().filter(|variant| matches!(variant.fields, Fields::Unit)).map(|variant| {
                let variant_ident = &variant.ident;
                let variant_attrs = parse_variant_attrs(&variant.attrs)?;
                let wire_variant = variant_wire_name(&variant_ident.to_string(), &variant_attrs.rename, &container.rename_all);
                Ok(quote! { #wire_variant => return Ok(Self::#variant_ident), })
            }).collect::<syn::Result<Vec<_>>>()?;
            let object_arms = data
                .variants
                .iter()
                .filter(|variant| !matches!(variant.fields, Fields::Unit))
                .map(|variant| {
                    let variant_ident = &variant.ident;
                    let variant_attrs = parse_variant_attrs(&variant.attrs)?;
                    let wire_variant = variant_wire_name(&variant_ident.to_string(), &variant_attrs.rename, &container.rename_all);
                    let arm: syn::Result<proc_macro2::TokenStream> = match &variant.fields {
                        Fields::Unnamed(unnamed) if unnamed.unnamed.len() == 1 => {
                            let payload_ty = &unnamed.unnamed[0].ty;
                            Ok(quote! {
                                #wire_variant => Self::#variant_ident(<#payload_ty as #value_crate::FromValue>::from_value(__payload)?),
                            })
                        }
                        Fields::Named(named) => {
                            let field_wire_names: Vec<String> = named
                                .named
                                .iter()
                                .map(|field| {
                                    let field_attrs = parse_field_attrs(&field.attrs).unwrap_or_default();
                                    let ident = field.ident.clone().expect("named field");
                                    field_wire_name(&ident.to_string(), &field_attrs.rename, &container.field_rename_all(&variant_attrs))
                                })
                                .collect();
                            let deny_check = if container.deny_unknown_fields {
                                deny_unknown_keys(&quote! { __variant_entries }, &field_wire_names, &value_crate)
                            } else {
                                quote! {}
                            };
                            let entries_ident = quote! { __variant_entries };
                            let reads = named
                                .named
                                .iter()
                                .map(|field| {
                                    let field_attrs = parse_field_attrs(&field.attrs).unwrap_or_default();
                                    let ident = field.ident.clone().expect("named field");
                                    let wire_name = field_wire_name(&ident.to_string(), &field_attrs.rename, &container.field_rename_all(&variant_attrs));
                                    variant_field_from_value_read(field, &field_attrs, &wire_name, &ident, &entries_ident, &value_crate)
                                })
                                .collect::<syn::Result<Vec<_>>>()?;
                            let idents = named.named.iter().map(|field| field.ident.clone().expect("named field"));
                            Ok(quote! {
                                #wire_variant => {
                                    let __variant_entries = #value_crate::DslValue::into_object(__payload)?;
                                    #deny_check
                                    #(#reads)*
                                    Self::#variant_ident { #(#idents),* }
                                },
                            })
                        }
                        other => Err(syn::Error::new_spanned(other, "#[derive(FromValue)] externally-tagged enum variants must be unit, a single unnamed payload, or named fields")),
                    };
                    arm
                })
                .collect::<syn::Result<Vec<_>>>()?;
            quote! {
                if let #value_crate::DslValue::String(__s) = &value {
                    match __s.as_str() {
                        #(#string_arms)*
                        _ => {}
                    }
                }
                let __entries = #value_crate::DslValue::into_object(value)?;
                if __entries.len() != 1 {
                    return Err(#value_crate::ValueError::new(#value_crate::ValueRefusalKind::InvalidValue, format!("expected an externally-tagged enum object with exactly one key, found {} keys", __entries.len())));
                }
                let (__key, __payload) = __entries.into_iter().next().expect("checked len == 1 above");
                Ok(match __key.as_str() {
                    #(#object_arms)*
                    other => return Err(#value_crate::ValueError::new(#value_crate::ValueRefusalKind::InvalidValue, format!("unknown variant `{other}`"))),
                })
            }
        }
        Data::Enum(data) => {
            let Some(tag) = &container.tag else {
                unreachable!("the tag.is_none() arm above already handles every tag-less enum");
            };
            let arms = data
                .variants
                .iter()
                .map(|variant| {
                    let variant_ident = &variant.ident;
                    let variant_attrs = parse_variant_attrs(&variant.attrs)?;
                    let wire_variant = variant_wire_name(&variant_ident.to_string(), &variant_attrs.rename, &container.rename_all);
                    let arm: syn::Result<proc_macro2::TokenStream> = match (&variant.fields, &container.content) {
                        (Fields::Unit, Some(_)) => Ok(quote! {
                            #wire_variant => Self::#variant_ident,
                        }),
                        (Fields::Unit, None) => {
                            // 🛡️ Internally-tagged unit variant: the whole entries object is nothing
                            // but the tag, so `deny_unknown_fields` allows exactly `{tag}`.
                            let deny_check = if container.deny_unknown_fields {
                                deny_unknown_keys(&quote! { __entries }, std::slice::from_ref(tag), &value_crate)
                            } else {
                                quote! {}
                            };
                            Ok(quote! {
                                #wire_variant => { #deny_check Self::#variant_ident },
                            })
                        }
                        (Fields::Unnamed(unnamed), Some(_)) if unnamed.unnamed.len() == 1 => {
                            let payload_ty = &unnamed.unnamed[0].ty;
                            Ok(quote! {
                                #wire_variant => Self::#variant_ident(<#payload_ty as #value_crate::FromValue>::from_value(__content()?)?),
                            })
                        }
                        (Fields::Unnamed(unnamed), None) if unnamed.unnamed.len() == 1 => {
                            // 🩹 Strip the tag key before handing the object to the payload type's
                            // own `FromValue` — `expand_to_value`'s sibling arm never puts the tag
                            // INTO the payload's own entries (it prepends the tag after taking the
                            // payload's `to_value()`, so the payload never emits it either), so
                            // leaving the tag in here was a decode/encode asymmetry: a payload type
                            // that itself carries `#[value(deny_unknown_fields)]` would reject its
                            // own valid wire form because the wrapper's tag key looked unknown to it.
                            //
                            // 🪆 The `__scalar` arm is the exact inverse of `expand_to_value`'s runtime
                            // branch: a payload whose `to_value()` is NOT an object cannot be spliced
                            // beside the tag, so the encoder carries it as the single entry
                            // `{"value": <scalar>}`. Which of the two shapes a given wire object is
                            // cannot be decided from the payload TYPE at expansion time (a struct whose
                            // only field is literally named `value` produces the same key set), so the
                            // object form is attempted first and the carrier is unwrapped only when it
                            // fails — that ordering decodes both shapes correctly, where a key-shape
                            // test alone would mis-decode `struct P { value: String }`.
                            let payload_ty = &unnamed.unnamed[0].ty;
                            Ok(quote! {
                                #wire_variant => Self::#variant_ident({
                                    let __payload: Vec<(String, #value_crate::DslValue)> = __entries.iter().filter(|(__k, _)| __k != #tag).cloned().collect();
                                    match <#payload_ty as #value_crate::FromValue>::from_value(#value_crate::DslValue::Object(__payload.clone())) {
                                        ::core::result::Result::Ok(__decoded) => __decoded,
                                        ::core::result::Result::Err(__object_error) => match __payload.as_slice() {
                                            [(__k, __scalar)] if __k == "value" => <#payload_ty as #value_crate::FromValue>::from_value(__scalar.clone())?,
                                            _ => return ::core::result::Result::Err(__object_error),
                                        },
                                    }
                                }),
                            })
                        }
                        (Fields::Named(named), content_key) => {
                            let source = if content_key.is_some() {
                                quote! { __content()?.into_object()? }
                            } else {
                                quote! { __entries.clone() }
                            };
                            let field_wire_names: Vec<String> = named
                                .named
                                .iter()
                                .map(|field| {
                                    let field_attrs = parse_field_attrs(&field.attrs).unwrap_or_default();
                                    let ident = field.ident.clone().expect("named field");
                                    field_wire_name(&ident.to_string(), &field_attrs.rename, &container.field_rename_all(&variant_attrs))
                                })
                                .collect();
                            let deny_check = if container.deny_unknown_fields {
                                let allowed: Vec<String> = if content_key.is_some() {
                                    field_wire_names
                                } else {
                                    let mut allowed = vec![tag.clone()];
                                    allowed.extend(field_wire_names);
                                    allowed
                                };
                                deny_unknown_keys(&quote! { __variant_entries }, &allowed, &value_crate)
                            } else {
                                quote! {}
                            };
                            let entries_ident = quote! { __variant_entries };
                            let reads = named
                                .named
                                .iter()
                                .map(|field| {
                                    let field_attrs = parse_field_attrs(&field.attrs).unwrap_or_default();
                                    let ident = field.ident.clone().expect("named field");
                                    let wire_name = field_wire_name(&ident.to_string(), &field_attrs.rename, &container.field_rename_all(&variant_attrs));
                                    variant_field_from_value_read(field, &field_attrs, &wire_name, &ident, &entries_ident, &value_crate)
                                })
                                .collect::<syn::Result<Vec<_>>>()?;
                            let idents = named.named.iter().map(|field| field.ident.clone().expect("named field"));
                            Ok(quote! {
                                #wire_variant => {
                                    let __variant_entries = #source;
                                    #deny_check
                                    #(#reads)*
                                    Self::#variant_ident { #(#idents),* }
                                },
                            })
                        }
                        (other, _) => Err(syn::Error::new_spanned(other, "#[derive(FromValue)] enum variants must be unit, a single unnamed payload, or named fields")),
                    };
                    arm
                })
                .collect::<syn::Result<Vec<_>>>()?;
            let content_helper = match &container.content {
                Some(content) => quote! {
                    let __content = || -> ::core::result::Result<#value_crate::DslValue, #value_crate::ValueError> {
                        __entries.iter().find(|(k, _)| k == #content).map(|(_, v)| v.clone()).ok_or_else(|| #value_crate::ValueError::new(#value_crate::ValueRefusalKind::InvalidValue, format!("missing content field `{}`", #content)))
                    };
                },
                None => quote! {},
            };
            // 🛡️ Adjacently-tagged outer-level `deny_unknown_fields`: the allowed key set here is
            // just `{tag, content}` regardless of which variant matches, so this check runs once,
            // before the tag is even read — unlike the internally-tagged case (no `content`),
            // where the allowed set depends on the variant and is checked per-arm above instead.
            let outer_deny_check = match (&container.content, container.deny_unknown_fields) {
                (Some(content), true) => deny_unknown_keys(&quote! { __entries }, &[tag.clone(), content.clone()], &value_crate),
                _ => quote! {},
            };
            quote! {
                let __entries = #value_crate::DslValue::into_object(value)?;
                #outer_deny_check
                let __tag = __entries.iter().find(|(k, _)| k == #tag).map(|(_, v)| v.clone()).ok_or_else(|| #value_crate::ValueError::new(#value_crate::ValueRefusalKind::InvalidValue, format!("missing tag field `{}`", #tag)))?;
                let __tag = match __tag { #value_crate::DslValue::String(s) => s, other => return Err(#value_crate::ValueError::new(#value_crate::ValueRefusalKind::InvalidValue, format!("expected a string tag, found {other:?}"))) };
                #content_helper
                Ok(match __tag.as_str() {
                    #(#arms)*
                    other => return Err(#value_crate::ValueError::new(#value_crate::ValueRefusalKind::InvalidValue, format!("unknown `{}` variant `{other}`", #tag))),
                })
            }
        }
        Data::Union(_) => return Err(syn::Error::new_spanned(&input.ident, "#[derive(FromValue)] does not support unions")),
    };

    let controlled_body = controlled_from_body(input, &container, &value_crate)?;
    let retirement_body = controlled_retirement_body(input, &container, &value_crate)?;
    let controlled_default_method=if let Some(path)=&container.default_controlled{let path:syn::Path=syn::parse_str(path)?;quote!{fn default_value_controlled(control:&mut #value_crate::NativeDecodeControl<'_>)->Result<Self,#value_crate::ValueError>{#path(control)}}}else{quote!{}};
    let edit_body = match &input.data {
        Data::Struct(data) if container.transparent => match &data.fields {
            Fields::Named(named) if named.named.len() == 1 => {
                let ident = named.named.first().expect("checked len == 1").ident.clone().expect("named field");
                quote! { #value_crate::FromValue::edit_value_at_path(&mut self.#ident, path, edit) }
            }
            Fields::Unnamed(unnamed) if unnamed.unnamed.len() == 1 => quote! { #value_crate::FromValue::edit_value_at_path(&mut self.0, path, edit) },
            other => return Err(syn::Error::new_spanned(other, "#[value(transparent)] requires exactly one field")),
        },
        Data::Struct(data) if matches!(&data.fields, Fields::Unnamed(unnamed) if unnamed.unnamed.len() == 1) => {
            quote! { #value_crate::FromValue::edit_value_at_path(&mut self.0, path, edit) }
        }
        Data::Struct(data) => {
            let fields = named_fields(&data.fields, &container)?;
            struct_edit_path_body(&fields, &value_crate)
        }
        Data::Enum(data) => enum_edit_path_body(data, &container, &value_crate)?,
        Data::Union(_) => unreachable!("union rejected above"),
    };

    Ok(quote! {
        impl #impl_generics #value_crate::FromValue for #name #ty_generics #where_clause {
            fn from_value(value: #value_crate::DslValue) -> ::core::result::Result<Self, #value_crate::ValueError> {
                #body
            }

            fn from_value_controlled(value: &#value_crate::DslValue, control: &mut #value_crate::NativeDecodeControl<'_>) -> ::core::result::Result<Self, #value_crate::ValueError> {
                control.scoped_depth(64, |control| control.scoped_stage(|control| { #controlled_body }))
            }
            #controlled_default_method
            fn retire_decoded(self) { #retirement_body }
            fn edit_value_at_path(&mut self, path: &[&str], edit: #value_crate::ValueEdit) -> ::core::result::Result<(), #value_crate::ValueError> {
                #edit_body
            }
        }
    })
}
//#endregion 🔖️Expand

fn controlled_field_decode(ty:&syn::Type,attrs:&FieldAttrs,value:proc_macro2::TokenStream,c:&syn::Path)->syn::Result<proc_macro2::TokenStream>{
    if attrs.effective_deserialize_with().is_some()&&attrs.retire_with.is_none(){return Ok(quote!{Err(#c::ValueError::new(#c::ValueRefusalKind::UnsupportedOwner, "custom controlled value conversion requires explicit retirement"))})}
    if let Some(path)=attrs.deserialize_controlled_with.clone(){
        let path:syn::Path=syn::parse_str(&path)?;return Ok(quote!{#path(#value,control)})
    }
    if attrs.effective_deserialize_with().is_some(){return Ok(quote!{Err(#c::ValueError::new(#c::ValueRefusalKind::UnsupportedOwner, "custom value conversion has no controlled constructor"))})}
    Ok(quote!{<#ty as #c::FromValue>::from_value_controlled(#value,control)})
}

fn controlled_field_default(ty:&syn::Type,attrs:&FieldAttrs,container:&ContainerAttrs,wire:&str,c:&syn::Path)->syn::Result<proc_macro2::TokenStream>{
    if attrs.skip&&attrs.retire_with.is_none(){return Ok(quote!{Err(#c::ValueError::new(#c::ValueRefusalKind::UnsupportedOwner, "skipped controlled default requires explicit retirement"))})}
    if attrs.required&&!attrs.skip{return Ok(quote!{Err(#c::ValueError::new(#c::ValueRefusalKind::InvalidValue, format!("missing field `{}`",#wire)))})}
    if let Some(path)=&attrs.default_controlled{let path:syn::Path=syn::parse_str(path)?;return Ok(quote!{#path(control)})}
    if matches!(attrs.default,FieldDefault::Path(_))||attrs.skip{return Ok(quote!{Err(#c::ValueError::new(#c::ValueRefusalKind::UnsupportedOwner, "custom or skipped default has no controlled constructor"))})}
    if matches!(attrs.default,FieldDefault::Bare)||container.default||type_is_option(ty){
        if attrs.effective_deserialize_with().is_some(){return Ok(quote!{Err(#c::ValueError::new(#c::ValueRefusalKind::UnsupportedOwner, "custom value default has no controlled constructor"))})}
        return Ok(quote!{<#ty as #c::FromValue>::default_value_controlled(control)})
    }
    Ok(quote!{Err(#c::ValueError::new(#c::ValueRefusalKind::InvalidValue, format!("missing field `{}`",#wire)))})
}

fn controlled_field_retire(ty:&syn::Type,attrs:&FieldAttrs,value:proc_macro2::TokenStream,c:&syn::Path)->syn::Result<proc_macro2::TokenStream>{
    if let Some(path)=&attrs.retire_with{let path:syn::Path=syn::parse_str(path)?;return Ok(quote!{#path(#value)})}
    if attrs.skip||attrs.effective_deserialize_with().is_some(){return Ok(quote!{drop(#value)})}
    Ok(quote!{<#ty as #c::FromValue>::retire_decoded(#value)})
}

fn controlled_named_fields(fields:&syn::FieldsNamed,container:&ContainerAttrs,rename:&Option<String>,entries:proc_macro2::TokenStream,extra:&[String],constructor:proc_macro2::TokenStream,c:&syn::Path)->syn::Result<proc_macro2::TokenStream>{
    let mut reads=Vec::new();let mut members=Vec::new();let mut names=Vec::new();
    for field in &fields.named{let attrs=parse_field_attrs(&field.attrs)?;let ident=field.ident.as_ref().unwrap();let wire=field_wire_name(&ident.to_string(),&attrs.rename,rename);if !attrs.flatten{names.push(wire.clone())}}
    let allowed:Vec<_>=names.iter().chain(extra.iter()).collect();
    let deny=if container.deny_unknown_fields{quote!{#c::DslValue::deny_fields_controlled(#entries,&[#(#allowed),*],control)?;}}else{quote!{}};
    for field in &fields.named{
        let attrs=parse_field_attrs(&field.attrs)?;let ident=field.ident.as_ref().unwrap();let ty=&field.ty;let wire=field_wire_name(&ident.to_string(),&attrs.rename,rename);let guard=format_ident!("__owned_{}",ident);
        let missing=controlled_field_default(ty,&attrs,container,&wire,c)?;
        let expression=if attrs.skip{missing}else if attrs.flatten{
            let decode=controlled_field_decode(ty,&attrs,quote!{__remaining.get()},c)?;
            quote!{{let __remaining=<#c::DslValue as #c::FromValue>::guard_decoded(#c::DslValue::filtered_object_controlled(#entries,&[#(#names),*],control)?);#decode}}
        }else{
            let decode=controlled_field_decode(ty,&attrs,quote!{__field},c)?;
            quote!{match #c::DslValue::field_controlled(#entries,#wire,control)?{Some(__field)=>#decode,None=>#missing}}
        };
        let retire=controlled_field_retire(ty,&attrs,quote!{__value},c)?;
        if attrs.effective_deserialize_with().is_some()||attrs.skip||attrs.retire_with.is_some(){
            reads.push(quote!{let #guard:#ty=#expression.map_err(|error:#c::ValueError|error.under(#wire))?;let #guard= #c::DecodedValue::new(#guard,|__value:#ty|{#retire});control.step()?;});
        }else{
            reads.push(quote!{let #guard=<#ty as #c::FromValue>::guard_decoded(#expression.map_err(|error:#c::ValueError|error.under(#wire))?);control.step()?;});
        }
        members.push(quote!{#ident:#guard.take()});
    }
    let count=fields.named.len();
    Ok(quote!{#deny control.begin_stage(#count)?;#(#reads)* Ok(#constructor{#(#members),*})})
}

fn type_mentions_owner(ty:&syn::Type,owner:&syn::Ident)->bool {
    match ty {
        syn::Type::Path(path)=>path.path.segments.last().is_some_and(|segment| {
            segment.ident==*owner||segment.ident=="Self"||match &segment.arguments {
                syn::PathArguments::AngleBracketed(arguments)=>arguments.args.iter().any(|argument|match argument {
                    syn::GenericArgument::Type(ty)=>type_mentions_owner(ty,owner),
                    syn::GenericArgument::AssocType(binding)=>type_mentions_owner(&binding.ty,owner),
                    _=>false,
                }),
                _=>false,
            }
        }),
        syn::Type::Array(array)=>type_mentions_owner(&array.elem,owner),
        syn::Type::Slice(slice)=>type_mentions_owner(&slice.elem,owner),
        syn::Type::Tuple(tuple)=>tuple.elems.iter().any(|ty|type_mentions_owner(ty,owner)),
        syn::Type::Paren(paren)=>type_mentions_owner(&paren.elem,owner),
        syn::Type::Group(group)=>type_mentions_owner(&group.elem,owner),
        syn::Type::Reference(reference)=>type_mentions_owner(&reference.elem,owner),
        syn::Type::Ptr(pointer)=>type_mentions_owner(&pointer.elem,owner),
        _=>false,
    }
}

fn controlled_from_body(input:&DeriveInput,container:&ContainerAttrs,c:&syn::Path)->syn::Result<proc_macro2::TokenStream>{
    let recursive=input.data.clone();
    let mentions=match &recursive{Data::Struct(data)=>data.fields.iter().any(|f|type_mentions_owner(&f.ty,&input.ident)),Data::Enum(data)=>data.variants.iter().flat_map(|v|v.fields.iter()).any(|f|type_mentions_owner(&f.ty,&input.ident)),_=>false};
    if mentions&&container.retire_with.is_none(){return Ok(quote!{control.checkpoint()?;Err(#c::ValueError::new(#c::ValueRefusalKind::InvalidValue, "recursive value owner requires explicit controlled retirement"))})}
    match &input.data{
        Data::Struct(data) if container.transparent||matches!(&data.fields,Fields::Unnamed(f)if f.unnamed.len()==1)=>{
            let field=data.fields.iter().next().unwrap();let attrs=parse_field_attrs(&field.attrs)?;let ty=&field.ty;let decode=controlled_field_decode(ty,&attrs,quote!{value},c)?;
            Ok(if let Some(ident)=&field.ident{quote!{control.begin_stage(0)?;Ok(Self{#ident:#decode?})}}else{quote!{control.begin_stage(0)?;Ok(Self(#decode?))}})
        },
        Data::Struct(data)=>{
            let Fields::Named(fields)=&data.fields else{return Err(syn::Error::new_spanned(&data.fields,"controlled value requires named fields"))};
            let fields=controlled_named_fields(fields,container,&container.rename_all,quote!{__entries},&[],quote!{Self},c)?;
            Ok(quote!{let __entries=value.object_controlled(control)?;#fields})
        },
        Data::Enum(data)=>{
            let mut string_arms=Vec::new();let mut object_arms=Vec::new();
            for variant in &data.variants{
                let attrs=parse_variant_attrs(&variant.attrs)?;let ident=&variant.ident;let wire=variant_wire_name(&ident.to_string(),&attrs.rename,&container.rename_all);
                if matches!(variant.fields,Fields::Unit){
                    string_arms.push(quote!{#wire=>Ok(Self::#ident),});
                    let deny=if container.deny_unknown_fields&&container.content.is_none(){let allowed:Vec<_>=container.tag.iter().collect();quote!{#c::DslValue::deny_fields_controlled(__entries,&[#(#allowed),*],control)?;}}else{quote!{}};
                    object_arms.push(quote!{#wire=>{#deny Ok(Self::#ident)},});continue
                }
                let payload=match(&container.tag,&container.content){
                    (None,_)=>quote!{__payload},
                    (Some(_),Some(content))=>quote!{#c::DslValue::field_controlled(__entries,#content,control)?.ok_or_else(||#c::ValueError::new(#c::ValueRefusalKind::InvalidValue, "missing enum content"))?},
                    (Some(_),None)=>quote!{value},
                };
                let body=match &variant.fields{
                    Fields::Unnamed(fields)if fields.unnamed.len()==1=>{
                        let field=fields.unnamed.first().unwrap();let ty=&field.ty;
                        if let(Some(tag),None)=(&container.tag,&container.content){
                            quote!{
                                let __remaining=<#c::DslValue as #c::FromValue>::guard_decoded(#c::DslValue::filtered_object_controlled(__entries,&[#tag],control)?);
                                match <#ty as #c::FromValue>::from_value_controlled(__remaining.get(),control){
                                    Ok(payload)=>Ok(Self::#ident(payload)),
                                    Err(error)=>match __remaining.get(){#c::DslValue::Object(fields)=>match fields.as_slice(){[(key,payload)]if key=="value"=>Ok(Self::#ident(<#ty as #c::FromValue>::from_value_controlled(payload,control)?)),_=>Err(error)},_=>Err(error)}
                                }
                            }
                        }else{let attrs=parse_field_attrs(&field.attrs)?;let decode=controlled_field_decode(ty,&attrs,payload,c)?;quote!{Ok(Self::#ident(#decode?))}}
                    },
                    Fields::Named(fields)=>{
                        let extra=if container.content.is_none(){container.tag.iter().cloned().collect::<Vec<_>>()}else{Vec::new()};
                        let body=controlled_named_fields(fields,container,&container.field_rename_all(&attrs),quote!{__variant_entries},&extra,quote!{Self::#ident},c)?;
                        quote!{let __variant_entries=(#payload).object_controlled(control)?;#body}
                    },
                    _=>return Err(syn::Error::new_spanned(&variant.fields,"unsupported controlled enum fields"))
                };
                object_arms.push(quote!{#wire=>{#body},});
            }
            if container.tag.is_none()&&data.variants.iter().all(|v|matches!(v.fields,Fields::Unit)){
                return Ok(quote!{control.begin_stage(1)?;control.step()?;let #c::DslValue::String(tag)=value else{return Err(#c::ValueError::new(#c::ValueRefusalKind::InvalidValue, "expected enum string"))};match tag.as_str(){#(#string_arms)*_=>Err(#c::ValueError::new(#c::ValueRefusalKind::InvalidValue, "unknown enum variant"))}})
            }
            if let Some(tag)=&container.tag{
                let deny=if let(Some(content),true)=(&container.content,container.deny_unknown_fields){quote!{#c::DslValue::deny_fields_controlled(__entries,&[#tag,#content],control)?;}}else{quote!{}};
                Ok(quote!{let __entries=value.object_controlled(control)?;#deny let __tag=#c::DslValue::field_controlled(__entries,#tag,control)?.ok_or_else(||#c::ValueError::new(#c::ValueRefusalKind::InvalidValue, "missing enum tag"))?;let #c::DslValue::String(__tag)=__tag else{return Err(#c::ValueError::new(#c::ValueRefusalKind::InvalidValue, "expected enum string tag"))};match __tag.as_str(){#(#object_arms)*_=>Err(#c::ValueError::new(#c::ValueRefusalKind::InvalidValue, "unknown enum variant"))}})
            }else{
                Ok(quote!{control.begin_stage(0)?;if let #c::DslValue::String(tag)=value{return match tag.as_str(){#(#string_arms)*_=>Err(#c::ValueError::new(#c::ValueRefusalKind::InvalidValue, "unknown enum variant"))}}let __entries=value.object_controlled(control)?;let[(__tag,__payload)]=__entries else{return Err(#c::ValueError::new(#c::ValueRefusalKind::InvalidValue, "externally tagged enum requires one field"))};match __tag.as_str(){#(#object_arms)*_=>Err(#c::ValueError::new(#c::ValueRefusalKind::InvalidValue, "unknown enum variant"))}})
            }
        },
        Data::Union(_)=>Err(syn::Error::new_spanned(input,"controlled unions unsupported"))
    }
}

fn controlled_retirement_body(input:&DeriveInput,container:&ContainerAttrs,c:&syn::Path)->syn::Result<proc_macro2::TokenStream>{
    if let Some(path)=&container.retire_with{let path:syn::Path=syn::parse_str(path)?;return Ok(quote!{#path(self);})}
    let body=|fields:&Fields,constructor:proc_macro2::TokenStream|->syn::Result<proc_macro2::TokenStream>{
        let mut names=Vec::new();let mut retirements=Vec::new();
        for(index,field)in fields.iter().enumerate(){let name=field.ident.clone().unwrap_or_else(||format_ident!("__field_{index}"));let attrs=parse_field_attrs(&field.attrs)?;retirements.push(controlled_field_retire(&field.ty,&attrs,quote!{#name},c)?);names.push(name);}
        let pattern=match fields{Fields::Named(_)=>quote!{#constructor{#(#names),*}},Fields::Unnamed(_)=>quote!{#constructor(#(#names),*)},Fields::Unit=>constructor};
        Ok(quote!{#pattern=>{#(#retirements;)*}})
    };
    match &input.data{Data::Struct(data)=>{let arm=body(&data.fields,quote!{Self})?;Ok(quote!{match self{#arm}})},Data::Enum(data)=>{let arms=data.variants.iter().map(|v|{let name=&v.ident;body(&v.fields,quote!{Self::#name})}).collect::<syn::Result<Vec<_>>>()?;Ok(quote!{match self{#(#arms),*}})},Data::Union(_)=>Ok(quote!{drop(self)})}
}

fn controlled_field_encode(attrs:&FieldAttrs,value:proc_macro2::TokenStream,c:&syn::Path)->syn::Result<proc_macro2::TokenStream>{
 if let Some(path)=&attrs.serialize_controlled_with{let path:syn::Path=syn::parse_str(path)?;return Ok(quote!{#path(#value,control)})}
 if attrs.effective_serialize_with().is_some(){return Ok(quote!{Err(#c::ValueError::new(#c::ValueRefusalKind::UnsupportedOwner, "custom value conversion has no controlled encoder"))})}
 Ok(quote!{#c::ToValue::to_value_controlled(#value,control)})
}

fn controlled_named_output(fields:&syn::FieldsNamed,rename:&Option<String>,source_self:bool,c:&syn::Path)->syn::Result<proc_macro2::TokenStream>{
 let count=fields.named.len();let mut flags=Vec::new();let mut capacities=Vec::new();let mut pushes=Vec::new();
 for(index,field)in fields.named.iter().enumerate(){
  let ident=field.ident.as_ref().unwrap();let flag=format_ident!("__emit_{index}");let attrs=parse_field_attrs(&field.attrs)?;let wire=field_wire_name(&ident.to_string(),&attrs.rename,rename);let alias=format_ident!("__source_field_{index}");let access=if source_self{quote!{&self.#ident}}else{quote!{#alias}};
  let flatten=attrs.flatten&&source_self;
  let emit=if attrs.skip{quote!{false}}else if !flatten{if let Some(path)=&attrs.skip_serializing_if{let path:syn::Path=syn::parse_str(path)?;quote!{!#path(#access)}}else{quote!{true}}}else{quote!{true}};
  flags.push(quote!{let #flag=#emit;control.step()?;});
  if attrs.skip{pushes.push(quote!{control.step()?;});continue}
  if !flatten{capacities.push(quote!{::core::primitive::usize::from(#flag)});}
  let value=controlled_field_encode(&attrs,access,c)?;
  let push=if flatten{quote!{#c::DslValue::flatten_encoding_controlled(__output.get_mut(),#value?,control)?;}}else{quote!{#c::DslValue::push_encoding_controlled(__output.get_mut(),#wire,#value?,control)?;}};
  pushes.push(quote!{if #flag{#push}control.step()?;});
 }
 Ok(quote!{control.begin_stage(#count)?;#(#flags)*control.begin_stage(#count)?;let mut __output=#c::DslValue::object_encoding_controlled(0usize #( + #capacities )*,control)?;#(#pushes)*Ok(#c::DslValue::Object(__output.take()))})
}

fn controlled_to_body(input:&DeriveInput,container:&ContainerAttrs,c:&syn::Path)->syn::Result<proc_macro2::TokenStream>{
 match &input.data{
  Data::Enum(data)if data.variants.is_empty()=>Ok(quote!{match *self{}}),
  Data::Struct(data)if container.transparent||matches!(&data.fields,Fields::Unnamed(fields)if fields.unnamed.len()==1)=>{
   let field=data.fields.iter().next().ok_or_else(||syn::Error::new_spanned(input,"transparent output requires field"))?;let access=if let Some(ident)=&field.ident{quote!{&self.#ident}}else{quote!{&self.0}};controlled_field_encode(&parse_field_attrs(&field.attrs)?,access,c)
  }
  Data::Struct(data)=>{let Fields::Named(fields)=&data.fields else{return Err(syn::Error::new_spanned(input,"controlled output requires named fields"))};controlled_named_output(fields,&container.rename_all,true,c)}
  Data::Enum(data)=>{
   let mut arms=Vec::new();
   for variant in &data.variants{
    let ident=&variant.ident;let attrs=parse_variant_attrs(&variant.attrs)?;let wire=variant_wire_name(&ident.to_string(),&attrs.rename,&container.rename_all);
    let(pattern,payload)=match &variant.fields{
     Fields::Unit=>(quote!{Self::#ident},None),
     Fields::Unnamed(fields)if fields.unnamed.len()==1=>{let field=fields.unnamed.first().unwrap();(quote!{Self::#ident(__payload)},Some(controlled_field_encode(&parse_field_attrs(&field.attrs)?,quote!{__payload},c)?))},
     Fields::Named(fields)=>{let idents=fields.named.iter().enumerate().map(|(index,field)|{let name=field.ident.as_ref().unwrap();let alias=format_ident!("__source_field_{index}");if parse_field_attrs(&field.attrs)?.skip{Ok(quote!{#name:_})}else{Ok(quote!{#name:#alias})}}).collect::<syn::Result<Vec<_>>>()?;(quote!{Self::#ident{#(#idents),*}},Some(controlled_named_output(fields,&container.field_rename_all(&attrs),false,c)?))},
     _=>return Err(syn::Error::new_spanned(variant,"unsupported controlled output fields"))
    };
    let body=match(&container.tag,&container.content,payload){
     (None,_,None)=>quote!{control.copy_text(#wire).map(#c::DslValue::String)},
     (None,_,Some(payload))=>quote!{let __payload=#c::DslValue::guard_encoded((||->::core::result::Result<#c::DslValue,#c::ValueError>{#payload})()?);let mut __wrapper=#c::DslValue::object_encoding_controlled(1,control)?;#c::DslValue::push_encoding_controlled(__wrapper.get_mut(),#wire,__payload.take(),control)?;Ok(#c::DslValue::Object(__wrapper.take()))},
     (Some(tag),content,payload)=>{
      let count=if payload.is_some(){2usize}else{1};let payload_push=match(payload,content){
       (None,_)=>quote!{},
       (Some(payload),Some(content))=>quote!{let __payload=#c::DslValue::guard_encoded((||->::core::result::Result<#c::DslValue,#c::ValueError>{#payload})()?);#c::DslValue::push_encoding_controlled(__wrapper.get_mut(),#content,__payload.take(),control)?;},
       (Some(payload),None)=>quote!{let __payload=#c::DslValue::guard_encoded((||->::core::result::Result<#c::DslValue,#c::ValueError>{#payload})()?);if matches!(__payload.get(),#c::DslValue::Object(_)){#c::DslValue::flatten_encoding_controlled(__wrapper.get_mut(),__payload.take(),control)?;}else{#c::DslValue::push_encoding_controlled(__wrapper.get_mut(),"value",__payload.take(),control)?;}}
      };
      quote!{let mut __wrapper=#c::DslValue::object_encoding_controlled(#count,control)?;let __tag=control.copy_text(#wire).map(#c::DslValue::String)?;#c::DslValue::push_encoding_controlled(__wrapper.get_mut(),#tag,__tag,control)?;#payload_push Ok(#c::DslValue::Object(__wrapper.take()))}
     }
    };
    arms.push(quote!{#pattern=>{#body}});
   }
   Ok(quote!{match self{#(#arms),*}})
  }
  Data::Union(_)=>Err(syn::Error::new_spanned(input,"controlled output unions unsupported"))
 }
}

```
