# Lowpoly Phase-8 Interactive-Job Migration Recipe

**Extraction Date:** 2026-09-03  
**Reference App:** `lowpoly` (48/48 migrated actions = 100% complete)  
**Status:** `InteractiveJobClassification::Migrated` for all actions

---

## 1. Action Chains: Two Representative Examples

### 1A. State-Mutating Action: `addPrimitive`

**1A.1 Action Declaration**  
📍 `/Users/ueli/Documents/semio/✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:285`

```rust
pub enum LowpolyCommand for LowpolySnapshot, LowpolyMutation, LowpolyConfig, LowpolyConfigMutation, ctx = LowpolyScratch {
    "addPrimitive" as "add-primitive" => add_primitive::AddPrimitive,
    // ... 47 other actions ...
}
```

**1A.2 Classification & Lane Registration**  
📍 `/Users/ueli/Documents/semio/✏️s/🔌️plugins/💠️lowpoly/🧪️interactive-job/🔣️.json:10-22`

```json
{
  "toolId": "addPrimitive",
  "classification": "Migrated",
  "lanes": ["Artifact", "Config", "Transient"],
  "preparation": ["Artifact", "Config"],
  "blocker": null
}
```

**Rationale for lanes:**
- **Artifact lane:** Emits `LowpolyMutation::CreateObject` to persist the new mesh object
- **Config lane:** Emits `LowpolyConfigMutation::SetActiveObject` to mark the new object as active
- **Transient lane:** Reaches `session::build_doc` → mutates session-local `mesh_workspace` cache (read and write)

**1A.3 Command Construction & Payload**  
📍 `/Users/ueli/Documents/semio/✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/➕️add-primitive/🦀️.rs:14-19`

```rust
#[derive(Clone, Debug, PartialEq, dsl::DslRecord, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(Serialize, Deserialize))]
#[dsl(keyword = "add-primitive")]
pub struct AddPrimitive {
    pub kind: Option<String>,  // User selects primitive kind (box, plane, cylinder, cone, ico_sphere)
}
```

**1A.4 Work Type & Job Execution**  
📍 `/Users/ueli/Documents/semio/✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:1681`

Type: `ArtifactCommandWork<EditorApp<LowpolyPlayApp>>`

Disposition: `LowpolyCommandDisposition::ArtifactConfigTransient` (enum variant 7)

**1A.5 Staging & Budget**  
📍 `/Users/ueli/Documents/semio/✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:354-361`

```rust
const LOWPOLY_RETAINED_RAW_BYTES: usize = 16_384;        // Max command payload size
const LOWPOLY_RETAINED_WORK_ITEMS: usize = 258;          // Max count of work items
const LOWPOLY_RETAINED_FIELD_BYTES: usize = 4_096;       // Max single field (id, name, etc.)
const LOWPOLY_RETAINED_OBJECTS: usize = 64;              // Max document objects
const LOWPOLY_RETAINED_PAINT_LAYERS_PER_OBJECT: usize = 8;
const LOWPOLY_RETAINED_PAINT_LAYER_BYTES: usize = 4 * 1024 * 1024;
```

Admission gate:  
📍 `/Users/ueli/Documents/semio/✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:545`

```rust
LowpolyCommand::AddPrimitive(payload) => payload.kind.as_deref().is_none_or(field),
```

**1A.6 Reducer & Handler Call**  
📍 `/Users/ueli/Documents/semio/✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:692`

```rust
LowpolyCommand::AddPrimitive(payload) => threaded!(|doc, cfg, ctx| add_primitive::handle(payload, doc, cfg, ctx)),
```

The `threaded!` macro (lines 610–620):
1. **Rehydrate:** `LowpolyScratch::from_transient(&context.transient, selection)` — rebuilds session state from persisted transient snapshot
2. **Execute:** `add_primitive::handle(payload, &doc, &cfg, &mut threaded)`
3. **Republish:** Returns `ArtifactCommandWorkStep::CompleteWithEphemeral { emit, ephemeral: EphemeralEmit { transient: vec![LowpolyTransientMutation::Snapshot { transient }] } }`

