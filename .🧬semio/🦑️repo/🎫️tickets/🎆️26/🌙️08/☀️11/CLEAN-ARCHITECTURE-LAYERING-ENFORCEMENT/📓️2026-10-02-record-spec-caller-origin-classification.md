# Record Spec Caller Origin Classification

Read-only inventory/source audit; no compiler/tests or edits. Inventory is a retained earlier snapshot; concurrent already-corrected sites are marked. No global replacement authorized by spelling.

Canonical authority: OS DSL `🗣️dsl/🦀️.rs:390–396` DslVariants::variants returns Vec<(String,RecordSpecProducer)>; `🗣️dsl/🧬️schema/🏭️producer/🦀️.rs:38–39` ordinary is fn()->RecordSpec. Variant iteration/find/get preserves that producer type. Ordinary parse/print/spec construction calls use (producer.ordinary)(); controlled production remains separate.

Registry exception: OS DSL `📇️registry/🦀️.rs:30–34` SCHEMA_REGISTRY stores plain fn()->RecordSpec and resolver.schemas has same callable type; line65 self.schemas.get(schema).map(|spec_fn| spec_fn()) is genuine Fn and must remain.

All inspected non-registry local variants declarations originate exactly from DslVariants (Self as dsl, crate::os_dsl, ::dsl or imported DslVariants). No alternate plain-function variants table found among these108 files. Current declaration/binding evidence accompanies each retained site below.

## ✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🦀️.rs

- Retained line 158: **RecordSpecProducer**, current direct call. Origin `153: let variants = <Self as dsl::DslVariants>::variants();`. Authored `let record = dsl::parse(body, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;`; current `let record = dsl::parse(body, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;`.
- Retained line 168: **RecordSpecProducer**, current direct call. Origin `166: let variants = <Self as dsl::DslVariants>::variants();`. Authored `let body = dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline);`; current `let body = dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline);`.

## 🌎️hub/🧩️compositions/🪐️space/⚙️engine/🪐️space/👥️presence/🦀️.rs

- Retained line 125: **RecordSpecProducer**, current direct call. Origin `120: let variants = <Self as dsl::DslVariants>::variants();`. Authored `let record = dsl::parse(body, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;`; current `let record = dsl::parse(body, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;`.
- Retained line 135: **RecordSpecProducer**, current direct call. Origin `133: let variants = <Self as dsl::DslVariants>::variants();`. Authored `let body = dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline);`; current `let body = dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline);`.

## 🌎️hub/🧩️compositions/🪐️space/⚙️engine/🪐️space/🎚️config/🦀️.rs

- Retained line 200: **RecordSpecProducer**, current direct call. Origin `196: let variants = <Self as dsl::DslVariants>::variants();`. Authored `let record = dsl::parse(line, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;`; current `let record = dsl::parse(line, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;`.
- Retained line 210: **RecordSpecProducer**, current direct call. Origin `208: let variants = <Self as dsl::DslVariants>::variants();`. Authored `dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)`; current `dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)`.
- Retained line 239: **RecordSpecProducer**, current direct call. Origin `237: let variants = <Self as dsl::DslVariants>::variants();`. Authored `let spec = spec_fn();`; current `let spec = spec_fn();`.

## ✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🦀️.rs

- Retained line 140: **RecordSpecProducer**, current direct call. Origin `135: let variants = <Self as dsl::DslVariants>::variants();`. Authored `let record = dsl::parse(body, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;`; current `let record = dsl::parse(body, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;`.
- Retained line 150: **RecordSpecProducer**, current direct call. Origin `148: let variants = <Self as dsl::DslVariants>::variants();`. Authored `let body = dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline);`; current `let body = dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline);`.

## ✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🦀️.rs

- Retained line 204: **RecordSpecProducer**, current direct call. Origin `200: let variants = <Self as dsl::DslVariants>::variants();`. Authored `let record = dsl::parse(line, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;`; current `let record = dsl::parse(line, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;`.
- Retained line 214: **RecordSpecProducer**, current direct call. Origin `212: let variants = <Self as dsl::DslVariants>::variants();`. Authored `dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)`; current `dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)`.
- Retained line 243: **RecordSpecProducer**, current direct call. Origin `241: let variants = <Self as dsl::DslVariants>::variants();`. Authored `let spec = spec_fn();`; current `let spec = spec_fn();`.

## ✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🦀️.rs

- Retained line 161: **RecordSpecProducer**, current direct call. Origin `156: let variants = <Self as dsl::DslVariants>::variants();`. Authored `let record = dsl::parse(body, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;`; current `let record = dsl::parse(body, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;`.
- Retained line 171: **RecordSpecProducer**, current direct call. Origin `169: let variants = <Self as dsl::DslVariants>::variants();`. Authored `let body = dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline);`; current `let body = dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline);`.

## ✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs

- Retained line 72: **RecordSpecProducer**, current direct call. Origin `70: let variants = <Self as dsl::DslVariants>::variants();`. Authored `let spec = spec_fn();`; current `let spec = spec_fn();`.

## ✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs

- Retained line 430: **RecordSpecProducer**, current direct call. Origin `428: let variants = <Self as dsl::DslVariants>::variants();`. Authored `let spec = spec_fn();`; current `let spec = spec_fn();`.

## ✏️s/🔌️plugins/📖️playbook/🧩️extensions/🌀️procedural/🧬️schema/🧬️mutations/🦀️.rs

- Retained line 24: **RecordSpecProducer**, current direct call. Origin `20: let variants = <Self as dsl::DslVariants>::variants();`. Authored `let record = dsl::parse(line, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;`; current `let record = dsl::parse(line, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;`.
- Retained line 34: **RecordSpecProducer**, current direct call. Origin `32: let variants = <Self as dsl::DslVariants>::variants();`. Authored `dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)`; current `dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)`.
- Retained line 63: **RecordSpecProducer**, current direct call. Origin `61: let variants = <Self as dsl::DslVariants>::variants();`. Authored `let spec = spec_fn();`; current `let spec = spec_fn();`.

## ✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs

- Retained line 98: **RecordSpecProducer**, current direct call. Origin `96: let variants = <Self as dsl::DslVariants>::variants();`. Authored `let spec = spec_fn();`; current `let spec = spec_fn();`.

## ✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🧬️mutations/📝️text/🦀️.rs

- Retained line 20: **RecordSpecProducer**, current direct call. Origin `16: let variants = <Self as dsl::DslVariants>::variants();`. Authored `let record = dsl::parse(line, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;`; current `let record = dsl::parse(line, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;`.
- Retained line 30: **RecordSpecProducer**, current direct call. Origin `28: let variants = <Self as dsl::DslVariants>::variants();`. Authored `dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)`; current `dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)`.

## ✏️s/🔌️plugins/📖️playbook/🗿️artifacts/📖️playbook/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🧬️mutations/🦀️.rs

- Retained line 26: **RecordSpecProducer**, current direct call. Origin `22: let variants = <Self as dsl::DslVariants>::variants();`. Authored `let record = dsl::parse(line, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;`; current `let record = dsl::parse(line, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;`.
- Retained line 36: **RecordSpecProducer**, current direct call. Origin `34: let variants = <Self as dsl::DslVariants>::variants();`. Authored `dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)`; current `dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)`.

## ✏️s/🔌️plugins/📖️playbook/🗿️artifacts/📖️playbook/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️text/🦀️.rs

- Retained line 26: **RecordSpecProducer**, current direct call. Origin `22: let variants = <Self as dsl::DslVariants>::variants();`. Authored `let record = dsl::parse(line, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;`; current `let record = dsl::parse(line, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;`.
- Retained line 36: **RecordSpecProducer**, current direct call. Origin `34: let variants = <Self as dsl::DslVariants>::variants();`. Authored `dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)`; current `dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)`.

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🌦️epw/🏅️standards/🔖️energyplus/🪆️subsets/✳️any/✏️editor/🦀️.rs

- Retained line 49: **RecordSpecProducer**, current direct call. Origin `45: let variants = <Self as dsl::DslVariants>::variants();`. Authored `let record = dsl::parse(line, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;`; current `let record = dsl::parse(line, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;`.
- Retained line 59: **RecordSpecProducer**, current direct call. Origin `57: let variants = <Self as dsl::DslVariants>::variants();`. Authored `dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)`; current `dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)`.
- Retained line 87: **RecordSpecProducer**, current direct call. Origin `85: let variants = <Self as dsl::DslVariants>::variants();`. Authored `let spec = spec_fn();`; current `let spec = spec_fn();`.

## 🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🧫️fixtures/🧬️mutation-laws/🧬️mutations/🦀️.rs

