---
name: Generalize Compose Rust VCS
overview: Rework the shared Rust VCS engine so every command, checkpoint, alternative, and operation is a real typed Rust enum/struct (no serde_json::Value dispatch, no whole-projection JSON replace), and put the backbone (dev JSON file, local sqlite folder, remote server) fully behind one trait so no caller — Rust or TypeScript — knows which storage implementation is active.
todos:
 - id: phase0-tests
   content: Extend semios spawn test to assert materialized projections for every technology; fix forms.dictionary/v1 vs forms.form/v1 mismatch
   status: completed
 - id: phase1-framework-rs
   content: "Rework framework/rs into a truly generic, typesafe engine: DocumentVcsStore<P, Op, A: ApplyOp<P, Op>> with no Value/JsonReplaceOp anywhere; refactor compose/client/lib/rs to consume it for its typed Operation enum"
   status: completed
 - id: phase1b-backbone-trait
   content: Design and implement the Backbone trait (attach/load/sync over opaque bytes) with DevJsonFileBackbone, SqliteFolderBackbone, RemoteHttpBackbone implementations and a resolve_backbone(uri) factory so callers only ever pass a URI
   status: pending
 - id: phase2-semios-rs
   content: Replace semios_studio's Value-based apply_studio_operation with a typed StudioOp enum and exhaustive match; StudioStore takes a backbone URI and never touches a concrete backbone type
   status: completed
 - id: phase3-existing-rust-tech
   content: Add typed Op enums (mirroring each technology's existing TS *EditOp union) plus checkpoint/alternative/undo/redo to raster/rs, writer/rs, puzzle/2d/rs, puzzle/3d/rs, gis/map/rs, flow/core, dag, trinity, reasoning/mindmap; replace their TS reducers with thin WASM clients
   status: completed
 - id: phase4-new-rust-tech
   content: Give draw/rs, forms/rs, shooting/rs, cad/rs, presentation/rs real typed Op enums ported 1:1 from their existing TS *EditOp unions (no placeholder empty projections, no JsonReplaceOp)
   status: completed
 - id: phase5-retire-ts-vcs
   content: Retire framework/core's DocumentVcsStore/AppVcsRegistry/JsonReplaceOp TypeScript implementations once all technologies are Rust-backed with typed ops
   status: completed
 - id: phase6-regression
   content: Run cargo test + WASM builds across all crates, re-run Phase 0 projection tests, and manually verify dev:semios parity (including backbone swap) across all technologies
   status: completed
isProject: false
---

# Generalize Compose's Rust Architecture to All of Semios (Typesafe Revision)

## Why this revision

The first pass built a shared engine (`framework/rs`) and per-technology Rust crates, but it cut a corner to move fast: the engine operates on `serde_json::Value` and technologies without a ported typed op enum fall back to a single `"replaceProjection"` whole-document swap. Concretely:

```26:36:framework/rs/lib.rs
pub struct DocumentChange {
    pub id: String,
    pub forwards: Vec<Value>,
    pub backwards: Vec<Value>,
    ...
}
```

```125:152:framework/rs/lib.rs
pub trait ApplyDocumentOp: Send + Sync {
    fn apply(&self, projection: &Value, operation: &Value) -> Result<Value, VcsError>;
}

pub struct JsonReplaceApplier;
impl ApplyDocumentOp for JsonReplaceApplier {
    fn apply(&self, _projection: &Value, operation: &Value) -> Result<Value, VcsError> {
        let projection = operation.get("projection").cloned()...
        Ok(projection)
    }
}
```

And `semios/rs/lib.rs`'s command dispatch is stringly-typed rather than a real enum:

```151:246:semios/rs/lib.rs
fn apply_studio_operation(projection: &SemiosStudioProjection, operation: &Value) -> SemiosStudioProjection {
    let op = operation.get("op").and_then(|v| v.as_str()).unwrap_or("");
    let payload = operation.get("payload").cloned().unwrap_or(json!({}));
    match op {
        "setActiveProgram" => { ... payload.get("programId").and_then(|v| v.as_str()) ... }
        "spawnAppInstance" => { ... }
        ...
        _ => {}
    }
}
```

This is not typesafe: unknown op strings silently no-op (the `_ => {}` arm), field access is `.get("...")` string lookups with runtime fallbacks instead of compiler-checked struct fields, and every technology without a real op enum degrades to swapping the entire document on every change (defeating the point of a change/edit log).

The backbone is closer to right — there is already a `BackbonePort` trait — but it is not the single interface a technology or the TS layer talks to. `DevJsonBackbone` and `RemoteJsonBackbone` are separate concrete types the caller must choose between, there is no sqlite-backed implementation, and there is no URI-scheme resolver, so "which backbone" is a decision every call site has to make instead of the storage layer.

This revision fixes both issues before continuing the rollout to more technologies.

## Target design

### 1. Fully generic, typed VCS engine (no `Value`)

`framework/rs` becomes generic over a projection type `P` and an operation type `Op`, both `Serialize + DeserializeOwned + Clone`:

```rust
pub struct DocumentChange<Op> {
    pub id: String,
    pub forwards: Vec<Op>,
    pub backwards: Vec<Op>,
    pub description: Option<String>,
    pub saved_at: Option<String>,
}

pub struct DocumentVcs<P, Op> {
    pub initial_projection: P,
    pub operations: Vec<DocumentChange<Op>>,
    pub checkpoints: Vec<DocumentCheckpoint>,
    pub alternatives: Vec<DocumentAlternative>,
}

pub struct DocumentVcsEnvelope<P, Op> {
    pub schema: String,
    pub id: String,
    pub vcs: DocumentVcs<P, Op>,
    pub backbone: Option<DocumentBackboneRef>,
}

pub enum DocumentVcsCommand<Op> {
    Apply { forwards: Vec<Op>, backwards: Vec<Op>, description: Option<String> },
    Undo,
    Redo,
    CommitCheckpoint { message: Option<String> },
    CreateAlternative { name: String },
    SwitchAlternative { alternative_id: String },
}

pub trait ApplyOp<P, Op> {
    fn apply(&self, projection: &P, operation: &Op) -> Result<P, VcsError>;
}

pub struct DocumentVcsStore<P, Op, A: ApplyOp<P, Op>> {
    envelope: DocumentVcsEnvelope<P, Op>,
    applier: A,
    applied_change_ids: Vec<String>,
    redo_change_ids: Vec<String>,
    generation: u64,
}
```

`JsonReplaceApplier`, `apply_json_replace_op`, `json_replace_op`, and `JsonDocumentStore` are deleted entirely — there is no generic "replace the whole projection" escape hatch. Every technology implements a real `ApplyOp<TechProjection, TechOp>` with an exhaustive `match` over its own op enum, the same shape `draw/core` already has in TypeScript:

```232:254:draw/core/index.ts
export type DrawEditOp =
	| { readonly op: "setLayerVisible"; readonly layerId: string; readonly visible: boolean }
	| { readonly op: "setLayerLocked"; readonly layerId: string; readonly locked: boolean }
	...
	| { readonly op: "setCamera"; readonly camera: DrawCamera };
```

`draw/rs` gets the literal Rust equivalent (`enum DrawOp { SetLayerVisible { layer_id: String, visible: bool }, ... }`) instead of the current placeholder `empty_draw_projection()` + JSON replace.

`wasm-bindgen` cannot export generic types, so each technology still exposes a concrete, monomorphized WASM struct (`DrawDocumentVcs`, `RasterDocumentVcs`, ...). The WASM boundary still carries JSON strings (unavoidable for any WASM/JS boundary), but the Rust side immediately `serde_json::from_str::<DrawOp>(...)`-decodes into the real enum and rejects anything that doesn't match a variant — the typesafety guarantee is that the _op application logic_ is a compiler-checked match, not that bytes never cross the FFI boundary as JSON.

### 2. Backbone fully behind one interface

```rust
pub trait Backbone: Send + Sync {
    fn attach(&mut self, uri: &str) -> Result<(), VcsError>;
    fn load(&self) -> Result<String, VcsError>;   // opaque envelope bytes
    fn sync(&self, envelope_json: &str) -> Result<(), VcsError>;
}

pub fn resolve_backbone(uri: &str) -> Result<Box<dyn Backbone>, VcsError> {
    match uri.split("://").next() {
        Some("dev") => Ok(Box::new(DevJsonFileBackbone::new(uri))),
        Some("local" | "sqlite") => Ok(Box::new(SqliteFolderBackbone::new(uri)?)),
        Some("remote" | "http" | "https") => Ok(Box::new(RemoteHttpBackbone::new(uri))),
        _ => Err(VcsError::Backbone(format!("unsupported backbone uri: {uri}"))),
    }
}
```

The trait operates on opaque serialized bytes (the envelope's own `serde_json::to_string`), not typed structs — that is the correct boundary: a backbone's job is storage/transport, not domain logic, so it never needs to know `P`/`Op` and can be `Box<dyn Backbone>` without generics leaking into it. This is different from the `ApplyDocumentOp`-on-`Value` problem above, where domain operation semantics (not storage) were erased to JSON.

`DocumentVcsStore` (and `StudioStore`) take a backbone URI string at construction, call `resolve_backbone` internally, and expose `sync()`/`load()` methods that hide which concrete implementation is active. Callers — technology crates, `semios/core`, and the TypeScript WASM-client wrappers — only ever see the URI and the store's own `sync`/`load` methods, never `DevJsonBackbone`/`SqliteFolderBackbone`/`RemoteHttpBackbone` directly. `SqliteFolderBackbone` is new (native-only, mirrors compose's existing `kit_backbone` SQLite path at `compose/client/lib/rs/lib.rs:11254-13697`); `RemoteHttpBackbone` replaces today's `RemoteJsonBackbone` stub with an implementation that can actually reach a server (e.g. the same HTTP contract as `compose-store`).

### 3. Semios StudioOp becomes a real enum

```rust
#[derive(Serialize, Deserialize, Clone)]
#[serde(tag = "op", rename_all = "camelCase")]
pub enum StudioOp {
    SpawnAppInstance { instance: SemiosAppInstance, position: MediaGraphPosition },
    RemoveAppInstance { instance_id: String },
    ConnectMediaPorts { edge: SemiosMediaGraphEdge },
    DisconnectMediaEdge { edge_id: String },
    MoveMediaNode { node_id: String, x: f64, y: f64 },
    PatchAppSource { instance_id: String, inline: String },
    ApplyAppOperation { instance_id: String, next_source: SemiosSourceDocument },
    SetActiveProgram { program_id: Option<String> },
    SetActiveAlternative { alternative_id: Option<String> },
}
```

`apply_studio_operation` becomes an exhaustive `match op { StudioOp::SpawnAppInstance { instance, position } => ..., ... }` with no `_ => {}` catch-all and no `.get("field").and_then(...)` — a typo'd or missing field is a compile error, not a silent no-op.

## Phases

### Phase 0 (done)

Test baseline already extended to materialize every spawned technology's projection; no rework needed.

### Phase 1 — Rework `framework/rs` to be generic and typed

- Replace the `Value`-based `DocumentChange`/`DocumentVcs`/`DocumentVcsEnvelope`/`DocumentVcsCommand`/`ApplyDocumentOp`/`DocumentVcsStore` with the generic `<P, Op>` versions above.
- Delete `JsonReplaceApplier`, `apply_json_replace_op`, `json_replace_op`, `JsonDocumentStore`.
- Refactor `compose/client/lib/rs` to instantiate the generic engine with its own real `Operation` type (it already has one — `compose/client/lib/rs/lib.rs`'s `operation` module) instead of the ad hoc integration test added previously.

### Phase 1b — Backbone trait

- Implement `Backbone` trait, `resolve_backbone(uri)`, and three implementations: `DevJsonFileBackbone`, `SqliteFolderBackbone` (native-only, sqlite-per-folder), `RemoteHttpBackbone`.
- Wire `DocumentVcsStore`/`StudioStore` to take a URI and call `resolve_backbone` internally; delete direct construction of `DevJsonBackbone`/`RemoteJsonBackbone` from any call site outside this module.

### Phase 2 — Typed `StudioOp` in `semios/rs`

- Replace `apply_studio_operation`'s `Value` dispatch with the `StudioOp` enum and exhaustive match shown above.
- `StudioStore::new` takes a backbone URI (from the studio document's existing `backbone` field) rather than nothing; `sync`/`load` go through the trait.

### Phase 3 — Typed ops for technologies with an existing Rust crate

For `raster`, `writer`, `puzzle.2d`, `puzzle.3d`, `gis.map`, `flow`, `dag`, `trinity`, `reasoning.wires`:

- Port each technology's existing TS `*EditOp` union (e.g. `raster/core`'s edit op type) into a Rust enum with matching variants.
- Implement `ApplyOp<TechProjection, TechOp>` with a real match, replacing any placeholder/JSON-replace behavior left over from the first pass.
- Extend the technology's WASM struct with `dispatch`/`undo`/`redo`/`commitCheckpoint`/`createAlternative`/`switchAlternative`/`projectionJson`, decoding incoming op JSON into the typed enum.

### Phase 4 — Typed ops for the newly created crates

For `draw`, `forms`, `shooting`, `cad`, `presentation`:

- Replace each crate's placeholder `empty_*_projection()` + `JsonDocumentStore` with a real projection struct and op enum ported 1:1 from the technology's existing TS domain model (e.g. `draw/core`'s `DrawDocument`/`DrawEditOp` shown above), and a real `ApplyOp` implementation mirroring the TS `applyDrawEditOp`-style reducer.

### Phase 5 — Retire the TypeScript generic VCS layer

- Once every technology has a typed Rust `ApplyOp` and WASM session, remove `framework/core`'s TypeScript `DocumentVcsStore`/`materializeDocumentProjection`/`JsonReplaceOp`/`AppVcsRegistry` (the sync mirror added in the first pass) entirely — TypeScript keeps only thin WASM-client wrappers, matching how `compose/js` relates to `compose/rs` today.

### Phase 6 — Regression

- `cargo test` across every crate (typed op round-trips, checkpoint/alternative, backbone swap between `dev://`, `local://`/sqlite, and a mocked `remote://`).
- Re-run Phase 0's per-technology materialization tests against the typed engine.
- Boot `dev:semios` and manually verify undo/redo/checkpoint/alternatives, plus swapping a document's backbone URI without any TS or Rust call site needing to change.

## Execution note

This is a rework of already-built infrastructure, not new scaffolding, but touches every technology crate again to remove the JSON-replace shortcut. It will be executed phase by phase with `cargo test` run after each phase before moving to the next.
