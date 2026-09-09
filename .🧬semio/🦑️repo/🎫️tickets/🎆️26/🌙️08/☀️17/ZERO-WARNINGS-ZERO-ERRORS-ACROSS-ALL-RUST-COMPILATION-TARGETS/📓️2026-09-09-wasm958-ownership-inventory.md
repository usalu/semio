# WASI Ownership Diagnostics

## clippy::unnecessary_wraps at ✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:3140

```rust
fn puzzle3d_retained_reduce_in_session(
    session: Option<(u32, Option<String>)>,
    command: &Puzzle3dCommand,
    snapshot: &Puzzle3dPlaySnapshot,
    config: &Puzzle3dConfig,
    interaction: &protocol::InteractionState,
    hover: &semio_framework_plugin::app::InteractionHoverState,
    view_state: Option<&semio_framework_plugin::ViewModel>,
) -> Result<Emit<Puzzle3dMutation, Puzzle3dConfigMutation>, Fault> {
```
for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#unnecessary_wraps
`-D clippy::unnecessary-wraps` implied by `-D warnings`
to override `-D warnings` add `#[allow(clippy::unnecessary_wraps)]`

## clippy::unnecessary_wraps at ✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⏳️precompute/🦀️.rs:1357

```rust
    pub(crate) fn set_scene_config(&mut self, scene: SceneConfig) -> Result<(), Puzzle3dError> {
```
for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#unnecessary_wraps

## clippy::unnecessary_wraps at ✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⏳️precompute/📐️geometry/🦀️.rs:1066

```rust
fn collision_index_backing_credit(credit: Option<(usize, usize)>) -> Option<(usize, usize)> {
```
for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#unnecessary_wraps

## clippy::unnecessary_wraps at ✏️s/🔌️plugins/🎪️demonstrator/🗿️artifacts/🎪️playground/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✒️change-schema/🦀️.rs:52

```rust
fn bridge_render(snapshot: &PlaygroundSnapshot, messages: Vec<String>) -> Result<String, String> {
```
for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#unnecessary_wraps
`-D clippy::unnecessary-wraps` implied by `-D warnings`
to override `-D warnings` add `#[allow(clippy::unnecessary_wraps)]`

## clippy::needless_pass_by_value at 🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/🦀️.rs:2705

```rust
fn node_eval_status_json(status: NodeEvalStatus) -> crate::os_pack::json::Value {
```
for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#needless_pass_by_value
`-D clippy::needless-pass-by-value` implied by `-D warnings`
to override `-D warnings` add `#[allow(clippy::needless_pass_by_value)]`

## clippy::needless_pass_by_value at 🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/📐️brep-geometry/🦀️.rs:236

```rust
pub fn map_kernel_error(error: semio_s_artifact_stdio_semio::standards::v1::subsets::brep::schema::engine::BrepError) -> EvalError {
```
for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#needless_pass_by_value

## clippy::large_enum_variant at ✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🦀️.rs:244

```rust
pub enum CadConfigMutation {
    #[dsl(key = "snapshot")]
    Snapshot {
        #[dsl(block)]
        config: CadConfig,
    },
    #[dsl(key = "contributions")]
    SetContributions { json: String },
}
```
for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#large_enum_variant
`-D clippy::large-enum-variant` implied by `-D warnings`
to override `-D warnings` add `#[allow(clippy::large_enum_variant)]`

## clippy::large_enum_variant at ✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️.rs:341

```rust
        enum $state {
            AwaitToken,
            Decode(store::OwnedSchemaHexAuthority<GIS_MAP_OWNED_FIELD_BYTES>),
            Ready,
            Published,
            Closing,
            Complete,
        }
```
for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#large_enum_variant
`-D clippy::large-enum-variant` implied by `-D warnings`
to override `-D warnings` add `#[allow(clippy::large_enum_variant)]`

## clippy::large_enum_variant at ✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️.rs:341

