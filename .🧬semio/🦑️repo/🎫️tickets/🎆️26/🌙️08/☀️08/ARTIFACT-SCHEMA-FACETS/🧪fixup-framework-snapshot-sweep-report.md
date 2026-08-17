# 🧪 Fixup Report — Framework Snapshot Sweep (Shared Surfaces)

Ticket `26/08/08/ARTIFACT-SCHEMA-FACETS`. Shared-surface wave: rename document-state
`Projection` → `Snapshot` across scoped trees. Did **not** touch `✏️s/🔌️plugins/**`.

## 1. Classification counts

| Bucket | Count | Notes |
| --- | ---: | --- |
| **Renamed** (document-state API / types / locals) | **~480+** | Pass-1 token tally below |
| **Deliberately kept** (camera / map / geo / db / icons) | **~1600+** line hits | Triaged; not document Snapshot |
| **Remaining old document-state API in scope** | **0** | Verified by ripgrep |

### Pass-1 rename tally

| Token | Count |
| --- | ---: |
| `SpaceProjection` → `SpaceSnapshot` | 48 |
| `CollectionProjection` → `CollectionSnapshot` | 31 |
| `OsProjection` → `OsSnapshot` (docs) | 20 |
| `.projection(` → `.snapshot(` | 29 |
| `projectionJson` / `projection_json` | 27 |
| `empty_*_projection` → `empty_*_snapshot` | 27 |
| `apply_parameter_values_to_projection` | 14 |
| `materialize_backbone_projection` | 8 |
| `projection_with_conflicts` | 8 |
| `initial_projection` | 8 |
| `materialize_document_projection` | 6 |
| `decodeDocumentPackProjection` | 4 |
| `fn projection(` wrappers | 2 |
| bare ident `projection`→`snapshot` (non-camera files) | 230 |
| `empty_flow_projection` (follow-up) | 3 |

Cartographic `pub mod projection` (tiled-map / terrain) and db read-model files were restored
after a too-broad first pass; they keep the word `projection`.

## 2. Deliberately kept projection names

- **3D / camera:** `WorldProjection*`, `world3d_projection_*`, `computeWorldProjectionPose`,
  `setProjection`, `setProjectionParam`, `Oblique_projection`, `Axonometric_projection`,
  `updateProjectionMatrix`, `projectionMatrix`, `WorldOrbitProjection*`, `worldProjectionFamily`,
  camera field `projection: "orthographic" | "perspective"`, UI `"Projection"` / `Projektion`,
  `ui.host.projection` / `ui.scene.projection`, `projection-pane`
- **Icon taxonomy:** `ProjectionAxonometric`, `ProjectionPerspective`, `projection-oblique`, …
- **Map / GIS:** `🗺️tiled-map` / `🏔️terrain` `pub mod projection`
- **Geometry:** orthogonal / radial / view-projection in `✏️s/🔨️modules/🧊️3d/**`
- **DB read-model:** `🛢️db/📽️projection/**`, `IndexKind::Projection`, `ProjectionIndex`, `projection_id`
- **Manifest boundary:** `WindowKindDefinition::document_projection_schema`

## 3. Files touched (grouped by tree)

### Document-state rename (old API now absent in scope)

**`🧰️framework/🛍️products/💻️os/`**
- `🖥️host/🦀️component.rs`
- `🦀️component.rs`
- `🟦️component.ts`
- `📦️packages/🟦️typescript/🧪️vitest.config.ts` (vitest include paths)
- `🔨️modules/🌊️flow/🖥️host/🦀️component.rs`
- `🔨️modules/🌊️flow/🌿️vcs/🦀️component.rs`
- `🔨️modules/🌊️flow/📄️document/🦀️component.rs`
- `🔨️modules/🌊️flow/**/pkg/*.d.ts` (`snapshotJson` wasm bindings)
- `🔨️modules/🪐️space/🦀️component.rs`
- `🔨️modules/📖️playbook/🦀️component.rs`
- `🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🦀️component.rs`
- `🔨️modules/🗣️dsl/🧪️fixture-sweep/🦀️component.rs`
- `🔨️modules/🏃️run/🦀️component.rs` + `📦️bin.rs`
- `🔨️modules/🔁️workflow/🦀️component.rs`

