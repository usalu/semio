---
name: Plan
overview: ""
todos: []
isProject: false
---

---

name: rs kit store materialization
overview: ""
todos:

- id: kitdiff_apply
  content: Port the canonical `KitDiff` family of types (sparse partials with `removed`/`updated`/`added` triples on every collection, recursive on Type/Design/Representation/Connector/Piece/Connection/Tag/Concept/File/Folder/Author/Attribute/Port) into the rs `operation` region with serde matching `compose/asset/compose/metabolism.kit.diff.compose.json`, and implement `Kit::apply_diff` as the single central mutation entry point; remove every per-field mutator helper on `Kit`/`Design`/`Piece`/...
  status: pending
- id: kit_deep_clone
  content: Implement `Kit::deep_clone() -> Arc<Kit>` walking Type/Representation/Connector/Design/Piece/Tag/Concept/Quality/Author/File/Folder/Prop/Attribute/Stat and rebuilding `*_by_id` weak maps.
  status: pending
  - id: scope_input_enums
    content: "Define a single shared `operation::Scope` enum with one variant per distinct id-shape used across the 15 commands (`Kit`, `Entity { entity_id }`, `Tag { tag_id }`, `Tags { tag_ids }`, `CreateTag { owner_id, tag_id, attribute_ids }`, `CreateTags { owner_id, tag_ids, attribute_ids }`, `CreateConcept { owner_id, concept_id, attribute_ids }`, `CreateQuality { owner_id, quality_id, attribute_ids, benchmark_ids }`, `CreateFixedPiece { design_id, piece_id, blueprint_id, attribute_ids }`, `PieceInDesign { design_id, piece_id }`, `PiecesInDesign { design_id, piece_ids }`, ...) — each variant carries every `Id` the operation references (target / owner / system-minted-on-creation). And a single shared `operation::Input` enum with one variant per distinct non-id payload shape (`None`, `Name { name }`, `Description { description }`, `Icon { icon }`, `Image { image }`, `Tag { tag }`, `Tags { tags }`, `Concept { concept }`, `Quality { quality }`, `FixedPiece { position, name, description }`, `Offset { offset }`, ...) — operations sharing a payload shape reuse the same variant (e.g. `RenameKit` and `RenameTag` both use `Input::Name`)."
    status: pending
  - id: kitop_enum
    content: "Define one-way `operation::KitOperation` with one variant per existing Command. Every variant is uniformly shaped as `{ scope: Scope, input: Input }` reusing both shared enums. The variant name encodes the operation kind; the (Scope variant, Input variant) pairing is fixed and runtime-validated inside `to_diff` / `to_backwards`. The user-facing GraphQL command never accepts an entity id for creations; the worker mints them into the matching `Scope` variant at record time. Two pure read methods per variant; `to_diff(&Arc<Kit>) -> KitDiff` (uses ids from `scope` to populate `KitDiff.added[*].id`) and `to_backwards(&Arc<Kit>) -> Vec<KitOperation>` (returns ordered forward-intent operations that undo the operation; deletions construct a fresh `Scope` reusing prior ids from `pre`). No undo state on variants; no separate inverse helper."
    status: pending
- id: vcs_root_freeze
  content: Change `Checkpoint.root` to immutable `frozen_root Arc<Kit>`; add `Draft.change_seq`; remove `Graph.the_kit` in favour of `parent_root_for_active_draft` + cached `materialized_kit()` (deep-clone + replay via operation->diff->apply_diff) + `record_op_in_open_transaction()`.
  status: pending
  - id: worker_rewrite
    content: Rewrite `worker::ChildRuntime::apply` so every command captures `before_kit`, mints all required ids via `Id::new()` for any entities the command will create (placed directly into the matching `Scope` variant alongside any owner / target ids the user supplied), builds the forward `KitOperation::<Variant> { scope: Scope::<…> { … }, input: Input::<…> { … } }`, derives backwards via `forward.to_backwards(&before_kit)`, records both onto the open transaction's `Change` (extends `Change.forwards` with the forward operation and `Change.backwards` with the returned `Vec`), invalidates the materialization cache, then emits the existing `OperationKind` events from the freshly materialized kit. The worker never mutates a `Kit` directly.
    status: pending
