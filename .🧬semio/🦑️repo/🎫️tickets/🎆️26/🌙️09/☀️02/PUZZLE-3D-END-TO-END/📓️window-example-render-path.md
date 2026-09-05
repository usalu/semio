# Puzzle3d Window Example Render Path — Audit Report

**Report Date:** 2026-09-05  
**Editor File mtime:** Sep 5 04:18:32 2026  
**Auditor:** Agent W (read-only)

---

## Quick Reference: "Is It Working?" Checklist

A correctly-rendering puzzle3d window shows:

✓ **Two panes open** (Top/Perspective split, 1:2 width ratio)  
✓ **Top pane** — orthographic view of the fixture, looking down  
✓ **Perspective pane** — three-point perspective view of the same fixture  
✓ **Objects visible** — 3D geometries from the fixture file, not an empty scene  
✓ **No parse errors** — console clean (DSL parse panics during boot; JSON deserialize failures are silent)

### Silent-Failure Modes (blank window, no error)

1. **DSL file missing or corrupted** — `parse_example_dsl` will PANIC at boot (caught at startup, not silent)
2. **DSL parses but JSON structure wrong** — `from_json_str(...).unwrap_or_else(|_| empty_fixture())` returns **EMPTY fixture silently** (line 103-104)
   - Window renders correctly but shows NO objects, NO attractions, NO vortices
   - Indistinguishable from a blank document by eye
3. **Fixture loaded but no layout instances** — window definition exists but instances `puzzle3d-main-top` / `puzzle3d-main-perspective` not in layout
4. **Instances rendered but fixture empty** — geometry JSON is `[]`, scene renders but geometry-less

---

## Detailed Trace

### 1. Window Definition & Layout

**File:** `./✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🧊️main/🦀️.rs`

#### Window Kind
- **ID:** `WINDOW_KIND_ID = "puzzle3d-main"` (line 33)
- **Label:** puzzle3d_localized → "Main Window" / "Hauptfenster"
- **Surface Kind:** `SurfaceKind::World3d` (line 46)
- **Body Key:** `BODY_KEY = "puzzle3d.play.composite"` (line 36)

#### Window Instances
```rust
pub const WINDOW_INSTANCE_TOP: &str = "puzzle3d-main-top";              // line 34
pub const WINDOW_INSTANCE_PERSPECTIVE: &str = "puzzle3d-main-perspective";  // line 35
```

#### Display Templates (Projection Encoding)
```rust
TEMPLATE_TOP = r#"world-projection:{"mode":{"kind":"orthographic"},"orientation":{"type":"cardinal","view":"top"}}"#
  // line 38 — Orthographic top-down view

TEMPLATE_PERSPECTIVE = r#"world-projection:{"mode":{"kind":"threePoint","fov":50},"orientation":{"type":"free"}}"#
  // line 40 — Three-point perspective, 50° FOV, free rotation
```

#### Default Layout
**File:** `./✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🦀️.rs`

```rust
fn layout() -> WindowLayout {
  WindowLayout {
    root: WindowLayoutRoot::Axis(WindowLayoutAxisNode {
      kind: "row",  // Horizontal split
      children: vec![
        WindowLayoutChild::Stack {
          size: Some(100.0 / 3.0),    // 33.33% width — LEFT pane
          children: [create_window_layout(
            main::WINDOW_KIND_ID, 
            "Top",
            main::WINDOW_INSTANCE_TOP,
            main::TEMPLATE_TOP
          )]
        },
        WindowLayoutChild::Stack {
          size: Some(200.0 / 3.0),    // 66.67% width — RIGHT pane
          children: [create_window_layout(
            main::WINDOW_KIND_ID,
            "Perspective", 
            main::WINDOW_INSTANCE_PERSPECTIVE,
            main::TEMPLATE_PERSPECTIVE
          )]
        },
      ],
    }),
  }
}
```
**Lines:** 28-44, edit/🦀️.rs

**Test coverage:** `default_layout_is_top_left_third_and_perspective_right_two_thirds()` at lines 48-66 confirms both panes and dimensions.

---

### 2. How Windows Get Content: Fixture → Scene

#### App Creation Entry Point
**File:** `./✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs`