```rust
        enum $state {
            AwaitToken,
            Decode(store::OwnedSchemaHexAuthority<GIS_MAP_OWNED_FIELD_BYTES>),
            Ready,
            Published,
            Closing,
            Complete,
        }
```
for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#large_enum_variant

## clippy::needless_pass_by_value at ✏️s/🔌️plugins/🏗️fem/⚙️engine/🖥️app-surface/🦀️.rs:24

```rust
pub fn canvas_2d_surface(id: impl Into<String>, scene: semio_framework_ui_scene::Canvas2dScene) -> semio_framework_plugin::UiAssemblyResult<BuiltNode> {
```
for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#needless_pass_by_value
`-D clippy::needless-pass-by-value` implied by `-D warnings`
to override `-D warnings` add `#[allow(clippy::needless_pass_by_value)]`

## clippy::needless_pass_by_value at ✏️s/🔌️plugins/🏗️fem/⚙️engine/🖥️app-surface/🦀️.rs:35

```rust
pub fn world_3d_surface(id: impl Into<String>, scene: semio_framework_ui_scene::World3dScene) -> semio_framework_plugin::UiAssemblyResult<BuiltNode> {
```
for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#needless_pass_by_value

## clippy::large_enum_variant at ✏️s/🔨️modules/🏗️fem/⚙️engine/🕸️mesh/🦀️.rs:182

```rust
enum MeshDomainOwner {
    Dynamic(PlanarDomain),
    Mounted(MountedPlanarDomain),
}
```
for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#large_enum_variant
`-D clippy::large-enum-variant` implied by `-D warnings`
to override `-D warnings` add `#[allow(clippy::large_enum_variant)]`

## clippy::unnecessary_wraps at ✏️s/🔨️modules/🏗️fem/⚙️engine/🔢️sparse/🦀️.rs:3445

```rust
    fn advance_orthogonalize(&mut self) -> Result<(), ()> {
```
for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#unnecessary_wraps
`-D clippy::unnecessary-wraps` implied by `-D warnings`
to override `-D warnings` add `#[allow(clippy::unnecessary_wraps)]`

## clippy::unnecessary_wraps at ✏️s/🔨️modules/🏗️fem/⚙️engine/🔢️sparse/🦀️.rs:3485

```rust
    fn advance_normalize(&mut self) -> Result<(), ()> {
```
for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#unnecessary_wraps

## clippy::needless_pass_by_value at ✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🧱️model/🦀️.rs:915

```rust
fn vector_layer(id: String, origin: (f64, f64), vector: [f64; 2], color: &str) -> dsl::json::Value {
```
for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#needless_pass_by_value

## clippy::needless_pass_by_value at ✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📊️results/🦀️.rs:24

```rust
fn filled_triangle_layer(id: String, p0: (f64, f64), p1: (f64, f64), p2: (f64, f64), color: &str, alpha: f64) -> Value {
```
for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#needless_pass_by_value

## clippy::needless_pass_by_value at ✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📊️results/🦀️.rs:42

```rust
fn filled_polygon_layer(id: String, points: &[(f64, f64)], color: &str, alpha: f64) -> Value {
```
for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#needless_pass_by_value

## clippy::unnecessary_wraps at ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧩️wfc-engine/💼️job/🦀️.rs:1477

```rust
    fn rebuild_one(&mut self) -> Result<(), String> {
```
for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#unnecessary_wraps
`-D clippy::unnecessary-wraps` implied by `-D warnings`
to override `-D warnings` add `#[allow(clippy::unnecessary_wraps)]`

## clippy::unnecessary_wraps at ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧩️wfc-engine/💼️job/🦀️.rs:1513

```rust
    fn finish(&mut self) -> Result<(), String> {
```
for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#unnecessary_wraps

## clippy::needless_pass_by_value at ✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs:394