**1A.7 Handler Implementation**  
📍 `/Users/ueli/Documents/semio/✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/➕️add-primitive/🦀️.rs:21-43`

```rust
pub fn handle(
    payload: &AddPrimitive,
    doc: &ArtifactView<'_, LowpolySnapshot>,
    cfg: &ConfigView<'_, LowpolyConfig>,
    ctx: &mut LowpolyScratch
) -> Result<Emit<LowpolyMutation, LowpolyConfigMutation>, Fault> {
    let projection = doc.snapshot;
    let kind = primitive_kind(payload.kind.as_deref().unwrap_or("box")).to_string();
    let Some(mut build) = build_doc(projection, cfg.snapshot, ctx) else { 
        return Ok(Emit::default()) 
    };
    let Ok(new_id) = build.add_primitive(&kind) else { 
        return Ok(Emit::default()) 
    };
    if build.sync_meshes_to_snapshot().is_err() {
        return Ok(Emit::default());
    }
    ctx.set_mesh_workspace_map(build.mesh_workspace().clone());
    
    let Some(new_object) = build.snapshot().objects.iter().find(|object| object.id == new_id).cloned() else {
        return Ok(Emit::default());
    };
    let index = projection.objects.len();
    
    Ok(Emit {
        artifact_mutations: vec![LowpolyMutation::CreateObject(CreateObject { index, object: new_object })],
        config_mutations: vec![LowpolyConfigMutation::SetActiveObject { object_id: new_id }],
        ..Default::default()
    })
}
```

**1A.8 Resulting State Change**

- **Artifact state** (`LowpolySnapshot`): `objects` array gains new `LowpolyObject` at position `index`
- **Config state** (`LowpolyConfig`): `active_object_id` changes to `new_id`
- **Transient state** (`LowpolyTransient`): `mesh_workspace` HashMap updated with new mesh data for `new_id`

---

### 1B. Fixture-Switching Action: `setFixtureJson`

**1B.1 Action Declaration**  
📍 `/Users/ueli/Documents/semio/✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:312`

```rust
pub enum LowpolyCommand {
    "setFixtureJson" as "set-fixture-json" => set_fixture_json::SetFixtureJson,
}
```

**1B.2 Classification & Lane Registration**  
📍 `/Users/ueli/Documents/semio/✏️s/🔌️plugins/💠️lowpoly/🧪️interactive-job/🔣️.json:330-337`

```json
{
  "toolId": "setFixtureJson",
  "classification": "Migrated",
  "lanes": ["HostOnly"],
  "preparation": [],
  "blocker": null
}
```

**Rationale for HostOnly lane:**
- No artifact or config mutations: uses `Effect::LoadDocument` instead (outside undo history)
- No transient state mutations
- Purely host-side effect (whole-document replace)

**1B.3 Command Construction & Payload**  
📍 `/Users/ueli/Documents/semio/✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📄️fixture/🦀️.rs:38-46`

```rust
pub mod set_fixture_json {
    use super::*;

    #[derive(Clone, Debug, PartialEq, dsl::DslRecord, value_derive::ToValue, value_derive::FromValue)]
    #[cfg_attr(test, derive(Serialize, Deserialize))]
    #[dsl(keyword = "set-fixture-json")]
    pub struct SetFixtureJson {
        pub json: String,  // Complete lowpoly document as JSON string
    }
}
```

**1B.4 Work Type & Job Execution**  
Type: `ArtifactCommandWork<EditorApp<LowpolyPlayApp>>`  
Disposition: `LowpolyCommandDisposition::HostOnly` (enum variant 3)

**1B.5 Staging & Budget Admission**  
📍 `/Users/ueli/Documents/semio/✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:506`

```rust
LowpolyCommand::SetFixtureJson(payload) => payload.json.len() <= LOWPOLY_RETAINED_RAW_BYTES,
```

**1B.6 Reducer & Handler Call**  
📍 `/Users/ueli/Documents/semio/✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:636`