- Retained line 38: **RecordSpecProducer**, current direct call. Origin `36: for (keyword, spec_fn) in <Self as crate::os_dsl::DslVariants>::variants() {`. Authored `let record = crate::os_dsl::parse(line, &spec_fn(), &crate::os_dsl::ParseOptions { limits: crate::os_dsl::Limits::default(), mode: crate::os_dsl::SourceMode::Inline })?;`; current `let record = crate::os_dsl::parse(line, &spec_fn(), &crate::os_dsl::ParseOptions { limits: crate::os_dsl::Limits::default(), mode: crate::os_dsl::SourceMode::Inline })?;`.
- Retained line 72: **RecordSpecProducer**, current direct call. Origin `70: let variants = <Self as crate::os_dsl::DslVariants>::variants();`. Authored `let (record, _) = crate::os_pack::decode_record_body(&bytes[reader.position()..], &spec_fn(), &crate::os_pack::DecodeOptions::default()).map_err(ProtocolError::from)?;`; current `let (record, _) = crate::os_pack::decode_record_body(&bytes[reader.position()..], &spec_fn(), &crate::os_pack::DecodeOptions::default()).map_err(ProtocolError::from)?;`.

## 🧰️framework/🛍️products/💻️os/🔨️modules/🪐️space/🗿️artifacts/🗂️collection/🦀️.rs

- Retained line 305: **RecordSpecProducer**, current bytes differ; inspect shifted/already-rebound site. Origin `301: let variants = <Self as dsl::DslVariants>::variants();`. Authored `let record = dsl::parse(line, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;`; current `let record = dsl::parse(line, &(spec_fn.ordinary)(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;`.
- Retained line 315: **RecordSpecProducer**, current bytes differ; inspect shifted/already-rebound site. Origin `313: let variants = <Self as dsl::DslVariants>::variants();`. Authored `dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)`; current `dsl::print(&record, &(spec_fn.ordinary)(), dsl::JoinMode::Inline)`.

## ✏️s/🔌️plugins/🪐️space/🗿️artifacts/🪐️space/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🦀️.rs

- Retained line 198: **RecordSpecProducer**, current direct call. Origin `194: let variants = <Self as dsl::DslVariants>::variants();`. Authored `let record = dsl::parse(line, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;`; current `let record = dsl::parse(line, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;`.
- Retained line 208: **RecordSpecProducer**, current direct call. Origin `206: let variants = <Self as dsl::DslVariants>::variants();`. Authored `dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)`; current `dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)`.
- Retained line 236: **RecordSpecProducer**, current direct call. Origin `234: let variants = <Self as dsl::DslVariants>::variants();`. Authored `let spec = spec_fn();`; current `let spec = spec_fn();`.

## 🧰️framework/🛍️products/💻️os/🔨️modules/🪐️space/🗿️artifacts/🪐️space/🦀️.rs

- Retained line 194: **RecordSpecProducer**, current bytes differ; inspect shifted/already-rebound site. Origin `190: let variants = <Self as dsl::DslVariants>::variants();`. Authored `let record = dsl::parse(line, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;`; current `let record = dsl::parse(line, &(spec_fn.ordinary)(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;`.
- Retained line 204: **RecordSpecProducer**, current bytes differ; inspect shifted/already-rebound site. Origin `202: let variants = <Self as dsl::DslVariants>::variants();`. Authored `dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)`; current `dsl::print(&record, &(spec_fn.ordinary)(), dsl::JoinMode::Inline)`.

## ✏️s/🔌️plugins/🪐️space/🗿️artifacts/🪐️space/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️text/🦀️.rs

- Retained line 25: **RecordSpecProducer**, current direct call. Origin `21: let variants = <Self as dsl::DslVariants>::variants();`. Authored `let record = dsl::parse(line, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;`; current `let record = dsl::parse(line, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;`.
- Retained line 35: **RecordSpecProducer**, current direct call. Origin `33: let variants = <Self as dsl::DslVariants>::variants();`. Authored `dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)`; current `dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)`.

## 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs

- Retained line 2036: **RecordSpecProducer**, current direct call. Origin `2032: let variants = <Self as crate::os_dsl::DslVariants>::variants();`. Authored `let record = crate::os_dsl::parse(line, &spec_fn(), &crate::os_dsl::ParseOptions { limits: crate::os_dsl::Limits::default(), mode: crate::os_dsl::SourceMode::Inline })?;`; current `let record = crate::os_dsl::parse(line, &spec_fn(), &crate::os_dsl::ParseOptions { limits: crate::os_dsl::Limits::default(), mode: crate::os_dsl::SourceMode::Inline })?;`.
- Retained line 2046: **RecordSpecProducer**, current direct call. Origin `2044: let variants = <Self as crate::os_dsl::DslVariants>::variants();`. Authored `crate::os_dsl::print(&record, &spec_fn(), crate::os_dsl::JoinMode::Inline)`; current `crate::os_dsl::print(&record, &spec_fn(), crate::os_dsl::JoinMode::Inline)`.
- Retained line 2075: **RecordSpecProducer**, current direct call. Origin `2073: let variants = <Self as crate::os_dsl::DslVariants>::variants();`. Authored `let spec = spec_fn();`; current `let spec = spec_fn();`.
- Retained line 6651: **RecordSpecProducer**, current direct call. Origin `6647: let variants = <Self as crate::os_dsl::DslVariants>::variants();`. Authored `let record = crate::os_dsl::parse(line, &spec_fn(), &crate::os_dsl::ParseOptions { limits: crate::os_dsl::Limits::default(), mode: crate::os_dsl::SourceMode::Inline })?;`; current `let record = crate::os_dsl::parse(line, &spec_fn(), &crate::os_dsl::ParseOptions { limits: crate::os_dsl::Limits::default(), mode: crate::os_dsl::SourceMode::Inline })?;`.
- Retained line 6661: **RecordSpecProducer**, current direct call. Origin `6659: let variants = <Self as crate::os_dsl::DslVariants>::variants();`. Authored `crate::os_dsl::print(&record, &spec_fn(), crate::os_dsl::JoinMode::Inline)`; current `crate::os_dsl::print(&record, &spec_fn(), crate::os_dsl::JoinMode::Inline)`.
- Retained line 6690: **RecordSpecProducer**, current direct call. Origin `6688: let variants = <Self as crate::os_dsl::DslVariants>::variants();`. Authored `let spec = spec_fn();`; current `let spec = spec_fn();`.
- Retained line 6902: **RecordSpecProducer**, current direct call. Origin `6898: let variants = <Self as crate::os_dsl::DslVariants>::variants();`. Authored `let record = crate::os_dsl::parse(line, &spec_fn(), &crate::os_dsl::ParseOptions { limits: crate::os_dsl::Limits::default(), mode: crate::os_dsl::SourceMode::Inline })?;`; current `let record = crate::os_dsl::parse(line, &spec_fn(), &crate::os_dsl::ParseOptions { limits: crate::os_dsl::Limits::default(), mode: crate::os_dsl::SourceMode::Inline })?;`.
- Retained line 6912: **RecordSpecProducer**, current direct call. Origin `6910: let variants = <Self as crate::os_dsl::DslVariants>::variants();`. Authored `crate::os_dsl::print(&record, &spec_fn(), crate::os_dsl::JoinMode::Inline)`; current `crate::os_dsl::print(&record, &spec_fn(), crate::os_dsl::JoinMode::Inline)`.
- Retained line 6941: **RecordSpecProducer**, current direct call. Origin `6939: let variants = <Self as crate::os_dsl::DslVariants>::variants();`. Authored `let spec = spec_fn();`; current `let spec = spec_fn();`.
- Retained line 8137: **RecordSpecProducer**, current direct call. Origin `8133: let variants = <Self as crate::os_dsl::DslVariants>::variants();`. Authored `let record = crate::os_dsl::parse(line, &spec_fn(), &crate::os_dsl::ParseOptions { limits: crate::os_dsl::Limits::default(), mode: crate::os_dsl::SourceMode::Inline })?;`; current `let record = crate::os_dsl::parse(line, &spec_fn(), &crate::os_dsl::ParseOptions { limits: crate::os_dsl::Limits::default(), mode: crate::os_dsl::SourceMode::Inline })?;`.
- Retained line 8147: **RecordSpecProducer**, current direct call. Origin `8145: let variants = <Self as crate::os_dsl::DslVariants>::variants();`. Authored `crate::os_dsl::print(&record, &spec_fn(), crate::os_dsl::JoinMode::Inline)`; current `crate::os_dsl::print(&record, &spec_fn(), crate::os_dsl::JoinMode::Inline)`.
- Retained line 8175: **RecordSpecProducer**, current direct call. Origin `8173: let variants = <Self as crate::os_dsl::DslVariants>::variants();`. Authored `let spec = spec_fn();`; current `let spec = spec_fn();`.

