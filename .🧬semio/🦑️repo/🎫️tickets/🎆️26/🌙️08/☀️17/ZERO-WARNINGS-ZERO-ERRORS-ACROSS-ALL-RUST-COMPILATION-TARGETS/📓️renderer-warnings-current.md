# Renderer Warning Inventory

Native287 checked all native renderer targets and native-bin successfully, with 235 dead-code diagnostics. Browser288 checked its three browser packages successfully, with 141 dead-code diagnostics. These are compiler diagnostics, not a claim that every runtime path is exercised. The following distinct native diagnostics remain under review; no source was removed by this inventory.

- method `close_step` is never used
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs:11156 — fn close_step(&mut self) -> bool {
- method `stage` is never used
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs:13921 — pub(crate) fn stage(&self) -> FrameTransactionStage {
- field `corner` is never read
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🛰️Dock/🎯️targets/🧊️wgpu/🦀️.rs:1415 — corner: WindowStackCorner,
- variants `Dag` and `Flow` are never constructed
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs:33 — Dag(GraphHost),
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs:34 — Flow(FlowHost),
- fields `width` and `height` are never read
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs:69 — width: u32,
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs:70 — height: u32,
- associated function `try_from_str` is never used
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs:87 — fn try_from_str(id: &str) -> Result<Self, ()> {
- struct `EngineSurfaceSnapshot` is never constructed
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs:132 — struct EngineSurfaceSnapshot {
- methods `contains_key`, `token`, `identity`, `get_token_mut`, `reserve`, and `publish_reserved` are never used
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs:169 — fn contains_key(&self, id: &str) -> bool {
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs:182 — fn token(&self, id: &str) -> Option<EngineSurfaceToken> {
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs:187 — fn identity(&self, id: &str) -> Option<EngineSurfaceIdentity> {
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs:192 — fn get_token_mut(&mut self, token: EngineSurfaceToken) -> Option<&mut EngineSurface> {
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs:198 — fn reserve(&mut self, id: &str) -> Option<EngineSurfaceToken> {
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs:222 — fn publish_reserved(&mut self, token: EngineSurfaceToken, value: EngineSurface) -> Result<(), EngineSurface> {
- enum `EngineCanvasPacketDestination` is never used
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs:738 — enum EngineCanvasPacketDestination {
- struct `EngineCanvasPacketReservation` is never constructed
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs:744 — struct EngineCanvasPacketReservation {
- fields `dpr`, `document_generation`, and `scene_revision` are never read
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs:753 — dpr: f64,
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs:754 — document_generation: u64,
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs:755 — scene_revision: u64,
- methods `dpr`, `try_reserve_packet`, `try_reserve_fresh_packet`, and `publish_reserved` are never used
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs:787 — pub(crate) fn dpr(&self) -> f64 {
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs:807 — fn try_reserve_packet(&mut self, surface: EngineSurfaceSnapshot) -> Result<EngineCanvasPacketReservation, EngineSurfaceSnapshot> {
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs:814 — fn try_reserve_fresh_packet(&mut self, surface: EngineSurfaceSnapshot) -> Result<EngineCanvasPacketReservation, EngineSurfaceSnapshot> {
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs:845 — fn publish_reserved(&mut self, reservation: EngineCanvasPacketReservation, scene: canvas::Scene, clear: Color, width: u32, height: u32) {
- fields `width` and `height` are never read
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs:878 — width: u32,
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs:879 — height: u32,
- function `scene_action` is never used
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs:1742 — fn scene_action(scene: &UiComponentSceneNode, action: &str, args: Value) -> ActionDescriptor {
- function `observe_engine_surface_packet_freshness` is never used
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs:1808 — fn observe_engine_surface_packet_freshness(surface: EngineSurfaceSnapshot, document_generation: u64, scene_revision: u64) -> bool {
- function `engine_now_ms` is never used
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs:1894 — fn engine_now_ms() -> f64 {
- function `map_tile_url` is never used
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs:2659 — fn map_tile_url(template: &str, z: u32, x: u32, y: u32) -> String {
- variants `Raster`, `Vector`, and `Terminal` are never constructed
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs:2672 — Raster,
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs:2673 — Vector,
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs:2674 — Terminal,
- fields `revision`, `raster_template`, `vector_template`, `raster`, `vector`, and `phase` are never read
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs:2679 — revision: u64,
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs:2680 — raster_template: u64,
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs:2681 — vector_template: u64,
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs:2682 — raster: Option<VisibleTileCursor>,
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs:2683 — vector: Option<VisibleTileCursor>,
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs:2684 — phase: MapTileRequestPhase,
- function `bounded_map_template_witness` is never used
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs:2687 — fn bounded_map_template_witness(template: &str) -> Result<u64, WorldAssetFault> {
- associated items `new`, `matches`, `current`, and `advance` are never used
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs:2695 — fn new(scene: &ui_wgpu::wgpu::TiledMapScene, host: &MapHost) -> Result<Self, WorldAssetFault> {
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs:2710 — fn matches(&self, scene: &ui_wgpu::wgpu::TiledMapScene, host: &MapHost) -> bool {
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs:2714 — fn current(&self) -> Option<(bool, framework_surface_tiled_map::tiled_map::tiles::VisibleTile)> {
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs:2722 — fn advance(&mut self) {
- static `POINTER_EDGE_STATE` is never used
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs:376 — static POINTER_EDGE_STATE: WorkerCell<std::collections::HashMap<String, (bool, i16)>> = WorkerCell::new();
- function `merge_action_args` is never used
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs:537 — fn merge_action_args(existing: Option<&semio_framework::DslValue>, patch: serde_json::Map<String, Value>) -> Option<semio_framework::DslValue> {
- multiple fields are never read
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs:1114 — engine_resources: &'ctx mut crate::engine_canvas::EngineCanvasBuildContext,
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs:1115 — world_resources: &'ctx mut infinite_world::world::World3dBuildContext,
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs:1121 — world3d_states: &'ctx mut AdmittedSurfaceMap<infinite_world::world::World3dState>,
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs:1122 — node_graph_states: &'ctx mut AdmittedSurfaceMap<NodeGraphSurface>,
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs:1123 — tiled_map_states: &'ctx mut AdmittedSurfaceMap<TiledMapSurface>,
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs:1124 — icon_render_states: &'ctx mut std::collections::HashMap<String, infinite_world::world::World3dState>,
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs:1125 — board2d_states: &'ctx mut AdmittedSurfaceMap<Board2dSurface>,
- associated items `from_json`, `from_typed`, and `world_to_screen` are never used
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:361 — fn from_json(raw: &str) -> Self {
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:368 — fn from_typed(viewport: Option<&ui_wgpu::wgpu::NodeGraphViewport>) -> Self {
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:378 — fn world_to_screen(&self, wx: f32, wy: f32, origin: Rect) -> (f32, f32) {
- variant `InkResize` is never constructed
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:392 — InkResize { handle: String, from: InkBoundsF, start_x: f32, start_y: f32, selected_ids: Vec<String> },
- fields `last_click_ms`, `last_click_target`, `node_positions`, `canvas_image_digests`, and `canvas_image_src_digests` are never read
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:409 — last_click_ms: f64,
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:410 — last_click_target: Option<String>,
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:411 — node_positions: HashMap<String, (f32, f32)>,
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:413 — canvas_image_digests: HashMap<String, u64>,
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:414 — canvas_image_src_digests: HashMap<String, u64>,
- function `scene_action` is never used
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:1041 — fn scene_action(scene: &UiComponentSceneNode, action: &str, args: Value) -> ActionDescriptor {
- function `now_ms` is never used
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:1223 — fn now_ms() -> f64 {
- fields `x`, `y`, `scale_x`, and `scale_y` are never read
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:1912 — x: f64,
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:1914 — y: f64,
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:1916 — scale_x: f64,
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:1918 — scale_y: f64,
- multiple fields are never read
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:1944 — id: String,
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:1946 — visible: bool,
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:1948 — opacity: f32,
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:1950 — transform: Paint2dTransformFields,
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:1951 — width: Option<u32>,
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:1952 — height: Option<u32>,
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:1953 — image_key: Option<String>,
- fields `visible`, `opacity`, `transform`, and `children` are never read
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:1958 — visible: bool,
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:1960 — opacity: f32,
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:1962 — transform: Paint2dTransformFields,
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:1964 — children: Vec<Paint2dLayerJson>,
- field `layers` is never read
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:1976 — layers: Vec<Paint2dLayerJson>,
- struct `Paint2dAssetJson` is never constructed
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:1980 — struct Paint2dAssetJson {
- struct `Paint2dFlatLayer` is never constructed
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:1985 — struct Paint2dFlatLayer {
- function `collect_paint2d_pixel_layers` is never used
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:1997 — fn collect_paint2d_pixel_layers(layers: &[Paint2dLayerJson], parent_x: f64, parent_y: f64, parent_sx: f64, parent_sy: f64, parent_opacity: f32, out: &mut Vec<Paint2dFlatLayer>) {
- constant `PAINT2D_NAVIGATOR_PADDING` is never used
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:2028 — const PAINT2D_NAVIGATOR_PADDING: f32 = 24.0;
- struct `TableColumn` is never constructed
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:2096 — struct TableColumn {
- struct `TableSortJson` is never constructed
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:2107 — struct TableSortJson {
- enum `TableCellPayload` is never used
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:2115 — enum TableCellPayload {
- struct `TableCellButtonPayload` is never constructed
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:2124 — struct TableCellButtonPayload {
- function `merge_action_args` is never used
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:2132 — fn merge_action_args(base: &ActionDescriptor, patch: Value) -> ActionDescriptor {
- struct `BlockListBlockJson` is never constructed
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:2395 — struct BlockListBlockJson {
- struct `BlockListStepJson` is never constructed
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:2403 — struct BlockListStepJson {
- struct `BlockListPaletteEntryJson` is never constructed
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:2415 — struct BlockListPaletteEntryJson {
- enum `DiffLineOperation` is never used
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:2796 — enum DiffLineOperation {
- struct `DiffLine` is never constructed
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:2803 — struct DiffLine<'a> {
- constant `DIFF_LCS_CELL_BUDGET` is never used
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:2812 — const DIFF_LCS_CELL_BUDGET: usize = 200_000;
- struct `EventFeedEntryJson` is never constructed
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:3078 — struct EventFeedEntryJson {
- struct `HistoryColumnAuthorJson` is never constructed
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:3331 — struct HistoryColumnAuthorJson {
- struct `HistoryColumnJson` is never constructed
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:3338 — struct HistoryColumnJson {
- constant `HISTORY_LANE_PITCH` is never used
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:3352 — const HISTORY_LANE_PITCH: f32 = 16.0;
- constant `HISTORY_LANE_PAD` is never used
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:3353 — const HISTORY_LANE_PAD: f32 = 8.0;
- constant `HISTORY_AUTHOR_SLOT` is never used
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:3354 — const HISTORY_AUTHOR_SLOT: f32 = 40.0;
- struct `CanvasFillJson` is never constructed
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:3662 — struct CanvasFillJson {
- struct `CanvasGradientStopJson` is never constructed
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:3688 — struct CanvasGradientStopJson {
- struct `CanvasStrokeJson` is never constructed
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:3698 — struct CanvasStrokeJson {
- struct `CanvasImageFieldJson` is never constructed
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:3711 — struct CanvasImageFieldJson {
- struct `CanvasTextFieldJson` is never constructed
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:3723 — struct CanvasTextFieldJson {
- struct `CanvasLayer` is never constructed
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:3731 — struct CanvasLayer {
- struct `Canvas2dPacketText` is never constructed
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:3784 — struct Canvas2dPacketText<'a> {
- struct `Canvas2dPacketItem` is never constructed
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:3792 — struct Canvas2dPacketItem<'a> {
- function `canvas2d_packet_text_size` is never used
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:3817 — fn canvas2d_packet_text_size() -> f64 {
- function `decode_canvas_image_source` is never used
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:3826 — fn decode_canvas_image_source(data_url: &str) -> Option<Vec<u8>> {
- function `decode_canvas_image_bytes` is never used
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:3831 — fn decode_canvas_image_bytes(bytes: &[u8]) -> Option<(Vec<u8>, u32, u32)> {
- function `queue_canvas_image_upload_sized` is never used
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:3952 — pub(crate) fn queue_canvas_image_upload_sized(surface_id: &str, layer_id: &str, data_url: &str) -> (Option<String>, Option<(u32, u32)>) {
- constant `CANVAS_GRADIENT_BANDS` is never used
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:4252 — const CANVAS_GRADIENT_BANDS: usize = 10;
- constant `CANVAS_CIRCLE_SEGMENTS` is never used
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:4253 — const CANVAS_CIRCLE_SEGMENTS: usize = 28;
- constant `CANVAS2D_SELECTION_RING` is never used
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:4394 — const CANVAS2D_SELECTION_RING: Rgba = Rgba::new(0.984_314, 0.749_02, 0.141_176, 0.95);
- constant `CANVAS2D_SELECTION_GLOW` is never used
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:4395 — const CANVAS2D_SELECTION_GLOW: Rgba = Rgba::new(0.984_314, 0.749_02, 0.141_176, 0.28);
- fields `grid_visible`, `grid_spacing`, `grid_subdivisions`, `grid_opacity`, and `pencil_width` are never read
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:5184 — grid_visible: Option<bool>,
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:5185 — grid_spacing: Option<f64>,
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:5186 — grid_subdivisions: Option<f64>,
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:5187 — grid_opacity: Option<f64>,
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:5190 — pencil_width: Option<f64>,
- constant `INK_RESIZE_HANDLES` is never used
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:6391 — const INK_RESIZE_HANDLES: [&str; 8] = ["nw", "n", "ne", "e", "se", "s", "sw", "w"];
- struct `IconRenderCameraFields` is never constructed
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:7691 — struct IconRenderCameraFields {
- function `icon_render_default_zoom` is never used
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:7702 — fn icon_render_default_zoom() -> f64 {
- struct `IconRenderLightsFields` is never constructed
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:7708 — struct IconRenderLightsFields {
- struct `IconRenderMaterialFields` is never constructed
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:7725 — struct IconRenderMaterialFields {
- struct `IconRenderRequestFields` is never constructed
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:7740 — struct IconRenderRequestFields {
- struct `VfsDescriptorKind` is never constructed
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:7995 — struct VfsDescriptorKind {
- struct `VfsFileNodeKind` is never constructed
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:8001 — struct VfsFileNodeKind {
- struct `VfsDescriptorColumn` is never constructed
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:8009 — struct VfsDescriptorColumn {
- struct `VfsSchema` is never constructed
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:8018 — struct VfsSchema {
- struct `VfsVisibleRow` is never constructed
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:8028 — struct VfsVisibleRow {
- struct `TextEditorUiState` is never constructed
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:8153 — struct TextEditorUiState {
- struct `TextEditorContextMenu` is never constructed
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:8168 — struct TextEditorContextMenu {
- struct `TextEditorMenuItem` is never constructed
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:8175 — struct TextEditorMenuItem {
- struct `TextEditorCompletionItem` is never constructed
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:8184 — struct TextEditorCompletionItem {
- struct `TextEditorSpan` is never constructed
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:8193 — struct TextEditorSpan {
- struct `TextEditorRenameInfo` is never constructed
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:8200 — struct TextEditorRenameInfo {
- static `TEXT_EDITOR_UI_STATE` is never used
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:8211 — static TEXT_EDITOR_UI_STATE: WorkerCell<HashMap<String, TextEditorUiState>> = WorkerCell::new();
- method `close_step` is never used
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:224 — fn close_step(&mut self) -> bool {
- fields `artifact_ref`, `dialect`, `role`, `plugin_id`, and `app_id` are never read
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:421 — artifact_ref: String,
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:422 — dialect: semio_framework::ArtifactDialect,
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:423 — role: semio_framework::AppRole,
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:424 — plugin_id: Option<String>,
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:425 — app_id: Option<String>,
- field `tour_auto_considered` is never read
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:1213 — tour_auto_considered: Option<String>,
- fields `epoch`, `generation`, and `surface` are never read
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:1997 — epoch: u64,
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:1998 — generation: u64,
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:1999 — surface: Option<SurfaceId>,
- methods `submit_shell_io`, `active_plugin_examples`, and `dock_tab_bars_for_drop` are never used
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:3030 — fn submit_shell_io(&mut self, operation: impl FnOnce() -> ShellIoCompletion + Send + 'static) {
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:3467 — fn active_plugin_examples(&self) -> Vec<ExampleDefinition> {
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:3511 — fn dock_tab_bars_for_drop(&self, atlas: &mut FontAtlas, theme: &Theme, canvas: Rect, labels: &HashMap<String, String>, icon_ids: &HashMap<String, String>) -> Vec<(Vec<usize>, WindowStackCorner, Rect, Vec<f32>)> {
- function `patch_ops_from_action_result` is never used
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:4146 — fn patch_ops_from_action_result(result: &semio_framework::kernel::InvocationResult) -> Vec<String> {
- associated items `sync_status_label` and `publish_presence_heartbeat` are never used
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:4306 — fn sync_status_label(status: &ArtifactSyncStatus) -> String {
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:4353 — fn publish_presence_heartbeat(&mut self) {
- function `chrome_text` is never used
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:7910 — fn chrome_text(target: &mut DrawList, atlas: &mut FontAtlas, input: &mut InputState<ActionDescriptor>, theme: &Theme, text: &str, x: f32, y: f32, size: f32, color: Rgba) {
- struct `WindowMeasuresRailOutcome` is never constructed
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:8187 — struct WindowMeasuresRailOutcome {
- constant `WINDOW_MEASURE_TRAVERSAL_CAPACITY` is never used
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:8192 — const WINDOW_MEASURE_TRAVERSAL_CAPACITY: usize = 64;
- variants `Select`, `Slider`, and `Toggle` are never constructed
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:8197 — Select,
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:8198 — Slider,
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:8199 — Toggle(bool),
- struct `WindowMeasureRenderFrame` is never constructed
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:8213 — struct WindowMeasureRenderFrame<'a> {
- method `try_upsert` is never used
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:8223 — fn try_upsert(&mut self, control: &str, descriptor: &ActionDescriptor, kind: WindowMeasureActionKind) -> Result<(), ()> {
- function `window_overlay_max_width` is never used
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:8240 — fn window_overlay_max_width(content_w: f32, inset: f32) -> f32 {
- function `measure_window_measure_height` is never used
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:8262 — fn measure_window_measure_height(theme: &Theme, collapsed_sections: &HashMap<String, bool>, measure: &WindowMeasure) -> Option<f32> {
- function `measure_window_measures_body_height` is never used
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:8283 — fn measure_window_measures_body_height<'a>(theme: &Theme, collapsed_sections: &HashMap<String, bool>, measures: impl Iterator<Item = &'a WindowMeasure>) -> Option<f32> {
- methods `register_tooltip`, `open_dialog`, and `start_introduction` are never used
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:10691 — fn register_tooltip(&mut self, control_id: impl Into<String>, title: impl Into<String>) {
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:10707 — fn open_dialog(&mut self, request: ChromeDialogRequest) {
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:10715 — fn start_introduction(&mut self) {
- field `fallback` is never read
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:10841 — fallback: bool,
- methods `register_element_rect_fallback` and `element_rect_is_fallback` are never used
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:10849 — fn register_element_rect_fallback(&mut self, id: impl Into<String>, rect: Rect) {
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:10858 — fn element_rect_is_fallback(&self, id: &str) -> bool {
- constant `INTRODUCED_PULSE_PERIOD_MS` is never used
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:10911 — const INTRODUCED_PULSE_PERIOD_MS: f64 = 1600.0;
- constant `INTRODUCTION_INFO_BOX_GAP` is never used
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:10922 — const INTRODUCTION_INFO_BOX_GAP: f32 = 16.0;
- function `engagement_completion_suffix` is never used
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:10970 — fn engagement_completion_suffix(query: &str, possibles: Option<&[ui_wgpu::wgpu::WindowEngagementPossible]>) -> String {
- function `engagement_ghost_accept_on_click` is never used
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:10997 — fn engagement_ghost_accept_on_click(ghost_rect: Rect, pointer_x: f32, pointer_y: f32, clicked_this_frame: bool, query: &str, suffix: &str) -> Option<String> {
- field `y` is never read
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:12121 — y: f32,
- multiple associated items are never used
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:12621 — fn left_tabs(&self, session: &ActiveSession) -> Vec<PanelTabDefinition> {
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:12657 — fn right_tabs(&self, session: &ActiveSession) -> Vec<PanelTabDefinition> {
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:12695 — fn active_left_tab_id(&self, session: &ActiveSession) -> String {
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:12716 — fn active_right_tab_id(&self, session: &ActiveSession) -> String {
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:14155 — fn intersect_content_rect(left: Rect, right: Rect) -> Option<Rect> {
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:14445 — fn resolve_introduction_element_rects(&self, id: &str, theme: &Theme, width: f32, height: f32, hit_targets: &[HitTarget<ActionDescriptor>]) -> Vec<Rect> {
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:14489 — fn resolve_introduction_window_silhouettes(&self, id: &str) -> Vec<WindowSilhouette> {
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:14507 — fn resolve_introduction_element_rect(&self, id: &str, theme: &Theme, width: f32, height: f32, hit_targets: &[HitTarget<ActionDescriptor>]) -> Option<Rect> {
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:14841 — fn render_example_dropdown(&self, overlay: &mut DrawList, atlas: &mut FontAtlas, input: &mut InputState<ActionDescriptor>, theme: &Theme, x: f32, y: f32, w: f32, items: &[(String, String, usize)], examples: &[ExampleDefinition]) {
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:14864 — fn render_action_list(
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:14918 — fn measures_for_kind(kind: &semio_framework::WindowKindDefinition) -> &[WindowMeasure] {
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:14922 — fn engagement_for_kind(&self, kind: &semio_framework::WindowKindDefinition) -> Option<WindowEngagement> {
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:15014 — fn render_utility_options_rail(
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:15057 — fn render_window_measure_tree(
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:15091 — fn render_window_measure_one(&mut self, draw: &mut DrawList, overlay: &mut Option<&mut DrawList>, atlas: &mut FontAtlas, icons: &IconAtlas, input: &mut InputState<ActionDescriptor>, theme: &Theme, bounds: Rect, measure: &WindowMeasure) {
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:15236 — fn render_engagement_input(
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:15298 — fn render_engagement_control(
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:15436 — fn staged_form_height(&self, theme: &Theme, action: &semio_framework::ActionDefinition) -> f32 {
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:15444 — fn staged_arg_height(&self, theme: &Theme, arg: &semio_framework::ActionArgDef) -> f32 {
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:15578 — fn staged_arg_display_string(&self, window_id: &str, action_id: &str, arg: &semio_framework::ActionArgDef, input: &InputState<ActionDescriptor>, control_id: Option<&str>) -> String {
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:15594 — fn render_staged_text_field(
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:15618 — fn paint_staged_input_box(&self, draw: &mut DrawList, atlas: &mut FontAtlas, input: &mut InputState<ActionDescriptor>, theme: &Theme, rect: Rect, display: &str, focused: bool, enabled: bool, control_id: &str) {
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:15639 — fn context_menu_level_width(items: &[ContextMenuItem], theme: &Theme) -> f32 {
- trait `PrefsStore` is never used
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:15785 — trait PrefsStore {
- struct `ShellPrefLocks` is never constructed
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:16073 — struct ShellPrefLocks {
- function `shell_pref_locks` is never used
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:16084 — fn shell_pref_locks() -> ShellPrefLocks {
- function `shell_panel_tab_label` is never used
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:16466 — fn shell_panel_tab_label(id: &str, fallback: &'static str, is_de: bool) -> String {
- associated function `capture` is never used
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:16486 — fn capture(state: &ShellState) -> Self {
- function `persist_custom_themes` is never used
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:16499 — fn persist_custom_themes(preferences: &ChromePrefsState) {
- methods `load_ui_prefs_once` and `persist_ui_prefs_if_changed` are never used
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:16510 — fn load_ui_prefs_once(&mut self) {
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:16531 — fn persist_ui_prefs_if_changed(&mut self) {
- methods `is_visible` and `fire` are never used
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/⏰️deadlines/🦀️.rs:60 — pub fn is_visible(&self) -> bool {
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/⏰️deadlines/🦀️.rs:86 — pub fn fire(&mut self, scheduler: &mut FrameScheduler, now_seconds: f64) {
- field `0` is never read
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🪢️kernel-seam/🦀️.rs:50 — pub struct HostWaker(HostWake);
- method `wake` is never used
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🪢️kernel-seam/🦀️.rs:66 — pub fn wake(&self) {
- fields `surface` and `detail` are never read
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🪢️kernel-seam/🦀️.rs:80 — pub surface: String,
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🪢️kernel-seam/🦀️.rs:81 — pub detail: Box<dyn std::any::Any + Send>,
- methods `submit_intents` and `drain_outcomes` are never used
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🪢️kernel-seam/🦀️.rs:97 — fn submit_intents(&self, intents: Vec<UiIntent>) -> Vec<UiIntent>;
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🪢️kernel-seam/🦀️.rs:100 — fn drain_outcomes(&self) -> Vec<KernelOutcome>;
- fields `outcomes` and `exchange` are never read
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🪢️kernel-seam/🦀️.rs:140 — outcomes: Arc<Mutex<OutcomeMailbox>>,
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🪢️kernel-seam/🦀️.rs:142 — exchange: IntentExchange,
- method `pending_len` is never used
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🪢️kernel-seam/🦀️.rs:153 — pub fn pending_len(&self) -> usize {
- fields `ready` and `in_flight` are never read
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🪢️kernel-seam/🦀️.rs:159 — ready: VecDeque<KernelOutcome>,
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🪢️kernel-seam/🦀️.rs:160 — in_flight: usize,
- methods `reserve` and `finish` are never used
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🪢️kernel-seam/🦀️.rs:168 — fn reserve(&mut self) -> bool {
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🪢️kernel-seam/🦀️.rs:176 — fn finish(&mut self, outcome: KernelOutcome) {
- fields `revision`, `generation`, `timestamp_us`, `dispatch_tree`, and `damage_regions` are never read
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📸️render-snapshot/🦀️.rs:35 — pub revision: u64,
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📸️render-snapshot/🦀️.rs:38 — pub generation: semio_framework_trace::Generation,
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📸️render-snapshot/🦀️.rs:39 — pub timestamp_us: u64,
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📸️render-snapshot/🦀️.rs:48 — pub dispatch_tree: Option<Arc<()>>,
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📸️render-snapshot/🦀️.rs:51 — pub damage_regions: Option<Vec<()>>,
- field `identity` is never read
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs:3391 — identity: JobProgressIdentity,
- methods `close_step` and `terminal_is_empty` are never used
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs:3579 — fn close_step(&mut self, maximum_bytes: usize) -> (bool, usize, usize) {
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs:3604 — fn terminal_is_empty(&self) -> bool {
- method `close_step` is never used
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs:3636 — fn close_step(&mut self, maximum_bytes: usize) -> (bool, usize, usize) {
- methods `instance` and `generation` are never used
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs:3928 — pub(crate) fn instance(&self) -> u32 {
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs:3932 — pub(crate) fn generation(&self) -> u64 {
- field `job` is never read
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs:4087 — job: u64,
- method `job` is never used
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs:4128 — fn job(&self) -> u64 {
- fields `lane` and `len` are never read
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs:4885 — pub lane: u8,
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs:4887 — len: usize,
- method `bytes` is never used
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs:4903 — pub(crate) fn bytes(&self) -> &[u8] {
- methods `take_page` and `acknowledge` are never used
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs:4945 — fn take_page(&self, receiver: u32) -> Option<TypedOperationResultPage>;
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs:4946 — fn acknowledge(&self, token: TypedOperationResultToken) -> bool;
- field `acknowledge` is never read
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs:4951 — acknowledge: Arc<dyn Fn(TypedOperationResultToken) -> bool + Send + Sync>,
- methods `take_typed_operation_result` and `acknowledge_typed_operation_result` are never used
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs:5111 — pub(crate) fn take_typed_operation_result(&self, receiver: u32) -> Option<TypedOperationResultPage> {
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs:5115 — pub(crate) fn acknowledge_typed_operation_result(&self, token: TypedOperationResultToken) -> bool {
- methods `take_typed_operation_result_page` and `acknowledge_typed_operation_result` are never used
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs:5246 — pub(crate) fn take_typed_operation_result_page(&self, receiver: u32) -> Option<TypedOperationResultPage> {
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs:5250 — pub(crate) fn acknowledge_typed_operation_result(&self, token: TypedOperationResultToken) -> bool {
- field `generation` is never read
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs:5409 — generation: u64,
- methods `get`, `get_mut`, and `terminal_is_empty` are never used
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs:5744 — fn get(&self, instance: u32, surface: &SurfaceId) -> Option<&RetainedSurfaceSlot> {
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs:5748 — fn get_mut(&mut self, instance: u32, surface: &SurfaceId) -> Option<&mut RetainedSurfaceSlot> {
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs:5791 — fn terminal_is_empty(&self) -> bool {
- fields `epoch`, `generation`, and `surface` are never read
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs:6142 — Closing { epoch: u64, batch: u64, generation: u64, surface: SurfaceId, document: UiDocumentLease },
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs:6142 — Closing { epoch: u64, batch: u64, generation: u64, surface: SurfaceId, document: UiDocumentLease },
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs:6142 — Closing { epoch: u64, batch: u64, generation: u64, surface: SurfaceId, document: UiDocumentLease },
- method `batch_is_empty` is never used
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs:6251 — fn batch_is_empty(&self, batch: u64, generation: u64) -> bool {
- methods `begin_shutdown` and `shutdown_step` are never used
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs:7920 — fn begin_shutdown(&self) -> bool {
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs:7928 — fn shutdown_step(&self, maximum_bytes: usize) -> (bool, usize, usize) {
- method `get_token_mut` is never used
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs:192 — fn get_token_mut(&mut self, token: EngineSurfaceToken) -> Option<&mut EngineSurface> {
- field `dpr` is never read
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs:753 — dpr: f64,
- methods `dpr` and `try_reserve_packet` are never used
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs:787 — pub(crate) fn dpr(&self) -> f64 {
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs:807 — fn try_reserve_packet(&mut self, surface: EngineSurfaceSnapshot) -> Result<EngineCanvasPacketReservation, EngineSurfaceSnapshot> {
- function `theme_is_dark` is never used
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs:1514 — pub(crate) fn theme_is_dark(theme: &Theme) -> bool {
- function `linear_to_rgba8_channel` is never used
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs:1521 — fn linear_to_rgba8_channel(linear: f32) -> u8 {
- function `dispatch_pointer_events` is never used
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs:1011 — fn dispatch_pointer_events(engine: &mut ui_wgpu::wgpu::Ui, window_id: &str, bounds: Rect, input: &ui_wgpu::wgpu::InputState<ActionDescriptor>) -> Vec<ui_wgpu::wgpu::UiCommand> {
- function `shift_instance` is never used
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs:1036 — fn shift_instance(instance: &ui_wgpu::wgpu::draw::UiInstance, dx: f32, dy: f32) -> ui_wgpu::wgpu::draw::UiInstance {
- function `shift_vertex` is never used
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs:1044 — fn shift_vertex(vertex: &ui_wgpu::wgpu::draw::VectorVertex, dx: f32, dy: f32) -> ui_wgpu::wgpu::draw::VectorVertex {
- function `shift_scissor` is never used
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs:1052 — fn shift_scissor(scissor: ui_wgpu::wgpu::draw::ScissorRect, dx: f32, dy: f32) -> ui_wgpu::wgpu::draw::ScissorRect {
- function `composite_retained_draw_list` is never used
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs:1070 — fn composite_retained_draw_list(target: &mut ui_wgpu::wgpu::DrawList, retained: &ui_wgpu::wgpu::DrawList, offset_x: f32, offset_y: f32) {
- function `render_ui_document` is never used
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs:1307 — pub(crate) fn render_ui_document(
- struct `DumpViewport` is never constructed
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs:2287 — struct DumpViewport {
- struct `DumpStructure` is never constructed
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs:2320 — struct DumpStructure {
- struct `DumpFrameStats` is never constructed
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs:2329 — struct DumpFrameStats {
- function `primary_window_id` is never used
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs:2560 — fn primary_window_id(engine: &ui_wgpu::wgpu::Ui) -> Option<String> {
- function `build_structure_dump` is never used
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs:2567 — fn build_structure_dump(engine: &ui_wgpu::wgpu::Ui, dpr: f32) -> DumpStructure {
- function `layer_is_nonempty` is never used
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs:2590 — fn layer_is_nonempty(layer: &ui_wgpu::wgpu::draw::DrawLayer) -> bool {
- function `is_glyph_instance` is never used
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs:2595 — fn is_glyph_instance(instance: &ui_wgpu::wgpu::draw::UiInstance) -> bool {
- function `build_frame_stats` is never used
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs:2600 — fn build_frame_stats(engine: &ui_wgpu::wgpu::Ui) -> DumpFrameStats {
- fields `canvas_image_digests` and `canvas_image_src_digests` are never read
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:413 — canvas_image_digests: HashMap<String, u64>,
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:414 — canvas_image_src_digests: HashMap<String, u64>,
- fields `id`, `image_key`, and `opacity` are never read
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:1986 — id: String,
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:1987 — image_key: Option<String>,
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:1992 — opacity: f32,
- fields `position`, `target`, `zoom`, `fov`, and `up` are never read
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:7692 — position: [f64; 3],
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:7693 — target: [f64; 3],
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:7695 — zoom: f64,
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:7697 — fov: Option<f64>,
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:7699 — up: Option<[f64; 3]>,
- fields `ambient_intensity`, `ambient_color`, `sun_azimuth`, `sun_elevation`, `sun_intensity`, and `sun_color` are never read
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:7710 — ambient_intensity: f64,
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:7712 — ambient_color: Option<String>,
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:7714 — sun_azimuth: f64,
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:7716 — sun_elevation: f64,
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:7718 — sun_intensity: f64,
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:7720 — sun_color: Option<String>,
- fields `color`, `metalness`, `roughness`, `emissive`, and `emissive_intensity` are never read
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:7727 — color: Option<String>,
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:7729 — metalness: Option<f64>,
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:7731 — roughness: Option<f64>,
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:7733 — emissive: Option<String>,
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:7735 — emissive_intensity: Option<f64>,
- multiple fields are never read
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:7741 — asset_url: String,
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:7742 — camera: IconRenderCameraFields,
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:7744 — lights: Option<IconRenderLightsFields>,
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:7748 — shape: Option<String>,
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:7750 — background: Option<String>,
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:7752 — shadow_enabled: Option<bool>,
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:7754 — material: Option<IconRenderMaterialFields>,
- field `presentation` is never read
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:7997 — presentation: String,
- field `descriptors` is never read
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:8005 — descriptors: Vec<VfsDescriptorColumn>,
- fields `id`, `label`, and `descriptor_kind_id` are never read
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:8010 — id: String,
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:8012 — label: String,
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:8014 — descriptor_kind_id: String,
- fields `descriptor_column_ids` and `descriptor_kinds` are never read
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:8020 — descriptor_column_ids: Vec<String>,
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:8022 — descriptor_kinds: HashMap<String, VfsDescriptorKind>,
- fields `was_pointer_down`, `last_click_ms`, `last_click_offset`, `context_menu`, and `pending_context_click` are never read
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:8154 — was_pointer_down: bool,
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:8157 — last_click_ms: f64,
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:8158 — last_click_offset: Option<usize>,
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:8161 — context_menu: Option<TextEditorContextMenu>,
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:8162 — pending_context_click: Option<(f32, f32, i16)>,
- field `insert_text` is never read
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:8189 — insert_text: Option<String>,
- methods `active_plugin_examples`, `dock_tab_bars_for_drop`, `build_display_windows_ui`, `build_display_layout_ui`, and `build_settings_general_ui` are never used
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:3467 — fn active_plugin_examples(&self) -> Vec<ExampleDefinition> {
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:3511 — fn dock_tab_bars_for_drop(&self, atlas: &mut FontAtlas, theme: &Theme, canvas: Rect, labels: &HashMap<String, String>, icon_ids: &HashMap<String, String>) -> Vec<(Vec<usize>, WindowStackCorner, Rect, Vec<f32>)> {
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:3693 — fn build_display_windows_ui(&self, session: &ActiveSession) -> UiNode {
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:3717 — fn build_display_layout_ui(&self, session: &ActiveSession) -> UiNode {
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:3741 — fn build_settings_general_ui(&self) -> UiNode {
- function `render_panel_tab_bar` is never used
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:8017 — fn render_panel_tab_bar(
- function `engagement_rail_width` is never used
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:8245 — fn engagement_rail_width(theme: &Theme, content_w: f32, inset: f32, measures_reserve: f32) -> f32 {
- function `measure_engagement_body_height` is never used
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:8292 — fn measure_engagement_body_height(theme: &Theme, engagement: &WindowEngagement) -> f32 {
- function `partition_utilities_by_category` is never used
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:8412 — fn partition_utilities_by_category(utilities: &[UtilityNode]) -> [Vec<UtilityNode>; 4] {
- function `render_presence_bar` is never used
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:8435 — fn render_presence_bar(draw: &mut DrawList, atlas: &mut FontAtlas, icons: &IconAtlas, input: &mut InputState<ActionDescriptor>, theme: &Theme, rows: &[ui_wgpu::wgpu::PresencePeerRow], right_edge: f32, btn_y: f32, btn_h: f32) {
- function `with_chrome_sink` is never used
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:8771 — fn with_chrome_sink<F, R>(draw: &mut DrawList, overlay: &mut Option<&mut DrawList>, f: F) -> R
- function `chrome_register_utility_tooltips` is never used
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:10759 — fn chrome_register_utility_tooltips(chrome: &mut ShellChromeBuildState, utilities: &[UtilityNode]) {
- method `element_rect_is_fallback` is never used
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:10858 — fn element_rect_is_fallback(&self, id: &str) -> bool {
- method `render_tutorial_bar` is never used
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:11741 — fn render_tutorial_bar(&mut self, draw: &mut DrawList, atlas: &mut FontAtlas, icons: &IconAtlas, input: &mut InputState<ActionDescriptor>, theme: &Theme, width: f32) {
- multiple associated items are never used
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:12621 — fn left_tabs(&self, session: &ActiveSession) -> Vec<PanelTabDefinition> {
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:12657 — fn right_tabs(&self, session: &ActiveSession) -> Vec<PanelTabDefinition> {
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:12695 — fn active_left_tab_id(&self, session: &ActiveSession) -> String {
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:12716 — fn active_right_tab_id(&self, session: &ActiveSession) -> String {
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:13655 — fn render_navbar(&mut self, draw: &mut DrawList, atlas: &mut FontAtlas, icons: &IconAtlas, input: &mut InputState<ActionDescriptor>, theme: &Theme, width: f32) {
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:13825 — fn render_footer(&mut self, draw: &mut DrawList, atlas: &mut FontAtlas, icons: &IconAtlas, input: &mut InputState<ActionDescriptor>, theme: &Theme, width: f32, height: f32) {
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:13888 — fn render_floating_panel(
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:13970 — fn render_left_panel(
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:13996 — fn render_right_panel(
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:14022 — fn render_main_window(
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:14125 — fn render_studio_canvas_bars(&self, draw: &mut DrawList, atlas: &mut FontAtlas, icons: &IconAtlas, input: &mut InputState<ActionDescriptor>, theme: &Theme, mut canvas: Rect, session: &ActiveSession) -> Rect {
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:14164 — fn render_window_content(
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:14217 — fn render_overlay(&mut self, overlay: &mut DrawList, atlas: &mut FontAtlas, icons: &IconAtlas, input: &mut InputState<ActionDescriptor>, theme: &Theme, width: f32, height: f32) {
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:14445 — fn resolve_introduction_element_rects(&self, id: &str, theme: &Theme, width: f32, height: f32, hit_targets: &[HitTarget<ActionDescriptor>]) -> Vec<Rect> {
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:14489 — fn resolve_introduction_window_silhouettes(&self, id: &str) -> Vec<WindowSilhouette> {
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:14507 — fn resolve_introduction_element_rect(&self, id: &str, theme: &Theme, width: f32, height: f32, hit_targets: &[HitTarget<ActionDescriptor>]) -> Option<Rect> {
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:14678 — fn render_chrome_tour(&mut self, overlay: &mut DrawList, atlas: &mut FontAtlas, input: &mut InputState<ActionDescriptor>, theme: &Theme, width: f32, height: f32) {
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:14841 — fn render_example_dropdown(&self, overlay: &mut DrawList, atlas: &mut FontAtlas, input: &mut InputState<ActionDescriptor>, theme: &Theme, x: f32, y: f32, w: f32, items: &[(String, String, usize)], examples: &[ExampleDefinition]) {
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:14864 — fn render_action_list(
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:14918 — fn measures_for_kind(kind: &semio_framework::WindowKindDefinition) -> &[WindowMeasure] {
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:14922 — fn engagement_for_kind(&self, kind: &semio_framework::WindowKindDefinition) -> Option<WindowEngagement> {
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:14927 — fn render_window_measures_rail(
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:15014 — fn render_utility_options_rail(
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:15057 — fn render_window_measure_tree(
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:15091 — fn render_window_measure_one(&mut self, draw: &mut DrawList, overlay: &mut Option<&mut DrawList>, atlas: &mut FontAtlas, icons: &IconAtlas, input: &mut InputState<ActionDescriptor>, theme: &Theme, bounds: Rect, measure: &WindowMeasure) {
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:15139 — fn render_window_engagement_rail(
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:15236 — fn render_engagement_input(
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:15298 — fn render_engagement_control(
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:15352 — fn render_window_actions_rail(
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:15436 — fn staged_form_height(&self, theme: &Theme, action: &semio_framework::ActionDefinition) -> f32 {
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:15444 — fn staged_arg_height(&self, theme: &Theme, arg: &semio_framework::ActionArgDef) -> f32 {
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:15463 — fn render_staged_form(
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:15503 — fn render_staged_arg(
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:15578 — fn staged_arg_display_string(&self, window_id: &str, action_id: &str, arg: &semio_framework::ActionArgDef, input: &InputState<ActionDescriptor>, control_id: Option<&str>) -> String {
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:15594 — fn render_staged_text_field(
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:15618 — fn paint_staged_input_box(&self, draw: &mut DrawList, atlas: &mut FontAtlas, input: &mut InputState<ActionDescriptor>, theme: &Theme, rect: Rect, display: &str, focused: bool, enabled: bool, control_id: &str) {
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:15633 — fn render_context_menu(&self, overlay: &mut DrawList, atlas: &mut FontAtlas, icons: &IconAtlas, input: &mut InputState<ActionDescriptor>, theme: &Theme, menu: &ContextMenuState, viewport_w: f32, viewport_h: f32) {
- associated function `new` is never used
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:15861 — fn new() -> Self {
- static `PREFS_STORE` is never used
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:15962 — static PREFS_STORE: std::sync::OnceLock<std::sync::Mutex<std::cell::RefCell<FilePrefsStore>>> = std::sync::OnceLock::new();
- fields `generation`, `timestamp_us`, `dispatch_tree`, and `damage_regions` are never read
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📸️render-snapshot/🦀️.rs:38 — pub generation: semio_framework_trace::Generation,
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📸️render-snapshot/🦀️.rs:39 — pub timestamp_us: u64,
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📸️render-snapshot/🦀️.rs:48 — pub dispatch_tree: Option<Arc<()>>,
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📸️render-snapshot/🦀️.rs:51 — pub damage_regions: Option<Vec<()>>,
- field `lane` is never read
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs:4885 — pub lane: u8,
- fields `epoch` and `surface` are never read
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs:6142 — Closing { epoch: u64, batch: u64, generation: u64, surface: SurfaceId, document: UiDocumentLease },
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs:6142 — Closing { epoch: u64, batch: u64, generation: u64, surface: SurfaceId, document: UiDocumentLease },