```rust
pub fn note_block_patch_diff(id: &str, block: NoteBlockNode) -> NoteDiff {
```
for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#needless_pass_by_value
`-D clippy::needless-pass-by-value` implied by `-D warnings`
to override `-D warnings` add `#[allow(clippy::needless_pass_by_value)]`

## clippy::large_enum_variant at ✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧵️retained/🦀️.rs:1059

```rust
enum NoteBlockPayloadMaterialization {
    Text {
        phase: u8,
        content: NoteTextContentMaterializationCursor,
        materialized_content: Option<crate::NoteTextChild>,
        font_weight: NoteStringMaterializationCursor,
        materialized_font_weight: Option<String>,
        align: NoteStringMaterializationCursor,
        materialized_align: Option<String>,
    },
    Image {
        image_key: NoteStringMaterializationCursor,
        materialized_image_key: Option<String>,
    },
    Table {
        phase: u8,
        column_cursor: usize,
        cell_row_cursor: usize,
        cell_cursor: usize,
        string: NoteStringMaterializationCursor,
        columns: Vec<String>,
        rows: Vec<Vec<crate::NoteTableCell>>,
    },
    Math {
        tex: NoteStringMaterializationCursor,
        materialized_tex: Option<String>,
    },
    Ink {
        point_cursor: usize,
        points: Vec<[f64; 2]>,
    },
    Group {
        child_cursor: usize,
        active: Option<Box<NoteBlockMaterializationCursor>>,
        children: Vec<crate::NoteBlockNode>,
    },
}
```
for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#large_enum_variant
`-D clippy::large-enum-variant` implied by `-D warnings`
to override `-D warnings` add `#[allow(clippy::large_enum_variant)]`

## clippy::unnecessary_wraps at ✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📤️export/🦀️.rs:1114

```rust
    fn escaped_string_step(value: &str, cursor: &mut JsonStringWriteCursor) -> Result<(Vec<u8>, bool), String> {
```
for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#unnecessary_wraps
`-D clippy::unnecessary-wraps` implied by `-D warnings`
to override `-D warnings` add `#[allow(clippy::unnecessary_wraps)]`

## clippy::unnecessary_wraps at ✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📤️export/🦀️.rs:2822

```rust
    fn close_pending(released_items: usize, released_bytes: usize) -> Result<PluginCloseStep, Fault> {
```
for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#unnecessary_wraps

## clippy::large_enum_variant at ✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🏗️run-reconstruction/🦀️.rs:125

```rust
enum RasterAssetProgress {
    Working,
    Mutation(RemodelingMutation),
    Complete(ReconstructionAssetCommit),
    Failed,
}
```
for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#large_enum_variant
`-D clippy::large-enum-variant` implied by `-D warnings`
to override `-D warnings` add `#[allow(clippy::large_enum_variant)]`

## clippy::needless_pass_by_value at ✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🏗️run-reconstruction/🦀️.rs:560

```rust
fn emit_step(job: ReconstructionJob, generation: u64, next: Option<AdvanceReconstruction>) -> Emit<RemodelingMutation, RemodelingConfigMutation> {
```
for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#needless_pass_by_value
`-D clippy::needless-pass-by-value` implied by `-D warnings`
to override `-D warnings` add `#[allow(clippy::needless_pass_by_value)]`

## clippy::unnecessary_wraps at ✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🏗️run-reconstruction/🦀️.rs:643

```rust
fn begin_requested_reconstruction(doc: &ArtifactView<'_, RemodelingSnapshot>, requested_stage: RequestedStage) -> Result<Emit<RemodelingMutation, RemodelingConfigMutation>, Fault> {
```
for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#unnecessary_wraps
`-D clippy::unnecessary-wraps` implied by `-D warnings`
to override `-D warnings` add `#[allow(clippy::unnecessary_wraps)]`

## clippy::needless_pass_by_value at ✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation/🦀️.rs:392