## 🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/📇️registry/🦀️.rs

- Retained line 65: **genuine callable**, current direct call. Origin `SCHEMA_REGISTRY fn()->RecordSpec; resolver map65`. Authored `self.schemas.get(schema).map(|spec_fn| spec_fn())`; current `self.schemas.get(schema).map(|spec_fn| spec_fn())`.

## 🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🧪️tests/🔬️unit/🦀️.rs

- Retained line 227: **RecordSpecProducer**, current direct call. Origin `225: let variants = <Self as DslVariants>::variants();`. Authored `let spec = spec_fn();`; current `let spec = spec_fn();`.
- Retained line 663: **RecordSpecProducer**, current direct call. Origin `659: let variants = <Self as DslVariants>::variants();`. Authored `let record = parse(line, &spec_fn(), &ParseOptions { limits: Limits::default(), mode: SourceMode::Inline })?;`; current `let record = parse(line, &spec_fn(), &ParseOptions { limits: Limits::default(), mode: SourceMode::Inline })?;`.
- Retained line 673: **RecordSpecProducer**, current direct call. Origin `671: let variants = <Self as DslVariants>::variants();`. Authored `print(&record, &spec_fn(), JoinMode::Inline)`; current `print(&record, &spec_fn(), JoinMode::Inline)`.
- Retained line 702: **RecordSpecProducer**, current direct call. Origin `700: let variants = <Self as DslVariants>::variants();`. Authored `let spec = spec_fn();`; current `let spec = spec_fn();`.

## ✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/🧬️schema/🧬️mutations/🦀️.rs

- Retained line 22: **RecordSpecProducer**, current direct call. Origin `18: let variants = <Self as dsl::DslVariants>::variants();`. Authored `let record = dsl::parse(line, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;`; current `let record = dsl::parse(line, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;`.
- Retained line 32: **RecordSpecProducer**, current direct call. Origin `30: let variants = <Self as dsl::DslVariants>::variants();`. Authored `dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)`; current `dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)`.

## ✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🦀️.rs

- Retained line 98: **RecordSpecProducer**, current direct call. Origin `93: let variants = <Self as dsl::DslVariants>::variants();`. Authored `let record = dsl::parse(body, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;`; current `let record = dsl::parse(body, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;`.
- Retained line 108: **RecordSpecProducer**, current direct call. Origin `106: let variants = <Self as dsl::DslVariants>::variants();`. Authored `let body = dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline);`; current `let body = dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline);`.

## ✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🦀️.rs

- Retained line 130: **RecordSpecProducer**, current direct call. Origin `126: let variants = <Self as dsl::DslVariants>::variants();`. Authored `let record = dsl::parse(line, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;`; current `let record = dsl::parse(line, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;`.
- Retained line 140: **RecordSpecProducer**, current direct call. Origin `138: let variants = <Self as dsl::DslVariants>::variants();`. Authored `dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)`; current `dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)`.
- Retained line 169: **RecordSpecProducer**, current direct call. Origin `167: let variants = <Self as dsl::DslVariants>::variants();`. Authored `let spec = spec_fn();`; current `let spec = spec_fn();`.

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/🏅️standards/7️⃣87a/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs

- Retained line 272: **RecordSpecProducer**, current direct call. Origin `268: let variants = <Self as dsl::DslVariants>::variants();`. Authored `let record = dsl::parse(line, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;`; current `let record = dsl::parse(line, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;`.
- Retained line 282: **RecordSpecProducer**, current direct call. Origin `280: let variants = <Self as dsl::DslVariants>::variants();`. Authored `dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)`; current `dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)`.

## ✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️text/🦀️.rs

- Retained line 20: **RecordSpecProducer**, current direct call. Origin `16: let variants = <Self as dsl::DslVariants>::variants();`. Authored `let record = dsl::parse(line, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;`; current `let record = dsl::parse(line, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;`.
- Retained line 30: **RecordSpecProducer**, current direct call. Origin `28: let variants = <Self as dsl::DslVariants>::variants();`. Authored `dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)`; current `dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)`.

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/🏅️standards/9️⃣89a/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/🦀️.rs

- Retained line 438: **RecordSpecProducer**, current direct call. Origin `434: let variants = <Self as dsl::DslVariants>::variants();`. Authored `let record = dsl::parse(line, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;`; current `let record = dsl::parse(line, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;`.
- Retained line 448: **RecordSpecProducer**, current direct call. Origin `446: let variants = <Self as dsl::DslVariants>::variants();`. Authored `dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)`; current `dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)`.

## ✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🧬️schema/🧬️mutations/🦀️.rs

- Retained line 24: **RecordSpecProducer**, current direct call. Origin `20: let variants = <Self as dsl::DslVariants>::variants();`. Authored `let record = dsl::parse(line, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;`; current `let record = dsl::parse(line, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;`.
- Retained line 34: **RecordSpecProducer**, current direct call. Origin `32: let variants = <Self as dsl::DslVariants>::variants();`. Authored `dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)`; current `dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)`.

## ✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🧬️mutations/🦀️.rs

- Retained line 28: **RecordSpecProducer**, current direct call. Origin `24: let variants = <Self as dsl::DslVariants>::variants();`. Authored `let record = dsl::parse(line, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;`; current `let record = dsl::parse(line, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;`.
- Retained line 38: **RecordSpecProducer**, current direct call. Origin `36: let variants = <Self as dsl::DslVariants>::variants();`. Authored `dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)`; current `dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)`.

## ✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🧬️mutations/📝️text/🦀️.rs

- Retained line 62: **RecordSpecProducer**, current direct call. Origin `58: let variants = <Self as dsl::DslVariants>::variants();`. Authored `let record = dsl::parse(line, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;`; current `let record = dsl::parse(line, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;`.
- Retained line 72: **RecordSpecProducer**, current direct call. Origin `70: let variants = <Self as dsl::DslVariants>::variants();`. Authored `dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)`; current `dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)`.

## 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🧪️tests/🔬️unit/🦀️.rs

- Retained line 288: **RecordSpecProducer**, current direct call. Origin `284: let variants = <Self as crate::os_dsl::DslVariants>::variants();`. Authored `let record = crate::os_dsl::parse(line, &spec_fn(), &crate::os_dsl::ParseOptions { limits: crate::os_dsl::Limits::default(), mode: crate::os_dsl::SourceMode::Inline })?;`; current `let record = crate::os_dsl::parse(line, &spec_fn(), &crate::os_dsl::ParseOptions { limits: crate::os_dsl::Limits::default(), mode: crate::os_dsl::SourceMode::Inline })?;`.
- Retained line 298: **RecordSpecProducer**, current direct call. Origin `296: let variants = <Self as crate::os_dsl::DslVariants>::variants();`. Authored `crate::os_dsl::print(&record, &spec_fn(), crate::os_dsl::JoinMode::Inline)`; current `crate::os_dsl::print(&record, &spec_fn(), crate::os_dsl::JoinMode::Inline)`.
- Retained line 307: **RecordSpecProducer**, current direct call. Origin `305: let variants = <Self as crate::os_dsl::DslVariants>::variants();`. Authored `let body = crate::os_pack::encode_record_body(&spec_fn(), &record, &PackEncodeOptions::default()).map_err(|e| crate::os_spr::ProtocolError::Malformed { what: "op pack", offset: 0, detail: e.to_string() })?;`; current `let body = crate::os_pack::encode_record_body(&spec_fn(), &record, &PackEncodeOptions::default()).map_err(|e| crate::os_spr::ProtocolError::Malformed { what: "op pack", offset: 0, detail: e.to_string() })?;`.
- Retained line 323: **RecordSpecProducer**, current direct call. Origin `321: let variants = <Self as crate::os_dsl::DslVariants>::variants();`. Authored `let spec = spec_fn();`; current `let spec = spec_fn();`.

## 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window/🎚️config/🧪️tests/📥️retained-pack-load/🦀️.rs

- Retained line 87: **RecordSpecProducer**, current direct call. Origin `85: for (keyword, spec_fn) in &<Self as dsl::DslVariants>::variants() {`. Authored `let record = dsl::parse(line, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;`; current `let record = dsl::parse(line, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;`.

## ✏️s/🔌️plugins/🧱️block/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🦀️.rs

- Retained line 122: **RecordSpecProducer**, current direct call. Origin `117: let variants = <Self as dsl::DslVariants>::variants();`. Authored `let record = dsl::parse(body, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;`; current `let record = dsl::parse(body, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;`.
- Retained line 132: **RecordSpecProducer**, current direct call. Origin `130: let variants = <Self as dsl::DslVariants>::variants();`. Authored `let body = dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline);`; current `let body = dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline);`.

