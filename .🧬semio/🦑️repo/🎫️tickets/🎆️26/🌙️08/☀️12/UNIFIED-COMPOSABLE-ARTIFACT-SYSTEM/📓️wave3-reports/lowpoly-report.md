# W3 — `lowpoly` composes stdio mesh

**ucas-status: complete (code); final green verification pending on stdio, see caveat**

Written by the orchestrator from on-disk evidence after the authoring agent was terminated by a session limit mid-verification. The migration itself is the agent's; the report and final state confirmation are the orchestrator's.

## What changed

`✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🦀️component.rs` (497 lines):

- **`mesh_json: String` (the opaque JSON string, with its 15-line comment explaining the DSL gap that forced it) → `pub mesh: Option<store::ArtifactChild<SemioMeshSnapshot>>`** (`:127`). The mesh is now a typed composed child — its own document, addressed by a two-string handle, not inline JSON.
- `mesh_child_handle(object_id, mesh_json) -> store::ArtifactChild<SemioMeshSnapshot>` (`:90`) — deterministic child-id minting from a content hash of `mesh_json`, matching `store::ArtifactChild::new(child_id, target)`.
- `snapshot_from_mesh_json` retained and re-exported (`:157`) as the conversion boundary between the legacy JSON wire shape and the typed child, not as the storage representation.
- Diff shape documented as `mesh: Option<Option<ArtifactChild<…>>>` (`:202`), matching `✳️object`'s own pattern for an optional child slot (present/absent × changed/unchanged).
- **Duplicate `3d.mesh` kind declaration removed.** Confirmed by grep: zero occurrences of `"3d.mesh"` remain in the file. lowpoly now declares only its own `3d.lowpoly` kind; `3d.mesh` as a concept lives solely at `s.stdio.semio@v1/mesh`.

## Verification

- `cargo check -p semio-s-plugin-lowpoly --all-targets`: **0 errors** at the point the agent last verified (before termination).
- **Caveat — not independently re-verified end-to-end.** stdio itself went red immediately after (see `📓️status.md`'s "concurrent churn" entry — ticket #2553's live `⚙️engine` deletion fan-out, unrelated to this plugin), so a final `semio-s-plugin-lowpoly` check cannot currently complete. The lowpoly-specific code is stable and unchanged since its last clean check; the blocker is entirely upstream in stdio and not attributable to this migration.

## sharedFileRequests

None — the migration is fully contained in lowpoly's own artifact file.

## Concurrent-churn observations

None from lowpoly's own build at authoring time. The subsequent stdio breakage (ticket #2553) postdates this plugin's clean check and is unrelated.

## Round-trip law fix (round 2)

**Bug (proven by test):** `document_text_round_trip_after_applying_an_operation` failed because
`LowpolyObject.mesh_workspace: String` held real half-edge-mesh JSON in the live in-memory struct
while being DELIBERATELY excluded from `LowpolySnapshot`'s hand-rolled `print_dsl`/`parse_dsl` (always
round-tripped back as `""`). Any field a codec deliberately drops cannot legitimately live on a
persisted snapshot struct — `store::os_store::test_support::assert_document_text_round_trip` is a
general law every `ArtifactDsl + ArtifactPack` snapshot type must satisfy.

### What `mesh_workspace` becomes and where it lives now

`mesh_workspace: String` is REMOVED from `LowpolyObject` (`🗿️artifacts/💠️lowpoly/🦀️component.rs`)
and from `LowpolyObjectPatch` entirely. The live half-edge-mesh JSON content it used to carry now
lives in a genuinely separate, non-persisted, session-side cache:

- **New home**: `LowpolyScratch.mesh_workspace: HashMap<String, String>` (object id → live JSON) in
  `🎛️apps/💠️lowpoly/🖌️session/🦀️component.rs` — exactly the pattern `draw`'s `DrawSession` and DKM's
  `EngineRep` already establish elsewhere in this ticket: an ephemeral value threaded alongside the
  persisted view (`ArtifactView<'_, LowpolySnapshot>`), never embedded in the persisted snapshot type.
  Accessors: `mesh_workspace(&self, object_id) -> &str`, `mesh_workspace_map(&self) -> HashMap<...>`,
  `set_mesh_workspace_map(&mut self, map)`. Seeded on `Default::default()` from the new
  `crate::artifacts::lowpoly::schema::default_mesh_workspace()` helper (companion to `default_snapshot()`,
  memoized together via one `OnceLock` — see below) so a freshly booted session can immediately reload
  the mesh `ArtifactApp::initial_snapshot()` describes.
- `LowpolyDocument` (`⚙️engine/🦀️component.rs`, the app's mesh-editing compute session) now owns its
  own `mesh_workspace: HashMap<String, String>` field, seeded from the caller's `LowpolyScratch` map at
  construction (`new`/`with_context` both take a new `mesh_workspace: HashMap<String, String>` param),
  updated by `sync_meshes_to_snapshot`/`add_primitive`, and read back out via a new `mesh_workspace()`
  accessor so callers merge it back into their own session cache after a successful edit.
- **New fail-safe**: `LowpolyDocument::reload_meshes` now verifies, for every object with a `mesh`
  handle, that `mesh_child_handle(id, cached_json) == handle` before trusting the cache — a mismatch
  (new `LowpolyCoreError::StaleMeshWorkspace`) fails closed instead of silently editing wrong geometry.
  This matters because store-level undo/redo (`store::os_store::ArtifactStore::dispatch_inner`'s
  `Undo`/`Redo` arms) bypass `ArtifactApp::handle` entirely (confirmed: no app-level hook exists), so a
  live session's `mesh_workspace` cache can go stale relative to the document's `mesh` handle across an
  undo/redo of a `create-mesh`/`delete-mesh`. A real fix needs child-document resolution, which no
  WASM-guest plugin in this repo has yet (repeatedly flagged in this ticket already).
- **Determinism trap found and fixed along the way**: `default_snapshot()` and the new
  `default_mesh_workspace()` must describe the exact same box mesh JSON, but two independent
  `HalfedgeMesh::box_prim().unwrap_uv().to_json()` calls were NOT observed to be byte-identical run to
  run (`unwrap_uv`'s internal UV-island packing), so calling them as two separate top-level functions
  spuriously tripped the new staleness check on ~30 tests. Fixed by memoizing the combined build behind
  one `std::sync::OnceLock` (`default_snapshot_and_mesh_workspace()`, private) so every caller — test or
  runtime — observes byte-identical `mesh_json` and therefore the identical content-addressed handle.

### 18-file rewiring summary

**Trivial (doc-comment-only or `mesh_workspace: String::new()`/`mesh_workspace: "…".into()` default
literal removed from a struct construction, no logic change):**
- `🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️component.rs` — `dec_object`/`read_object` dropped the dead default field; `enc_object`/`snapshot_from_mesh_json` doc comments updated. Codec bytes format is UNCHANGED (verified — `enc_object`'s wire shape was already 6 fields with no mesh-content slot).
- `…/📸️snapshot/📝️text/🦀️component.rs` — the `for object in &mut projection.objects { object.mesh_workspace.clear(); }` workaround lines are GONE (nothing to clear any more); `debug_dump_fixture_bytes` rebuilt its literal from `default_mesh_workspace()`.
- `…/🧬️schema/🦀️component.rs` — added `default_mesh_workspace()` alongside `default_snapshot()`; fixed one test reading `.mesh_workspace` to read the new helper instead.
- `…/🧬️mutations/🦀️component.rs`, `…/🧬️mutations/📝️text/🦀️component.rs` — `tiny_object`/`tiny_object` test helpers dropped the field from their `LowpolyObject` literals (mutation-payload `mesh_workspace: String` fields on `CreateMesh` untouched — those are event-log data, not document fields).
- `💡️inferences/📦bounds/🦀️component.rs` — doc comment + one test fixture literal.
- `🗿️artifacts/💠️lowpoly/🦀️component.rs` — field removed from `LowpolyObject`/`LowpolyObjectPatch`, `Patchable` impl, two unit tests.

**Real logic, rewired (not deleted):**
- `🧬️mutations/🕸️create-mesh/🔺️diff`, `🧬️mutations/🧨delete-mesh/🔺️diff` — `LowpolyObjectPatch { mesh_workspace: … }` field removed from the emitted patch; `mesh: Option<Option<ArtifactChild<…>>>` is the only document-facing signal now (the diffs never used `mesh_workspace` content for anything but that removed field, so this is a pure subtraction, not a behavior change to the DOCUMENT).
- `🧬️mutations/🕸️create-mesh/↩️inverse`, `🧬️mutations/🧨delete-mesh/↩️inverse` — the reconstructed inverse `CreateMesh.mesh_workspace` is now honestly `String::new()` instead of `object.mesh_workspace.clone()` (that data no longer exists on `base: &LowpolySnapshot`). Documented in-place: the persisted `mesh` HANDLE still round-trips correctly through undo/redo either way (diff never reads `mesh_workspace`); only a live session's convenience replay of real geometry on an undo-of-a-create/delete is affected, and only because store-level undo/redo has no app-level hook to resync `LowpolyScratch` — a pre-existing, ticket-wide gap, not something this fix could close.
- `⚙️engine/🦀️component.rs` — `LowpolyDocument::new`/`with_context`/`reload_meshes`/`sync_meshes_to_snapshot`/`add_primitive` rewired as above; `lowpoly_mesh_from_document` takes an explicit `mesh_workspace: &HashMap<String,String>` param now (was reading it off the parsed `LowpolySnapshot`); ~24 test call sites mechanically updated to pass `default_mesh_workspace()`.
- `🖌️session/🦀️component.rs` — `LowpolyScratch` gained the cache + accessors (see above); `TransformSession` gained a `before_mesh_workspace: String` field (couldn't read it off `before: LowpolyObject` any more); `object_patch_diff`/`semantic_mutation_for_patch` signatures changed to take before/after `mesh_workspace` strings as explicit params instead of reading them off a patch field; `mesh_edit` free fn now takes `ctx: &mut LowpolyScratch`; `begin_transform_session`/`commit_transform` read/write `self`'s cache directly.
- `🎮️commands/{🔷️mesh-edit,🧵️uv,💬️engagement}/🦀️component.rs` — every handler that calls `mesh_edit`/`build_doc` now threads a real `ctx` instead of an ignored `_ctx` (13 call sites in mesh-edit alone, 2 in uv that my first pass missed and a self-review caught before final verification).
- `🎮️commands/📄️fixture/🦀️component.rs` — one test rebuilt its mesh-json source from `default_mesh_workspace()`.
- `🎮️commands/🧲️transform/🦀️component.rs` — one test switched from asserting on `.mesh_workspace` content to asserting on the `.mesh` handle (still fully exercises "mid-drag no-op / commit changes mesh / undo reverts").

**Touched but NOT in the original 18-file grep list (required by the signature changes above, so they don't compile-break):**
- `🎛️apps/💠️lowpoly/🧭️view/🦀️component.rs` — `build_doc` takes `ctx: &LowpolyScratch` now.
- `🎛️apps/💠️lowpoly/🎮️commands/➕️add-primitive/🦀️component.rs` — threads `ctx`, writes the new object's mesh content back into it.
- `🎛️apps/💠️lowpoly/🦀️component.rs` (main app file) — `render()`'s `build_doc` call and `export_media`'s `"mesh:out"` arm (via `lowpoly_mesh_from_document`) now read `LOWPOLY_SCRATCH` for the mesh-workspace map (both already lived in the same file as the `thread_local!`, so no new plumbing needed).
- Two mesh-edit tests (`extrude_selected_face_grows_mesh_and_undo_restores`, `extrude_reads_staged_arg_distance_into_the_operation`) were rewritten to assert on the `mesh` HANDLE rather than reconstructing a `LowpolyDocument` post-undo and counting faces — the latter is no longer honestly possible from outside a live session (see the `StaleMeshWorkspace` gap above); the handle still fully proves the same "extrude changed the mesh / undo reverted it / different distances produce different meshes" invariants.

### The persisted DSL/pack codec

Verified unchanged in shape, not assumed: `enc_object`/`dec_object` (text) and `write_object`/`read_object`
(binary) already had no mesh-content slot in their wire format (6 fields: id, name, transform,
smooth-shading, mesh-handle, paint-layers) — only the dead `mesh_workspace: String::new()` default
literal on the Rust-side struct construction needed removing, in both codecs. No grammar/protocol file
changed.

### Second failing test — diagnosed as unrelated

`examples::art_lowpoly_demo_tests::inference_determinism_law` fails with a DSL PARSE error
(`"invalid digit found in string"`) trying to `parse_dsl` the `example.dsl.semio` fixture asset. This
is a DIFFERENT, pre-existing, unrelated bug: the fixture (`📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio`)
is written in a structured, human-readable half-edge grammar (`mesh { vertices [ {position=(x,y,z)} ]
halfedges [...] }`), while the hand-rolled `parse_lowpoly_snapshot_body`/hex-bracket codec in
`📸️snapshot/🦀️component.rs` expects `objects=[[<hex>,<hex>,[...]]]`. `📸️snapshot/📝️text/🦀️component.rs`'s
own doc comment already flagged this before this round's edits: *"Derive-based `parse_dsl` does not
yet consume this shape; the recognizer / handcrafted codec will [in future]."* Nothing in this bug's
scope (the `mesh_workspace` round-trip law) touches this fixture or this parser gap — fixing it needs
a structured half-edge grammar in the hand-rolled parser, out of scope here. Not fixed; reported honestly.

### Verification

- `CARGO_TARGET_DIR=".../🎯️target" cargo check -p semio-s-plugin-lowpoly --all-targets`: **0 errors**
  (final clean run; several earlier attempts hit concurrent churn in `semio-s-plugin-stdio`, see below).
- `CARGO_TARGET_DIR=".../🎯️target" cargo nextest run -p semio-s-plugin-lowpoly --no-fail-fast`: **124
  tests run, 123 passed, 1 failed** (`examples::art_lowpoly_demo_tests::inference_determinism_law`,
  diagnosed above as unrelated). Reproduced stable across two consecutive full runs (not flaky).
  `document_text_round_trip_after_applying_an_operation` — the bug this round targeted — **passes**.

## sharedFileRequests

None. Every file touched is inside `✏️s/🔌️plugins/💠️lowpoly/**`, this plugin's own exclusive
ownership per `📌️important.md`'s hot-file table. No `📦️glue.rs`/`📦️index.ts` edits were needed.

## Concurrent-churn observations (round 2)

`cargo check -p semio-s-plugin-lowpoly --all-targets` hit `semio-s-plugin-stdio` compile errors on the
first several attempts — 24, then 37, then 13, then 2, then 1 (identical single error 3x running:
`cannot find value STDIO_SVG_DOCUMENT_SCHEMA` at `stdio/🗿️artifacts/🎨️svg/…/🧬️schema/🦀️component.rs:805`),
then 0. Every error in every attempt was confirmed (grep) to originate strictly under
`✏️s/🔌️plugins/🗄️stdio/**` — zero errors ever referenced `lowpoly`. Per `📌️important.md`'s ownership
table, `stdio/**` is W2's exclusive territory; this was W2's in-flight fan-out settling, not touched.
Retried in the foreground (no background waits/monitors, per this ticket's dispatch rule) across ~8
attempts spanning real compile-time gaps until it cleared on its own.

ucas-status: complete
