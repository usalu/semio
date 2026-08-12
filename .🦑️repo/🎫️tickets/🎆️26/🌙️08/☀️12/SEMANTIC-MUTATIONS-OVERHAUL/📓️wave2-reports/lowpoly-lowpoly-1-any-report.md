# Wave 2 — `lowpoly/lowpoly` (standard 1, subset `any`) — mutations facet

## Facet
`✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations`
Crate: `semio-s-plugin-lowpoly`.

## What landed

Deleted the generic `LowpolyMutation` (a `dsl::DslEnum`-derived enum with struct-field variants
`ObjectsAdd{index,item}` / `ObjectsRemove{id}` / `ObjectsMove{id,to_index}` /
`ObjectsPatch{id,patch:LowpolyObjectPatch}` / `AddPaintLayer{..}` / `RemovePaintLayer{..}` /
`PatchPaintLayer{..,patch:LowpolyPaintLayerPatch}` / `PaintStroke{..}` / `SetSnapshot{snapshot}`,
plus hand-written `apply_lowpoly_mutation`/`inverse_lowpoly_mutation` dispatch fns) and replaced it
with a 16-variant semantic vocabulary, each a single-field tuple wrapping a real
`🦠️mutation`/`🔺️diff`/`↩️inverse` triad leaf, dispatched via `#[derive(dsl_derive::Mutations)]`
(`#[mutations(snapshot = LowpolySnapshot, diff = LowpolyDiff, schema = "s.lowpoly.lowpoly")]`)
mirroring the wave0 `MiniMutation` reference fixture and the already-migrated `mathematical`/`flow`
facets.