```rust
pub fn create_puzzle3d_app() -> AppDefinition {
    let envelope = Puzzle3dScene {
        fixture: default_fixture(),              // ← Initial fixture loaded here
        runtime: Puzzle3dRuntime::default(),
        active_utility: PUZZLE3D_DEFAULT_UTILITY.into(),
    };
    // ... define window kinds, modes, panels ...
    .default_layout(edit::layout())
    // ...
}
```
**Line:** 7157

#### Default Fixture Selection
**File:** Same file

```rust
pub fn default_fixture() -> Puzzle3dFixture {
    CONCRETE_FOREST_EXAMPLE_FIXTURE.clone()    // line 280
}

pub fn empty_fixture() -> Puzzle3dFixture {
    Puzzle3dFixture {
        schema: PUZZLE3D_FIXTURE_SCHEMA,
        domain: "architecture",
        meta: Puzzle3dFixtureMeta::default(),
        objects: Vec::new(),           // ← NO objects
        attractions: Vec::new(),
        target_volumes: Vec::new(),
        references: Vec::new(),
    }
}
```
**Lines:** 275-281

**Key point:** At boot, `default_fixture()` returns the concrete-forest example. This is the **initial state**. No dev flag or parameter overrides it yet (see §5 below).

#### Window Rendering Path: Fixture → Scene JSON
**File:** `./✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🧊️main/🦀️.rs`

The window's `render()` function (line 537) converts a `Puzzle3dScene.fixture` into JSON payloads:

```rust
pub fn render(
    envelope: &Puzzle3dScene,
    precompute: &Puzzle3dPrecomputeSession,
    labels: &Puzzle3dLabels,
    instances_json: String,    // From fixture geometry
    meshes_json: String,       // Mesh definitions
) -> UiAssemblyResult<BuiltNode> {
    let scene = world3d_scene_extended(
        camera_json(&envelope.runtime),
        meshes_json,
        instances_json,            // ← Comes from fixture.objects
        world_selection_json(...),
        Some(world_vortices_json(&envelope.fixture, ...)),     // ← Comes from fixture.objects[*].vortices
        Some(world_attractions_json(&envelope.fixture)),       // ← From fixture.attractions
        Some(world_target_volumes_json(&envelope.fixture)),    // ← From fixture.target_volumes
        Some(world_references_json(&envelope.fixture)),        // ← From fixture.references
        brush_preview,
        Some(world_interaction_json(...)),
        // ... other scene properties ...
    );
    semio_framework_plugin::scene_surface(SURFACE_VIEWPORT, World3d, &scene)
}
```
**Lines:** 537–620

**Critical data flow:**
- `world_instances_geometry_json(fixture)` @ line 361 → objects array → mesh instances rendered
- `world_vortices_json(fixture, runtime)` @ line 445 → vortex markers (if `PUZZLE3D_VORTEX_SHOW_ALWAYS`)
- `world_attractions_json(fixture)` @ line 481 → lines between vortices
- `world_target_volumes_json(fixture)` @ line 500 → target zones

**If fixture is empty:** all four return `"[]"`, scene renders successfully but empty.

---

### 3. Example Loading & DSL-to-JSON Pipeline

#### Example Constants
**File:** `./✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs`

```rust
pub const PUZZLE3D_EXAMPLE_CONCRETE_FOREST: &str = "concrete-forest";       // line 65
pub const PUZZLE3D_EXAMPLE_NAKAGIN: &str = "nakagin-capsule-tower";         // line 66

pub static CONCRETE_FOREST_EXAMPLE_JSON: LazyLock<String> = LazyLock::new(||
    parse_example_dsl(
        crate::artifacts::puzzle3d::dsl::PUZZLE3D_CONCRETE_FOREST_EXAMPLE_TEXT,
        "concrete-forest"
    )
);  // line 101

pub static NAKAGIN_EXAMPLE_JSON: LazyLock<String> = LazyLock::new(||
    parse_example_dsl(
        crate::artifacts::puzzle3d::dsl::PUZZLE3D_NAKAGIN_EXAMPLE_TEXT,
        "nakagin"
    )
);  // line 102

static CONCRETE_FOREST_EXAMPLE_FIXTURE: LazyLock<Puzzle3dFixture> =
    LazyLock::new(||
        from_json_str(CONCRETE_FOREST_EXAMPLE_JSON.as_str())
            .unwrap_or_else(|_| empty_fixture())  // ← SILENT FAILURE HERE
    );  // line 103

static NAKAGIN_EXAMPLE_FIXTURE: LazyLock<Puzzle3dFixture> =
    LazyLock::new(||
        from_json_str(NAKAGIN_EXAMPLE_JSON.as_str())
            .unwrap_or_else(|_| empty_fixture())  // ← SILENT FAILURE HERE
    );  // line 104
```