## ✏️s/🔌️plugins/🧱️block/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🦀️.rs

- Retained line 91: **RecordSpecProducer**, current direct call. Origin `87: let variants = <Self as dsl::DslVariants>::variants();`. Authored `let record = dsl::parse(line, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;`; current `let record = dsl::parse(line, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;`.
- Retained line 101: **RecordSpecProducer**, current direct call. Origin `99: let variants = <Self as dsl::DslVariants>::variants();`. Authored `dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)`; current `dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)`.
- Retained line 130: **RecordSpecProducer**, current direct call. Origin `128: let variants = <Self as dsl::DslVariants>::variants();`. Authored `let spec = spec_fn();`; current `let spec = spec_fn();`.

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/4️⃣1.4/🪆️subsets/🗄️a/✏️editor/🦀️.rs

- Retained line 51: **RecordSpecProducer**, current direct call. Origin `47: let variants = <Self as dsl::DslVariants>::variants();`. Authored `let record = dsl::parse(line, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;`; current `let record = dsl::parse(line, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;`.
- Retained line 61: **RecordSpecProducer**, current direct call. Origin `59: let variants = <Self as dsl::DslVariants>::variants();`. Authored `dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)`; current `dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)`.
- Retained line 89: **RecordSpecProducer**, current direct call. Origin `87: let variants = <Self as dsl::DslVariants>::variants();`. Authored `let spec = spec_fn();`; current `let spec = spec_fn();`.

## ✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🦀️.rs

- Retained line 142: **RecordSpecProducer**, current direct call. Origin `137: let variants = <Self as dsl::DslVariants>::variants();`. Authored `let record = dsl::parse(body, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;`; current `let record = dsl::parse(body, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;`.
- Retained line 152: **RecordSpecProducer**, current direct call. Origin `150: let variants = <Self as dsl::DslVariants>::variants();`. Authored `let body = dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline);`; current `let body = dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline);`.

## ✏️s/🔌️plugins/🧱️block/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️text/🦀️.rs

- Retained line 18: **RecordSpecProducer**, current direct call. Origin `14: let variants = <Self as dsl::DslVariants>::variants();`. Authored `let record = dsl::parse(line, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;`; current `let record = dsl::parse(line, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;`.
- Retained line 28: **RecordSpecProducer**, current direct call. Origin `26: let variants = <Self as dsl::DslVariants>::variants();`. Authored `dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)`; current `dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)`.

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/4️⃣1.4/🪆️subsets/🧱️base/✏️editor/🦀️.rs

- Retained line 51: **RecordSpecProducer**, current direct call. Origin `47: let variants = <Self as dsl::DslVariants>::variants();`. Authored `let record = dsl::parse(line, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;`; current `let record = dsl::parse(line, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;`.
- Retained line 61: **RecordSpecProducer**, current direct call. Origin `59: let variants = <Self as dsl::DslVariants>::variants();`. Authored `dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)`; current `dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)`.
- Retained line 89: **RecordSpecProducer**, current direct call. Origin `87: let variants = <Self as dsl::DslVariants>::variants();`. Authored `let spec = spec_fn();`; current `let spec = spec_fn();`.

## 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs

- Retained line 13115: **RecordSpecProducer**, current direct call. Origin `13108: let variants = <Self as ::dsl::DslVariants>::variants();`. Authored `&spec_fn(),`; current `&spec_fn(),`.
- Retained line 13127: **RecordSpecProducer**, current direct call. Origin `13125: let variants = <Self as ::dsl::DslVariants>::variants();`. Authored `let body = ::dsl::print(&record, &spec_fn(), ::dsl::JoinMode::Inline);`; current `let body = ::dsl::print(&record, &spec_fn(), ::dsl::JoinMode::Inline);`.
- Retained line 13189: **RecordSpecProducer**, current direct call. Origin `13182: let variants = <Self as ::dsl::DslVariants>::variants();`. Authored `&spec_fn(),`; current `&spec_fn(),`.
- Retained line 13201: **RecordSpecProducer**, current direct call. Origin `13199: let variants = <Self as ::dsl::DslVariants>::variants();`. Authored `let body = ::dsl::print(&record, &spec_fn(), ::dsl::JoinMode::Inline);`; current `let body = ::dsl::print(&record, &spec_fn(), ::dsl::JoinMode::Inline);`.
- Retained line 13264: **RecordSpecProducer**, current direct call. Origin `13257: let variants = <Self as ::dsl::DslVariants>::variants();`. Authored `&spec_fn(),`; current `&spec_fn(),`.
- Retained line 13276: **RecordSpecProducer**, current direct call. Origin `13274: let variants = <Self as ::dsl::DslVariants>::variants();`. Authored `let body = ::dsl::print(&record, &spec_fn(), ::dsl::JoinMode::Inline);`; current `let body = ::dsl::print(&record, &spec_fn(), ::dsl::JoinMode::Inline);`.
- Retained line 13347: **RecordSpecProducer**, current direct call. Origin `13340: let variants = <Self as ::dsl::DslVariants>::variants();`. Authored `&spec_fn(),`; current `&spec_fn(),`.
- Retained line 13359: **RecordSpecProducer**, current direct call. Origin `13357: let variants = <Self as ::dsl::DslVariants>::variants();`. Authored `let body = ::dsl::print(&record, &spec_fn(), ::dsl::JoinMode::Inline);`; current `let body = ::dsl::print(&record, &spec_fn(), ::dsl::JoinMode::Inline);`.

## 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧬️mutation-fixtures-surface/🦀️.rs

- Retained line 117: **RecordSpecProducer**, current direct call. Origin `112: let variants = <Self as ::dsl::DslVariants>::variants();`. Authored `let record = ::dsl::parse(body, &spec_fn(), &::dsl::ParseOptions { limits: ::dsl::Limits::default(), mode: ::dsl::SourceMode::Inline })?;`; current `let record = ::dsl::parse(body, &spec_fn(), &::dsl::ParseOptions { limits: ::dsl::Limits::default(), mode: ::dsl::SourceMode::Inline })?;`.
- Retained line 127: **RecordSpecProducer**, current direct call. Origin `125: let variants = <Self as ::dsl::DslVariants>::variants();`. Authored `let body = ::dsl::print(&record, &spec_fn(), ::dsl::JoinMode::Inline);`; current `let body = ::dsl::print(&record, &spec_fn(), ::dsl::JoinMode::Inline);`.

## 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧬️mutation-fixtures-dummy/🦀️.rs

- Retained line 117: **RecordSpecProducer**, current direct call. Origin `112: let variants = <Self as ::dsl::DslVariants>::variants();`. Authored `let record = ::dsl::parse(body, &spec_fn(), &::dsl::ParseOptions { limits: ::dsl::Limits::default(), mode: ::dsl::SourceMode::Inline })?;`; current `let record = ::dsl::parse(body, &spec_fn(), &::dsl::ParseOptions { limits: ::dsl::Limits::default(), mode: ::dsl::SourceMode::Inline })?;`.
- Retained line 127: **RecordSpecProducer**, current direct call. Origin `125: let variants = <Self as ::dsl::DslVariants>::variants();`. Authored `let body = ::dsl::print(&record, &spec_fn(), ::dsl::JoinMode::Inline);`; current `let body = ::dsl::print(&record, &spec_fn(), ::dsl::JoinMode::Inline);`.

## 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧬️mutation-fixtures-transaction/🦀️.rs

- Retained line 120: **RecordSpecProducer**, current direct call. Origin `115: let variants = <Self as ::dsl::DslVariants>::variants();`. Authored `let record = ::dsl::parse(body, &spec_fn(), &::dsl::ParseOptions { limits: ::dsl::Limits::default(), mode: ::dsl::SourceMode::Inline })?;`; current `let record = ::dsl::parse(body, &spec_fn(), &::dsl::ParseOptions { limits: ::dsl::Limits::default(), mode: ::dsl::SourceMode::Inline })?;`.
- Retained line 130: **RecordSpecProducer**, current direct call. Origin `128: let variants = <Self as ::dsl::DslVariants>::variants();`. Authored `let body = ::dsl::print(&record, &spec_fn(), ::dsl::JoinMode::Inline);`; current `let body = ::dsl::print(&record, &spec_fn(), ::dsl::JoinMode::Inline);`.

## ✏️s/🔌️plugins/🧱️block/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🦀️.rs