```rust
LowpolyCommand::SetFixtureJson(payload) => set_fixture_json::handle(payload, &doc, &cfg, &mut bounded),
```

No `threaded!` macro—uses blank `LowpolyScratch::default()` (no transient read/write needed)

**1B.7 Handler Implementation**  
📍 `/Users/ueli/Documents/semio/✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📄️fixture/🦀️.rs:48-50`

```rust
pub fn handle(
    payload: &SetFixtureJson,
    _doc: &ArtifactView<'_, LowpolySnapshot>,
    _cfg: &ConfigView<'_, LowpolyConfig>,
    _ctx: &mut LowpolyScratch
) -> Result<Emit<LowpolyMutation, LowpolyConfigMutation>, Fault> {
    Ok(reset_from_json(&payload.json))
}

fn reset_from_json(json: &str) -> Emit<LowpolyMutation, LowpolyConfigMutation> {
    match dsl::json::from_json_str::<LowpolySnapshot>(json) {
        Ok(parsed) => Emit { 
            effects: vec![crate::editor::lowpoly::reset_document_effect(&parsed)], 
            ..Default::default() 
        },
        Err(_) => Emit::default(),
    }
}
```

**1B.8 Resulting State Change**

- **Artifact state:** Wholesale replacement via `Effect::LoadDocument` (outside undo history)
- **Config state:** Unmodified by handler (host orchestrates reset)
- **Transient state:** Unmodified
- **Effect:** Calls `reset_document_effect` (lines 1851–1856) to build `Effect::LoadDocument { pack, spr }`

---

## 2. App-Owned Infrastructure Registration

### 2A. Retained Preparation Factories

**Artifact Preparation Factory**  
📍 `/Users/ueli/Documents/semio/✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:1599-1601`

```rust
fn build_artifact_store_one_item_preparation_factory() 
    -> Option<std::sync::Arc<dyn store::ArtifactStoreOneItemPreparationFactory<Self::Artifact, Self::ArtifactMutation>>> 
{
    Some(std::sync::Arc::new(LowpolyArtifactStorePreparationFactory))
}
```

**Type:** `LowpolyArtifactStorePreparationFactory` — implements `ArtifactStoreOneItemPreparationFactory<LowpolySnapshot, LowpolyMutation>`

**Config Preparation Factory**  
📍 `/Users/ueli/Documents/semio/✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:1603-1605`

```rust
fn build_config_store_one_item_preparation_factory() 
    -> Option<std::sync::Arc<dyn store::ArtifactStoreOneItemPreparationFactory<Self::Config, Self::ConfigMutation>>> 
{
    Some(std::sync::Arc::new(LowpolyConfigStorePreparationFactory))
}
```

**Type:** `LowpolyConfigStorePreparationFactory` — implements `ArtifactStoreOneItemPreparationFactory<LowpolyConfig, LowpolyConfigMutation>`

### 2B. Root Retirement Factory (Tool Job Factory)

**Registration Point**  
📍 `/Users/ueli/Documents/semio/✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:1665-1668`

```rust
fn register_tool_job_factories(registry: &mut ArtifactToolFactoryRegistry<'_, EditorApp<Self>>) 
    -> Result<(), Fault> 
{
    let controller = registry.controller_id().to_string();
    registry.register(LowpolyCommandJobFactory::new(&controller))
}
```

**Type:** `LowpolyCommandJobFactory` — implements `ArtifactOwnedToolJobFactory<EditorApp<LowpolyPlayApp>>`

### 2C. Retained Tool ID List

📍 `/Users/ueli/Documents/semio/✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:362-410`

```rust
pub const LOWPOLY_MIGRATED_TOOL_IDS: &[&str] = &[
    "patchObject",
    "addPaintLayer",
    "paintStrokeEnd",
    // ... 45 more ...
    "addPrimitive",
];  // 48 total actions all marked as "Migrated"
```

### 2D. App Construction Registration

**App Builder Chain**  
📍 `/Users/ueli/Documents/semio/✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:1877-1914`