```rust
pub fn equation_snapshot_with_state(graph: EquationGraph, geometry: EquationGeometry) -> EquationSnapshot {
```
for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#needless_pass_by_value
`-D clippy::needless-pass-by-value` implied by `-D warnings`
to override `-D warnings` add `#[allow(clippy::needless_pass_by_value)]`

## clippy::needless_pass_by_value at ✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation/🦀️.rs:392

```rust
pub fn equation_snapshot_with_state(graph: EquationGraph, geometry: EquationGeometry) -> EquationSnapshot {
```
for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#needless_pass_by_value

## clippy::needless_pass_by_value at ✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs:179

```rust
    pub fn replace(&mut self, label: EquationNodeLabel, new_kind: EquationNodeKind) -> bool {
```
for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#needless_pass_by_value

## clippy::needless_pass_by_value at ✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:385

```rust
    pub fn from_snapshot(snapshot: SequenceSnapshot) -> Self {
```
for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#needless_pass_by_value
`-D clippy::needless-pass-by-value` implied by `-D warnings`
to override `-D warnings` add `#[allow(clippy::needless_pass_by_value)]`

## clippy::unnecessary_wraps at ✏️s/🔌️plugins/🔋️energy/🔨️modules/⚡️simulation/⚙️engine/🌰️kernel/🦀️.rs:1446

```rust
fn finish_schedule_lookup(work: &mut Option<ScheduleLookupWork>, value: f64) -> Option<f64> {
```
for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#unnecessary_wraps
`-D clippy::unnecessary-wraps` implied by `-D warnings`
to override `-D warnings` add `#[allow(clippy::unnecessary_wraps)]`

## clippy::large_enum_variant at ✏️s/🔌️plugins/🔋️energy/🔨️modules/⚡️simulation/⚙️engine/🧪️sim/🦀️.rs:97

```rust
enum EnergyWireLeaseRecoverySlot {
    Vacant,
    Reserved(u64),
    Abandoned(u64, EnergyWirePacket),
}
```
for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#large_enum_variant
`-D clippy::large-enum-variant` implied by `-D warnings`
to override `-D warnings` add `#[allow(clippy::large_enum_variant)]`

## clippy::result_large_err at ✏️s/🔌️plugins/🔋️energy/🔨️modules/⚡️simulation/⚙️engine/🧪️sim/🦀️.rs:162

```rust
    fn push(&mut self, packet: EnergyWirePacket) -> Result<(), EnergyWirePacket> {
```
try reducing the size of `sim::EnergyWirePacket`, for example by boxing large elements or replacing it with `Box<sim::EnergyWirePacket>`
for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#result_large_err
`-D clippy::result-large-err` implied by `-D warnings`
to override `-D warnings` add `#[allow(clippy::result_large_err)]`

## clippy::result_large_err at ✏️s/🔌️plugins/🔋️energy/🔨️modules/⚡️simulation/⚙️engine/🧪️sim/🦀️.rs:180

```rust
    fn push_reserved(&mut self, packet: EnergyWirePacket) -> Result<(), EnergyWirePacket> {
```
try reducing the size of `sim::EnergyWirePacket`, for example by boxing large elements or replacing it with `Box<sim::EnergyWirePacket>`
for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#result_large_err

## clippy::result_large_err at ✏️s/🔌️plugins/🔋️energy/🔨️modules/⚡️simulation/⚙️engine/🧪️sim/🦀️.rs:209

```rust
    fn retry(&mut self, mut lease: EnergyWireLease) -> Result<(), EnergyWireLease> {
```
try reducing the size of `sim::EnergyWireLease`, for example by boxing large elements or replacing it with `Box<sim::EnergyWireLease>`
for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#result_large_err

## clippy::result_large_err at ✏️s/🔌️plugins/🔋️energy/🔨️modules/⚡️simulation/⚙️engine/🧪️sim/🦀️.rs:225

```rust
    fn ack(&mut self, mut lease: EnergyWireLease) -> Result<(), EnergyWireLease> {
```
try reducing the size of `sim::EnergyWireLease`, for example by boxing large elements or replacing it with `Box<sim::EnergyWireLease>`
for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#result_large_err