- id: resolvers_switch
  content: Switch every `graph.the_kit.*` access in `gql`, `iface`, `kit_backbone`, and `kit_graph_engine` to `graph.materialized_kit().await`.
  status: pending
- id: abort_via_invalidation
  content: Make `Graph::abort_transaction` simply drop the open transaction's `Change` list and invalidate `materialized_cache`; the next `materialized_kit()` re-replays only the surviving finalized operations via operation->diff->apply_diff. `Change.backwards` is preserved on disk for explicit undo/redo flows.
  status: pending
- id: tests_update
  content: Update existing tests that reach into `g.the_kit` and add new assertions; root immutability after rename, abort restores prior materialized state, and `Kit::apply_diff` is the only mutation entry point (grep guard test).
  status: pending
- id: ticket
  content: Open MCP ticket 'Refactor RS Kit Store To Materialized Reads', close with summary at the end.
  status: pending
  isProject: false

---

# Refactor `compose/rs` Kit Store Read/Write to Pure Materialization

## Problem (current state)

In [compose/rs/lib.rs](compose/rs/lib.rs):

- `Graph.the_kit: Arc<Kit>` is a single live mutable instance.
- `Checkpoint.root` stores the **same** `Arc<Kit>` (see `ensure_default_seed_state` at line 3618: `root: RwLock::new(Some(self.the_kit.clone()))`), so root and live state are aliased.
- `worker::ChildRuntime::apply` (lines 6097–6206) mutates `the_kit` directly for every command (e.g. `*self.graph.the_kit.name.write().await = name.clone();` at line 6129).
- `Change.forwards` / `Change.backwards` (lines 3110–3111), `Transaction.changes`, `Draft.transactions` exist but the worker never appends to them.

Result: opening a dev kit and renaming it visibly mutates `Checkpoint.root.name` — the bug the user reported.

## Target architecture

Two-stage pipeline: **operations are intent data**, **diffs are state-transition descriptions**, and **a single central `Kit::apply_diff` is the only thing that mutates a Kit**. Operations themselves never touch a `Kit`.

```mermaid
flowchart LR
  RootKit["Checkpoint.frozen_root<br/>Arc<Kit> (immutable)"] -->|deep_clone| Scratch["scratch Arc<Kit>"]
  Scratch -->|"replay: operation.to_diff(scratch); apply_diff(scratch, diff)"| Materialized["Graph.materialized_kit()<br/>Arc<Kit> (cached per draft state)"]
  OpenTx["Transaction.changes<br/>Change.forwards: Vec<KitOperation><br/>Change.backwards: Vec<KitOperation>"] -->|"forward operations feed into replay"| Scratch
  FinalizedTx["Draft.finalized_transactions"] -->|"forward operations feed into replay"| Scratch
  Cmd["Mutation.renameKit / changeDescription / dragPiece / ..."] -->|append forward+backward operations| OpenTx
  Cmd -.->|"invalidate cache"| Materialized
  Commit["Mutation.checkpointCommit (future)"] -->|"compresses operations; freezes new root"| NewCp["new Checkpoint.frozen_root = previous Materialized.deep_clone()"]
```

Pipeline per replay step:

1. `let diff: KitDiff = operation.to_diff(&kit).await?;` — pure function on the operation; reads pre-state to resolve indices/ids; never mutates.
2. `kit.apply_diff(&diff).await?;` — the **single** central mutation entry point. All structural edits (set field, add/remove vec entry, update map, …) flow through here.

Invariants:

- `Checkpoint.frozen_root` is set **once** at checkpoint creation from a deep clone and is never written to again.
- `Graph` no longer owns a live `the_kit`; it owns the active draft and the parent checkpoint of that draft.
- `Graph.materialized_kit()` = `parent_checkpoint.frozen_root.deep_clone()` then for every operation in every finalized transaction and the open transaction (in order): `kit.apply_diff(&operation.to_diff(&kit).await?).await?`.
- Caches: per-`Checkpoint` `frozen_root` is itself the cache; per-`Draft` materialization caches an `Arc<Kit>` keyed by a monotonic `change_seq` bumped on every transaction mutation and reset on commit/abort.
- **Only** `Kit::apply_diff` mutates a `Kit`. Operations, the worker, resolvers, and replay logic never reach into `Kit` fields directly.