```rust
pub fn create_lowpoly_app() -> semio_framework_plugin::AppDefinition {
    Editor::builder(crate::artifacts::lowpoly::LOWPOLY_DIALECT)
        .document(["semio", "lowpoly"])
        .artifact_kind(artifact_kind())
        .icon_id("shapes")
        .mode_def(edit::definition())
        .mode_def(paint_mode::definition())
        .default_mode_id(edit::LOWPOLY_PLAY_MODE_EDIT)
        .window_kind_def(edit::windows::model::definition())
        .window_kind_def(paint_mode::windows::uv::definition())
        // ... panel/option definitions ...
        .build()
}
```

**ArtifactEditor Trait Implementation**  
📍 `/Users/ueli/Documents/semio/✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:1587-1817`

Trait bounds: `ArtifactEditor<Artifact = LowpolySnapshot, ArtifactMutation = LowpolyMutation, Config = LowpolyConfig, ConfigMutation = LowpolyConfigMutation, Draft = NoDraft, DraftMutation = NoDraftMutation, Command = LowpolyCommand, Transient = LowpolyTransient>`

---

## 3. Migration Diff: `Migrated` vs `BatchOnlyPendingRewrite`

### 3A. Schema Signatures

**For `Migrated` actions:**  
📍 `/Users/ueli/Documents/semio/✏️s/🔌️plugins/💠️lowpoly/🧪️interactive-job/🔣️.schema.json:98-237`

```json
{
  "classification": "Migrated",
  "lanes": [...non-empty array],
  "preparation": [...may be empty or populated],
  "blocker": null
}
```

Valid lane/preparation pairs (8 signatures):
1. `lanes: ["Artifact"], preparation: ["Artifact"]`
2. `lanes: ["Config"], preparation: ["Config"]`
3. `lanes: ["HostOnly"], preparation: []`
4. `lanes: ["Transient"], preparation: []`
5. `lanes: ["Config", "Transient"], preparation: ["Config"]`
6. `lanes: ["Artifact", "Transient"], preparation: ["Artifact"]`
7. `lanes: ["Artifact", "Config"], preparation: ["Artifact", "Config"]`
8. `lanes: ["Artifact", "Config", "Transient"], preparation: ["Artifact", "Config"]`

**For `BatchOnlyPendingRewrite` actions:**

```json
{
  "classification": "BatchOnlyPendingRewrite",
  "lanes": [],
  "preparation": [],
  "blocker": "<blocking_reason_string>"
}
```

### 3B. Code Differences

**Migrated Action Requirements:**

1. **Command enum entry with keyword:**  
   📍 `/Users/ueli/Documents/semio/✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:280-333`
   ```rust
   "actionId" as "dsl-keyword" => module::PayloadStruct,
   ```

2. **Payload struct with DSL derive:**  
   ```rust
   #[derive(Clone, Debug, PartialEq, dsl::DslRecord, value_derive::ToValue, value_derive::FromValue)]
   #[dsl(keyword = "dsl-keyword")]
   pub struct PayloadStruct { /* fields */ }
   ```

3. **Synchronous handle function:**  
   ```rust
   pub fn handle(
       payload: &PayloadStruct,
       doc: &ArtifactView<'_, LowpolySnapshot>,
       cfg: &ConfigView<'_, LowpolyConfig>,
       ctx: &mut LowpolyScratch
   ) -> Result<Emit<LowpolyMutation, LowpolyConfigMutation>, Fault>
   ```
   Must return typed `Emit` with appropriate mutations.

4. **Disposition classifier:**  
   📍 `/Users/ueli/Documents/semio/✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:430-477`
   ```rust
   fn lowpoly_command_disposition(tool_id: &str) -> Option<LowpolyCommandDisposition> {
       Some(match tool_id {
           "actionId" => LowpolyCommandDisposition::XyzLane,
           // ...
       })
   }
   ```

5. **Budgeting admission gate:**  
   📍 `/Users/ueli/Documents/semio/✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:495-547`
   ```rust
   fn lowpoly_command_admitted(command: &LowpolyCommand, ...) -> bool {
       match command {
           LowpolyCommand::ActionId(payload) => /* budget checks */,
       }
   }
   ```