#### DSL Text Assets
**File:** `./✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📝️text/🦀️.rs`

```rust
pub const PUZZLE3D_CONCRETE_FOREST_EXAMPLE_TEXT: &str =
    include_str!("../../../📚️examples/🌲️concrete-forest/🖼️assets/🧪️forest/🗣️.dsl.semio");

pub const PUZZLE3D_NAKAGIN_EXAMPLE_TEXT: &str =
    include_str!("../../../📚️examples/🏗️nakagin-capsule-tower/🖼️assets/🧪️tower/🗣️.dsl.semio");
```

**Absolute paths:**
- Concrete Forest: `./✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🌲️concrete-forest/🖼️assets/🧪️forest/🗣️.dsl.semio`
- Nakagin: `./✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🏗️nakagin-capsule-tower/🖼️assets/🧪️tower/🗣️.dsl.semio`

#### DSL Parse → JSON Conversion
**File:** `./✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs`

```rust
fn parse_example_dsl(dsl_text: &str, label: &str) -> String {
    let projection = <Puzzle3dSnapshot as store::ArtifactDsl>::parse_dsl(dsl_text)
        .unwrap_or_else(|error|
            panic!("{label} example fixture parses as dsl: {error}")  // ← PANICS on DSL error
        );
    dsl::json::to_json_string(&projection)
}
```
**Lines:** 107–110

**Failure modes:**
1. **DSL parse failure** → `panic!()` → app fails to boot, error visible in console
2. **DSL to JSON succeeds** → returns valid JSON string
3. **JSON → Puzzle3dFixture deserialization fails** → `from_json_str` at line 103-104 returns `empty_fixture()` silently

### 4. Initial Snapshot & Default State

**File:** `./✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs`

```rust
fn initial_snapshot() -> Puzzle3dPlaySnapshot {
    LazyLock::force(&NAKAGIN_EXAMPLE_FIXTURE);  // Force fixture init
    LazyLock::force(&PUZZLE3D_EXAMPLE_OPERATIONS);
    let snapshot = Puzzle3dPlaySnapshot::new(
        (&dsl::ToValue::to_value(&default_fixture())).into()
    );  // ← Uses default_fixture() = CONCRETE_FOREST
    let config = Puzzle3dConfig::default();
    let active_utility = puzzle3d_scene_active_utility(&config, None);
    let scene = scene_from_projection(&puzzle3d_projection_value(snapshot.value()), config, &active_utility);
    let mut app = Puzzle3dPlayApp::default();
    sync_precompute_session(&mut app.precompute.borrow_mut(), &scene);
    snapshot
}
```
**Lines:** 6836–6847

**Test confirmation:**
```rust
#[test]
async fn initial_snapshot_is_the_concrete_forest_fixture() {
    let app = app();
    assert_eq!(projection_of(&app).get("schema").and_then(|value| value.as_str()),
               Some(PUZZLE3D_FIXTURE_SCHEMA));
    assert!(object_count(&app) > 0, "the concrete-forest default fixture ships with objects");
}
```
**Lines:** 8558–8562

**Key point:** At first boot, the app **always** loads with `CONCRETE_FOREST_EXAMPLE_FIXTURE`. No environment variable or initialization parameter changes this yet.

---

### 5. Runtime Example Switching

#### setActiveExample Action
**File:** `./✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🛍️set-active-example/🦀️.rs`

```rust
pub fn set_active_example(ctx: &mut Puzzle3dActionCtx<'_>, args: Option<&Value>) {
    let example_id = args
        .and_then(|value| value.get("exampleId"))
        .and_then(|value| value.as_str())
        .unwrap_or("");

    let next = if example_id.is_empty() {
        Some(empty_fixture())
    } else if example_id == PUZZLE3D_EXAMPLE_CONCRETE_FOREST || example_id == "concrete" {
        Some(default_fixture())
    } else if example_id == PUZZLE3D_EXAMPLE_NAKAGIN || example_id == "nakagin" {
        Some(nakagin_fixture())
    } else {
        None
    };

    if let Some(fixture) = next {
        ctx.scene.fixture = fixture;
        ctx.scene.runtime = Puzzle3dRuntime::default();  // Reset runtime state
    }
    resolve_puzzle3d_attractions(&mut ctx.scene.fixture);
}
```