## Affected regions in [compose/rs/lib.rs](compose/rs/lib.rs)

- `📦 kit` (lines 1148–2833):
  - Add `Kit::deep_clone()` walking every sub-entity (Type, Representation, Connector, Design, Piece, Tag, Concept, Quality, Author, File, Folder, Prop, Attribute, Stat, plus all `*_by_id` weak maps re-pointing into the cloned graph).
  - Add **the single central mutation entry point** `Kit::apply_diff(&self, diff: &KitDiff) -> Result<(), ComposeError>`. This is the only public mutator on `Kit`. Internally it walks the canonical `KitDiff` (sparse scalar overrides + `removed` / `updated` / `added` triples on every collection) and writes the corresponding `RwLock<…>` slot (or pushes/removes from `Vec`/`HashMap`). Every direct field-mutation in the file that currently reaches into `kit.name.write().await`, `kit.description.write().await`, `kit.tags.write().await.push(...)`, `design.pieces.write().await.push(...)`, etc., is collapsed into this one method.
  - All existing per-field mutator helpers on `Kit` (e.g. `create_and_register_tag`, `delete_tag_by_id`, `register_tag`, `register_concept`, `register_quality`) are removed; their behaviour is expressed as `KitDiff` fragments produced from operations and applied centrally.
  - Strip `bump_touch_epoch` from `Kit` (now done at `Graph` level on materialization invalidation).