- Retained line 125: **RecordSpecProducer**, current direct call. Origin `120: let variants = <Self as dsl::DslVariants>::variants();`. Authored `let record = dsl::parse(body, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;`; current `let record = dsl::parse(body, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;`.
- Retained line 135: **RecordSpecProducer**, current direct call. Origin `133: let variants = <Self as dsl::DslVariants>::variants();`. Authored `let body = dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline);`; current `let body = dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline);`.

## ✏️s/🔌️plugins/🧱️block/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🦀️.rs

- Retained line 88: **RecordSpecProducer**, current direct call. Origin `84: let variants = <Self as dsl::DslVariants>::variants();`. Authored `let record = dsl::parse(line, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;`; current `let record = dsl::parse(line, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;`.
- Retained line 98: **RecordSpecProducer**, current direct call. Origin `96: let variants = <Self as dsl::DslVariants>::variants();`. Authored `dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)`; current `dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)`.
- Retained line 127: **RecordSpecProducer**, current direct call. Origin `125: let variants = <Self as dsl::DslVariants>::variants();`. Authored `let spec = spec_fn();`; current `let spec = spec_fn();`.

## ✏️s/🔌️plugins/🧱️block/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️text/🦀️.rs

- Retained line 18: **RecordSpecProducer**, current direct call. Origin `14: let variants = <Self as dsl::DslVariants>::variants();`. Authored `let record = dsl::parse(line, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;`; current `let record = dsl::parse(line, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;`.
- Retained line 28: **RecordSpecProducer**, current direct call. Origin `26: let variants = <Self as dsl::DslVariants>::variants();`. Authored `dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)`; current `dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)`.

## 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️plugin-runtime-plugin-builder-contract/🦀️.rs

- Retained line 382: **RecordSpecProducer**, current direct call. Origin `377: let variants = <Self as ::dsl::DslVariants>::variants();`. Authored `let record = ::dsl::parse(body, &spec_fn(), &::dsl::ParseOptions { limits: ::dsl::Limits::default(), mode: ::dsl::SourceMode::Inline })?;`; current `let record = ::dsl::parse(body, &spec_fn(), &::dsl::ParseOptions { limits: ::dsl::Limits::default(), mode: ::dsl::SourceMode::Inline })?;`.
- Retained line 392: **RecordSpecProducer**, current direct call. Origin `390: let variants = <Self as ::dsl::DslVariants>::variants();`. Authored `let body = ::dsl::print(&record, &spec_fn(), ::dsl::JoinMode::Inline);`; current `let body = ::dsl::print(&record, &spec_fn(), ::dsl::JoinMode::Inline);`.

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎥️mp4/🏅️standards/🔖️isobmff/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs

- Retained line 92: **RecordSpecProducer**, current direct call. Origin `88: let variants = <Self as dsl::DslVariants>::variants();`. Authored `let record = dsl::parse(line, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits { max_bytes: 32 * 1024 * 1024, ..dsl::Limits::default() }, mode: dsl::SourceMode::Inline })?;`; current `let record = dsl::parse(line, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits { max_bytes: 32 * 1024 * 1024, ..dsl::Limits::default() }, mode: dsl::SourceMode::Inline })?;`.
- Retained line 102: **RecordSpecProducer**, current direct call. Origin `100: let variants = <Self as dsl::DslVariants>::variants();`. Authored `dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)`; current `dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)`.

## 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️builder/🧫️fixtures/🔗️dependency-contribution/🧬️mutations/🦀️.rs

- Retained line 23: **RecordSpecProducer**, current direct call. Origin `21: for (keyword, spec_fn) in <Self as dsl::DslVariants>::variants() {`. Authored `let record = dsl::parse(line, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;`; current `let record = dsl::parse(line, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;`.

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/4️⃣1.4/🪆️subsets/🖨️x/✏️editor/🦀️.rs

- Retained line 51: **RecordSpecProducer**, current direct call. Origin `47: let variants = <Self as dsl::DslVariants>::variants();`. Authored `let record = dsl::parse(line, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;`; current `let record = dsl::parse(line, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;`.
- Retained line 61: **RecordSpecProducer**, current direct call. Origin `59: let variants = <Self as dsl::DslVariants>::variants();`. Authored `dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)`; current `dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)`.
- Retained line 89: **RecordSpecProducer**, current direct call. Origin `87: let variants = <Self as dsl::DslVariants>::variants();`. Authored `let spec = spec_fn();`; current `let spec = spec_fn();`.

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🗄️a/✏️editor/🦀️.rs

- Retained line 51: **RecordSpecProducer**, current direct call. Origin `47: let variants = <Self as dsl::DslVariants>::variants();`. Authored `let record = dsl::parse(line, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;`; current `let record = dsl::parse(line, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;`.
- Retained line 61: **RecordSpecProducer**, current direct call. Origin `59: let variants = <Self as dsl::DslVariants>::variants();`. Authored `dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)`; current `dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)`.
- Retained line 89: **RecordSpecProducer**, current direct call. Origin `87: let variants = <Self as dsl::DslVariants>::variants();`. Authored `let spec = spec_fn();`; current `let spec = spec_fn();`.

## ✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🦀️.rs

- Retained line 139: **RecordSpecProducer**, current direct call. Origin `134: let variants = <Self as dsl::DslVariants>::variants();`. Authored `let record = dsl::parse(body, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;`; current `let record = dsl::parse(body, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;`.
- Retained line 149: **RecordSpecProducer**, current direct call. Origin `147: let variants = <Self as dsl::DslVariants>::variants();`. Authored `let body = dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline);`; current `let body = dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline);`.

## ✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🦀️.rs

- Retained line 155: **RecordSpecProducer**, current direct call. Origin `151: let variants = <Self as dsl::DslVariants>::variants();`. Authored `let record = dsl::parse(line, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;`; current `let record = dsl::parse(line, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;`.
- Retained line 165: **RecordSpecProducer**, current direct call. Origin `163: let variants = <Self as dsl::DslVariants>::variants();`. Authored `dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)`; current `dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)`.
- Retained line 194: **RecordSpecProducer**, current direct call. Origin `192: let variants = <Self as dsl::DslVariants>::variants();`. Authored `let spec = spec_fn();`; current `let spec = spec_fn();`.

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/♿️ua/✏️editor/🦀️.rs

- Retained line 51: **RecordSpecProducer**, current direct call. Origin `47: let variants = <Self as dsl::DslVariants>::variants();`. Authored `let record = dsl::parse(line, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;`; current `let record = dsl::parse(line, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;`.
- Retained line 61: **RecordSpecProducer**, current direct call. Origin `59: let variants = <Self as dsl::DslVariants>::variants();`. Authored `dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)`; current `dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)`.
- Retained line 89: **RecordSpecProducer**, current direct call. Origin `87: let variants = <Self as dsl::DslVariants>::variants();`. Authored `let spec = spec_fn();`; current `let spec = spec_fn();`.

## ✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️text/🦀️.rs

- Retained line 152: **RecordSpecProducer**, current direct call. Origin `148: let variants = <Self as dsl::DslVariants>::variants();`. Authored `let record = dsl::parse(line, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;`; current `let record = dsl::parse(line, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;`.
- Retained line 162: **RecordSpecProducer**, current direct call. Origin `160: let variants = <Self as dsl::DslVariants>::variants();`. Authored `dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)`; current `dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)`.

## ✏️s/🔌️plugins/🧱️block/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🌐️world/🫧️transient/🧬️schema/🧬️mutations/🦀️.rs

- Retained line 22: **RecordSpecProducer**, current direct call. Origin `18: let variants = <Self as dsl::DslVariants>::variants();`. Authored `let record = dsl::parse(line, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;`; current `let record = dsl::parse(line, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;`.
- Retained line 33: **RecordSpecProducer**, current direct call. Origin `31: let variants = <Self as dsl::DslVariants>::variants();`. Authored `dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)`; current `dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)`.

## ✏️s/🔌️plugins/🧱️block/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🦀️.rs

- Retained line 123: **RecordSpecProducer**, current direct call. Origin `118: let variants = <Self as dsl::DslVariants>::variants();`. Authored `let record = dsl::parse(body, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;`; current `let record = dsl::parse(body, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;`.
- Retained line 133: **RecordSpecProducer**, current direct call. Origin `131: let variants = <Self as dsl::DslVariants>::variants();`. Authored `let body = dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline);`; current `let body = dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline);`.

## ✏️s/🔌️plugins/🧱️block/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🦀️.rs

