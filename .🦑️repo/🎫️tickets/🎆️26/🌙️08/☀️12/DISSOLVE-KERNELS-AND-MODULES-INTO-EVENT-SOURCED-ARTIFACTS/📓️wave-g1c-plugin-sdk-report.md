# Wave G1c — `semio-framework-plugin` mesh escape-hatch cleanup

Boundary: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs` (crate `semio-framework-plugin`). No other files were edited.

## Census (repo-wide grep, excluding `target/`)

Command: `grep -rn "export_mesh_obj\|export_mesh_glb_bytes\|MeshData\|mesh_to_obj\|mesh_to_glb\|export_mesh" --include="*.rs" .` (excl. `./target`), plus a narrower confirmation pass `grep -rn "export_mesh_obj\b\|export_mesh_glb_bytes\b" --include="*.rs" .`

| Helper | Call sites outside this file (live, non-ticket-backup) | Verdict |
|---|---|---|
| `export_mesh_obj` (was `🦀️component.rs:10364`) | **Zero.** Only definition + one `pub use world3d_host::{...}` re-export line, both inside this same file. | **Deleted outright.** |
| `export_mesh_glb_bytes` (was `🦀️component.rs:10368`) | **Zero.** Same as above. | **Deleted outright.** |
| `MeshData` explicit symbol name in this file | Only a private `use semio_framework::{...}` import (line 9806) and the two deleted fn signatures. No `pub use ...::MeshData` exists in this file as a standalone item. | See "MeshData public surface" below — not deletable within boundary. |

Two hits in `.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️06/.../🧪m3-plugin-component.pre-patch.rs` (a frozen pre-patch snapshot in a *different* ticket's folder) were excluded as non-live scratch material, not real call sites.

## What I changed

1. **Deleted `export_mesh_obj` and `export_mesh_glb_bytes`** (was `component.rs:10364-10370`) — both called `semio_framework::mesh_to_obj` / `mesh_to_glb`, mesh codecs slated for deletion, with zero external callers.
2. **Trimmed the now-dangling import** at `component.rs:9806` from `use semio_framework::{mesh_from_kind, mesh_to_glb, mesh_to_obj, MeshData};` to `use semio_framework::mesh_from_kind;` — `mesh_to_obj`/`mesh_to_glb` were only used by the two deleted fns; `MeshData` (after the two fns were gone) was also unused (confirmed by a compiler warning on first pass, then removed) — `mesh_from_kind` is still called by `world3d_meshes_json_from_kinds`/`world3d_meshes_json_from_kinds_and_urls`, which remain untouched (out of my explicit scope, see Remainders).
3. **Removed `export_mesh_glb_bytes, export_mesh_obj` from the crate-root `pub use world3d_host::{...}` re-export list** (was `component.rs:10772`, now `component.rs:10764` after the deletions shifted line numbers).

No other lines in this file were touched.

## MeshData on this crate's public surface — NOT removed, and why

Step 3 of the brief asked me to remove any `MeshData` re-export from this crate's public surface. I looked for it and found there is **no explicit `pub use ...::MeshData` anywhere in this file** — the only place `MeshData` (and every other framework-core symbol: `mesh_from_kind`, `mesh_to_obj`, `mesh_to_glb`, `mesh_from_glb`, `mesh_from_obj`, `mesh_from_stl`, `mesh_to_stl`, `GlbExporter`, `GlbImporter`, etc.) becomes part of `semio_framework_plugin`'s public surface is the blanket glob at `component.rs:10761`:

```rust
pub use semio_framework::*;
```

This glob is the actual mechanism the mission is really pointing at. It cannot be narrowed to "everything except `MeshData`" without converting it into an explicit allow-list of every other symbol `semio_framework` exports — Rust has no glob-minus-one syntax. I did not attempt that conversion because a repo-wide census (same grep pass) shows `semio_framework_plugin::MeshData` (i.e. this exact re-export path) is depended on directly by **files outside my boundary, in at least 10 other plugin crates**, none of which I am permitted to touch:

- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️component.rs` (+ its `🗺️geometry-import/🦀️component.rs` sibling, + `🧬️schema/💡️inferences/🦀️component.rs`)
- `✏️s/🔌️plugins/🌀️procedural/🎛️apps/🧊️3d/🦀️component.rs`
- `✏️s/🔌️plugins/🧩️puzzle/🎛️apps/🧊️3d/🦀️component.rs` and `🎛️apps/🖐️5d/🦀️component.rs`
- `✏️s/🔌️plugins/💠️lowpoly/🎛️apps/💠️lowpoly/⚙️engine/🦀️component.rs` and `🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️component.rs`
- `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️component.rs`
- `✏️s/🔌️plugins/🏭️process/🎛️apps/🧊️3d/🦀️component.rs` and `🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️component.rs`
- `✏️s/🔌️plugins/📖️playbook/🧩️extensions/🌀️procedural/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🦀️component.rs` and `🧰️framework/🛍️products/💻️os/🖥️host/🦀️component.rs` (both reference `semio_framework_plugin::MeshData` in `register_mesh_exporter`/`register_mesh_importer`/`register_mesh_dwg_*_handler` signatures)