- `🌿 vcs` (lines 3089–4156):
  - Replace `Graph.the_kit: Arc<Kit>` with: `parent_root_for_active_draft: RwLock<Arc<Kit>>` (clone of the seed checkpoint's `frozen_root`) and `materialized_cache: RwLock<Option<MaterializedSlot>>` where `MaterializedSlot { change_seq: u64, kit: Arc<Kit> }`.
  - Add `Graph::materialized_kit(&self) -> Arc<Kit>` (deep-clones parent root and replays draft operations; reuses cache when `change_seq` matches).
  - Add `Graph::active_draft()` accessor and `Graph::record_op_in_open_transaction(forward: KitOperation, backward: KitOperation)` that appends to the current `Change`, bumps `Draft.change_seq`, and invalidates the materialization cache.
  - `Checkpoint.root` → `Checkpoint.frozen_root: Arc<Kit>` (no `RwLock`, no `Option`); fixed at construction.
  - `Draft` gains `change_seq: AtomicU64`.
  - Remove the in-place `the_kit` mutation paths: `Graph::apply_create_fixed_piece`\*, `Graph::apply_drag_piece_in_design`, `Graph::apply_drag_pieces_in_design`. Their logic moves into the operation's `to_diff` (see below); structural mutation only happens in `Kit::apply_diff`.
- `⚙️ operation` (lines 4488–5146):
  - Introduce a **strictly one-way** semantic operation enum. **Every variant has the same uniform shape: `{ scope: Scope, input: Input }`** — both `Scope` and `Input` are single shared enums. `Scope` carries every `Id` the operation references (entities it reads, mutates, deletes, or creates). `Input` carries the non-id payload (values, embedded `TagInput` / `ConceptInput` / `Position` / `Offset` / `String` / …). No variant carries undo state. Variant–scope–input pairings are documented per variant and validated at runtime inside `to_diff` / `to_backwards`.

    ```rust
    /// Single shared scope enum. One variant per distinct id-shape used across operations.
    /// Ids the worker mints at record time (creations) sit alongside ids the worker reads
    /// from existing state (targets, owners) — both are just `Id`s by the time they reach
    /// the operation log.
    pub enum Scope {
        Kit,                                                                                                            // kit is implicit
        Entity           { entity_id: Id },
        Tag              { tag_id: Id },
        Tags             { tag_ids: Vec<Id> },
        CreateTag        { owner_id: Id, tag_id: Id,        attribute_ids: Vec<Id> },                                   // tag_id + attribute_ids minted
        CreateTags       { owner_id: Id, tag_ids: Vec<Id>,  attribute_ids: Vec<Vec<Id>> },
        CreateConcept    { owner_id: Id, concept_id: Id,    attribute_ids: Vec<Id> },
        CreateQuality    { owner_id: Id, quality_id: Id,    attribute_ids: Vec<Id>, benchmark_ids: Vec<Id> },
        CreateFixedPiece { design_id: Id, piece_id: Id,     blueprint_id: Id,       attribute_ids: Vec<Id> },
        PieceInDesign    { design_id: Id, piece_id: Id },
        PiecesInDesign   { design_id: Id, piece_ids: Vec<Id> },
        // ...one variant per distinct id-shape used by the 15 commands.
    }

    /// Single shared input enum. One variant per distinct non-id payload shape used
    /// across operations. Two operations sharing a payload shape (e.g. `RenameKit` and
    /// `RenameTag` both carry just a `name: String`) reuse the same `Input::Name` variant.
    pub enum Input {
        None,                                                                                                           // Delete*, FixPieceInDesign
        Name        { name: String },                                                                                   // RenameKit, RenameTag
        Description { description: Option<String> },                                                                    // ChangeDescription
        Icon        { icon: Option<String> },                                                                           // ChangeIcon
        Image       { image: Option<String> },                                                                          // ChangeImage
        Tag         { tag: TagInput },                                                                                  // CreateTag
        Tags        { tags: Vec<TagInput> },                                                                            // CreateTags
        Concept     { concept: ConceptInput },                                                                          // CreateConcept
        Quality     { quality: QualityInput },                                                                          // CreateQuality
        FixedPiece  { position: Position, name: Option<String>, description: Option<String> },                          // CreateFixedPiece
        Offset      { offset: Offset },                                                                                 // DragPieceInDesign, DragPiecesInDesign
        // ...one variant per distinct payload shape used by the 15 commands.
    }

    pub enum KitOperation {
        RenameKit          { scope: Scope, input: Input },   // scope = Scope::Kit                            ; input = Input::Name { .. }
        ChangeDescription  { scope: Scope, input: Input },   // scope = Scope::Entity { .. }                  ; input = Input::Description { .. }
        ChangeIcon         { scope: Scope, input: Input },   // scope = Scope::Entity { .. }                  ; input = Input::Icon { .. }
        ChangeImage        { scope: Scope, input: Input },   // scope = Scope::Entity { .. }                  ; input = Input::Image { .. }
        CreateTag          { scope: Scope, input: Input },   // scope = Scope::CreateTag { .. }               ; input = Input::Tag { .. }
        CreateTags         { scope: Scope, input: Input },   // scope = Scope::CreateTags { .. }              ; input = Input::Tags { .. }
        DeleteTag          { scope: Scope, input: Input },   // scope = Scope::Tag { .. }                     ; input = Input::None
        DeleteTags         { scope: Scope, input: Input },   // scope = Scope::Tags { .. }                    ; input = Input::None
        RenameTag          { scope: Scope, input: Input },   // scope = Scope::Tag { .. }                     ; input = Input::Name { .. }
        CreateConcept      { scope: Scope, input: Input },   // scope = Scope::CreateConcept { .. }           ; input = Input::Concept { .. }
        CreateQuality      { scope: Scope, input: Input },   // scope = Scope::CreateQuality { .. }           ; input = Input::Quality { .. }
        CreateFixedPiece   { scope: Scope, input: Input },   // scope = Scope::CreateFixedPiece { .. }        ; input = Input::FixedPiece { .. }
        DeletePieceInDesign{ scope: Scope, input: Input },   // scope = Scope::PieceInDesign { .. }           ; input = Input::None
        DragPieceInDesign  { scope: Scope, input: Input },   // scope = Scope::PieceInDesign { .. }           ; input = Input::Offset { .. }
        DragPiecesInDesign { scope: Scope, input: Input },   // scope = Scope::PiecesInDesign { .. }          ; input = Input::Offset { .. }
        FixPieceInDesign   { scope: Scope, input: Input },   // scope = Scope::PieceInDesign { .. }           ; input = Input::None
        // ...one variant per existing Command. Every variant has identical signature
        // `{ scope: Scope, input: Input }`. The variant name encodes the operation kind;
        // the (Scope variant, Input variant) pairing is fixed and runtime-validated.
    }

    impl KitOperation {
        /// Pure: read pre-state, produce a structural `KitDiff`. Uses ids from `scope`
        /// (worker-minted for creations) to populate `KitDiff.added[*].id`. Never mutates `kit`.
        pub async fn to_diff(&self, kit: &Arc<Kit>) -> Result<KitDiff, ComposeError>;

        /// Pure: read pre-state, return the ordered list of one-way `KitOperation`s
        /// that, applied in order through the same operation → diff → `apply_diff` pipeline,
        /// undo this operation. For Creations the backward `Delete*` operation references the
        /// ids in `scope`. For Deletions the backward Creation operation constructs a fresh
        /// `scope` re-using the prior ids from `pre`. Never mutates `kit`.
        pub async fn to_backwards(&self, kit: &Arc<Kit>) -> Result<Vec<KitOperation>, ComposeError>;
    }
    ```

  - Adopt the **canonical `KitDiff`** schema already defined for the compose kit format. It is a partial mirror of `Kit` where:
    - Every scalar field on `Kit` (`name`, `description`, `icon`, `image`, `preview`, `version`, `remote`, `homepage`, `license`, `createdAt`, `updatedAt`, plus `id`) is `Option<…>`; `Some` means "set to this value", `None` means "untouched". Serde uses `skip_serializing_if = Option::is_none` to keep the JSON representation sparse.
    - Every collection field on `Kit` (`types`, `designs`, `tags`, `files`, `folders`, `ports`, `authors`, `attributes`, `concepts`) becomes a `*Diff` container with the canonical triple shape:
      ```rust
      pub struct TypesDiff {
          pub removed: Vec<TypeId>,
          pub updated: Vec<TypeDiffUpdate>,
          pub added:   Vec<Type>,
      }
      pub struct TypeDiffUpdate { pub r#type: TypeId, pub diff: TypeDiff }
      ```
      and identically for `DesignsDiff` / `TagsDiff` / `FilesDiff` / `FoldersDiff` / `PortsDiff` / `AuthorsDiff` / `AttributesDiff` / `ConceptsDiff`.
    - Each per-entity `*Diff` (e.g. `TypeDiff`, `DesignDiff`, `RepresentationDiff`, `ConnectorDiff`, `PieceDiff`, `ConnectionDiff`, `TagDiff`, `ConceptDiff`, `FileDiff`, `FolderDiff`, `AuthorDiff`, `AttributeDiff`) recursively follows the same pattern (sparse scalar fields + nested `*Diff` containers for its own collections; e.g. `TypeDiff.representations: Option<RepresentationsDiff>`, `TypeDiff.connectors: Option<ConnectorsDiff>`, `DesignDiff.pieces: Option<PiecesDiff>`, `DesignDiff.connections: Option<ConnectionsDiff>`, `RepresentationDiff.tags: Option<TagsDiff>`, etc.).
    - The id-only references inside `removed[]` / `updated[*].<entity>` are dedicated `*Id` newtype structs (`TypeId`, `DesignId`, `PieceId`, `ConnectionId`, `RepresentationId`, `ConnectorId`, `TagId`, `ConceptId`, `FileId`, `FolderId`, `AuthorId`, `AttributeId`) so the on-wire JSON looks like `{ "id": "..." }` (matching the canonical example).
    - Reference shape: [compose/asset/compose/metabolism.kit.diff.compose.json](compose/asset/compose/metabolism.kit.diff.compose.json) is the authoritative on-disk sample; the Rust types must serde to / deserialize from exactly that JSON shape (camelCase field names, `id` for entity references, `diff` for nested updates, `removed` / `updated` / `added` keys).
  - All these types live in the `operation` region of [compose/rs/lib.rs](compose/rs/lib.rs) next to `KitOperation`.
  - `Kit::apply_diff(&self, diff: &KitDiff)` is the **single central mutation entry point**. Internally it walks the sparse `KitDiff` tree, applies scalar overrides, removes entities by id, applies per-entity `*Diff` updates (recursively re-using `apply_diff` on the same pattern for sub-entities), and appends `added` entities. No other `Kit`-mutating call exists.
  - Each existing `OperationKind` (`RenamedKit`, `CreatedFixedPiece`, `DraggedPiece`, `ChangedDescription`, `FixedPiece`) is constructed from the corresponding `KitOperation` + post-apply materialized kit so the GraphQL operation event stream stays unchanged.

- **System mints all new ids; user input never carries an id for a created entity.** Existing GraphQL mutation arguments stay id-free for creations (`createTag(ownerId, tag: TagInput)`, `createConcept(ownerId, concept: ConceptInput)`, `addFixedPieceToDesign(draftId, transactionId, designId, blueprintId, position)`, ...). At command acceptance the worker calls `Id::new().await` once per entity that will be created (entity itself + any nested attributes / benchmarks the input carries) and packs the resulting ids into the operation's `scope` (creation `scope` structs include the freshly-minted entity id alongside any owner / target ids the operation also references). Replay (`to_diff`) reads ids from `scope` to populate `KitDiff.added[*].id`, so persisted operation logs replay deterministically without re-minting.
- **Inverse computation lives on the operation itself via `KitOperation::to_backwards`.** Every variant knows how to read pre-state and produce the ordered list of forward-intent `KitOperation`s that undo it. The list lets one forward operation fan out to multiple backward operations where the structural change is non-atomic (e.g. deleting a piece may recreate the piece + recreate any dependent connections; a batch `DeleteTags` returns one creation operation per id). For Creations the backward `Delete`\* operation references the ids in `self.scope`; for Deletions the backward Creation operation constructs a fresh `Scope` re-using the prior ids from `pre` so undo restores the same identity. Examples (illustrative — every variant must implement its own):
  - forward `RenameKit { scope: Scope::Kit, input: Input::Name { name: "Bar" } }` → backwards `[ RenameKit { scope: Scope::Kit, input: Input::Name { name: <pre.name> } } ]`
  - forward `DeleteTag { scope: Scope::Tag { tag_id }, input: Input::None }` → backwards `[ CreateTag { scope: Scope::CreateTag { owner_id: <pre.tag.owner>, tag_id, attribute_ids: <pre.tag attribute ids> }, input: Input::Tag { tag: <pre.tag input snapshot> } } ]`
  - forward `CreateTag { scope: Scope::CreateTag { owner_id, tag_id, attribute_ids }, input: Input::Tag { tag } }` → backwards `[ DeleteTag { scope: Scope::Tag { tag_id }, input: Input::None } ]`
  - forward `DragPieceInDesign { scope: Scope::PieceInDesign { design_id, piece_id }, input: Input::Offset { offset } }` → backwards `[ DragPieceInDesign { scope: Scope::PieceInDesign { design_id, piece_id }, input: Input::Offset { offset: -offset } } ]`
  - forward `CreateFixedPiece { scope: Scope::CreateFixedPiece { design_id, piece_id, blueprint_id, attribute_ids }, input: Input::FixedPiece { .. } }` → backwards `[ DeletePieceInDesign { scope: Scope::PieceInDesign { design_id, piece_id }, input: Input::None } ]`
  - forward `DeletePieceInDesign { scope: Scope::PieceInDesign { design_id, piece_id }, input: Input::None }` → backwards `[ CreateFixedPiece { scope: Scope::CreateFixedPiece { design_id, piece_id, blueprint_id: <pre.blueprint>, attribute_ids: <pre.attrs> }, input: Input::FixedPiece { position: <pre.position>, name: <pre.name>, description: <pre.description> } }, /* one connection-recreating operation per connection that referenced the piece, scope reusing prior connection ids */ ]`
  - forward `DeleteTags { scope: Scope::Tags { tag_ids }, input: Input::None }` → backwards = ordered `Vec` of `CreateTag` operations (one per id, oldest first), each with `Scope::CreateTag.tag_id` set to the original id from `pre` and `input: Input::Tag { tag: <pre.tag input snapshot> }`.
    `KitOperation` stays a flat data type with no undo state inside variants; `to_backwards` is a pure read on `kit` returning a fresh `Vec`. `Change.forwards: Vec<KitOperation>` / `Change.backwards: Vec<KitOperation>` are symmetric — both are just lists of one-way operations that go through the same operation → diff → `apply_diff` pipeline at replay time. There is no separate `operation::inverse_for` helper; the logic is on the variants of `KitOperation` itself.
- `🧵 worker` (lines 5887–6211): `ChildRuntime::apply` becomes a flat dispatcher per `Command` that:
  1. Captures `before_kit = self.graph.materialized_kit().await` (used by `to_backwards` to derive the inverse operations, e.g. previous name / previous tag input snapshot).
  2. For every entity the command will create, mints fresh ids via the system id generator (`Id::new().await`); these ids go directly into the matching `Scope` variant (alongside any owner / target ids the user did supply).
  3. Builds the forward `KitOperation::<Variant> { scope: Scope::<…> { … }, input: Input::<…> { … } }` from the populated `Scope` and the user-supplied non-id payload wrapped in the matching `Input` variant.
  4. Computes `backwards: Vec<KitOperation> = forward.to_backwards(&before_kit).await?`.
  5. Calls `self.graph.record_op_in_open_transaction(draft_id, transaction_id, forward.clone(), backwards).await?` which (a) ensures the open transaction exists, (b) extends the current `Change.forwards` with `forward` and the current `Change.backwards` with `backwards` (kept in their natural order; replay during undo iterates the change's `backwards` in reverse), (c) bumps `draft.change_seq`, (d) invalidates `materialized_cache`. The worker itself never calls `to_diff` or `apply_diff` — those run lazily inside `materialized_kit()` so cancelled (aborted) operations never touch a kit.
  6. Re-materializes via `self.graph.materialized_kit().await` (which, internally, deep-clones root then for each recorded forward operation runs `operation.to_diff(&kit) → kit.apply_diff(diff)`).
  7. Builds the existing `OperationKind` (e.g. `RenamedKit { kit: materialized_kit, … }`) and pushes to `op_history` + emits on the bus, exactly as today.
  - `Graph::abort_transaction` simply drops the open transaction's `Change` list and invalidates the materialization cache; the next `materialized_kit()` deep-clones root and replays only the surviving (finalized) operations, automatically yielding the pre-transaction state — no manual undo execution needed. `Change.backwards` is preserved on disk for explicit undo/redo flows (future).
- `🌐 gql` (lines 6213–6638): every resolver that calls `graph.the_kit.`\* (`Graph::the_kit`, `Graph::projection_fingerprint`, `Graph::root_snapshot_hash`, `Query::node`, `Query::piece_in_design`, `Query::kit_store_bundle_json`, `Mutation::kit_store_bundle_hydrate`, `Mutation::kit_store_initialize_defaults`) switches to `let kit = graph.materialized_kit().await;`.
- `🗄️ kit_backbone` (lines 5241–5830):
  - `KitStoreBundleFile::from_graph` serializes from `graph.materialized_kit()` for `wip.root` and from each `Checkpoint.frozen_root.kit_full_snapshot_value()` for the checkpoint list.
  - `KitStoreBundleFile::hydrate_into_graph` materializes a fresh `Arc<Kit>` from the bundle JSON, freezes it as the seed `Checkpoint.frozen_root`, and points `Graph.parent_root_for_active_draft` at a clone of it.
  - `clear_piece_projections_for_backbone_replay` (line 5699) is removed; replay just re-hydrates a fresh checkpoint root.
- `🧪 tests` (lines 6783–7492): update `g.the_kit` references in tests (lines 7239, 7240, 7265, 7292, 7330, 7185–7186, 7217, 4262–4264) to `g.materialized_kit().await`. Extend `kit_store_bundle_serialize_hydrate_round_trip_via_graphql` (line 6850) with assertions:
  - After `renameKit` to "Hello Bundle", `wip.checkpoints.edges[0].node.frozenRoot.name` == previous name (root unchanged).
  - `wip.theKit.name` == "Hello Bundle" (materialized changed).
  - After `transactionAbort` of the rename transaction, `wip.theKit.name` reverts to the previous name.

## Implementation order (single ticket, no sub-delegation needed since this is one cohesive refactor)

1. Canonical `KitDiff` family of types (`KitDiff`, `TypesDiff` / `TypeDiff` / `TypeDiffUpdate`, `DesignsDiff` / `DesignDiff` / `DesignDiffUpdate`, `RepresentationsDiff` / `RepresentationDiff` / `…Update`, `ConnectorsDiff` / `ConnectorDiff` / `…Update`, `PiecesDiff` / `PieceDiff` / `…Update`, `ConnectionsDiff` / `ConnectionDiff` / `…Update`, `TagsDiff` / `TagDiff` / `…Update`, `ConceptsDiff` / `ConceptDiff` / `…Update`, `FilesDiff` / `FileDiff` / `…Update`, `FoldersDiff` / `FolderDiff` / `…Update`, `AuthorsDiff` / `AuthorDiff` / `…Update`, `AttributesDiff` / `AttributeDiff` / `…Update`, `PortsDiff` / `PortDiff` / `…Update`, plus the corresponding `*Id` newtype refs) + serde camelCase config matching [compose/asset/compose/metabolism.kit.diff.compose.json](compose/asset/compose/metabolism.kit.diff.compose.json), and `Kit::apply_diff` walking that tree as the single central mutation entry point; deletion of all per-field mutators on `Kit`.
2. `Kit::deep_clone()` walking every sub-entity and rebuilding `*_by_id` weak maps.
3. Shared `Scope` enum (one variant per distinct id-shape used across the 15 commands: `Kit`, `Entity { entity_id }`, `Tag { tag_id }`, `Tags { tag_ids }`, `CreateTag { owner_id, tag_id, attribute_ids }`, `CreateTags { owner_id, tag_ids, attribute_ids }`, `CreateConcept { .. }`, `CreateQuality { .. }`, `CreateFixedPiece { design_id, piece_id, blueprint_id, attribute_ids }`, `PieceInDesign { design_id, piece_id }`, `PiecesInDesign { design_id, piece_ids }`, ...) and shared `Input` enum (one variant per distinct non-id payload shape: `None`, `Name { name }`, `Description { description }`, `Icon { icon }`, `Image { image }`, `Tag { tag }`, `Tags { tags }`, `Concept { concept }`, `Quality { quality }`, `FixedPiece { position, name, description }`, `Offset { offset }`, ...). Operations sharing a payload shape reuse the same `Input` variant.
4. `KitOperation` enum (one-way) covering the existing 15 `Command` variants, every variant uniformly shaped as `{ scope: Scope, input: Input }` reusing both shared enums. The variant name encodes the operation kind; the (Scope variant, Input variant) pairing is fixed and runtime-validated inside `to_diff` / `to_backwards`. Two pure read methods per variant: `to_diff(&Arc<Kit>) -> KitDiff` and `to_backwards(&Arc<Kit>) -> Vec<KitOperation>`.
5. `Checkpoint.frozen_root: Arc<Kit>`, `Draft.change_seq`, `Graph.parent_root_for_active_draft` + `materialized_kit` (deep-clone + replay via operation → diff → `apply_diff`) + `record_op_in_open_transaction`.
6. Remove `Graph.the_kit`; rewrite `worker::ChildRuntime::apply` to record operations only (no mutation; mutation happens inside `materialized_kit()`).
7. Switch every resolver / backbone path to `materialized_kit()`.
8. Update `kit_full_snapshot_value` consumers and tests.

## Validation

- `cargo test -p compose` (covers golden fingerprint, bundle round-trip, transaction lifecycle, alternative-from-tip, tag CRUD).
- New `KitDiff` JSON round-trip test that loads [compose/asset/compose/metabolism.kit.diff.compose.json](compose/asset/compose/metabolism.kit.diff.compose.json), deserializes it into the new `KitDiff` type, re-serializes it, and asserts JSON equivalence.
- New assertions on root immutability + abort-restores-state in `kit_store_bundle_serialize_hydrate_round_trip_via_graphql`.
- `npm run build` for `compose/rs/pkg` (wasm32 target).
- `compose/js` + `compose/react` test suites unchanged (GraphQL surface is preserved; only internal Rust storage changes).

## Ticketing

Open a new MCP ticket "Refactor RS Kit Store To Materialized Reads" under the kit-store goal; close with summary listing `compose/rs/lib.rs` as the only touched file.