## clippy::result_large_err at ✏️s/🔌️plugins/🔋️energy/🔨️modules/⚡️simulation/⚙️engine/🧪️sim/🦀️.rs:244

```rust
    fn ack_transfer(&mut self, mut lease: EnergyWireLease) -> Result<EnergyWirePacket, EnergyWireLease> {
```
try reducing the size of `sim::EnergyWireLease`, for example by boxing large elements or replacing it with `Box<sim::EnergyWireLease>`
for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#result_large_err

## clippy::result_large_err at ✏️s/🔌️plugins/🔋️energy/🔨️modules/⚡️simulation/⚙️engine/🧪️sim/🦀️.rs:545

```rust
    pub fn retry(self, bounds: EnergyNumericalBounds) -> Result<EnergyJob, Self> {
```
try reducing the size of `sim::EnergyAdmissionRejected`, for example by boxing large elements or replacing it with `Box<sim::EnergyAdmissionRejected>`
for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#result_large_err

## clippy::result_large_err at ✏️s/🔌️plugins/🔋️energy/🔨️modules/⚡️simulation/⚙️engine/🧪️sim/🦀️.rs:588

```rust
    pub fn retry(self, bounds: EnergyNumericalBounds) -> Result<EnergyRestoreJob, Self> {
```
try reducing the size of `sim::EnergyCheckpointRejected`, for example by boxing large elements or replacing it with `Box<sim::EnergyCheckpointRejected>`
for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#result_large_err

## clippy::result_large_err at ✏️s/🔌️plugins/🔋️energy/🔨️modules/⚡️simulation/⚙️engine/🧪️sim/🦀️.rs:698

```rust
    pub fn admit(operation: Operation, model: Model, config: SimulationConfig, packet: EnergyWirePacket, bounds: EnergyNumericalBounds) -> Result<Self, EnergyCheckpointRejected> {
```
try reducing the size of `sim::EnergyCheckpointRejected`, for example by boxing large elements or replacing it with `Box<sim::EnergyCheckpointRejected>`
for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#result_large_err

## clippy::result_large_err at ✏️s/🔌️plugins/🔋️energy/🔨️modules/⚡️simulation/⚙️engine/🧪️sim/🦀️.rs:994

```rust
    pub fn finish(mut self, context: &StepContext<'_>) -> Result<EnergyJob, Self> {
```
try reducing the size of `sim::EnergyRestoreJob`, for example by boxing large elements or replacing it with `Box<sim::EnergyRestoreJob>`
for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#result_large_err

## clippy::result_large_err at ✏️s/🔌️plugins/🔋️energy/🔨️modules/⚡️simulation/⚙️engine/🧪️sim/🦀️.rs:1897

```rust
    pub fn new(operation: Operation, model: Model, config: SimulationConfig) -> Result<Self, EnergyAdmissionRejected> {
```
try reducing the size of `sim::EnergyAdmissionRejected`, for example by boxing large elements or replacing it with `Box<sim::EnergyAdmissionRejected>`
for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#result_large_err

## clippy::result_large_err at ✏️s/🔌️plugins/🔋️energy/🔨️modules/⚡️simulation/⚙️engine/🧪️sim/🦀️.rs:1901

```rust
    pub fn admit(operation: Operation, model: Model, config: SimulationConfig, bounds: EnergyNumericalBounds) -> Result<Self, EnergyAdmissionRejected> {
```
try reducing the size of `sim::EnergyAdmissionRejected`, for example by boxing large elements or replacing it with `Box<sim::EnergyAdmissionRejected>`
for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#result_large_err

## clippy::result_large_err at ✏️s/🔌️plugins/🔋️energy/🔨️modules/⚡️simulation/⚙️engine/🧪️sim/🦀️.rs:2015