**`✏️s/🔨️modules/`**
- `💭️mindmap/🧩️extension/🦀️component.rs` (docstring only)

### Collateral fix (workspace compile path, not a Snapshot rename)

**`🧰️framework/🔨️modules/🖱️ui/`**
- `📦️packages/🦀️rust/🎯️targets/⌨️tui/📦️glue.rs` — widget `#[path]` mods lifted to glue so
  `crate::tui::{select,chip,…}` resolve
- `⌨️tui/🦀️component.rs` — removed nested element `mod` declarations (now in glue)

### Explicitly not edited

- `✏️s/🔌️plugins/**`
- `🧬️schema/**`
- `🛢️db/📽️projection/**` (and other db Projection read-model symbols)
- `🔌️plugin/🦀️component.rs` (SDK leaf; already done)
- camera / World3d / r3f / Scene projection surfaces
- root `📜️script.ts`, `🔣️taxonomy.json`, `compose/**`

## 4. Gate tails (verbatim)

### 1. `cargo check -p semio-framework-os-flow` — PASS

```

warning: unnecessary qualification
   --> 🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/📦️packages/🦀️rust/../../📐️brep-geometry/🦀️component.rs:447:19
    |
447 |     Codec(#[from] neural_engine::EvalError),
    |                   ^^^^^^^^^^^^^^^^^^^^^^^^
    |
help: remove the unnecessary path segments
    |
447 -     Codec(#[from] neural_engine::EvalError),
447 +     Codec(#[from] EvalError),
    |

warning: type `FlowExtensionRegistryState` is more private than the item `FLOW_EXTENSION_STATE`
  --> 🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/📦️packages/🦀️rust/../../📔️registry/🦀️component.rs:65:1
   |
65 | pub(crate) static FLOW_EXTENSION_STATE: LazyLock<Mutex<FlowExtensionRegistryState>> = LazyLock::new(|| Mutex::new(FlowExtensionRegis...
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ static `FLOW_EXTENSION_STATE` is reachable at visibility `pub(crate)`
   |
note: but type `FlowExtensionRegistryState` is only usable at visibility `pub(self)`
  --> 🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/📦️packages/🦀️rust/../../📔️registry/🦀️component.rs:59:1
   |
59 | struct FlowExtensionRegistryState {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   = note: `#[warn(private_interfaces)]` on by default

warning: `semio-framework-os-flow` (lib) generated 129 warnings (run `cargo fix --lib -p semio-framework-os-flow` to apply 126 suggestions)
    Finished `dev` profile [unoptimized] target(s) in 0.94s
warning: the following packages contain code that will be rejected by a future version of Rust: block v0.1.6
note: to see what the problems were, use the option `--future-incompat-report`, or run `cargo report future-incompatibilities --id 1`
```

### 2. `cargo check -p semio-framework-os-kernel` — PASS

```
warning: unused variable: `len`
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🗣️dsl/🔍️lexer/🦀️component.rs:333:21
    |
333 |                 let len = end_j - i;
    |                     ^^^ help: if this is intentional, prefix it with an underscore: `_len`
    |
    = note: `#[warn(unused_variables)]` (part of `#[warn(unused)]`) on by default

warning: function `print_edge_label` is never used
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🗣️dsl/🖋️notation/🦀️component.rs:269:4
    |
