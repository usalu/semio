# WP-02 Contracts (A2): diagram, handle, field-spec, lane, unit schemas

Status: code complete; build and test verification is queued behind host memory pressure (see "Verification").

## Built API

### `NORM/🎬️structural-scene/🧬️schema/📈️diagram2d` (`🦀️.rs`, `🛰️.proto`, `🔗️.graphql`, `🟦️.ts`, `🧪️tests`)
- Records: `Diagram2d{id,title,description,axes,series,markers,annotations,focus,legend,table_twin}`, `Axis{label,quantity,unit(symbol),scale,domain,ticks}`, `Marker{..,binding:Option<FieldBinding>}`, `DiagramNote`, `DataTable/DataColumn`, `Stroke`, `Fill`, `ScatterPoint`, `Line2`, `SectionProfile`, `DiagramMesh`, `Interval`.
- Variants: `Series` = `Polyline|Polygon|Band|Scatter|Limit|Section|Ray|Surface3` (every variant carries `id` and a bilingual `label`), `TickRule`.
- Choices: `AxisScale{Linear,Log10,Sqrt}`, `AxisRef`, `Dash`, `MarkerShape`.
- `Validate`: exactly two axes, unique ids across series/markers/notes, log/sqrt domain and data positivity, polygon/band/ray/mesh sanity, bilingual non-empty text, equal-length table-twin columns.
- Deviations from §4.2: arrays/tuples are replaced by `Vec2`/`Vec<..>` and `Interval` (the twin generator knows only records, variants, choices); `Axis.unit` is a unit symbol string (the structural-scene crate has no dependency on the compliance crate); `Diagram2d` gained `description` (a11y text).

### `NORM/🎬️structural-scene/🧬️schema/🕹️handles`
- `Handle{id,entity,kind,anchor,grip,writes,snap,limits,accessible_name}`, `FieldBinding{path,map:Affine,unit,quantise}` (`FieldBinding::new`, `value_at(s)`), `Affine{base,k}` (`apply`, `invert`), `SnapRule{grid,increment,targets,catalogue,tolerance}` (`apply`, `NONE`), `HandleDrag`, `ResolvedWrite`.
- Variants `HandleKind` = `Translate|TranslatePlane|ScalarAlong|Radial|Angular|DimensionLine|CountStepper|FaceOffset`, `Quantise` = `Round|Choice|Int`; choice `GripStyle`.
- Pure drag evaluation: `Handle::resolve_s(s)` (clamp, snap, clamp) and `Handle::writes_at(s) -> Vec<ResolvedWrite>`.
- `limits` bound the drag parameter `s` (handle units), not the field value.

### Twins
`wp02-gen-twins.py` (ticket folder) regenerates the part twins and the live-lanes twins from the Rust declarations (`--check` mode). Part protos import top-level `🛰️.proto` for shared types while the top-level imports the parts (circular, as A1 requested); no protoc run exists in the repo, so this is only a design caveat.

### `SURFACE/🫧️live-lanes` (+ `🧬️schema/{🦀️.rs,🛰️.proto,🟦️.ts,🔗️.graphql}`)
`NormTransient{drag,ghost,eval}`, `DragPreview{handle,writes,started_seq}` with `DragPreview::at(&Handle, s, seq)`, `GhostPreview{writes,source}`, `GhostSource::{Remedy{check_id,index},Hover}`, `EvalProgress{running(check-unit ids as String),done,total,cancelled}` (`fraction`, `is_idle`), `NormPresence{user,selection,hover,camera,drag}`, `CameraPose`, `FieldWrite{path,value:DslValue}`, `Lane` (the four persistence/sharing lanes). `NormTransient::{with_drag,with_ghost,with_eval,is_clear,overlay}`. Plain value types only: the framework lane traits (`RetireOwned`, `ArtifactDsl`, `ArtifactPack`, `ArtifactStoreSnapshot`, mutation types) are left to A10 (`🧵️lane-runtime`). Mounted in `SURFACE/🦀️.rs` by A2 (`mounts.md`).