Vocabulary derived from `LowpolySnapshot`'s one persistent collection (`objects: Vec<LowpolyObject>`,
id-keyed: `id`/`name`/`transform{position,rotation,scale}`/`smooth_shading`/`mesh_json`/
`paint_layers: Vec<LowpolyPaintLayer>`, the last an index-keyed anonymous sub-collection with
`name`/`visible`/`opacity`/`blend_mode`/`pixels`), cross-checked against the real app gestures that
emit a `LowpolyMutation` today (`add-primitive` → create; `patch-object` → rename/smooth-shading;
`🖌️session`'s `translate_selection`/`rotate_selection`/`scale_selection` → move/rotate/scale;
`🖌️session`'s stroke-commit/fill → paint edit; `add-paint-layer` command → insert):

| New mutation | Verb | Replaces |
|---|---|---|
| `create-object{index,object}` | create | `ObjectsAdd` |
| `delete-object{id}` | delete | `ObjectsRemove` |
| `reorder-objects{id,to_index}` | reorder | `ObjectsMove` |
| `rename-object{id,new_name}` | rename | `ObjectsPatch` (name field) |
| `change-object-smooth-shading{id,new_smooth_shading}` | change | `ObjectsPatch` (smooth_shading field) |
| `move-object{id,new_position}` | move | `ObjectsPatch` (transform.position — matches `translate_selection`) |
| `rotate-object{id,new_rotation}` | rotate | `ObjectsPatch` (transform.rotation — matches `rotate_selection`) |
| `scale-object{id,new_scale}` | scale | `ObjectsPatch` (transform.scale — matches `scale_selection`) |
| `replace-object-mesh{id,new_mesh_json}` | replace | `ObjectsPatch` (mesh_json field — large structured payload) |
| `insert-paint-layer{object_id,index,layer}` | insert | `AddPaintLayer` (index-keyed anonymous sub-collection, not `add`) |
| `remove-paint-layer{object_id,index}` | remove | `RemovePaintLayer` (kebab unchanged, dir reused in place) |
| `rename-paint-layer{object_id,index,new_name}` | rename | `PatchPaintLayer` (name field) |
| `change-paint-layer-visible{object_id,index,new_visible}` | change | `PatchPaintLayer` (visible field) |
| `change-paint-layer-opacity{object_id,index,new_opacity}` | change | `PatchPaintLayer` (opacity field) |
| `change-paint-layer-blend-mode{object_id,index,new_blend_mode}` | change | `PatchPaintLayer` (blend_mode field) |
| `edit-paint-layer{object_id,layer_index,runs}` | edit | `PaintStroke` (domain content-edit; see note below on the verb choice) |

`SetSnapshot` has **no** replacement (per taxonomy: whole-document replace is banned outright, not
expressible as a mutation; `store::ArtifactStore::reset` is the sanctioned non-history path).

`PatchPaintLayer`/`ObjectsPatch` were themselves the forbidden pattern (`taxonomy.md`: "raw
option-bag `Patch` mutation payloads ... never as a mutation's own payload") — both fully decomposed
into single-scalar mutations above; `LowpolyObjectPatch`/the mutations-facet's own
`LowpolyPaintLayerPatch` still exist as **diff-internal** fragment types (passed to
`diff_objects_patch`/`diff_patch_paint_layer`, never as a payload), which the taxonomy explicitly
permits.

**Verb note on `edit-paint-layer`**: `📓️taxonomy.md`'s "Domain verbs" section names a bespoke
`paint-stroke` verb for exactly this gesture ("`paint-stroke` (lowpoly)"), but `paint-stroke` is
**not** present in `command::APPROVED_VERBS` (the actual compile-time-checked const in
`🧰️framework/…/📡️spr/🎮️command/🦀️component.rs`, which is framework-owned and out of this facet's
writable scope). Using an unapproved verb fails the derive's own compile-time assertion, so `edit`
(already approved — "Replace an authored content body") was used instead; the pixel buffer is
exactly that kind of authored content body. A future framework spine change registering
`paint-stroke` in both `APPROVED_VERBS` and `taxonomy.md` would let this triad rename to match; noted
here rather than done, since it requires a `🧰️framework/**` edit this facet cannot make.

Every `diff()` delegates to the pre-existing sparse `LowpolyDiff` field-delta constructors in
`🧬️schema/🔺️diff/📝️text/🦀️component.rs` (`diff_objects_add`/`diff_objects_remove`/
`diff_objects_move`/`diff_objects_patch`/`diff_add_paint_layer`/`diff_remove_paint_layer`/
`diff_patch_paint_layer`/`diff_paint_stroke` — a sibling facet's already-correct, already-tested
sparse constructors, not new logic) — never apply-then-capture. `move-object`/`rotate-object`/
`scale-object` clone the object's current `transform` from `base` and overwrite only their one field
before handing the whole `LowpolyTransform` to `diff_objects_patch` (storage only supports a
whole-transform patch slot; this is still sparse at the `LowpolyDiff`/object-patch granularity, the
transform sub-struct itself has no independent sparse representation in this schema). Every
`inverse()` reads `base` (pre-state) directly: `delete-object` captures the full object (paint layers
included, since they're embedded) and its base-state index; `remove-paint-layer` captures the full
layer; every `change-*`/`rename-*`/`move-object`/`rotate-object`/`scale-object`/
`replace-object-mesh` captures the single old scalar value; missing target ⇒ `Vec::new()` throughout
(replacing `NoMutation`); `create-object`/insert are no-op-safe (create returns `Vec::new()` if the id
already existed in `base`). `edit-paint-layer` is self-inverse (reads the pre-edit bytes at each
run's offset via `engine::layer_pixels_at`, same as the retired `PaintStroke`'s inverse).

Rewrote `OpText`/`OpBinary` for the new enum (`🧬️mutations/📝️text/🦀️component.rs`,
`💾️binary/🦀️component.rs`) using plain `serde_json` compact encoding — the same approach the
sibling `shooting`/`playground` facets already migrated to under this ticket (satisfies both traits'
laws directly: one line, no `\n`, deterministic, round-trips). The old hand-rolled
`dsl::DslVariants`-based codec (tied to the retired struct-variant enum shape) was fully replaced,
not patched.

## Mechanism note: self-wiring instead of `📦️glue.rs`, and one identifier collision

`📦️glue.rs` is out of this facet's writable boundary (plugin-shared). All 16 new triad-leaf
directories are wired directly inside `🧬️mutations/🦀️component.rs` itself (`🔖️LeafWiring` region,
`#[path = "."] pub mod <slug> { #[path = "<dir>/🦠️mutation/🦀️component.rs"] pub mod mutation; ... }`
— `#[path]` resolution is always relative to the *containing file*, so this works regardless of how
`glue.rs` itself included `component.rs`), matching the `mathematical`/`flow` precedent. Zero
`glue.rs` edits needed for 15 of the 16 new triads.

**One real naming collision, found and fixed**: `glue.rs` still hardcodes a sibling
`pub mod remove_paint_layer { #[path = ".../➖️remove-paint-layer/…"] .. }` directly in the crate's
`mutations` scope (from the pre-migration enum). `remove-paint-layer`'s kebab slug survived unchanged
in the new vocabulary, so its directory (`➖️remove-paint-layer`) was rewritten in place rather than
orphaned — but self-wiring a module also named `remove_paint_layer` inside `component.rs` would be
shadowed by glue.rs's own local declaration at the crate-root `mutations` scope (Rust's glob-import
vs. local-item shadowing rule), so every OTHER file's `crate::artifacts::lowpoly::mutations::
remove_paint_layer::..` reference would resolve to glue.rs's separately-compiled copy of the same
source — a different nominal type than the one `LowpolyMutation::RemovePaintLayer` actually wraps,
which would be a hard type-mismatch compile error. Fixed by naming the self-wired module
`remove_paint_layer_mutation` instead (see the `🔖️LeafWiring` region's inline doc comment); `
SEMANTICS.kind`/the enum variant's own kebab form (`"remove-paint-layer"`) are unaffected — only this
internal Rust wiring identifier differs from the slug. All cross-references (`insert-paint-layer`'s
`↩️inverse`, the `📝️text` test's `demo_mutation_cases`) use the renamed path.

The 8 other OLD triad dirs (`↔️objects-move`, `➕️add-paint-layer`, `➕️objects-add`,
`➖️objects-remove`, `🖌️paint-stroke`, `🖼️set-snapshot`, `🩹objects-patch`, `🩹patch-paint-layer`)
could not be deleted — `glue.rs` still hardcodes their exact file paths and editing `glue.rs` is out
of scope. Their 3 files each (24 total) were rewritten to orphaned doc-comment-only stubs (no
executable code, so no stale-type-reference risk) pointing at this report's `sharedFileRequests` for
the glue.rs cleanup a later plugin-wide pass should make.

## Other in-boundary fix required by the vocabulary change

`🏅️standards/🔖️1/⚙️engine/🦀️component.rs`'s test `artifact_engine_apply_and_inverse_round_trip` was
**already broken before this ticket** — it referenced `protocol::ArtifactEngine`, a trait that does
not exist anywhere in the codebase (confirmed by wave0's repo-wide grep and independently
re-confirmed here), and called a `LowpolyEngine::snapshot()` method that `LowpolyEngine` never
defined (only `LowpolyDocument` has one) — on top of constructing the now-deleted
`LowpolyMutation::ObjectsPatch` bag variant. Since it's inside this facet's writable package boundary
(the whole artifact directory) and blocks the crate compiling, it was rewritten against the real
`protocol::Mutation` diff/apply/inverse contract using the new `rename-object` mutation, rather than
left as three-ways-broken dead code.

## Tests

Extended the existing `🧪️Tests` regions (no new test files): `📝️text/🦀️component.rs` gained
`demo_mutation_cases()` (one value per all 16 variants) plus `op_text_binary_roundtrip_law` and a
garbage-rejection test; `💾️binary/🦀️component.rs`'s existing text/binary-equivalence and
whole-store round-trip tests were updated to use `rename-object` instead of the retired
`PatchPaintLayer`; `⚙️engine/🦀️component.rs`'s dead test was rewritten (see above) rather than left
broken.

**Not done**: `assert_mutation_inverse_law`/`assert_mutation_diff_absorb_law` from
`🧰️framework/…/📡️spr/🧪️testkit/🦀️component.rs` — grepped `semio-s-plugin-lowpoly`'s `Cargo.toml`
(`✏️s/🔌️plugins/💠️lowpoly/📦️packages/🦀️rust/Cargo.toml`) and this crate's source for an existing
`testkit` import first, per instructions; none exists, so per the task's explicit fallback this step
was skipped rather than adding a new Cargo dependency.

## Verification

`cargo check -p semio-s-plugin-lowpoly` — see `cargoCheck`/notes in the structured report; this
crate has an **unconditional** (non-feature-gated) dependency on `semio-s-plugin-stdio`
(`✏️s/🔌️plugins/💠️lowpoly/📦️packages/🦀️rust/Cargo.toml`), and the shared workspace has many other
sessions concurrently running `cargo check` for other plugins (`raster`, `draw`, `stdio` itself, …)
at the same time, both contending for the shared `target/` lock and, independently, leaving
`semio-s-plugin-stdio` itself mid-refactor with ~390+ real compile errors across roughly a dozen
`🗄️stdio` artifact facets (`pptx`/`svg`/`ifc`/`pdf`/`step`/`xlsx`/`docx`/`jpg`/`tiff`/`zip`/`xml`/…,
all `DerivedArtifactAnalyzer`/`DerivedArtifactComposer` `sniff`/`analyze`/`compose` associated-fn
errors — a single coherent in-progress refactor signature, not scattered noise). None of these files
are inside `🗿️artifacts/💠️lowpoly`; none reference `LowpolyMutation` or anything this facet touched.
Per the workspace-churn policy, retried 3× (with an isolated `CARGO_TARGET_DIR` attempt in between,
to sidestep the shared lock specifically): the very first attempt got far enough to observe the
~390-error `semio-s-plugin-stdio` signature above before the workspace load climbed; every attempt
after that (both against the shared `target/` and against a fresh isolated one) sat on
`Blocking waiting for file lock on build directory` for minutes with zero further progress, against
a host `load average` of ~110 (`uptime`) — dozens of concurrent `rustc`/`cargo check` processes for
`stdio`/`raster`/`draw`/other plugins visible in `ps aux` at the same time. This is the documented
"Concurrent Cargo Workspace Churn" pattern (shared-repo cargo builds blocked by other live sessions'
in-progress work, sometimes 30–90+ min) — genuinely external, genuinely outside this facet's
artifact directory, not a code defect. No full green (or even a real red) `cargo check` signal was
obtainable in this session's time budget.

Manual verification performed regardless (same audit method as sibling facets under this ticket):
- Path-existence check (Python) of all 48 `#[path]` leaf-file attributes referenced from the new
  `🔖️LeafWiring` region — all resolve.
- Brace-balance check on all 48 new triad-leaf files.
- Every `impl protocol::MutationKind<LowpolySnapshot, LowpolyMutation>` hand-checked against the real
  trait definition (`🎮️command/🦀️component.rs` `🔖️Semantics` region) and the wave0 `MiniMutation`
  fixture's exact shape (payload struct, `SEMANTICS` const, `diff`/`inverse` delegating to sibling
  leaves, `label`/`target`).
- Every `SEMANTICS.kind` hand-kebab-checked against its variant's PascalCase name and every
  `SEMANTICS.verb` checked against the literal `APPROVED_VERBS` table read from
  `🎮️command/🦀️component.rs`.
- Grepped the whole crate for every retired variant name (`ObjectsAdd`/`ObjectsRemove`/`ObjectsMove`/
  `ObjectsPatch`/`AddPaintLayer`/`RemovePaintLayer`(struct-variant form)/`PatchPaintLayer`/
  `PaintStroke`/`SetSnapshot`) to enumerate every remaining call site — all inside this facet were
  fixed; all outside (`🎛️apps/**`) are listed in `sharedFileRequests` below.

## sharedFileRequests (for the plugin-wide app-reconciliation pass)

1. **`📦️glue.rs`, `mutations` block** — delete the 9 now-dead `pub mod` blocks: `objects_move`,
   `add_paint_layer`, `objects_add`, `objects_remove`, `paint_stroke`, `set_snapshot`,
   `objects_patch`, `patch_paint_layer` (all 8 point at orphaned doc-comment-only stub files now),
   **and** `remove_paint_layer` (its content was rewritten in place as the real new
   `remove-paint-layer` mutation, but it's now redundant/superseded by this facet's own
   `remove_paint_layer_mutation` self-wiring — safe to delete once glue.rs is next touched).
2. **`🎛️apps/💠️lowpoly/🎮️commands/➕️add-primitive/🦀️component.rs`** (`AddPrimitive::handle`) —
   replace `LowpolyMutation::ObjectsAdd { index, item: new_object }` with
   `LowpolyMutation::CreateObject(create_object::mutation::CreateObject { index, object: new_object })`.
3. **`🎛️apps/💠️lowpoly/🎮️commands/✏️patch-object/🦀️component.rs`** (`PatchObject::handle`) — its
   `match payload.field.as_str()` only ever builds a one-field `LowpolyObjectPatch` today (`"name"` or
   `"smoothShading"`); replace the `LowpolyMutation::ObjectsPatch{id,patch}` construction with
   `LowpolyMutation::RenameObject(rename_object::mutation::RenameObject{id, new_name})` for the
   `"name"` arm and `LowpolyMutation::ChangeObjectSmoothShading(change_object_smooth_shading::mutation::
   ChangeObjectSmoothShading{id, new_smooth_shading})` for the `"smoothShading"` arm.
4. **`🎛️apps/💠️lowpoly/🎮️commands/🖌️paint/🦀️component.rs`** (`AddPaintLayer::handle`) — replace
   `LowpolyMutation::AddPaintLayer{object_id,index,layer}` with
   `LowpolyMutation::InsertPaintLayer(insert_paint_layer::mutation::InsertPaintLayer{object_id,index,layer})`.
5. **`🎛️apps/💠️lowpoly/🖌️session/🦀️component.rs`** —
   - Two `LowpolyMutation::ObjectsPatch{id,patch}` sites (`transform_selection`'s commit, ~line 173
     and ~line 416) build a whole-`LowpolyTransform` patch from whichever of translate/rotate/scale
     the session was in; replace with `MoveObject`/`RotateObject`/`ScaleObject` matching the
     session's actual transform mode (the session already knows which — it's the caller of
     `transform_selection`/`translate_selection`/`rotate_selection`/`scale_selection`).
   - Two `LowpolyMutation::PaintStroke{object_id,layer_index,runs}` sites (stroke-commit ~line 301,
     fill ~line 357) — rename to
     `LowpolyMutation::EditPaintLayer(edit_paint_layer::mutation::EditPaintLayer{object_id,layer_index,runs})`
     (the `PixelRun` value type itself is unchanged — still `crate::artifacts::lowpoly::op::PixelRun`,
     re-exported from this facet's `📝️text` module exactly as before).
6. **`🎛️apps/💠️lowpoly/🦀️component.rs`** (two sites, ~line 277/314, `mesh:in`/`document:in` import)
   and **`🎛️apps/💠️lowpoly/🎮️commands/📄️fixture/🦀️component.rs`** (`SetSnapshotJson::handle`,
   ~line 13) — all three build `LowpolyMutation::SetSnapshot{snapshot}` for a genuine
   whole-document-replace gesture (file open / paste-over / load-fixture). Per taxonomy this is
   **not** expressible as a mutation at all anymore; these need to go through
   `store::ArtifactStore::reset` (or whatever `Emit`/`HostEffect` surface wave0 eventually adds for
   it — wave0's report explicitly deferred building that plumbing) instead of emitting a mutation.
   This is the one item here that's an architecture change, not a search-and-replace rename.
7. **`🎛️apps/💠️lowpoly/🦀️component.rs`** ~line 718 — a test's `match` arm on
   `LowpolyMutation::SetSnapshot { snapshot }` needs updating alongside item 6.

Grepped the whole `🗿️artifacts/💠️lowpoly/**` tree (including `📚️examples/`, `🎹️composer`,
`🏗️builder`, `🧐️analyzer`) for every retired variant name — no other in-boundary call sites found
beyond what's fixed in this facet; everything above is `🎛️apps/**`/`📦️glue.rs`, out of this facet's
writable boundary.