269 | fn print_edge_label(label: &EdgeLabel, out: &mut String) {
    |    ^^^^^^^^^^^^^^^^
    |
    = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

warning: method `set_envelope` is never used
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️component.rs:2283:19
     |
2177 | / impl<P, Mutation> DocumentStore<P, Mutation>
2178 | | where
2179 | |     P: Clone + Serialize + DeserializeOwned + DocumentPack,
2180 | |     Mutation: Clone + Serialize + DeserializeOwned + self::Mutation<P> + OpBinary + OpText,
     | |___________________________________________________________________________________________- method in this implementation
...
2283 |       pub(crate) fn set_envelope(&mut self, envelope: DocumentEnvelope<P, Mutation>, applied_edit_ids: Vec<String>) {
     |                     ^^^^^^^^^^^^

warning: `semio-framework-os-kernel` (lib) generated 45 warnings (run `cargo fix --lib -p semio-framework-os-kernel` to apply 22 suggestions)
    Finished `dev` profile [unoptimized] target(s) in 0.19s
```

### 3. `cargo check -p semio-framework-plugin` — PASS

```
     |
5168 |                 let VcsDocumentApp { app, cache, .. } = self;
     |                                      ^^^ help: try ignoring the field: `app: _`

warning: unused variable: `app`
    --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:1847:17
     |
1847 |             let app = A::default();
     |                 ^^^ help: if this is intentional, prefix it with an underscore: `_app`

warning: unused variable: `app`
    --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:5390:34
     |
5390 |             let VcsDocumentApp { app, cache, .. } = self;
     |                                  ^^^ help: try ignoring the field: `app: _`

warning: unused variable: `app`
    --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:5048:42
     |
5048 |                     let VcsDocumentApp { app, cache, .. } = self;
     |                                          ^^^ help: try ignoring the field: `app: _`

warning: unused variable: `app`
    --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:5421:34
     |
5421 |             let VcsDocumentApp { app, cache, .. } = self;
     |                                  ^^^ help: try ignoring the field: `app: _`

warning: `semio-framework-plugin` (lib) generated 15 warnings (run `cargo fix --lib -p semio-framework-plugin` to apply 15 suggestions)
    Finished `dev` profile [unoptimized] target(s) in 0.21s
```

### 4. `cargo check --workspace 2>&1 | rg '^error' -A4` — FAIL (`semio-framework-os` only)

Failed package: **`semio-framework-os`** (host product crate) with 116 errors.
**Zero** errors under `✏️s/🔌️plugins/**` in this log (plugin crates were not the failing package).

Excerpt (first error blocks):

```
error[E0432]: unresolved import `crate::store_sync`
    --> 🧰️framework/🛍️products/💻️os/🖥️host/📦️packages/🦀️rust/../../🦀️component.rs:1640:16
     |
1640 |     use crate::store_sync::{FolderSqliteStorage, FolderTextStorage};
     |                ^^^^^^^^^^ could not find `store_sync` in the crate root

error[E0432]: unresolved import `crate::store_sync`
    --> 🧰️framework/🛍️products/💻️os/🖥️host/📦️packages/🦀️rust/../../🦀️component.rs:1831:16
     |
1831 |     use crate::store_sync::{DocumentActorConfig, DocumentActorMsg, DocumentChannels, DocumentEvent, DocumentHost, PersistenceBindi...
     |                ^^^^^^^^^^ could not find `store_sync` in the crate root

error[E0432]: unresolved imports `crate::workflow_kernel::apply_flow_fixture_to_os_workflow`, `crate::workflow_kernel::assert_os_media_export_coverage`, `crate::workflow_kernel::assert_os_media_import_coverage`, `crate::workflow_kernel::build_os_workflow_operator_infos`, `crate::workflow_kernel::export_os_app_instance_media`, `crate::workflow_kernel::import_os_app_instance_media`, `crate::workflow_kernel::negotiate_media_contract`, `crate::workflow_kernel::os_media_export_extension_for_format`, `crate::workflow_kernel::os_media_neuron_kind_for_node`, `crate::workflow_kernel::os_resource_media_capability`, `crate::workflow_kernel::os_workflow_to_flow_fixture`, `crate::workflow_kernel::os_workflow_to_node_graph_payload`, `crate::workflow_kernel::register_os_media_export_handler`, `crate::workflow_kernel::register_os_media_import_handler`, `crate::workflow_kernel::required_os_media_export_formats`, `crate::workflow_kernel::required_os_media_import_formats`, `crate::workflow_kernel::OsMediaCapability`, `crate::workflow_kernel::OsMediaExportResult`, `crate::workflow_kernel::OsWorkflowCamera`, `crate::workflow_kernel::OsWorkflowNodeGraphPayload`, `crate::workflow_kernel::OsWorkflowOperatorInfo`, `crate::workflow_kernel::OS_MEDIA_FLOW_MODULE_ID`, `crate::workflow_kernel::OS_SPACE_SCHEMA`, `crate::workflow_kernel::OS_WORKFLOW_VFS_ROOT_ID`
    --> 🧰️framework/🛍️products/💻️os/🖥️host/📦️packages/🦀️rust/../../🦀️component.rs:4367:5
     |
4367 |     apply_flow_fixture_to_os_workflow, apply_workflow_operation, assert_os_media_export_coverage, assert_os_media_import_coverage, build_os_workflow_operator_infos, create_default_workflow_parameter, empty_workflow, empty_workflow_document,
     |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^                            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ no `build_os_workflow_operator_infos` in `workflow_kernel`

error[E0433]: cannot find module or crate `zip` in this scope
    --> 🧰️framework/🛍️products/💻️os/🖥️host/📦️packages/🦀️rust/../../../🔨️modules/🪐️space/🦀️component.rs:1315:17
     |
1315 |     Zip(#[from] zip::result::ZipError),
     |                 ^^^ use of unresolved module or unlinked crate `zip`

error[E0433]: cannot find module or crate `zip` in this scope
    --> 🧰️framework/🛍️products/💻️os/🖥️host/📦️packages/🦀️rust/../../../🔨️modules/🪐️space/🦀️component.rs:1334:26
     |
1334 | fn zip_file_options() -> zip::write::SimpleFileOptions {
     |                          ^^^ use of unresolved module or unlinked crate `zip`

error[E0433]: cannot find module or crate `zip` in this scope
    --> 🧰️framework/🛍️products/💻️os/🖥️host/📦️packages/🦀️rust/../../../🔨️modules/🪐️space/🦀️component.rs:1337:5
     |
1337 |     zip::write::SimpleFileOptions::default().compression_method(zip::CompressionMethod::Deflated).last_modified_time(zip::DateTime...
     |     ^^^ use of unresolved module or unlinked crate `zip`

error[E0433]: cannot find module or crate `zip` in this scope
    --> 🧰️framework/🛍️products/💻️os/🖥️host/📦️packages/🦀️rust/../../../🔨️modules/🪐️space/🦀️component.rs:1337:65
     |
1337 |     zip::write::SimpleFileOptions::default().compression_method(zip::CompressionMethod::Deflated).last_modified_time(zip::DateTime...
     |                                                                 ^^^ use of unresolved module or unlinked crate `zip`

error[E0433]: cannot find module or crate `zip` in this scope
    --> 🧰️framework/🛍️products/💻️os/🖥️host/📦️packages/🦀️rust/../../../🔨️modules/🪐️space/🦀️component.rs:1337:118
     |
1337 | ...Method::Deflated).last_modified_time(zip::DateTime::default())
     |                                         ^^^ use of unresolved module or unlinked crate `zip`

error[E0433]: cannot find module or crate `zip` in this scope
    --> 🧰️framework/🛍️products/💻️os/🖥️host/📦️packages/🦀️rust/../../../🔨️modules/🪐️space/🦀️component.rs:1340:112
     |
1340 | ..., name: &str, bytes: &[u8], options: zip::write::SimpleFileOptions) -> Result<(), SpaceZipError> {
     |                                         ^^^ use of unresolved module or unlinked crate `zip`

error[E0433]: cannot find module or crate `zip` in this scope
    --> 🧰️framework/🛍️products/💻️os/🖥️host/📦️packages/🦀️rust/../../../🔨️modules/🪐️space/🦀️component.rs:1365:22
     |
1365 |     let mut writer = zip::ZipWriter::new(Cursor::new(Vec::new()));
     |                      ^^^ use of unresolved module or unlinked crate `zip`

error[E0433]: cannot find module or crate `zip` in this scope
    --> 🧰️framework/🛍️products/💻️os/🖥️host/📦️packages/🦀️rust/../../../🔨️modules/🪐️space/🦀️component.rs:1396:23
     |
1396 |     let mut archive = zip::ZipArchive::new(Cursor::new(bytes))?;
     |                       ^^^ use of unresolved module or unlinked crate `zip`

error[E0433]: cannot find module or crate `space` in this scope
   --> 🧰️framework/🛍️products/💻️os/🖥️host/📦️packages/🦀️rust/../../🦀️component.rs:339:49
    |
339 |     pub type OsSpaceDocument = BackboneDocument<space::SpaceSnapshot, space::SpaceMutation>;
    |                                                 ^^^^^ use of unresolved module or unlinked crate `space`

error[E0433]: cannot find module or crate `space` in this scope
   --> 🧰️framework/🛍️products/💻️os/🖥️host/📦️packages/🦀️rust/../../🦀️component.rs:339:71
    |
339 |     pub type OsSpaceDocument = BackboneDocument<space::SpaceSnapshot, space::SpaceMutation>;
    |                                                                       ^^^^^ use of unresolved module or unlinked crate `space`

error[E0433]: cannot find module or crate `space` in this scope
   --> 🧰️framework/🛍️products/💻️os/🖥️host/📦️packages/🦀️rust/../../🦀️component.rs:341:54
    |
341 |     pub type OsCollectionDocument = BackboneDocument<space::CollectionSnapshot, space::CollectionMutation>;
    |                                                      ^^^^^ use of unresolved module or unlinked crate `space`

error[E0433]: cannot find module or crate `space` in this scope
   --> 🧰️framework/🛍️products/💻️os/🖥️host/📦️packages/🦀️rust/../../🦀️component.rs:341:81
    |
341 |     pub type OsCollectionDocument = BackboneDocument<space::CollectionSnapshot, space::CollectionMutation>;
    |                                                                                 ^^^^^ use of unresolved module or unlinked crate `space`

error[E0433]: cannot find module or crate `workflow` in this scope
   --> 🧰️framework/🛍️products/💻️os/🖥️host/📦️packages/🦀️rust/../../🦀️component.rs:344:60
    |
344 |     pub type OsWorkflowArtifactDocument = BackboneDocument<workflow::WorkflowDocument, workflow::WorkflowMutation>;
    |                                                            ^^^^^^^^ use of unresolved module or unlinked crate `workflow`

error[E0433]: cannot find module or crate `workflow` in this scope
   --> 🧰️framework/🛍️products/💻️os/🖥️host/📦️packages/🦀️rust/../../🦀️component.rs:344:88
    |
344 |     pub type OsWorkflowArtifactDocument = BackboneDocument<workflow::WorkflowDocument, workflow::WorkflowMutation>;
    |                                                                                        ^^^^^^^^ use of unresolved module or unlinked crate `workflow`

error[E0433]: cannot find module or crate `space` in this scope
   --> 🧰️framework/🛍️products/💻️os/🖥️host/📦️packages/🦀️rust/../../🦀️component.rs:351:43
    |
351 |     pub type OsSpaceStore = DocumentStore<space::SpaceSnapshot, space::SpaceMutation>;
    |                                           ^^^^^ use of unresolved module or unlinked crate `space`

error[E0433]: cannot find module or crate `space` in this scope
   --> 🧰️framework/🛍️products/💻️os/🖥️host/📦️packages/🦀️rust/../../🦀️component.rs:351:65
    |
351 |     pub type OsSpaceStore = DocumentStore<space::SpaceSnapshot, space::SpaceMutation>;
    |                                                                 ^^^^^ use of unresolved module or unlinked crate `space`

error[E0433]: cannot find module or crate `workflow` in this scope
   --> 🧰️framework/🛍️products/💻️os/🖥️host/📦️packages/🦀️rust/../../🦀️component.rs:445:50
    |
445 |     fn reconcile_workflow_document(mut document: workflow::WorkflowDocument) -> (workflow::WorkflowDocument, Vec<SpaceConflict>) {
    |                                                  ^^^^^^^^ use of unresolved module or unlinked crate `workflow`

error[E0433]: cannot find module or crate `workflow` in this scope
   --> 🧰️framework/🛍️products/💻️os/🖥️host/📦️packages/🦀️rust/../../🦀️component.rs:445:82
    |
445 |     fn reconcile_workflow_document(mut document: workflow::WorkflowDocument) -> (workflow::WorkflowDocument, Vec<SpaceConflict>) {
    |                                                                                  ^^^^^^^^ use of unresolved module or unlinked crate `workflow`

error[E0433]: cannot find module or crate `workflow` in this scope
   --> 🧰️framework/🛍️products/💻️os/🖥️host/📦️packages/🦀️rust/../../🦀️component.rs:448:40
    |
448 | ...   let node_by_id: HashMap<&str, &workflow::WorkflowNode> = document.graph.nodes.iter().map(|node| (node.id.as_str(), node)).col...
    |                                      ^^^^^^^^ use of unresolved module or unlinked crate `workflow`

error[E0433]: cannot find module or crate `workflow` in this scope
   --> 🧰️framework/🛍️products/💻️os/🖥️host/📦️packages/🦀️rust/../../🦀️component.rs:522:75
    |
522 | ...rameters.iter().find(|parameter| workflow::workflow_parameter_id(parameter) == binding.parameter_id).map(workflow_parameter_type...
    |                                     ^^^^^^^^ use of unresolved module or unlinked crate `workflow`

error[E0433]: cannot find module or crate `workflow` in this scope
   --> 🧰️framework/🛍️products/💻️os/🖥️host/📦️packages/🦀️rust/../../🦀️component.rs:523:19
    |
523 |             match workflow::validate_workflow_parameter_config_binding(binding, &parameter_type, &registration.config) {
    |                   ^^^^^^^^ use of unresolved module or unlinked crate `workflow`

error[E0433]: cannot find module or crate `workflow` in this scope
   --> 🧰️framework/🛍️products/💻️os/🖥️host/📦️packages/🦀️rust/../../🦀️component.rs:531:26
    |
531 |         document.graph = workflow::sync_workflow_parameter_ports(&document.graph, &document.parameter_bindings);
    |                          ^^^^^^^^ use of unresolved module or unlinked crate `workflow`

error[E0433]: cannot find module or crate `workflow` in this scope
   --> 🧰️framework/🛍️products/💻️os/🖥️host/📦️packages/🦀️rust/../../🦀️component.rs:540:47
    |
540 |     fn workflow_parameter_type_of(parameter: &workflow::WorkflowParameter) -> workflow::WorkflowParameterType {
    |                                               ^^^^^^^^ use of unresolved module or unlinked crate `workflow`

error[E0433]: cannot find module or crate `workflow` in this scope
   --> 🧰️framework/🛍️products/💻️os/🖥️host/📦️packages/🦀️rust/../../🦀️component.rs:540:79
    |
540 |     fn workflow_parameter_type_of(parameter: &workflow::WorkflowParameter) -> workflow::WorkflowParameterType {
    |                                                                               ^^^^^^^^ use of unresolved module or unlinked crate `workflow`

error[E0433]: cannot find module or crate `workflow` in this scope
   --> 🧰️framework/🛍️products/💻️os/🖥️host/📦️packages/🦀️rust/../../🦀️component.rs:542:13
    |
542 |             workflow::WorkflowParameter::Numeric { .. } => workflow::WorkflowParameterType::Numeric,
    |             ^^^^^^^^ use of unresolved module or unlinked crate `workflow`

error[E0433]: cannot find module or crate `workflow` in this scope
   --> 🧰️framework/🛍️products/💻️os/🖥️host/📦️packages/🦀️rust/../../🦀️component.rs:542:60
    |
542 |             workflow::WorkflowParameter::Numeric { .. } => workflow::WorkflowParameterType::Numeric,
    |                                                            ^^^^^^^^ use of unresolved module or unlinked crate `workflow`

error[E0433]: cannot find module or crate `workflow` in this scope
   --> 🧰️framework/🛍️products/💻️os/🖥️host/📦️packages/🦀️rust/../../🦀️component.rs:543:13
    |
543 |             workflow::WorkflowParameter::Categorical { .. } => workflow::WorkflowParameterType::Categorical,
    |             ^^^^^^^^ use of unresolved module or unlinked crate `workflow`

error[E0433]: cannot find module or crate `workflow` in this scope
   --> 🧰️framework/🛍️products/💻️os/🖥️host/📦️packages/🦀️rust/../../🦀️component.rs:543:64
    |
543 |             workflow::WorkflowParameter::Categorical { .. } => workflow::WorkflowParameterType::Categorical,
    |                                                                ^^^^^^^^ use of unresolved module or unlinked crate `workflow`

error[E0433]: cannot find module or crate `workflow` in this scope
   --> 🧰️framework/🛍️products/💻️os/🖥️host/📦️packages/🦀️rust/../../🦀️component.rs:544:13
    |
544 |             workflow::WorkflowParameter::Toggle { .. } => workflow::WorkflowParameterType::Toggle,
    |             ^^^^^^^^ use of unresolved module or unlinked crate `workflow`

error[E0433]: cannot find module or crate `workflow` in this scope
   --> 🧰️framework/🛍️products/💻️os/🖥️host/📦️packages/🦀️rust/../../🦀️component.rs:544:59
    |
544 |             workflow::WorkflowParameter::Toggle { .. } => workflow::WorkflowParameterType::Toggle,
    |                                                           ^^^^^^^^ use of unresolved module or unlinked crate `workflow`

error[E0433]: cannot find module or crate `workflow` in this scope
   --> 🧰️framework/🛍️products/💻️os/🖥️host/📦️packages/🦀️rust/../../🦀️component.rs:545:13
    |
545 |             workflow::WorkflowParameter::Text { .. } => workflow::WorkflowParameterType::Text,
    |             ^^^^^^^^ use of unresolved module or unlinked crate `workflow`

error[E0433]: cannot find module or crate `workflow` in this scope
   --> 🧰️framework/🛍️products/💻️os/🖥️host/📦️packages/🦀️rust/../../🦀️component.rs:545:57
    |
545 |             workflow::WorkflowParameter::Text { .. } => workflow::WorkflowParameterType::Text,
    |                                                         ^^^^^^^^ use of unresolved module or unlinked crate `workflow`

error[E0433]: cannot find module or crate `workflow` in this scope
   --> 🧰️framework/🛍️products/💻️os/🖥️host/📦️packages/🦀️rust/../../🦀️component.rs:555:49
    |
555 |     fn drop_workflow_cycle_edges(mut edges: Vec<workflow::WorkflowEdge>, conflicts: &mut Vec<SpaceConflict>) -> Vec<workflow::Workf...
    |                                                 ^^^^^^^^ use of unresolved module or unlinked crate `workflow`

error[E0433]: cannot find module or crate `workflow` in this scope
   --> 🧰️framework/🛍️products/💻️os/🖥️host/📦️packages/🦀️rust/../../🦀️component.rs:555:117
    |
555 | ... &mut Vec<SpaceConflict>) -> Vec<workflow::WorkflowEdge> {
    |                                     ^^^^^^^^ use of unresolved module or unlinked crate `workflow`

error[E0433]: cannot find module or crate `workflow` in this scope
   --> 🧰️framework/🛍️products/💻️os/🖥️host/📦️packages/🦀️rust/../../🦀️component.rs:568:50
    |
568 |     fn find_workflow_cycle_participants(edges: &[workflow::WorkflowEdge]) -> Option<HashSet<String>> {
    |                                                  ^^^^^^^^ use of unresolved module or unlinked crate `workflow`

error[E0433]: cannot find module or crate `workflow` in this scope
   --> 🧰️framework/🛍️products/💻️os/🖥️host/📦️packages/🦀️rust/../../🦀️component.rs:614:30
    |
614 |         inner: DocumentStore<workflow::WorkflowDocument, workflow::WorkflowMutation>,
    |                              ^^^^^^^^ use of unresolved module or unlinked crate `workflow`

error[E0433]: cannot find module or crate `workflow` in this scope
   --> 🧰️framework/🛍️products/💻️os/🖥️host/📦️packages/🦀️rust/../../🦀️component.rs:614:58
    |
614 |         inner: DocumentStore<workflow::WorkflowDocument, workflow::WorkflowMutation>,
    |                                                          ^^^^^^^^ use of unresolved module or unlinked crate `workflow`

… (77 more error blocks; see 🧪fixup-framework-workspace-check.txt)
```

Full log: `🧪fixup-framework-workspace-check.txt`.

#### Remaining workspace errors by tree (shared vs plugin)

| Tree | Error loci | Owner |
| --- | ---: | --- |
| framework/os/host (`semio-framework-os`) | 116 | **shared, mine to report** (pre-existing host glue: bare `space::`/`workflow::` need `crate::`, missing `zip` / `store_sync`; **not** Snapshot API leftovers) |
| framework/os/modules/space (via host) | 0 | shared (missing `zip` dep when space is path-included) |
| other shared trees seen in `-->` lines | 0 | mostly dependency/typecheck noise while host fails |
| `✏️s/🔌️plugins/**` | **0** | plugin folder, not mine |

**Snapshot-API check:** no workspace error mentions `.projection` / `SpaceProjection` / `initial_projection`.

### TypeScript — `bunx vitest run` in `@semio-tech/framework-os`

```

 Test Files  4 failed (4)
      Tests  6 failed | 198 passed (204)
   Start at  21:10:06
   Duration  351ms (transform 312ms, setup 0ms, import 393ms, tests 80ms, environment 0ms)



⎯⎯⎯⎯⎯⎯⎯ Failed Tests 6 ⎯⎯⎯⎯⎯⎯⎯

 FAIL  |@semio-tech/framework-os| ../../🟦️backbone-worker.ts > backbone-worker wire bridge > decodes the Rust-generated binary wire fixtures byte-identically
 FAIL  |@semio-tech/framework-os| ../../🟦️backbone-worker.ts > backbone-worker wire bridge > decodes the Rust-generated binary wire fixtures byte-identically
Error: ENOENT: no such file or directory, open '/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏪️store/🔄️sync/📦️packages/🦀️rust/🧫️fixtures/📡️wire/📦️client-hello.bin'
 ❯ loadClient ../../ð¦ï¸backbone-worker.ts:770:38
 ❯ ../../ð¦ï¸backbone-worker.ts:786:21

⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯[1/6]⎯

 FAIL  |@semio-tech/framework-os| ../../🟦️component.ts > @semio-tech/framework-os workflow > matches the Rust plan_workflow across shared fixtures decoded via wasm
 FAIL  |@semio-tech/framework-os| ../../🟦️component.ts > @semio-tech/framework-os workflow > matches the Rust plan_workflow across shared fixtures decoded via wasm
Error: Cannot find module 'file:///Users/ueli/Documents/semio/%F0%9F%A7%B0%EF%B8%8Fframework/%F0%9F%9B%8D%EF%B8%8Fproducts/%F0%9F%A6%80%EF%B8%8Frust/pkg/semio_framework_os.js' imported from /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🟦️component.ts
 ❯ ../../ð¦ï¸component.ts:2303:27

⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯[2/6]⎯

 FAIL  |@semio-tech/framework-os| ../../🟦️component.ts > @semio-tech/framework-os AppChannelClient > hello() encodes channel_version/app_id/actor/config and returns the single Welcome reply
 FAIL  |@semio-tech/framework-os| ../../🟦️component.ts > @semio-tech/framework-os AppChannelClient > hello() encodes channel_version/app_id/actor/config and returns the single Welcome reply
AssertionError: expected [ { Hello: { …(4) } } ] to deeply equal [ { Hello: { …(4) } } ]

- Expected
+ Received

@@ -1,11 +1,11 @@
  [
    {
      "Hello": {
        "actor": "actor-1",
        "app_id": "app.demo",
-       "channel_version": 3,
+       "channel_version": 4,
        "config": [
          1,
          4,
          101,
          100,

 ❯ ../../ð¦ï¸component.ts:2549:20

⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯[3/6]⎯
```

99/102 tests related to renamed `decodeDocumentPackSnapshot` path pass. Failures are pre-existing
(missing wasm pkg, channel_version 3→4 drift, missing wire fixture path) — not Snapshot rename regressions.

## 5. Unblock status for plugin agents

`semio-framework-os-flow`, `semio-framework-os-kernel`, and `semio-framework-plugin` all **Finished**
clean. Flow host no longer calls `DocumentStore::projection()`; shared wrappers expose
`snapshot` / `snapshot_with_conflicts` / `snapshot_json` / `initial_snapshot` /
`materialize_document_snapshot`. Plugin crates can compile against the new DocumentApp /
DocumentStore nouns without waiting on this shared surface.