### `SURFACE/🔖️field-meta` -> `FieldSpec`
- `FieldSpec{label:FieldLabel,help,clause,kind:FieldKind,range,soft_range,step,precision,group,order,tier,visible_if,ndp,handle,affects}`, const `FieldSpec::EMPTY` and const constructors `new(en,de)`, `quantity(en,de,UnitId)`, `choice(en,de,&[NormFieldChoice])`.
- `FieldKind = Inferred|Text|Int|Bool|Angle|Ratio|Percent|Choice|EntityRef{to}|Quantity{stored,display,units}`. Deviation: `Quantity` carries the **stored** unit and the display unit (not a `QuantityKind`) because several existing tables (DIN: `°C`, `a`, `%`) do not store SI base units; conversion runs `display -> SI base -> stored` at the field boundary only (`to_stored`, `to_display`, `parse`, `format`, `accepts`, `units`, `display_unit(UnitSystem)`).
- Lookup unchanged: `lookup_norm_field_meta`, `resolve_field_meta`, `empty_field_meta`, `NormFieldMetaFn` (type now returns `FieldSpec`). `NormFieldMeta` is deleted; `NormFieldChoice` unchanged. Re-exports `UnitId, UnitSystem, UnitFamily, Measure, UnitError`.
- Converted mechanically (`wp02-convert-field-meta.py`, no content change): ten Eurocode tables plus din16798, din18599, din4108, iso16757, vdi3805 (same type), en1997's table (lives in `schema/inferences/checks`), and every test reading `meta.label_en/unit/choices`.

### `NORM/⚖️compliance/📐️quantities-clauses-and-text/🔢️units`
- `UnitId` (about 110 units, families in `UnitFamily`), `symbol()` (wire form), `name(Locale)` EN/DE, `family()`, `factor()/offset()` (`si = v*factor + offset`), `compatible()`, `base()`, `quantity_kind()`, `to_si/from_si/convert`, `lookup(symbol|ASCII alias|EN/DE name)`, `in_system(UnitSystem::{Si,Engineering})`.
- `Measure{value,unit}` with `parse(text, fallback)` (decimal comma or point, grouping spaces, `e` exponents, names) and `format(precision, Locale)`; `format_number` (thin-space grouping from five integer digits, locale decimal mark); `UnitError`.
- `QuantityKind` and the compliance wire schema are unchanged on purpose.

## Tests and oracles
- Unit tests: `diagram2d/🧪️tests/🔬️unit`, `handles/🧪️tests/🔬️unit`, `SURFACE/🫧️live-lanes/🧪️tests/🔬️unit`, `SURFACE/🔖️field-meta/🧪️tests/🔬️unit`, `units/🧪️tests/🔬️unit`.
- `.feature` per feature: next to each of the above (`🧪️tests/🥒️.feature`).
- Oracle: `units/🔮️oracles/🐍️.py` (`python -I`, stdin/stdout JSON, exact rational algebra, no pint available in this environment): factor, offset and SI dimension of every unit symbol, all compatible conversions, number formatting and parsing; spawned by the Rust test through `python_oracle`.

## Verification
NOT YET VERIFIED BY A BUILD. All `repo` capacity requests stayed queued for over 60 minutes behind other agents (host memory pressure critical, swap above ceiling). A detached `repo run @semio-s/plugin-norm-app-surface:check` (compiles compliance, structural-scene and app-surface) is still queued; its log is `🗑️generated/wp02/surface-check.txt` (last line `exit N` when done). Nothing was compiled or run by A2: no cargo, no rustfmt, only `python -I` runs of the unit oracle (answers for 28 symbols, conversions, number text and parse cases looked right by hand) and the twin generator.
Next steps for whoever picks this up: read that log, fix compile errors (likely spots: `use super::*` glob chains in `🔢️units`, `FieldSpec` const struct-update, `DslValue`/`Number` import in live-lanes), then `repo run @semio-s/plugin-norm-app-surface:test`, the compliance test project (name not discoverable without `show projects`; crate `semio-s-plugin-norm-compliance`), the structural-scene crate tests, and each artifact `@semio-tech/norm-enXXXX-rs:test`.
Log folder `🗑️generated/wp02` is intentionally kept until the check log is read.

## Open requests
Listed in `📌️important/requests.md` (A2 -> A6 `properties` field accessors and `UnitPreset`, A2 -> A10 lane runtime, A2 -> artifact agents `FieldSpec` rows, inputs windows still call the removed `render_document_editor`).