**Accepted example IDs:**
- `""` (empty string) → `empty_fixture()` (blank scene)
- `"concrete-forest"` or `"concrete"` → `default_fixture()` (CONCRETE_FOREST_EXAMPLE_FIXTURE)
- `"nakagin-capsule-tower"` or `"nakagin"` → `nakagin_fixture()` (NAKAGIN_EXAMPLE_FIXTURE)
- Anything else → no change (None)

**Effect:** Fixture is replaced in the running scene; runtime is reset (camera, selections, etc.). Windows re-render with new fixture data in their next frame.

---

### 6. Dev Startup Flags

#### Package.json Commands
**File:** `./package.json`

```json
{
  "scripts": {
    "dev:puzzle:3d": "bun ./📜️script.ts dev 3d",
    "dev:puzzle:3d:concrete-forest": "bun ./📜️script.ts dev 3d fixture concrete"
  }
}
```
**Lines:** 79–80

#### Launch Configuration
**File:** `./.vscode/launch.json`

```json
{
  "name": "🛠️dev🧩️puzzle🏙️3d⚛️react",
  "command": "bun run dev:puzzle:3d"
},
{
  "name": "🛠️dev🧩️puzzle🏙️3d🎛️concrete🌲️forest⚛️react",
  "command": "bun run dev:puzzle:3d:concrete-forest"
}
```
**Lines:** 1297–1351 (approximately)

#### Script Dispatch
Both commands route through `./📜️script.ts dev <args>`, which dispatches to the framework OS dev host. The "fixture concrete" argument is consumed by that framework layer, but **I found no integration path that reads it back and passes it to `create_puzzle3d_app()`**.

**Status:** The dev flag infrastructure exists, but initial fixture selection from `dev 3d fixture concrete` is **not yet implemented**. The app boots with `default_fixture()` (concrete-forest) regardless of the dev command variant used. Switching examples at runtime requires the `setActiveExample` action (panel/menu action, not startup parameter).

---

## Summary: Render Path for a "Working" Example Window

### Boot Sequence
1. App calls `create_puzzle3d_app()`
2. `create_puzzle3d_app()` calls `default_fixture()` → returns `CONCRETE_FOREST_EXAMPLE_FIXTURE`
   - Concrete-forest fixture DSL was parsed once at compile time via `include_str!` + `LazyLock`
   - If DSL parse fails: **panic at boot** (visible error)
   - If JSON deserialize fails: **returns `empty_fixture()` silently** (blank scene, no error)
3. `initial_snapshot()` wraps that fixture
4. Default layout opens two window instances: `puzzle3d-main-top` (33% left) and `puzzle3d-main-perspective` (67% right)
5. Each window calls `render()` with the shared `Puzzle3dScene.fixture`
6. `render()` emits geometry/vortex/attraction/target-volume JSON from fixture arrays
7. World3d host renders the scene

### What a "Working" Window Contains
- **Objects:** 3D meshes from `fixture.objects` array, positioned/oriented/scaled per JSON
- **Vortices:** Arrows (if `PUZZLE3D_VORTEX_SHOW_ALWAYS`) from `fixture.objects[*].vortices`
- **Attractions:** Lines from `fixture.attractions` (vortex-to-vortex connections)
- **Target Volumes:** Pink boxes from `fixture.target_volumes`
- **References:** Images from `fixture.references` (background references)

### Silent Failure: The Empty Fixture Trap
If the JSON produced by `parse_example_dsl` doesn't deserialize to a valid `Puzzle3dFixture` struct, the app will:
1. Silently fall back to `empty_fixture()` (line 103-104)
2. Boot without error
3. Render two panes that are **completely empty** (no objects, no grid marks, no errors)
4. Appear to be a fully functional but blank workspace

**Acceptance Test:** Confirm objects are visible in both panes, not just blank scenes.

---

## Files to Monitor for Changes

- `./✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs` — app lifecycle, example loading
- `./✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🦀️.rs` — layout definition
- `./✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🧊️main/🦀️.rs` — window rendering, scene JSON
- `./✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/` — both DSL fixture files (must be present and syntactically valid)

