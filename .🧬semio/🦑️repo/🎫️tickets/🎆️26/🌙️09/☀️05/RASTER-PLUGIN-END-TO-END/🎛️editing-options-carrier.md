# Paint2d Editing Options Carrier Map

Read-only implementation map for required Paint2dEditingOptions `{filterKind,filterAmount,imageGrid:{x,y,width,height,bilinear}}`. Scene types must stay framework-owned and domain-neutral; project RasterConfig into this shape rather than importing the Raster plugin type into UI.

## Carrier and Projection

- Rust scene owner: `🧰️framework/🔨️modules/🖱️ui/🎬️scene/🎬️scenes/🖼️plane/🎨️paint2d-scene/🦀️.rs`. Add required editing_options to Paint2dScene, plus nested records with serde camelCase and actual ToValue/FromValue conversions. No optional/default legacy behavior. New small options belong in the bounded spine; three large lanes remain document/assets/pixelSelection.
- Explicit native exports: plane `🦀️.rs` and scene `📦️packages/🦀️rust/🦀️.rs`. Any use through plugin/SDK facades must expose the new type explicitly if those facades do not already wildcard it.
- TypeScript scene owner: same scene root `📄️documents/🟦️.ts`; explicit type exports in scene `🟦️.ts` and `🧰️framework/📦️packages/🟦️typescript/🟦️.ts:36`.
- Native config projection: Raster plugin `🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧰️scene-helpers/🦀️.rs::raster_scene`. Composite and Navigator both call it. Copy the existing runtime filter_kind, filter_amount and image_grid fields explicitly into the neutral scene records. No new document authority.
- Typed Raster Composite view-model twin: same editor `🎭️modes/✏️edit/🪟️windows/🖼️composite/🟦️.ts`; keep public model aligned if it declares these chrome measures.
- React integration: renderer engine `🧱️elements/🖌️Paint2dHost/🟦️.tsx:719` forwards options to editing `🟦️.tsx`. Replace private filter/amount/width/height states with controlled options and semantic setFilterKind/setFilterAmount/setImageGrid dispatch. Retain active-layer dimensions as display/validation data, not a useEffect that silently overwrites persisted staged imageGrid. Shape crop and staged-grid crop should remain distinguishable actions.

## Mandatory Constructors, Fixtures and Tests

Relative to UI scene root: `🧪️tests/🔬️plane-unit/🦀️.rs` Paint2dScene literal and lane roundtrip; `🧪️tests/🧫️fixtures/🚚️paint2d-scene-lanes/🔣️.json` bounded spine. Lane reconstruction itself is generic and requires no fourth lane.

Relative to UI root: `🧪️tests/🔬️targets-wgpu-component-ui-ui-node-wire-format/🦀️.rs` full Paint2dScene constructor and escaped GOLDEN_SCENES_JSON (line402); `🧬️contract/🧵️retained/🧪️tests/🧫️fixtures/🧾️typed-scene/🔣️.json` typed retained scene golden.

Relative to renderer engine root: native literals `🎯️targets/🧊️wgpu/🧿️ui-engine-tests/🧰️surface-fixtures/🦀️.rs` and `🧱️elements/⚙️EngineCanvas/🧪️tests/🖌️wgpu-paint2d-engine/🧰️scene-fixtures-and-drains/🦀️.rs`; TS authored scenes `🧪️tests/🎞️timeline-paint-vfs-hosts/🟦️.ts`, `🧪️tests/🪶️surface-idle-frames/🟦️.tsx`, `🧱️elements/🖌️Paint2dHost/🧪️tests/🧭️navigator-camera/🟦️.tsx`, `🧱️elements/🖌️Paint2dHost/📖️stories/🧪️.story.tsx`. Editing mounted `✍️editing/🧪️tests/🖱️selection-focus/🟦️.tsx` needs semantic config republishing for kind, amount and each grid field, plus required controlled prop in every direct editor construction.

Raster native scene laws to extend: editor `🧪️tests/🔬️unit/📥️envelope-and-scene-laws/🦀️.rs`, `🏔️mounted-boot-laws/🦀️.rs`, `🧭️layer-and-camera-laws/🦀️.rs`. Assert actual config→scene options, roundtrip and preserved unrelated config. Shared JSON fixtures should pin nondefault gamma/amount and nonzero grid origin/bilinear to catch hardcoded defaults.

Native controls agent confirms current fields: filter_kind:String, filter_amount:f64; RasterImageGrid x/y/width/height:u32 and bilinear:bool. Defaults invert/0 and 0,0,512,512,false. Current commands accept setFilterKind `{value}`, setFilterAmount `{value}` and setImageGrid `{field,value}` with grid value string-normalized by existing bridge. Controlled React should dispatch these existing actions, then wait for republished scene rather than optimistic private staging.

No source edits or validation execution performed by this audit.

## Full Native Literal and Facade Follow-Up

Completed Rust-only authored-source scan of framework and S, excluding dependency/build/generated/dot directories (`literal-audit.ts`, session70415 exit0). It found no further actual Paint2dScene constructors beyond the four framework sites above and Raster scene-helpers. Definitions, impl blocks and return signatures were not treated as constructors. This is read-only inventory evidence, not compilation.

The additional explicit plugin facade is `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:584–587`, whose semio_framework_ui_scene export list names Paint2dScene. It must also expose new options/grid records used by the plugin projection. Exact export clauses were sent to parent.

New serde carrier structs deny unknown fields; initial manual FromValue codecs did not. Parent was notified and supplied the exact existing same-crate refusal pattern from `🖋️ink-canvas-scene/🦀️.rs:26–33`: ValueError::new(protocol::value::ValueRefusalKind::InvalidValue, format!("unknown field `{field}`")), with the same constructor for duplicate fields after counting allowed keys. Final strictness changes remain owned by parent.

## Fixture Implementation

At parent request, updated the four enumerated framework Rust fixture files and both enumerated lane/typed JSON fixtures. Every literal now supplies explicit brightness/0 options with grid 0,0,512,512,bilinear=true. Renderer fixtures use their existing scene-plane crate path. Updated wire-format GOLDEN_SCENES_JSON to the actual serde nested record shape and floating 0.0. Typed-scene grid u32 fields use the existing native $u64 tags. Lane spineFields, assembled and spine include editingOptions; large lane references/hashes remain unchanged.

Both JSON fixtures parsed through inspect --json, and the Rust escaped golden string was decoded as a string then parsed as JSON, confirming its exact options. These are JSON inspections, not Rust compilation or test execution. Concurrent native-selection regression code in the surface fixture was preserved.

## TypeScript Scene Fixture Implementation

Updated eight required scene literals in the four previously enumerated renderer files: timeline-paint-vfs-hosts (five), surface-idle-frames (one), navigator-camera (one), Paint2dHost story (one). Each supplies explicit editingOptions brightness/0 and grid 0,0,512,512,bilinear=true; brightness uses a const assertion so inferred standalone scene objects retain the required filter union. Parent-owned Paint2dHost forwarding, editing component and editing mounted tests were untouched.

inspect --syntax accepted all four changed TS/TSX files. This is Bun syntax scanning only, not type checking or test execution. The renderer-engine fillTolerance census identifies only those four files plus the three parent-owned files; no additional renderer scene literal was found. Remaining UI scene-module matches are declarations/generic lane reconstruction, while Raster config matches are config objects/interfaces rather than Paint2dScene constructors.