```rust
    pub fn retry_preview_packet(&mut self, packet: EnergyWirePacket) -> Result<(), EnergyWirePacket> {
```
try reducing the size of `sim::EnergyWirePacket`, for example by boxing large elements or replacing it with `Box<sim::EnergyWirePacket>`
for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#result_large_err

## clippy::result_large_err at ✏️s/🔌️plugins/🔋️energy/🔨️modules/⚡️simulation/⚙️engine/🧪️sim/🦀️.rs:2030

```rust
    pub fn retry_checkpoint_packet(&mut self, lease: EnergyWireLease) -> Result<(), EnergyWireLease> {
```
try reducing the size of `sim::EnergyWireLease`, for example by boxing large elements or replacing it with `Box<sim::EnergyWireLease>`
for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#result_large_err

## clippy::result_large_err at ✏️s/🔌️plugins/🔋️energy/🔨️modules/⚡️simulation/⚙️engine/🧪️sim/🦀️.rs:2034

```rust
    pub fn ack_checkpoint_packet(&mut self, lease: EnergyWireLease) -> Result<(), EnergyWireLease> {
```
try reducing the size of `sim::EnergyWireLease`, for example by boxing large elements or replacing it with `Box<sim::EnergyWireLease>`
for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#result_large_err

## clippy::result_large_err at ✏️s/🔌️plugins/🔋️energy/🔨️modules/⚡️simulation/⚙️engine/🧪️sim/🦀️.rs:2038

```rust
    pub fn ack_checkpoint_for_restore(&mut self, lease: EnergyWireLease) -> Result<EnergyWirePacket, EnergyWireLease> {
```
try reducing the size of `sim::EnergyWireLease`, for example by boxing large elements or replacing it with `Box<sim::EnergyWireLease>`
for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#result_large_err

## clippy::result_large_err at ✏️s/🔌️plugins/🔋️energy/🔨️modules/⚡️simulation/⚙️engine/🧪️sim/🦀️.rs:2049

```rust
    pub fn retry_commit_packet(&mut self, lease: EnergyWireLease) -> Result<(), EnergyWireLease> {
```
try reducing the size of `sim::EnergyWireLease`, for example by boxing large elements or replacing it with `Box<sim::EnergyWireLease>`
for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#result_large_err

## clippy::result_large_err at ✏️s/🔌️plugins/🔋️energy/🔨️modules/⚡️simulation/⚙️engine/🧪️sim/🦀️.rs:2053

```rust
    pub fn ack_commit_packet(&mut self, lease: EnergyWireLease) -> Result<(), EnergyWireLease> {
```
try reducing the size of `sim::EnergyWireLease`, for example by boxing large elements or replacing it with `Box<sim::EnergyWireLease>`
for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#result_large_err

## clippy::result_large_err at ✏️s/🔌️plugins/🔋️energy/🔨️modules/⚡️simulation/⚙️engine/🧪️sim/🦀️.rs:2070

```rust
    pub fn retry_fault_packet(&mut self, lease: EnergyWireLease) -> Result<(), EnergyWireLease> {
```
try reducing the size of `sim::EnergyWireLease`, for example by boxing large elements or replacing it with `Box<sim::EnergyWireLease>`
for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#result_large_err

## clippy::result_large_err at ✏️s/🔌️plugins/🔋️energy/🔨️modules/⚡️simulation/⚙️engine/🧪️sim/🦀️.rs:2074

```rust
    pub fn ack_fault_packet(&mut self, lease: EnergyWireLease) -> Result<(), EnergyWireLease> {
```
try reducing the size of `sim::EnergyWireLease`, for example by boxing large elements or replacing it with `Box<sim::EnergyWireLease>`
for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#result_large_err

## clippy::result_large_err at ✏️s/🔌️plugins/🔋️energy/🔨️modules/⚡️simulation/⚙️engine/🧪️sim/🦀️.rs:4611