6. **Reducer arm (dispatch):**  
   📍 `/Users/ueli/Documents/semio/✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:622-699`
   ```rust
   fn lowpoly_retained_reduce(...) -> Result<ArtifactCommandWorkStep<...>, Fault> {
       let emit = match command {
           LowpolyCommand::ActionId(payload) => {
               // Either: direct call or threaded! macro (if transient lane)
               handler::handle(payload, &doc, &cfg, &mut scratch)
           }
       };
   }
   ```

7. **Sandbox in bounded_first_step_tool_proofs! macro:**  
   📍 `/Users/ueli/Documents/semio/✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:1607-1663`
   ```rust
   semio_framework_plugin::bounded_first_step_tool_proofs! {
       tools: {
           "actionId" => semio_framework::ToolExecutionContract::resumable(...),
       }
   }
   ```

8. **Interactive-job partition entry:**  
   📍 `/Users/ueli/Documents/semio/✏️s/🔌️plugins/💠️lowpoly/🧪️interactive-job/🔣️.json`
   ```json
   {
     "toolId": "actionId",
     "classification": "Migrated",
     "lanes": [...],
     "preparation": [...],
     "blocker": null
   }
   ```

---

## 4. 3D Scene Preview Render Path

### 4A. Window Definition and Surface Registration

**Window Definition**  
📍 `/Users/ueli/Documents/semio/✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🌐️model/🦀️.rs:55-80`

```rust
pub fn definition() -> WindowKindDefinition {
    let projection = crate::artifacts::lowpoly::schema::default_snapshot();
    let config = LowpolyConfig::default();
    WindowKindDefinition {
        id: LOWPOLY_PLAY_WINDOW_MAIN.into(),
        label: "Model",
        body_key: LOWPOLY_PLAY_BODY_MAIN.into(),
        surface_kind: SurfaceKind::World3d,  // <-- 3D surface
        icon_id: "lowpoly-model".into(),
        // ... interactions, utilities ...
    }
}
```

**Constants:**
- Window ID: `"lowpoly-main"` (line 15)
- Body key: `"lowpoly.play.main"` (line 16)
- Surface: `"lowpoly.play.main"` (line 17)
- Interaction domain: `"mesh"` (line 73)

### 4B. Scene JSON Construction and Geometry Flow

**Render Entry Point**  
📍 `/Users/ueli/Documents/semio/✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:1787-1801`

```rust
fn render_with_request_context(
    _owner: &semio_framework_plugin::ArtifactInstanceOperationOwnerHandle,
    body_key: &str,
    doc: &ArtifactView<'_, LowpolySnapshot>,
    cfg: &ConfigView<'_, LowpolyConfig>,
    transient: &semio_framework_plugin::TransientView<'_, LowpolyTransient>,
    _interaction: &InteractionView<'_>,
) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
    let mut scratch = LowpolyScratch::from_transient(transient.snapshot, LowpolySelection::default())?;
    lowpoly_render(body_key, doc, cfg, &mut scratch)
}
```

**Scene Construction**  
📍 `/Users/ueli/Documents/semio/✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🌐️model/🦀️.rs:99-150`

Three JSON layers:

1. **Selection State** (line 99–109):
   ```rust
   fn world_selection_json_for(view: LowpolyView<'_>, active_utility: &str) -> String {
       // Returns: { transformMode, interactionMode, activeObjectId, showEdges }
   }
   ```
   - Read from: `LowpolyConfig` (active object, edge visibility)
   - Reads from: Utility ID to determine paint vs. model mode