Removing or narrowing the glob would break all of the above simultaneously. That is a coordinated, multi-crate wave, not a single-file edit I can safely make and verify with `cargo check -p semio-framework-plugin` alone (each of those is a separate crate/package with its own compile unit).

### sharedFileRequests

I am not proposing exact patches for the 10+ files above — the correct fix depends on decisions outside my boundary (e.g. does each plugin crate gain a direct Cargo dependency on `semio-framework` so it can write `semio_framework::MeshData` instead of `semio_framework_plugin::MeshData`? Or does each of these mesh bridge functions get rerouted through `io_dispatch`/`io_resolve`/`IoKey`/`register_composer_entries` from `🧰️framework/🔨️modules/🚪️io/🦀️component.rs` instead of naming `MeshData` at all, matching the pattern this wave's brief pointed at?). Flagging this as the real next step rather than guessing at a repo-wide Cargo topology change:

**Suggested follow-up ticket scope:** for each of the ~10 files above, replace the `MeshData`-typed `mesh_from_document`/`document_from_mesh` bridge functions with entries registered via `register_composer_entries`/`io_dispatch` (already the artifact-to-artifact typed IO registry — see `🧰️framework/🔨️modules/🚪️io/🦀️component.rs:266-414` for `IoDirection`, `IoKey`, `register_composer_entries`, `io_dispatch`), then delete `pub use semio_framework::*;` from `component.rs:10761` and replace it with an explicit list of the non-geometry symbols still needed (media types, artifact traits, etc. — everything currently re-exported except the mesh/geometry family). That conversion touches this file too, so it belongs in a follow-up wave once the ~10 dependent files are migrated off `semio_framework_plugin::MeshData`, not before.

## Verification (mandatory exact form, run three times: baseline, after first edit pass, after cleanup)

```
TD="/Users/ueli/Documents/semio/.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/DISSOLVE-KERNELS-AND-MODULES-INTO-EVENT-SOURCED-ARTIFACTS/🎯️target"
touch "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs"
RUSTC_WRAPPER="" CARGO_TARGET_DIR="$TD" cargo check -p semio-framework-plugin --all-targets
```

- **Baseline (before any edit)** — `scratch-g1c-baseline.txt` — exit code 0, 0 errors, only pre-existing warnings (40 lib + 50 test, mostly unrelated `unnecessary qualification`/`unused variable` noise already present).
- **After deleting the two fns + trimming `mesh_to_glb`/`mesh_to_obj` from the import** — `scratch-g1c-postedit.txt` — exit code 0, 0 errors, but introduced one new warning: `unused import: MeshData` at the (then) line 9806 — caught this myself before declaring done.
- **After also dropping `MeshData` from that import** (final state) — `scratch-g1c-final.txt` — exit code 0, 0 errors, warning count back to baseline-equivalent (41 lib-ish / 50 test, the two remaining pre-existing unused-import warnings `ChildGenesis`/`GroupReceipt`/`OwnerRef` and `ChildStoreFactory` are unrelated to this change and were already present in the baseline).

All three logs are preserved in this ticket folder as `.txt` files for inspection: `scratch-g1c-baseline.txt`, `scratch-g1c-postedit.txt`, `scratch-g1c-final.txt`.

I did not run the full test suite (`cargo test`) — only `cargo check --all-targets`, which is what the brief's verification block specifies. I have not independently confirmed runtime behavior of anything downstream; this crate compiles cleanly, that is the extent of what I verified.

## Honest remainders

- `world3d_meshes_json_from_kinds` and `world3d_meshes_json_from_kinds_and_urls` (in the same `world3d_host` module, untouched) still call `semio_framework::mesh_from_kind` internally and serialize its result to JSON via `serde_json`. Their public signatures don't name `MeshData` (they take `&[String]` and return `String`), so they weren't in scope per my literal instructions ("geometry-named pub fn" = names a geometry type in its signature), but they are still a live call into a mesh-kernel constructor that is slated for deletion. Flagging for whoever picks up the `pub use semio_framework::*` follow-up wave above — these two functions will need to move to whatever replaces `mesh_from_kind` once framework-core's mesh module is dissolved.
- The blanket `pub use semio_framework::*;` at `component.rs:10761` still exists and is the real Cargo-law violation surface (it will re-export nothing once framework-core's mesh symbols are deleted, breaking this crate's own compile at that point) — not addressed this wave, see sharedFileRequests above.
- I did not touch `🧰️framework/🛍️products/💻️os/🦀️component.rs` or `🖥️host/🦀️component.rs`, both of which declare `register_mesh_exporter`/`register_mesh_importer`/`register_mesh_dwg_*_handler` with `semio_framework_plugin::MeshData` in their signatures — outside my boundary, listed above.