```rust
    pub fn job(model: Model, config: SimulationConfig) -> Result<EnergyJob, EnergyAdmissionRejected> {
```
try reducing the size of `sim::EnergyAdmissionRejected`, for example by boxing large elements or replacing it with `Box<sim::EnergyAdmissionRejected>`
for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#result_large_err

## clippy::large_enum_variant at ✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧵️simulation-session/🦀️.rs:937

```rust
enum MountedAdmissionError {
    Stale { checkpoint: Option<EnergyWirePacket> },
    Rejected(&'static str),
}
```
for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#large_enum_variant

## clippy::result_large_err at ✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧵️simulation-session/🦀️.rs:989

```rust
    fn admit_job(&mut self, render: AppRenderOperationContext, live_request: u64, expected: MountedIdentity, checkpoint: Option<EnergyWirePacket>) -> Result<(), MountedAdmissionError> {
```
try reducing the size of `energy_simulation_session::MountedAdmissionError`, for example by boxing large elements or replacing it with `Box<energy_simulation_session::MountedAdmissionError>`
for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#result_large_err

## clippy::needless_pass_by_value at ✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧵️simulation-session/🦀️.rs:1069

```rust
    fn install_preview(&mut self, preview: EnergyJobPreview) -> bool {
```
or consider marking this type as `Copy`
for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#needless_pass_by_value
`-D clippy::needless-pass-by-value` implied by `-D warnings`
to override `-D warnings` add `#[allow(clippy::needless_pass_by_value)]`

## clippy::large_enum_variant at ✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️.rs:867

```rust
        enum $state {
            AwaitToken,
            Decode(store::OwnedSchemaHexAuthority<RASTER_OWNED_FIELD_BYTES>),
            Ready,
            Published,
            Closing,
            Complete,
        }
```
for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#large_enum_variant
`-D clippy::large-enum-variant` implied by `-D warnings`
to override `-D warnings` add `#[allow(clippy::large_enum_variant)]`

## clippy::large_enum_variant at ✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️.rs:867

```rust
        enum $state {
            AwaitToken,
            Decode(store::OwnedSchemaHexAuthority<RASTER_OWNED_FIELD_BYTES>),
            Ready,
            Published,
            Closing,
            Complete,
        }
```
for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#large_enum_variant

## clippy::unnecessary_wraps at ✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️.rs:1245

```rust
    fn next<'a, T>(&self, values: &'a RasterOwnedMap<T>) -> Result<Option<(&'a String, &'a T)>, &'static str> {
```
for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#unnecessary_wraps
`-D clippy::unnecessary-wraps` implied by `-D warnings`
to override `-D warnings` add `#[allow(clippy::unnecessary_wraps)]`

## clippy::large_enum_variant at ✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🧬️mutations/💾️binary/🦀️.rs:99

```rust
enum PresentationPackSnapshotState {
    AwaitToken,
    Decode(store::OwnedSchemaHexAuthority<PRESENTATION_ENVELOPE_SNAPSHOT_PACK_BYTES>),
    Ready,
    Published,
    Closing,
    Complete,
}
```
for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#large_enum_variant
`-D clippy::large-enum-variant` implied by `-D warnings`
to override `-D warnings` add `#[allow(clippy::large_enum_variant)]`

## clippy::result_large_err at ✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🧬️mutations/💾️binary/🦀️.rs:338

```rust
    fn try_adopt(&mut self, value: PresentationSnapshot) -> Result<(), PresentationSnapshot>;
```
try reducing the size of `standards::v1::subsets::any::schema::snapshot::component::PresentationSnapshot`, for example by boxing large elements or replacing it with `Box<standards::v1::subsets::any::schema::snapshot::component::PresentationSnapshot>`
for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#result_large_err
`-D clippy::result-large-err` implied by `-D warnings`
to override `-D warnings` add `#[allow(clippy::result_large_err)]`

## clippy::result_large_err at ✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🧬️mutations/💾️binary/🦀️.rs:452

