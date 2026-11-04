# 1104 Window Disposer Birth Audit

STATUS: CURRENT PHYSICAL SOURCE INVENTORY. Read-only recursive filesystem scan includes untracked Rust files. No Cargo or source edits.

The constructor bounded_window_config_store_disposer::<Self>() must pair with explicit store_disposer_birth calling bounded_window_config_store_disposer_birth::<Self>(). Trait default intentionally returns UnsupportedOwner at owner-contract line35; matching helper at187 quotes ArtifactDocumentStoreDisposer<State,Mutation> size and depth1, while constructor at52 builds that concrete disposer. No noop/default or budget changes are warranted.

## Exact Owner Inventory

### ✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/✒️main/🎚️config/🦀️.rs

- impl semio_framework_plugin::WindowConfigOwner for WriterMainWindowConfigOwner { at line56: MISSING matching birth

Macro/owner declarations:

- pub struct WriterMainWindowConfigOwner;

### ✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🕸️graph/🎚️config/🦀️.rs

- impl semio_framework_plugin::WindowConfigOwner for EquationGraphWindowConfigOwner { at line58: MISSING matching birth

Macro/owner declarations:

- pub struct EquationGraphWindowConfigOwner;

### ✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🔲️grid2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🪟️window/🦀️.rs

- impl semio_framework_plugin::WindowConfigOwner for $config { at line466: MISSING matching birth

Macro/owner declarations:

- owners!(Grid2dGridWindowConfigOwner, Grid2dGridWindowTransientOwner, grid::WINDOW_KIND_ID);
- owners!(Grid2dPreviewWindowConfigOwner, Grid2dPreviewWindowTransientOwner, preview::WINDOW_KIND_ID);

### ✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🖼️bitmap/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🖼️input/🎚️config/🦀️.rs

- impl semio_framework_plugin::WindowConfigOwner for BitmapInputWindowConfigOwner { at line223: MISSING matching birth

Macro/owner declarations:

- pub struct BitmapInputWindowConfigOwner;

### ✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🖼️bitmap/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🧩️output/🎚️config/🦀️.rs

- impl semio_framework_plugin::WindowConfigOwner for BitmapOutputWindowConfigOwner { at line222: MISSING matching birth

Macro/owner declarations:

- pub struct BitmapOutputWindowConfigOwner;

### ✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🧱️grid3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🪟️window/🦀️.rs

- impl semio_framework_plugin::WindowConfigOwner for $owner { at line380: MISSING matching birth

Macro/owner declarations:

- config_owner!(Grid3dGridWindowConfigOwner, grid::WINDOW_KIND_ID);
- config_owner!(Grid3dPreviewWindowConfigOwner, preview::WINDOW_KIND_ID);

### ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/👁️preview/🎚️config/🦀️.rs

- impl semio_framework_plugin::WindowConfigOwner for Generation2dEditPreviewWindowConfigOwner { at line41: MISSING matching birth

Macro/owner declarations:

- pub struct Generation2dEditPreviewWindowConfigOwner;

### ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🕸️flow/🎚️config/🦀️.rs

- impl semio_framework_plugin::WindowConfigOwner for Generation2dMainWindowConfigOwner { at line42: MISSING matching birth

Macro/owner declarations:

- pub struct Generation2dMainWindowConfigOwner;

### ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/🧬️generate/🪟️windows/👁️preview/🎚️config/🦀️.rs

- impl semio_framework_plugin::WindowConfigOwner for Generation2dGeneratePreviewWindowConfigOwner { at line41: MISSING matching birth

Macro/owner declarations:

- pub struct Generation2dGeneratePreviewWindowConfigOwner;

### ✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🌊️main/🎚️config/🦀️.rs

- impl semio_framework_plugin::WindowConfigOwner for FlowMainWindowConfigOwner { at line55: MISSING matching birth

Macro/owner declarations:

- pub struct FlowMainWindowConfigOwner;

### ✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/👁️view/🪟️windows/🏔️terrain/🎚️config/🦀️.rs

- impl semio_framework_plugin::WindowConfigOwner for GisTerrainWindowConfigOwner { at line90: MISSING matching birth

Macro/owner declarations:

- pub struct GisTerrainWindowConfigOwner;

### ✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🗺️map/🎚️config/🦀️.rs

- impl semio_framework_plugin::WindowConfigOwner for MapWindowConfigOwner { at line124: MISSING matching birth

Macro/owner declarations:

- pub struct MapWindowConfigOwner;

### ✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🪟️windows/🗺️map/🎚️config/🦀️.rs

- impl semio_framework_plugin::WindowConfigOwner for GisMapViewerWindowConfigOwner { at line63: MISSING matching birth

Macro/owner declarations:

- pub struct GisMapViewerWindowConfigOwner;

### ✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📽️main/🎚️config/🦀️.rs

- impl semio_framework_plugin::WindowConfigOwner for SequenceMainWindowConfigOwner { at line200: MISSING matching birth

Macro/owner declarations:

- pub struct SequenceMainWindowConfigOwner;

### ✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📊️results/🎚️config/🦀️.rs

- impl semio_framework_plugin::WindowConfigOwner for Fem2dResultsWindowConfigOwner { at line169: MISSING matching birth

Macro/owner declarations:

- pub struct Fem2dResultsWindowConfigOwner;

### ✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🧱️model/🎚️config/🦀️.rs

- impl semio_framework_plugin::WindowConfigOwner for Fem2dModelWindowConfigOwner { at line151: MISSING matching birth

Macro/owner declarations:

- pub struct Fem2dModelWindowConfigOwner;

### ✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📊️results/🎚️config/🦀️.rs

- impl semio_framework_plugin::WindowConfigOwner for Fem3dResultsWindowConfigOwner { at line163: MISSING matching birth

Macro/owner declarations:

- pub struct Fem3dResultsWindowConfigOwner;

### ✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🧱️model/🎚️config/🦀️.rs

- impl semio_framework_plugin::WindowConfigOwner for Fem3dModelWindowConfigOwner { at line155: MISSING matching birth

Macro/owner declarations:

- pub struct Fem3dModelWindowConfigOwner;

### ✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🖌️render/🪟️window-config/🦀️.rs

- impl semio_framework_plugin::WindowConfigOwner for $owner { at line129: MISSING matching birth

Macro/owner declarations:


### ✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/↔️adjacency/🎚️config/🦀️.rs

- impl semio_framework_plugin::WindowConfigOwner for ArchitectAdjacencyWindowConfigOwner { at line195: MISSING matching birth

Macro/owner declarations:

- pub struct ArchitectAdjacencyWindowConfigOwner;

### ✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📋️register/🎚️config/🦀️.rs

- impl semio_framework_plugin::WindowConfigOwner for ArchitectRegisterWindowConfigOwner { at line207: MISSING matching birth

Macro/owner declarations:

- pub struct ArchitectRegisterWindowConfigOwner;

### ✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📓️report/🎚️config/🦀️.rs

- impl semio_framework_plugin::WindowConfigOwner for ArchitectReportWindowConfigOwner { at line197: MISSING matching birth

Macro/owner declarations:

- pub struct ArchitectReportWindowConfigOwner;

### ✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🕸️graph/🎚️config/🦀️.rs

- impl semio_framework_plugin::WindowConfigOwner for ArchitectGraphWindowConfigOwner { at line190: MISSING matching birth

Macro/owner declarations:

- pub struct ArchitectGraphWindowConfigOwner;

### ✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🕸️canvas/🎚️config/🦀️.rs

- impl semio_framework_plugin::WindowConfigOwner for WiresCanvasWindowConfigOwner { at line64: MISSING matching birth

Macro/owner declarations:

- pub struct WiresCanvasWindowConfigOwner;

### ✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/📝️blueprint/🪟️windows/▶️try/🎚️config/🦀️.rs

- impl semio_framework_plugin::WindowConfigOwner for FormsTryWindowConfigOwner { at line52: MISSING matching birth

Macro/owner declarations:

- pub struct FormsTryWindowConfigOwner;

### ✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📐️blueprint/🎚️config/🦀️.rs

- impl semio_framework_plugin::WindowConfigOwner for $owner { at line211: MISSING matching birth

Macro/owner declarations:

- config_owner!(LayoutBlueprintWindowConfigOwner, super::LAYOUT_PLAY_WINDOW_BLUEPRINT);
- config_owner!(LayoutPreviewWindowConfigOwner, crate::modes::edit::windows::preview::LAYOUT_PLAY_WINDOW_PREVIEW);

### ✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🎚️config/🦀️.rs

- impl semio_framework_plugin::WindowConfigOwner for $owner { at line212: MISSING matching birth

### ✏️s/🔌️plugins/📕️norm/🪟️results/🎚️config/🦀️.rs

- impl semio_framework_plugin::WindowConfigOwner for $owner { at line27: MISSING matching birth

### ✏️s/🔌️plugins/📕️norm/🪟️viewport/🎚️config/🦀️.rs

- impl semio_framework_plugin::WindowConfigOwner for $owner { at line29: MISSING matching birth

### ✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/📷️capture/🪟️windows/🖼️frames/🎚️config/🦀️.rs

- impl semio_framework_plugin::WindowConfigOwner for RemodelingFramesWindowConfigOwner { at line110: MISSING matching birth

Macro/owner declarations:

- impl_codecs!();
- pub struct RemodelingFramesWindowConfigOwner;

### ✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/🔍️analyze/🪟️windows/📊️report/🎚️config/🦀️.rs

- impl semio_framework_plugin::WindowConfigOwner for RemodelingReportWindowConfigOwner { at line97: MISSING matching birth

Macro/owner declarations:

- pub struct RemodelingReportWindowConfigOwner;

### ✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/🧊️model/🪟️windows/🧊️model/🎚️config/🦀️.rs

- impl semio_framework_plugin::WindowConfigOwner for RemodelingModelWindowConfigOwner { at line179: MISSING matching birth

Macro/owner declarations:

- pub struct RemodelingModelWindowConfigOwner;

### ✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🧊️model/🎚️config/🦀️.rs

- impl semio_framework_plugin::WindowConfigOwner for EnergyModelWindowConfigOwner { at line74: PAIRED

Macro/owner declarations:

- pub struct EnergyModelWindowConfigOwner;

### ✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🪟️windows/🧊️model/🎚️config/🦀️.rs

- impl semio_framework_plugin::WindowConfigOwner for EnergyModelViewerWindowConfigOwner { at line77: PAIRED

Macro/owner declarations:

- pub struct EnergyModelViewerWindowConfigOwner;

### ✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewriting/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🪟️window/🎚️config/🦀️.rs

- impl semio_framework_plugin::WindowConfigOwner for $name { at line76: MISSING matching birth

Macro/owner declarations:

- window_owner!(BeforeWindowConfigOwner, TRINITY_REWRITING_PLAY_WINDOW_BEFORE);
- window_owner!(AfterWindowConfigOwner, TRINITY_REWRITING_PLAY_WINDOW_AFTER);
- window_owner!(LhsWindowConfigOwner, TRINITY_REWRITING_PLAY_WINDOW_LHS);
- window_owner!(RhsWindowConfigOwner, TRINITY_REWRITING_PLAY_WINDOW_RHS);

### ✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🌐️graph/🎚️config/🦀️.rs

- impl semio_framework_plugin::WindowConfigOwner for JackGraphWindowConfigOwner { at line73: MISSING matching birth

Macro/owner declarations:

- pub struct JackGraphWindowConfigOwner;

### ✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🖼️canvas/🎚️config/🦀️.rs

- impl semio_framework_plugin::WindowConfigOwner for DrawingCanvasWindowConfigOwner { at line196: MISSING matching birth

Macro/owner declarations:

- pub struct DrawingCanvasWindowConfigOwner;

### ✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🪟️windows/🖼️canvas/🎚️config/🦀️.rs

- impl semio_framework_plugin::WindowConfigOwner for DrawingViewerCanvasWindowConfigOwner { at line196: MISSING matching birth

Macro/owner declarations:

- pub struct DrawingViewerCanvasWindowConfigOwner;

### ✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🪟️window/🦀️.rs

- impl semio_framework_plugin::WindowConfigOwner for NoteCompositeWindowConfigOwner { at line195: MISSING matching birth

Macro/owner declarations:

- pub struct NoteCompositeWindowConfigOwner;

### ✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🪟️window/🦀️.rs

- impl semio_framework_plugin::WindowConfigOwner for $config { at line664: MISSING matching birth

Macro/owner declarations:

- owners!(Puzzle2dOverviewWindowConfigOwner, Puzzle2dOverviewWindowTransientOwner, crate::editor::puzzle2d::config::PUZZLE2D_OVERVIEW_WINDOW_KIND_ID);
- owners!(Puzzle2dDetailWindowConfigOwner, Puzzle2dDetailWindowTransientOwner, crate::editor::puzzle2d::config::PUZZLE2D_DETAIL_WINDOW_KIND_ID);
- owners!(Puzzle2dSelectionWindowConfigOwner, Puzzle2dSelectionWindowTransientOwner, crate::editor::puzzle2d::config::PUZZLE2D_SELECTION_WINDOW_KIND_ID);

### ✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🪟️window/🦀️.rs

- impl semio_framework_plugin::WindowConfigOwner for $config { at line449: MISSING matching birth

Macro/owner declarations:

- owners!(Puzzle5dBoardWindowConfigOwner, Puzzle5dBoardWindowConfig, Puzzle5dBoardWindowConfigMutation, "puzzle.5dboardwindowconfig", Puzzle5dBoardWindowTransientOwner, PUZZLE5D_BOARD_WINDOW_KIND_ID);
- owners!(Puzzle5dWorldWindowConfigOwner, Puzzle5dWorldWindowConfig, Puzzle5dWorldWindowConfigMutation, "puzzle.5dworldwindowconfig", Puzzle5dWorldWindowTransientOwner, PUZZLE5D_WORLD_WINDOW_KIND_ID);

### ✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🪟️window/🦀️.rs

- impl semio_framework_plugin::WindowConfigOwner for Puzzle3dWindowConfigOwner { at line625: PAIRED

Macro/owner declarations:

- pub struct Puzzle3dWindowConfigOwner;

### ✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🔢️grid/🎚️config/🦀️.rs

- impl semio_framework_plugin::WindowConfigOwner for GridWindowConfigOwner { at line192: MISSING matching birth

Macro/owner declarations:

- pub struct GridWindowConfigOwner;

### 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/♻️publication-retirement-authority/🦀️.rs

- impl WindowConfigOwner for RetirementWindowConfigOwner { at line134: MISSING matching birth

Macro/owner declarations:

- eprintln!("window transient retired a rejected authority over {turns} Ok turns, refused a stale re-begin, and admitted the refreshed one at generation {}", app.stores.window_transient_store.capture(Some(&view)).unwrap().unwrap().generation);

### 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️tool-run/🦀️.rs

- impl WindowConfigOwner for ToyWorldWindowConfig { at line387: MISSING matching birth

Macro/owner declarations:


### 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window/🎚️config/🗄️store-owners/🌱️initialization/🎟️birth/🧪️tests/🧸️fixture/🦀️.rs

- impl WindowConfigOwner for Owner{ at line61: PAIRED

### 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window/🎚️config/🧪️tests/📷️camera-owner/🦀️.rs

- impl WindowConfigOwner for $owner { at line197: MISSING matching birth
- impl WindowConfigOwner for IdentityWindowOwner { at line231: MISSING matching birth

Macro/owner declarations:

- retained_load_owner!(RetainedLoadOwnerA, "retained-load-a", "test.window.retained-load.a");
- retained_load_owner!(RetainedLoadOwnerB, "retained-load-b", "test.window.retained-load.b");

## Demonstrator Priorities

Puzzle3d is the observed aggregator failure; its explicit pairing became present during this audit (concurrent execution_baseline fix), so its row is PAIRED at scan time. Sourcing GridWindowConfigOwner is an aussuchen pane owner. FEM3d model/results are statik pane owners; GIS map editor is verfolgen; CAD shared macro owns koordinator windows. Flow main is the flow editor/extension ecosystem; procedural generation2d owners are not the generator generation3d pane. Energy editor/viewer already pair explicitly. Other listed app owners remain off direct eight-pane ownership unless runtime closure separately proves reachability. Framework files under tests are test-only; missing contracts there affect law fixtures rather than production pane initialization. This is an inventory, not blanket authorization to edit unrelated owners.

CAD macro concrete owners: CadShapeWindowConfigOwner, CadBuildingWindowConfigOwner, CadEnergyWindowConfigOwner and CadStructureClassicWindowConfigOwner, confirmed from current() dispatch in the macro source. Other shared macros in Norm/BIM list templates whose per-artifact expansion is not enumerated here; do not interpret one template as one runtime owner.