2. **Mesh Geometry** (line 111–126):
   ```rust
   fn world_meshes_json(doc: &LowpolyDocument, texture_cache: &HashMap<String, String>) -> String {
       let items: Vec<...> = dsl::json::from_json_str(&doc.tessellate_all_json()?)?;
       let meshes: Vec<...> = items
           .iter()
           .filter_map(|item| {
               let tessellation: serde_json::Value = item.get("tessellation")?;
               let texture = texture_cache.get(id).cloned();
               Some(dsl::DslValue::object([
                   ("id", id),
                   ("data", mesh_data_from_transfer(&tessellation, texture)),
               ]))
           })
   }
   ```
   - Read from: `LowpolyDocument::tessellate_all_json()` (engine state computed from `LowpolySnapshot`)
   - Reads mesh workspace from: `LowpolyScratch::mesh_workspace` (via `LowpolyDocument` constructor)
   - Texture cache source: `LowpolyScratch::texture_cache`

3. **Object Instances** (line 130–150):
   ```rust
   fn world_instances_json(view: LowpolyView<'_>) -> String {
       let instances: Vec<...> = view.snapshot.objects
           .iter()
           .map(|object| {
               dsl::DslValue::object([
                   ("id", object.id),
                   ("meshId", object.id),
                   ("position", object.transform.position),
                   ("rotation", euler_degrees_to_quaternion(object.transform.rotation)),
                   ("scale", object.transform.scale),
                   ("label", object.name),
                   ("smoothShading", object.smooth_shading),
               ])
           })
   }
   ```
   - Read from: `LowpolySnapshot::objects[].transform` (per-frame state)

### 4C. Geometry Entry Points

**Source State Flow:**

```
LowpolySnapshot.objects[i] 
  ↓ (contains mesh handle + transform)
LowpolyDocument::new(snapshot, transient.mesh_workspace)
  ↓ (rehydrates half-edge mesh from workspace)
doc.tessellate_all_json() 
  ↓ (GPU mesh data: vertices, faces, UVs)
mesh_data_from_transfer(&tessellation, texture)
  ↓ (bridges to renderer format)
world_meshes_json() returns [{id, data: {...geometry}}]
```

**Key Bridge Function**  
📍 `/Users/ueli/Documents/semio/✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs` (path from import in model/🦀️.rs line 5)

```rust
pub fn mesh_data_from_transfer(tessellation: &serde_json::Value, texture: Option<String>) -> DslValue {
    // Adapts { vertices, faces, normals, uvs } from tessellation
    // Binds texture handle if present
}
```

---

## 5. Testkit Conformance Assertions

### 5A. Plugin-Root Level (Viewer & Dialect Conformance)

📍 `/Users/ueli/Documents/semio/✏️s/🔌️plugins/💠️lowpoly/🦀️.rs:50-64`

```rust
#[cfg(test)]
mod surface_tests {
    use semio_framework_plugin::testkit::{
        assert_editor_and_viewer_share_dialect,
        assert_viewer_never_mutates,
    };

    #[semio_framework_async_macros::async_test]
    async fn lowpoly_viewer_never_mutates() {
        assert_viewer_never_mutates::<crate::viewer::lowpoly::LowpolyViewer>();
    }

    #[semio_framework_async_macros::async_test]
    async fn lowpoly_editor_and_viewer_share_dialect() {
        assert_editor_and_viewer_share_dialect::<
            crate::editor::lowpoly::LowpolyPlayApp,
            crate::viewer::lowpoly::LowpolyViewer
        >();
    }
}
```

These two assertions:
1. Verify that the viewer surface is read-only (never emits mutations)
2. Verify that editor and viewer share identical artifact/config dialects

### 5B. Action-Level Tests (Handler Verification)

**Pattern across all 48 actions:** Each `🎮️commands/*` handler includes `#[cfg(test)]` tests verifying:

Example from `setFixtureJson`:  
📍 `/Users/ueli/Documents/semio/✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📄️fixture/🦀️.rs:54-93`

