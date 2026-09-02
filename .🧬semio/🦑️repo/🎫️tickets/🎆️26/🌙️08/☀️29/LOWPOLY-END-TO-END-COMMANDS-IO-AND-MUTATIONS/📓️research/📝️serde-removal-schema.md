# Serde Removal — Lowpoly Schema Subtree

**ArtifactChild<S>**: `store::🦀️.rs` (os-kernel) — `Serialize`/`Deserialize` are
`#[cfg_attr(test, derive(..))]`-gated (test-only oracle), `bound = ""`. Unconditionally implements
`dsl::ToValue`/`FromValue` (hand-written, round-trips only `child_id`/`target`). Confirmed by reading
the framework source directly, not assumed.

**Owned subtree state before this pass**: every type transitively containing
`LowpolyObject.mesh: Option<ArtifactChild<SemioMeshSnapshot>>` (`LowpolyObject`, `LowpolyObjectPatch`,
`LowpolyMutation`, `LowpolyDiff`, `LowpolyObjectsDelta`, `LowpolyObjectPatchEntry`, `LowpolySnapshot`,
`LowpolyArtifact`, `LowpolySelection*`, `CreateObject`, all mutation-leaf payload structs) already had
serde correctly `cfg_attr(test)`-gated — a prior pass had done this correctly. `PixelRun` (both
copies, no ArtifactChild field) correctly kept unconditional serde for JSON io.

**Real bugs found and fixed** (verified via framework source, not guessed): two production call
sites still called `serde_json` directly on now-test-only-serde types, which would fail to compile
outside `#[cfg(test)]`:

1. `🧬️mutations/📝️text/🦀️.rs` — `OpText`/`OpBinary` for `LowpolyMutation` called
   `serde_json::to_string(self)`/`from_str::<Self>` unconditionally. `LowpolyMutation` (via
   `CreateObject.object: LowpolyObject`) carries the mesh handle, so this can't derive serde outside
   tests. **Fix**: bridge through `dsl::ToValue`/`FromValue` (unconditional) + the framework's
   `DslValue`↔`serde_json::Value` conversion (`🌱️value/🦀️.rs`, unconditional) — same pattern already
   used by the `shooting` facet's migrated `OpText`. Confirmed `value_derive::ToValue`'s generated impl
   targets `::semio_framework_os_kernel::ToValue` exactly, i.e. `dsl::ToValue` (crate aliased via
   `extern crate semio_framework_os_kernel as dsl;` in the plugin's root `🦀️.rs`).
2. `🧬️schema/🦀️.rs` `MediaConversion` region — `lowpoly_document_from_mesh` (`serde_json::to_value` on
   `LowpolySnapshot`), `mesh_document_from_mesh`/`mesh_from_mesh_document` (`serde_json::to_value`/
   `from_value` on `MeshData`, which independently lost production serde the same way, confirmed in
   `🔺️mesh-engine/🦀️.rs`). **Fix**: same `dsl::ToValue`/`FromValue` bridge; `MeshData` already
   implements these first-party (hand-written, same crate).

No case required dropping information, weakening a field, or a serde/ToValue both-needed conflict
with no bridge available — the `DslValue`↔`serde_json::Value` conversion made every case solvable
without a hand-rolled grammar rewrite.

**Kept serde as-is**: `PixelRun` (both copies), test-only round-trip assertions in
`🦀️.rs`/`🔺️diff/📝️text` tests (already correctly `#[cfg(test)]`-scoped, untouched).

**Out of scope, not touched**: `✏️editor/` (other agent). Framework `MeshData`/mesh-engine and
`🔌️plugin`'s wit-bindgen host glue are mid-migration by a different concurrent session (git shows
`MM` on those files) — blocks a full workspace `cargo check` from reaching this crate at all during
this session (`semio-framework-plugin` fails on its own `MeshData: Serialize` errors, unrelated to
lowpoly). Verified my fix by direct trait/type tracing through framework source (ArtifactChild,
value_derive's macro expansion, the DslValue/serde_json::Value `From` impls, MeshData's own
`pack::value::ToValue`/`FromValue`), not by a green `cargo check`, since that dependency is currently
broken by someone else's in-flight work.

**Final error count in owned files**: 0 known remaining after these 2 fixes, pending a clean
workspace build once the concurrent framework-plugin/mesh-engine churn settles.