```rust
    fn pump_field_return(&mut self) -> Result<bool, store::OwnedSchemaDecodeDiagnostic> {
```
try reducing the size of `dsl::OwnedSchemaDecodeDiagnostic`, for example by boxing large elements or replacing it with `Box<dsl::OwnedSchemaDecodeDiagnostic>`
for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#result_large_err

## clippy::result_large_err at ✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🧬️mutations/💾️binary/🦀️.rs:480

```rust
    fn begin_completed_close(&mut self) -> Result<(), store::OwnedSchemaDecodeDiagnostic> {
```
try reducing the size of `dsl::OwnedSchemaDecodeDiagnostic`, for example by boxing large elements or replacing it with `Box<dsl::OwnedSchemaDecodeDiagnostic>`
for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#result_large_err

## clippy::large_enum_variant at ✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧰️owned/🦀️.rs:453

```rust
        enum $state {
            AwaitToken,
            Decode(store::OwnedSchemaHexAuthority<DRAWING_OWNED_FIELD_BYTES>),
            Ready,
            Published,
            Closing,
            Complete,
        }
```
for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#large_enum_variant
`-D clippy::large-enum-variant` implied by `-D warnings`
to override `-D warnings` add `#[allow(clippy::large_enum_variant)]`

## clippy::large_enum_variant at ✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧰️owned/🦀️.rs:453

```rust
        enum $state {
            AwaitToken,
            Decode(store::OwnedSchemaHexAuthority<DRAWING_OWNED_FIELD_BYTES>),
            Ready,
            Published,
            Closing,
            Complete,
        }
```
for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#large_enum_variant

## clippy::unnecessary_wraps at ✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧰️owned/🦀️.rs:1676

```rust
    fn skeleton(source: &DrawingLayerNode) -> Result<DrawingLayerNode, &'static str> {
```
for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#unnecessary_wraps
`-D clippy::unnecessary-wraps` implied by `-D warnings`
to override `-D warnings` add `#[allow(clippy::unnecessary_wraps)]`

## clippy::unnecessary_wraps at ✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧰️owned/🦀️.rs:3397

```rust
    fn close_step(&mut self, _maximum_bytes: usize) -> Result<store::SnapshotRetirementStep, String> {
```
for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#unnecessary_wraps

## clippy::unnecessary_wraps at ✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧰️owned/🦀️.rs:3712

```rust
    fn close_step(&mut self, _maximum_bytes: usize) -> Result<store::SnapshotRetirementStep, String> {
```
for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#unnecessary_wraps

## clippy::needless_pass_by_value at ✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs:498

```rust
pub fn diff_from_snapshot(snapshot: DrawingSnapshot) -> DrawingDiff {
```
for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#needless_pass_by_value
`-D clippy::needless-pass-by-value` implied by `-D warnings`
to override `-D warnings` add `#[allow(clippy::needless_pass_by_value)]`

## clippy::unnecessary_wraps at ✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/✏️editor/🪆️1-any/🎮️commands/🖱️canvas-pointer-down/🦀️.rs:1016

```rust
fn queue_trace_pointer(payload: &CanvasPointerDown, job: &TracePointerJob) -> Option<Effect> {
```
for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#unnecessary_wraps

## clippy::needless_pass_by_value at ✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🗂️catalog/🦀️.rs:760

```rust
pub fn patch_register_item_operation(program: &ProgramSnapshot, register: &str, entity_id: EntityId, patch: Value) -> Option<ProgramMutation> {
```
for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#needless_pass_by_value
`-D clippy::needless-pass-by-value` implied by `-D warnings`
to override `-D warnings` add `#[allow(clippy::needless_pass_by_value)]`

## clippy::needless_pass_by_value at ✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🗂️catalog/🦀️.rs:760

```rust
pub fn patch_register_item_operation(program: &ProgramSnapshot, register: &str, entity_id: EntityId, patch: Value) -> Option<ProgramMutation> {
```
for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#needless_pass_by_value