- Retained line 166: **RecordSpecProducer**, current direct call. Origin `162: let variants = <Self as dsl::DslVariants>::variants();`. Authored `let record = dsl::parse(line, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;`; current `let record = dsl::parse(line, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;`.
- Retained line 176: **RecordSpecProducer**, current direct call. Origin `174: let variants = <Self as dsl::DslVariants>::variants();`. Authored `dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)`; current `dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)`.
- Retained line 205: **RecordSpecProducer**, current direct call. Origin `203: let variants = <Self as dsl::DslVariants>::variants();`. Authored `let spec = spec_fn();`; current `let spec = spec_fn();`.

## ✏️s/🔌️plugins/🧱️block/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️text/🦀️.rs

- Retained line 18: **RecordSpecProducer**, current direct call. Origin `14: let variants = <Self as dsl::DslVariants>::variants();`. Authored `let record = dsl::parse(line, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;`; current `let record = dsl::parse(line, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;`.
- Retained line 28: **RecordSpecProducer**, current direct call. Origin `26: let variants = <Self as dsl::DslVariants>::variants();`. Authored `dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)`; current `dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)`.

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧾️vt/✏️editor/🦀️.rs

- Retained line 51: **RecordSpecProducer**, current direct call. Origin `47: let variants = <Self as dsl::DslVariants>::variants();`. Authored `let record = dsl::parse(line, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;`; current `let record = dsl::parse(line, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;`.
- Retained line 61: **RecordSpecProducer**, current direct call. Origin `59: let variants = <Self as dsl::DslVariants>::variants();`. Authored `dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)`; current `dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)`.
- Retained line 89: **RecordSpecProducer**, current direct call. Origin `87: let variants = <Self as dsl::DslVariants>::variants();`. Authored `let spec = spec_fn();`; current `let spec = spec_fn();`.

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/✏️editor/🦀️.rs

- Retained line 51: **RecordSpecProducer**, current direct call. Origin `47: let variants = <Self as dsl::DslVariants>::variants();`. Authored `let record = dsl::parse(line, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;`; current `let record = dsl::parse(line, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;`.
- Retained line 61: **RecordSpecProducer**, current direct call. Origin `59: let variants = <Self as dsl::DslVariants>::variants();`. Authored `dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)`; current `dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)`.
- Retained line 89: **RecordSpecProducer**, current direct call. Origin `87: let variants = <Self as dsl::DslVariants>::variants();`. Authored `let spec = spec_fn();`; current `let spec = spec_fn();`.

## ✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🦀️.rs

- Retained line 130: **RecordSpecProducer**, current direct call. Origin `125: let variants = <Self as dsl::DslVariants>::variants();`. Authored `let record = dsl::parse(body, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;`; current `let record = dsl::parse(body, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;`.
- Retained line 140: **RecordSpecProducer**, current direct call. Origin `138: let variants = <Self as dsl::DslVariants>::variants();`. Authored `let body = dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline);`; current `let body = dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline);`.

## ✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️text/🦀️.rs

- Retained line 18: **RecordSpecProducer**, current direct call. Origin `14: let variants = <Self as dsl::DslVariants>::variants();`. Authored `let record = dsl::parse(line, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;`; current `let record = dsl::parse(line, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;`.
- Retained line 28: **RecordSpecProducer**, current direct call. Origin `26: let variants = <Self as dsl::DslVariants>::variants();`. Authored `dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)`; current `dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)`.

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🖨️x/✏️editor/🦀️.rs

- Retained line 51: **RecordSpecProducer**, current direct call. Origin `47: let variants = <Self as dsl::DslVariants>::variants();`. Authored `let record = dsl::parse(line, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;`; current `let record = dsl::parse(line, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;`.
- Retained line 61: **RecordSpecProducer**, current direct call. Origin `59: let variants = <Self as dsl::DslVariants>::variants();`. Authored `dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)`; current `dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)`.
- Retained line 89: **RecordSpecProducer**, current direct call. Origin `87: let variants = <Self as dsl::DslVariants>::variants();`. Authored `let spec = spec_fn();`; current `let spec = spec_fn();`.

## ✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewriting/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🪟️window/🎚️config/🧬️schema/🧬️mutations/🦀️.rs

- Retained line 27: **RecordSpecProducer**, current direct call. Origin `23: let variants = <Self as dsl::DslVariants>::variants();`. Authored `let record = dsl::parse(line, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;`; current `let record = dsl::parse(line, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;`.
- Retained line 37: **RecordSpecProducer**, current direct call. Origin `35: let variants = <Self as dsl::DslVariants>::variants();`. Authored `dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)`; current `dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)`.

## ✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🎚️config/🦀️.rs

- Retained line 77: **RecordSpecProducer**, current direct call. Origin `74: let variants = <Self as dsl::DslVariants>::variants();`. Authored `let record = dsl::parse(line, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;`; current `let record = dsl::parse(line, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;`.

## ✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewriting/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs

- Retained line 402: **RecordSpecProducer**, current direct call. Origin `398: let variants = <Self as dsl::DslVariants>::variants();`. Authored `let record = dsl::parse(line, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;`; current `let record = dsl::parse(line, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;`.
- Retained line 412: **RecordSpecProducer**, current direct call. Origin `410: let variants = <Self as dsl::DslVariants>::variants();`. Authored `dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)`; current `dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)`.
- Retained line 443: **RecordSpecProducer**, current direct call. Origin `441: let variants = <Self as dsl::DslVariants>::variants();`. Authored `let spec = spec_fn();`; current `let spec = spec_fn();`.

## ✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🦀️.rs

- Retained line 151: **RecordSpecProducer**, current direct call. Origin `146: let variants = <Self as dsl::DslVariants>::variants();`. Authored `let record = dsl::parse(body, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;`; current `let record = dsl::parse(body, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;`.
- Retained line 161: **RecordSpecProducer**, current direct call. Origin `159: let variants = <Self as dsl::DslVariants>::variants();`. Authored `let body = dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline);`; current `let body = dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline);`.

## ✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🦀️.rs

- Retained line 176: **RecordSpecProducer**, current direct call. Origin `172: let variants = <Self as dsl::DslVariants>::variants();`. Authored `let record = dsl::parse(line, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;`; current `let record = dsl::parse(line, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;`.
- Retained line 186: **RecordSpecProducer**, current direct call. Origin `184: let variants = <Self as dsl::DslVariants>::variants();`. Authored `dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)`; current `dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)`.
- Retained line 215: **RecordSpecProducer**, current direct call. Origin `213: let variants = <Self as dsl::DslVariants>::variants();`. Authored `let spec = spec_fn();`; current `let spec = spec_fn();`.

## ✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️text/🦀️.rs

- Retained line 20: **RecordSpecProducer**, current direct call. Origin `16: let variants = <Self as dsl::DslVariants>::variants();`. Authored `let record = dsl::parse(line, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;`; current `let record = dsl::parse(line, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;`.
- Retained line 30: **RecordSpecProducer**, current direct call. Origin `28: let variants = <Self as dsl::DslVariants>::variants();`. Authored `dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)`; current `dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)`.

## ✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewriting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️text/🦀️.rs

- Retained line 59: **RecordSpecProducer**, current direct call. Origin `55: let variants = <Self as dsl::DslVariants>::variants();`. Authored `let record = dsl::parse(line, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;`; current `let record = dsl::parse(line, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;`.
- Retained line 69: **RecordSpecProducer**, current direct call. Origin `67: let variants = <Self as dsl::DslVariants>::variants();`. Authored `dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)`; current `dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)`.

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/💾️binary/🏅️standards/🔖️raw/🪆️subsets/✳️any/✏️editor/🦀️.rs

- Retained line 49: **RecordSpecProducer**, current direct call. Origin `45: let variants = <Self as dsl::DslVariants>::variants();`. Authored `let record = dsl::parse(line, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;`; current `let record = dsl::parse(line, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;`.
- Retained line 59: **RecordSpecProducer**, current direct call. Origin `57: let variants = <Self as dsl::DslVariants>::variants();`. Authored `dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)`; current `dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)`.
- Retained line 87: **RecordSpecProducer**, current direct call. Origin `85: let variants = <Self as dsl::DslVariants>::variants();`. Authored `let spec = spec_fn();`; current `let spec = spec_fn();`.

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/⚕️h/✏️editor/🦀️.rs

- Retained line 51: **RecordSpecProducer**, current direct call. Origin `47: let variants = <Self as dsl::DslVariants>::variants();`. Authored `let record = dsl::parse(line, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;`; current `let record = dsl::parse(line, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;`.
- Retained line 61: **RecordSpecProducer**, current direct call. Origin `59: let variants = <Self as dsl::DslVariants>::variants();`. Authored `dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)`; current `dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)`.
- Retained line 89: **RecordSpecProducer**, current direct call. Origin `87: let variants = <Self as dsl::DslVariants>::variants();`. Authored `let spec = spec_fn();`; current `let spec = spec_fn();`.

## ✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️text/🦀️.rs

- Retained line 20: **RecordSpecProducer**, current direct call. Origin `16: let variants = <Self as dsl::DslVariants>::variants();`. Authored `let record = dsl::parse(line, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;`; current `let record = dsl::parse(line, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;`.
- Retained line 30: **RecordSpecProducer**, current direct call. Origin `28: let variants = <Self as dsl::DslVariants>::variants();`. Authored `dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)`; current `dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)`.

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/💾️binary/🏅️standards/🔖️raw/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs

- Retained line 126: **RecordSpecProducer**, current direct call. Origin `124: let variants = <Self as dsl::DslVariants>::variants();`. Authored `let spec = spec_fn();`; current `let spec = spec_fn();`.
- Retained line 140: **RecordSpecProducer**, current direct call. Origin `138: let variants = <Self as dsl::DslVariants>::variants();`. Authored `dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)`; current `dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)`.

## ✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🦀️.rs

- Retained line 145: **RecordSpecProducer**, current direct call. Origin `140: let variants = <Self as dsl::DslVariants>::variants();`. Authored `let record = dsl::parse(body, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;`; current `let record = dsl::parse(body, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;`.
- Retained line 155: **RecordSpecProducer**, current direct call. Origin `153: let variants = <Self as dsl::DslVariants>::variants();`. Authored `let body = dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline);`; current `let body = dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline);`.

## ✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️text/🦀️.rs

- Retained line 18: **RecordSpecProducer**, current direct call. Origin `14: let variants = <Self as dsl::DslVariants>::variants();`. Authored `let record = dsl::parse(line, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;`; current `let record = dsl::parse(line, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;`.
- Retained line 28: **RecordSpecProducer**, current direct call. Origin `26: let variants = <Self as dsl::DslVariants>::variants();`. Authored `dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)`; current `dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)`.

## ✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🌐️graph/🎚️config/🧬️schema/🧬️mutations/🦀️.rs

- Retained line 27: **RecordSpecProducer**, current direct call. Origin `23: let variants = <Self as dsl::DslVariants>::variants();`. Authored `let record = dsl::parse(line, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;`; current `let record = dsl::parse(line, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;`.
- Retained line 37: **RecordSpecProducer**, current direct call. Origin `35: let variants = <Self as dsl::DslVariants>::variants();`. Authored `dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)`; current `dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)`.

## ✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📊️results/🫧️transient/🧬️schema/🧬️mutations/🦀️.rs

- Retained line 22: **RecordSpecProducer**, current direct call. Origin `18: let variants = <Self as dsl::DslVariants>::variants();`. Authored `let record = dsl::parse(line, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;`; current `let record = dsl::parse(line, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;`.
- Retained line 32: **RecordSpecProducer**, current direct call. Origin `30: let variants = <Self as dsl::DslVariants>::variants();`. Authored `dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)`; current `dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)`.

## ✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📝️editor/🫧️transient/🧬️schema/🧬️mutations/🦀️.rs

- Retained line 22: **RecordSpecProducer**, current direct call. Origin `18: let variants = <Self as dsl::DslVariants>::variants();`. Authored `let record = dsl::parse(line, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;`; current `let record = dsl::parse(line, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;`.
- Retained line 33: **RecordSpecProducer**, current direct call. Origin `31: let variants = <Self as dsl::DslVariants>::variants();`. Authored `dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)`; current `dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)`.

## ✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs

- Retained line 236: **RecordSpecProducer**, current direct call. Origin `232: let variants = <Self as dsl::DslVariants>::variants();`. Authored `let record = dsl::parse(line, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;`; current `let record = dsl::parse(line, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;`.
- Retained line 246: **RecordSpecProducer**, current direct call. Origin `244: let variants = <Self as dsl::DslVariants>::variants();`. Authored `dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)`; current `dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)`.
- Retained line 277: **RecordSpecProducer**, current direct call. Origin `275: let variants = <Self as dsl::DslVariants>::variants();`. Authored `let spec = spec_fn();`; current `let spec = spec_fn();`.

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/📐️e/✏️editor/🦀️.rs

- Retained line 51: **RecordSpecProducer**, current direct call. Origin `47: let variants = <Self as dsl::DslVariants>::variants();`. Authored `let record = dsl::parse(line, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;`; current `let record = dsl::parse(line, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;`.
- Retained line 61: **RecordSpecProducer**, current direct call. Origin `59: let variants = <Self as dsl::DslVariants>::variants();`. Authored `dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)`; current `dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)`.
- Retained line 89: **RecordSpecProducer**, current direct call. Origin `87: let variants = <Self as dsl::DslVariants>::variants();`. Authored `let spec = spec_fn();`; current `let spec = spec_fn();`.

## ✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs

- Retained line 959: **RecordSpecProducer**, current direct call. Origin `955: let variants = <Self as dsl::DslVariants>::variants();`. Authored `let record = dsl::parse(line, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;`; current `let record = dsl::parse(line, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;`.
- Retained line 969: **RecordSpecProducer**, current direct call. Origin `967: let variants = <Self as dsl::DslVariants>::variants();`. Authored `dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)`; current `dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)`.

## ✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🛜️wire-runtime/🦀️.rs

- Retained line 115: **RecordSpecProducer**, current direct call. Origin `111: let variants = <Self as dsl::DslVariants>::variants();`. Authored `let record = dsl::parse(line, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;`; current `let record = dsl::parse(line, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;`.
- Retained line 125: **RecordSpecProducer**, current direct call. Origin `123: let variants = <Self as dsl::DslVariants>::variants();`. Authored `dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)`; current `dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)`.

## ✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🕸️graph/🎚️config/🧬️schema/🧬️mutations/🦀️.rs

- Retained line 30: **RecordSpecProducer**, current direct call. Origin `26: let variants = <Self as dsl::DslVariants>::variants();`. Authored `let record = dsl::parse(line, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;`; current `let record = dsl::parse(line, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;`.
- Retained line 40: **RecordSpecProducer**, current direct call. Origin `38: let variants = <Self as dsl::DslVariants>::variants();`. Authored `dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)`; current `dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)`.

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/🌐️iso21320/✏️editor/🦀️.rs

- Retained line 40: **RecordSpecProducer**, current direct call. Origin `36: let variants = <Self as dsl::DslVariants>::variants();`. Authored `let record = dsl::parse(line, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;`; current `let record = dsl::parse(line, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;`.
- Retained line 50: **RecordSpecProducer**, current direct call. Origin `48: let variants = <Self as dsl::DslVariants>::variants();`. Authored `dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)`; current `dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)`.
- Retained line 78: **RecordSpecProducer**, current direct call. Origin `76: let variants = <Self as dsl::DslVariants>::variants();`. Authored `let spec = spec_fn();`; current `let spec = spec_fn();`.

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗜️deflate/🏅️standards/🔖️rfc1950/🪆️subsets/✳️any/✏️editor/🦀️.rs

- Retained line 50: **RecordSpecProducer**, current direct call. Origin `46: let variants = <Self as dsl::DslVariants>::variants();`. Authored `let record = dsl::parse(line, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;`; current `let record = dsl::parse(line, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;`.
- Retained line 60: **RecordSpecProducer**, current direct call. Origin `58: let variants = <Self as dsl::DslVariants>::variants();`. Authored `dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)`; current `dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)`.
- Retained line 88: **RecordSpecProducer**, current direct call. Origin `86: let variants = <Self as dsl::DslVariants>::variants();`. Authored `let spec = spec_fn();`; current `let spec = spec_fn();`.

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/🧱️base/✏️editor/🦀️.rs

- Retained line 42: **RecordSpecProducer**, current direct call. Origin `38: let variants = <Self as dsl::DslVariants>::variants();`. Authored `let record = dsl::parse(line, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;`; current `let record = dsl::parse(line, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;`.
- Retained line 52: **RecordSpecProducer**, current direct call. Origin `50: let variants = <Self as dsl::DslVariants>::variants();`. Authored `dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)`; current `dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)`.
- Retained line 80: **RecordSpecProducer**, current direct call. Origin `78: let variants = <Self as dsl::DslVariants>::variants();`. Authored `let spec = spec_fn();`; current `let spec = spec_fn();`.

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗜️deflate/🏅️standards/🔖️rfc1950/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs

- Retained line 106: **RecordSpecProducer**, current direct call. Origin `102: let variants = <Self as dsl::DslVariants>::variants();`. Authored `let record = dsl::parse(line, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;`; current `let record = dsl::parse(line, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;`.
- Retained line 116: **RecordSpecProducer**, current direct call. Origin `114: let variants = <Self as dsl::DslVariants>::variants();`. Authored `dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)`; current `dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)`.

## ✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🧬️schema/🧬️mutations/🦀️.rs

- Retained line 19: **RecordSpecProducer**, current direct call. Origin `17: for (keyword, spec_fn) in <Self as dsl::DslVariants>::variants() {`. Authored `let record = dsl::parse(line, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;`; current `let record = dsl::parse(line, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;`.

## ✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🧬️mutations/🦀️.rs

- Retained line 19: **RecordSpecProducer**, current direct call. Origin `17: for (keyword, spec_fn) in <Self as dsl::DslVariants>::variants() {`. Authored `let record = dsl::parse(line, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;`; current `let record = dsl::parse(line, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;`.

## ✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🧬️mutations/🦀️.rs

- Retained line 25: **RecordSpecProducer**, current direct call. Origin `22: let variants = <Self as dsl::DslVariants>::variants();`. Authored `let record = dsl::parse(line, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;`; current `let record = dsl::parse(line, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;`.

## ✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🧬️schema/🧬️mutations/🦀️.rs

- Retained line 19: **RecordSpecProducer**, current direct call. Origin `17: for (keyword, spec_fn) in <Self as dsl::DslVariants>::variants() {`. Authored `let record = dsl::parse(line, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;`; current `let record = dsl::parse(line, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;`.

## ✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🧬️mutations/📝️text/🦀️.rs

- Retained line 16: **RecordSpecProducer**, current direct call. Origin `12: let variants = <Self as dsl::DslVariants>::variants();`. Authored `let record = dsl::parse(line, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;`; current `let record = dsl::parse(line, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;`.
- Retained line 26: **RecordSpecProducer**, current direct call. Origin `24: let variants = <Self as dsl::DslVariants>::variants();`. Authored `dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)`; current `dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)`.

## ✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🦀️.rs

- Retained line 132: **RecordSpecProducer**, current direct call. Origin `127: let variants = <Self as dsl::DslVariants>::variants();`. Authored `let record = dsl::parse(body, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;`; current `let record = dsl::parse(body, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;`.
- Retained line 142: **RecordSpecProducer**, current direct call. Origin `140: let variants = <Self as dsl::DslVariants>::variants();`. Authored `let body = dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline);`; current `let body = dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline);`.

## ✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🦀️.rs

- Retained line 156: **RecordSpecProducer**, current direct call. Origin `154: let variants = <Self as dsl::DslVariants>::variants();`. Authored `let spec = spec_fn();`; current `let spec = spec_fn();`.

## ✏️s/🔌️plugins/📜️imperative/🗿️artifacts/📜️procedure/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️.rs

- Retained line 45: **RecordSpecProducer**, current direct call. Origin `41: let variants = <Self as dsl::DslVariants>::variants();`. Authored `let record = dsl::parse(line, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;`; current `let record = dsl::parse(line, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;`.
- Retained line 55: **RecordSpecProducer**, current direct call. Origin `53: let variants = <Self as dsl::DslVariants>::variants();`. Authored `dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)`; current `dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)`.

## ✏️s/🔌️plugins/📜️imperative/🗿️artifacts/📜️procedure/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🧬️mutations/🦀️.rs

- Retained line 29: **RecordSpecProducer**, current direct call. Origin `27: for (keyword, spec_fn) in <Self as dsl::DslVariants>::variants() {`. Authored `let record = dsl::parse(line, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;`; current `let record = dsl::parse(line, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;`.

## ✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🔲️grid2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️text/🦀️.rs

- Retained line 144: **RecordSpecProducer**, current direct call. Origin `140: let variants = <Self as dsl::DslVariants>::variants();`. Authored `let record = dsl::parse(line, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;`; current `let record = dsl::parse(line, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;`.
- Retained line 154: **RecordSpecProducer**, current direct call. Origin `152: let variants = <Self as dsl::DslVariants>::variants();`. Authored `dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)`; current `dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)`.

## ✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/🧬️schema/🧬️mutations/🦀️.rs

- Retained line 23: **RecordSpecProducer**, current direct call. Origin `19: let variants = <Self as dsl::DslVariants>::variants();`. Authored `let record = dsl::parse(line, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;`; current `let record = dsl::parse(line, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;`.
- Retained line 33: **RecordSpecProducer**, current direct call. Origin `31: let variants = <Self as dsl::DslVariants>::variants();`. Authored `dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)`; current `dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)`.

## ✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🧬️mutations/🦀️.rs

- Retained line 32: **RecordSpecProducer**, current direct call. Origin `28: let variants = <Self as dsl::DslVariants>::variants();`. Authored `let record = dsl::parse(line, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;`; current `let record = dsl::parse(line, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;`.
- Retained line 42: **RecordSpecProducer**, current direct call. Origin `40: let variants = <Self as dsl::DslVariants>::variants();`. Authored `dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)`; current `dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)`.

## ✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🧬️mutations/📝️text/🦀️.rs

- Retained line 178: **RecordSpecProducer**, current direct call. Origin `174: let variants = <Self as dsl::DslVariants>::variants();`. Authored `let record = dsl::parse(line, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;`; current `let record = dsl::parse(line, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;`.
- Retained line 188: **RecordSpecProducer**, current direct call. Origin `186: let variants = <Self as dsl::DslVariants>::variants();`. Authored `dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)`; current `dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)`.

## ✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🖼️bitmap/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/🧬️schema/🧬️mutations/🦀️.rs

- Retained line 23: **RecordSpecProducer**, current direct call. Origin `19: let variants = <Self as dsl::DslVariants>::variants();`. Authored `let record = dsl::parse(line, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;`; current `let record = dsl::parse(line, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;`.
- Retained line 33: **RecordSpecProducer**, current direct call. Origin `31: let variants = <Self as dsl::DslVariants>::variants();`. Authored `dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)`; current `dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)`.

## ✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🖼️bitmap/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️text/🦀️.rs

- Retained line 125: **RecordSpecProducer**, current direct call. Origin `121: let variants = <Self as dsl::DslVariants>::variants();`. Authored `let record = dsl::parse(line, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;`; current `let record = dsl::parse(line, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;`.
- Retained line 135: **RecordSpecProducer**, current direct call. Origin `133: let variants = <Self as dsl::DslVariants>::variants();`. Authored `dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)`; current `dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)`.

## ✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🧱️grid3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️text/🦀️.rs

- Retained line 142: **RecordSpecProducer**, current direct call. Origin `138: let variants = <Self as dsl::DslVariants>::variants();`. Authored `let record = dsl::parse(line, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;`; current `let record = dsl::parse(line, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;`.
- Retained line 152: **RecordSpecProducer**, current direct call. Origin `150: let variants = <Self as dsl::DslVariants>::variants();`. Authored `dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)`; current `dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)`.

## ✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/🧬️schema/🧬️mutations/🦀️.rs

- Retained line 23: **RecordSpecProducer**, current direct call. Origin `19: let variants = <Self as dsl::DslVariants>::variants();`. Authored `let record = dsl::parse(line, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;`; current `let record = dsl::parse(line, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;`.
- Retained line 33: **RecordSpecProducer**, current direct call. Origin `31: let variants = <Self as dsl::DslVariants>::variants();`. Authored `dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)`; current `dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)`.

## ✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️text/🦀️.rs

- Retained line 185: **RecordSpecProducer**, current direct call. Origin `181: let variants = <Self as dsl::DslVariants>::variants();`. Authored `let record = dsl::parse(line, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;`; current `let record = dsl::parse(line, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;`.
- Retained line 195: **RecordSpecProducer**, current direct call. Origin `193: let variants = <Self as dsl::DslVariants>::variants();`. Authored `dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)`; current `dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)`.

## ✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🧬️mutations/🦀️.rs

- Retained line 30: **RecordSpecProducer**, current direct call. Origin `26: let variants = <Self as dsl::DslVariants>::variants();`. Authored `let record = dsl::parse(line, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;`; current `let record = dsl::parse(line, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;`.
- Retained line 40: **RecordSpecProducer**, current direct call. Origin `38: let variants = <Self as dsl::DslVariants>::variants();`. Authored `dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)`; current `dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)`.

## Additional caller names outside literal inventory

Retained-pack-load fixture lines95 and builder dependency-contribution fixture31 call `spec()` from DslVariants .find/.map; these are the same producer origin despite another variable name. Literal spec_fn inventory does not exhaust this API. Inspect Shape::Record/other aliases by typed constructor origin, preserve genuine registry functions.

Classification totals: {'RecordSpecProducer': 249, 'genuine callable': 1}. Retained sites 250, files 108. This is source typing evidence, not a compiler passing receipt. Current already-rebound producer cases must not be edited again from stale line numbers.