```rust
#[semio_framework_async_macros::async_test]
async fn import_snapshot_json_replaces_the_whole_document() {
    let mesh_json = crate::artifacts::lowpoly::schema::default_mesh_workspace()["obj-1"].clone();
    let replacement = crate::artifacts::lowpoly::snapshot_from_mesh_json(&mesh_json, "obj-x", "X");
    let json = serde_json::to_string(&Into::<serde_json::Value>::into(dsl::ToValue::to_value(&replacement)))?;
    let snapshot = default_snapshot();
    let history = semio_framework_plugin::HistoryView::empty();
    let doc = ArtifactView::new(&snapshot, &history);
    let cfg_snapshot = LowpolyConfig::default();
    let cfg = ConfigView { snapshot: &cfg_snapshot };
    let mut scratch = LowpolyScratch::default();
    let emit = set_snapshot_json::handle(&set_snapshot_json::ImportSnapshotJson { json }, &doc, &cfg, &mut scratch)?;
    
    let semio_framework_plugin::Effect::LoadDocument { pack, .. } = 
        emit.effects.first().expect("importSnapshotJson must emit a LoadDocument effect") 
    else {
        panic!("expected a LoadDocument effect");
    };
    let loaded = <LowpolySnapshot as store::ArtifactPack>::decode_pack(pack)?;
    assert_eq!(loaded.objects[0].id, "obj-x");
}
```

**Testkit helpers imported:**
```rust
use crate::editor::lowpoly::testkit::{app, dispatch};
```

These provide:
- `app()` — constructs a live `EditorApp<LowpolyPlayApp>` for end-to-end testing
- `dispatch()` — routes a command through the full reduced dispatcher (not just `handle`)

---

## Recipe Summary: Per-App Wiring Checklist

### Required Infrastructure (Register at App Construction)

- [ ] **Artifact Preparation Factory** implementing `ArtifactStoreOneItemPreparationFactory<Artifact, ArtifactMutation>`
- [ ] **Config Preparation Factory** implementing `ArtifactStoreOneItemPreparationFactory<Config, ConfigMutation>`
- [ ] **Tool Job Factory** implementing `ArtifactOwnedToolJobFactory<EditorApp<YourApp>>`
- [ ] **Transient Type** if any action reads/writes mid-gesture state (e.g., paint stroke, transform drag)

### Per-Action Wiring Steps

For each action targeting Phase-8 migration:

1. **Declare payload struct** with `#[dsl(keyword = "...")]` + `ToValue`/`FromValue` derives
2. **Write `pub fn handle(payload, doc, cfg, ctx)` → `Result<Emit<Mutation, ConfigMutation>, Fault>`**
3. **Add enum variant** in `LowpolyCommand` using `app_commands!` macro
4. **Map disposition** in `lowpoly_command_disposition(tool_id)` → returns `LowpolyCommandDisposition::{Artifact|Config|HostOnly|Transient|ConfigTransient|ArtifactTransient|ArtifactConfigTransient}`
5. **Add budget gate** in `lowpoly_command_admitted(command, snapshot, config)` → `bool`
6. **Add reducer arm** in `lowpoly_retained_reduce()` match — either direct call or `threaded!` macro (if transient lane)
7. **Sandbox in `bounded_first_step_tool_proofs!`** with `ToolExecutionContract::resumable(...)`
8. **Populate interactive-job partition JSON** with `{toolId, classification: "Migrated", lanes, preparation, blocker: null}`

### 3D Preview Render Path (Per-Window)

- **Window surface kind:** `SurfaceKind::World3d`
- **Scene JSON layers:**
  - Selection: `{ transformMode, interactionMode, activeObjectId, showEdges }`
  - Meshes: `[{ id, data: {vertices, faces, normals, uvs, texture} }]` from `LowpolyDocument::tessellate_all_json()`
  - Instances: `[{ id, meshId, position, rotation, scale, label, smoothShading }]` from snapshot
- **Transient dependency:** `LowpolyScratch::mesh_workspace` (half-edge mesh cache) seeded on every `render_with_request_context` call via `LowpolyScratch::from_transient()`

### Testkit Conformance

- [ ] `assert_viewer_never_mutates::<ViewerType>()` at plugin root
- [ ] `assert_editor_and_viewer_share_dialect::<EditorType, ViewerType>()` at plugin root
- [ ] Per-action `#[async_test]` with `app()` and `dispatch()` covering happy path + edge cases

