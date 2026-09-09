# Retained Reducer Signature Review

The framework's ArtifactCommandReducer function pointer has eight borrowed inputs. The following reported functions use that interface; their direct constructor references are reviewed before adding exact-site expectations.

✏️s/🔌️plugins/🧱️block/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs
fn block2d_retained_reduce(
    command: &Block2dCommand,
    snapshot: &Block2dSnapshot,
    config: &Block2dConfig,
    history: &semio_framework_plugin::HistoryView,
    _interaction: &protocol::InteractionState,
    _hover: &semio_framework_plugin::app::InteractionHoverState,
    _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<Block2dPlayApp>>>,
    operation: &AppOperationContext,
) -> Result<Emit<Block2dMutation, Block2dConfigMutation, NoDraftMutation>, Fault> {

✏️s/🔌️plugins/🌿️vcs/🗿️artifacts/🌿️vcs/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs
fn vcs_bounded_reduce(
    command: &VcsCommand,
    snapshot: &VcsSnapshot,
    config: &VcsDemoConfig,
    history: &semio_framework_plugin::HistoryView,
    _interaction: &protocol::InteractionState,
    _hover: &semio_framework_plugin::app::InteractionHoverState,
    _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<VcsPlayApp>>>,
    operation: &AppOperationContext,
) -> Result<Emit<VcsDemoMutation, VcsDemoConfigMutation, NoDraftMutation>, Fault> {

✏️s/🔌️plugins/🎪️demonstrator/🗿️artifacts/🎪️playground/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs
fn playground_retained_reduce(
    command: &PlaygroundCommand,
    snapshot: &PlaygroundSnapshot,
    config: &NoConfig,
    history: &semio_framework_plugin::HistoryView,
    _interaction: &protocol::InteractionState,
    _hover: &semio_framework_plugin::app::InteractionHoverState,
    _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<PlaygroundEditor>>>,
    operation: &AppOperationContext,
) -> Result<Emit<PlaygroundMutation, NoConfigMutation, NoDraftMutation>, Fault> {

✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs
fn sourcing_curation_retained_reduce(
    command: &SourcingCurationCommand,
    snapshot: &CurationSnapshot,
    config: &SourcingCurationConfig,
    history: &semio_framework_plugin::HistoryView,
    _interaction: &protocol::InteractionState,
    _hover: &semio_framework_plugin::app::InteractionHoverState,
    _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<SourcingCurationApp>>>,
    operation: &AppOperationContext,
) -> Result<Emit<SourcingMutation, SourcingCurationConfigMutation, NoDraftMutation>, Fault> {

✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs
fn process3d_retained_reduce(
    command: &Process3dCommand,
    snapshot: &Process3dSnapshot,
    config: &Process3dConfig,
    history: &semio_framework_plugin::HistoryView,
    interaction: &protocol::InteractionState,
    _hover: &semio_framework_plugin::app::InteractionHoverState,
    context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<Process3dPlayApp>>>,
    operation: &AppOperationContext,
) -> Result<Emit<Process3dMutation, Process3dConfigMutation, NoDraftMutation>, Fault> {

✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs
fn cad_retained_reduce(
    command: &CadCommand,
    snapshot: &CadSnapshot,
    config: &CadConfig,
    history: &semio_framework_plugin::HistoryView,
    interaction: &protocol::InteractionState,
    _hover: &semio_framework_plugin::app::InteractionHoverState,
    context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<CadPlayApp>>>,
    operation: &AppOperationContext,
) -> Result<Emit<CadMutation, CadConfigMutation, NoDraftMutation>, Fault> {

✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs
fn gis2d_retained_reduce(
    command: &Gis2dCommand,
    snapshot: &GisMapSnapshot,
    config: &Gis2dConfig,
    history: &semio_framework_plugin::HistoryView,
    _interaction: &protocol::InteractionState,
    _hover: &semio_framework_plugin::app::InteractionHoverState,
    _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<Gis2dPlayApp>>>,
    operation: &AppOperationContext,
) -> Result<Emit<GisMapMutation, Gis2dConfigMutation, NoDraftMutation>, Fault> {

✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🦀️.rs
fn fem2d_retained_reduce(
    command: &Fem2dCommand,
    snapshot: &Fem2dSnapshot,
    config: &Fem2dConfig,
    history: &semio_framework_plugin::HistoryView,
    _interaction: &protocol::InteractionState,
    _hover: &semio_framework_plugin::app::InteractionHoverState,
    _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<Fem2dPlayApp>>>,
    operation: &AppOperationContext,
) -> Result<Emit<Fem2dMutation, Fem2dConfigMutation, NoDraftMutation>, Fault> {

✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs
fn dag_retained_config_reduce(
    command: &DagCommand,
    snapshot: &DagSnapshot,
    config: &DagConfig,
    history: &semio_framework_plugin::HistoryView,
    _interaction: &protocol::InteractionState,
    _hover: &semio_framework_plugin::app::InteractionHoverState,
    _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<DagPlayApp>>>,
    operation: &AppOperationContext,
) -> Result<Emit<DagMutation, DagConfigMutation, NoDraftMutation>, Fault> {

✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs
fn layout_retained_reduce(
    command: &LayoutCommand,
    snapshot: &LayoutSnapshot,
    config: &LayoutConfig,
    history: &semio_framework_plugin::HistoryView,
    _interaction: &protocol::InteractionState,
    _hover: &semio_framework_plugin::app::InteractionHoverState,
    _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<LayoutPlayApp>>>,
    operation: &AppOperationContext,
) -> Result<Emit<LayoutMutation, LayoutConfigMutation, NoDraftMutation>, Fault> {

✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs
fn remodeling_retained_reduce(
    command: &RemodelingCommand,
    snapshot: &RemodelingSnapshot,
    config: &RemodelingConfig,
    history: &semio_framework_plugin::HistoryView,
    _interaction: &protocol::InteractionState,
    _hover: &semio_framework_plugin::app::InteractionHoverState,
    _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<RemodelingPlayApp>>>,
    operation: &AppOperationContext,
) -> Result<Emit<RemodelingMutation, RemodelingConfigMutation, NoDraftMutation>, Fault> {

✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs
fn shooting_bounded_reduce(
    command: &ShootingCommand,
    snapshot: &ShootingSnapshot,
    config: &ShootingConfig,
    history: &semio_framework_plugin::HistoryView,
    _interaction: &protocol::InteractionState,
    _hover: &semio_framework_plugin::app::InteractionHoverState,
    _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<ShootingPlayApp>>>,
    operation: &AppOperationContext,
) -> Result<Emit<ShootingMutation, ShootingConfigMutation, NoDraftMutation>, Fault> {

✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs
fn raster_retained_reduce(
    command: &RasterCommand,
    snapshot: &RasterSnapshot,
    config: &RasterConfig,
    history: &semio_framework_plugin::HistoryView,
    _interaction: &protocol::InteractionState,
    _hover: &semio_framework_plugin::app::InteractionHoverState,
    _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<RasterPlayApp>>>,
    operation: &AppOperationContext,
) -> Result<Emit<RasterMutation, RasterConfigMutation, NoDraftMutation>, Fault> {

✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs
fn animate_presentation_retained_reduce(
    command: &PresentationCommand,
    snapshot: &PresentationSnapshot,
    config: &PresentationConfig,
    history: &semio_framework_plugin::HistoryView,
    _interaction: &protocol::InteractionState,
    _hover: &semio_framework_plugin::app::InteractionHoverState,
    _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<AnimatePresentationPlayApp>>>,
    operation: &AppOperationContext,
) -> Result<Emit<PresentationMutation, PresentationConfigMutation, NoDraftMutation>, Fault> {

✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs
fn drawing_bounded_reduce(
    command: &DrawingCommand,
    snapshot: &DrawingSnapshot,
    config: &DrawingConfig,
    history: &semio_framework_plugin::HistoryView,
    interaction: &::protocol::InteractionState,
    _hover: &semio_framework_plugin::app::InteractionHoverState,
    context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<semio_framework_plugin::EditorApp<DrawingPlayApp>>>,
    operation: &semio_framework_plugin::AppOperationContext,
) -> Result<Emit<DrawingMutation, DrawingConfigMutation, NoDraftMutation>, Fault> {


All fifteen candidates were verified as direct reducer function pointers passed to BoundedArtifactCommandWork::new. Added only the exact-site too_many_arguments expectation required by that shared eight-input callback interface. Function signatures and bodies are unchanged. Other oversized function argument lists remain to be refactored. All fifteen Rust files parsed before guarded writes; strict compiler fulfillment remains pending.
