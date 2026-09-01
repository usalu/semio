# addPrimitive scratch-rehydration bug — fix report

**Bug confirmed real.** `add_primitive::handle` both reads scratch (`build_doc`, which calls
`LowpolyDocument::with_context` → `reload_meshes`) and writes it (`ctx.set_mesh_workspace_map`).
`reload_meshes` (⚙️engine/🦀️component.rs:160) rejects any object whose id is missing from
`mesh_workspace`, or whose cached JSON hashes to a different `mesh_child_handle`, with
`StaleMeshWorkspace`. `LowpolyScratch::default()`'s `mesh_workspace` only ever contains the single
fresh default object (`schema::default_mesh_workspace()`), so `build_doc` returns `None` — and
`handle` silently no-ops (`Ok(Emit::default())`) — for every `addPrimitive` dispatched after any
object already has a real (non-default) mesh handle, including a second sequential `addPrimitive`
itself (the pre-existing `add_primitive_supports_every_known_kind` test already exercises this and
was failing pre-fix).

## Fix (owned file only)

`✏️editor/🦀️component.rs`:
- `LowpolyCommandDisposition::ArtifactConfig` → renamed `ArtifactConfigTransient` (same discriminant 7).
- `lowpoly_command_disposition("addPrimitive")` → `ArtifactConfigTransient`.
- `lowpoly_retained_reduce`'s `AddPrimitive` arm moved into the `threaded!` macro (rehydrates
  `LowpolyScratch` from `context.transient`, runs the handler, republishes the resulting scratch as
  `ArtifactToolPublicationLane::Transient`), same treatment as `extrude`/`inset`/etc.
- `PUBLICATION_CONTRACTS["addPrimitive"]` lanes: `[Artifact, Config]` → `[Artifact, Config, Transient]`.

Framework gate at `🔌️plugin/🦀️component.rs:22151` rejects publishing `ephemeral.transient` unless
`Transient` is in `publication_lanes` — confirms the 3-lane contract is required and sufficient (no
framework limit on lane count; only `HostOnly` may not combine).

## Regression test (added to `➕️add-primitive/🦀️component.rs`)

`add_primitive_after_mesh_edit_adds_object_and_preserves_the_edit`: dispatches `Extrude` on the
default object, then `AddPrimitive`; asserts `objects.len() == 2`, the new object is present, and
`objects[0].mesh` (the extruded handle) is unchanged.

## Exact edits needed in the three files I do NOT own

**`🧪️interactive-job/🔣️schema.json`**
- `routes.items.properties.lanes.maxItems`: `2` → `3`.
- Add an 8th `oneOf` entry under the `Migrated` branch:
  `{ "properties": { "lanes": { "const": ["Artifact","Config","Transient"] }, "preparation": { "const": ["Artifact","Config"] } } }`
  (`preparation.maxItems` stays `2` — unchanged.)

**`🧪️interactive-job/🔣️component.json`**
- `addPrimitive` route: `"lanes": ["Artifact","Config"]` → `["Artifact","Config","Transient"]`.
  `"preparation"` stays `["Artifact","Config"]`.

**`📦️packages/🟦️typescript/📜️script.ts`**
- Line ~43, the `signature` allowlist array: add `"Artifact+Config+Transient|Artifact+Config"`.

## Compile/test verification

**Rust compile/test: BLOCKED, not failed — and not by this work.**

Coordinator ran, 2026-09-01 17:20–17:32:
`cargo check -p semio-s-plugin-lowpoly --all-targets --keep-going --message-format short`

Result `EXIT=101`, with 2199 error lines. Attribution by originating path:

| origin | error lines |
|---|---|
| `✏️s/🔌️plugins/🗄️stdio` | 2196 |
| `🧰️framework` | 0 |
| `✏️s/🔌️plugins/💠️lowpoly` | **0** |

The single terminating line is
`error: could not compile 'semio-s-plugin-stdio' (lib) due to 2196 previous errors`.
`semio-s-plugin-stdio` is a hard dependency of lowpoly's io layer, so lowpoly is never reached — this
is a transitive block, not a lowpoly defect. The stdio working tree carries 1410 uncommitted files from
a peer session's in-flight refactor (serde `Serialize`/`Deserialize` trait bounds unsatisfied across its
mutation leaves), with no `.rs` write in the preceding 30 minutes — i.e. parked mid-refactor.

So the regression test `add_primitive_after_mesh_edit_adds_object_and_preserves_the_edit` is
**written but unrun**. It has never been observed to fail-then-pass. Treat it as unverified.

What IS verified: `bunx nx run "@semio-tech/lowpoly-js:test" --skip-nx-cache` is GREEN with the
`ArtifactConfigTransient` disposition in place — 47 Migrated, 0 BatchOnlyPendingRewrite, and the Ajv
hostile-fixture oracle passes. All three non-owned files were landed by the coordinator exactly as this
report specifies.

To close this out once the peer lands stdio:
```
cd "/Users/ueli/Documents/semio" && export DEVELOPER_DIR=/Library/Developer/CommandLineTools
cargo check -p semio-s-plugin-lowpoly --all-targets
cargo check -p semio-s-plugin-lowpoly --target wasm32-wasip2
cargo clippy -p semio-s-plugin-lowpoly --all-targets -- -D warnings
cargo test -p semio-s-plugin-lowpoly --lib
```

