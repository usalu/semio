# Mesh Module Dissolution — `🧰️framework/🔨️modules/🔺️mesh/🦀️component.rs` → `✳️mesh`

Assigned lane: "dissolve the standalone framework mesh module into the mesh artifact." Read
`📌️important.md` in full first, per instruction. Result: **honest partial — zero lines of
production code moved**, because measurement (not reluctance) found a hard, load-bearing
architectural blocker that a code change cannot route around within this wave's boundaries.
Reported precisely below rather than forcing a completion that would either break brep's 413
passing tests or leave the workspace red.

## What changed

**Nothing was edited.** After the census and dependency-graph proof below, every code motion I
could make would either (a) break `🧰️framework/🔨️modules/🧊️3d/📐️brep/📦️mesh-io/🦀️component.rs`
(read-only to me, owned by the W3a brep lane, explicit "do not regress 413 passing" mandate), or
(b) require `semio-framework`-layer crates to depend on `semio-s-plugin-stdio`, which is a hard
Cargo dependency cycle, not a style question. Making a partial move that only relocates the pieces
brep doesn't touch (the shape-primitive generators) still leaves the framework file in existence
(contradicting the literal goal) while forcing a redesign of ~20 call sites for no crate-boundary
benefit, so I did not do that partial move either — see "What I did NOT do and why" below for the
reasoning against a half-measure.

## The census (measured, not grepped-and-assumed)

Per-symbol `grep -rl` across every live `.rs` file (excluding `🎯️target` and ticket-scratch dirs),
then read (not just counted) every hit to classify real usage vs. incidental mention:

| Symbol group | Real live consumers found |
|---|---|
| `mesh_box`/`mesh_plane`/`mesh_uv_sphere`/`mesh_ico_sphere`/`mesh_cylinder`/`mesh_cone`/`mesh_torus` (Primitives) | Only the mesh module itself + `🧰️framework/📦️packages/🦀️rust/📦️glue.rs`'s re-export list. **Zero external call sites** — these are only reached indirectly through `mesh_from_kind`. |
| `mesh_from_kind` | `🧰️framework/🛍️products/💻️os/{🦀️component.rs, 🔨️modules/🔌️plugin/🦀️component.rs, 🔨️modules/♾️infinite/🦀️component.rs, 🔨️modules/♾️infinite/🌍️world/🦀️component.rs, 🖥️host/🦀️component.rs}` (5 framework-layer files) + ~15 plugin files (`remodel`, `cad`, `puzzle`, `lowpoly`, `procedural`, `process`, `playbook`) — real, live, used for ephemeral viewport markers/placeholder/demo geometry, not persisted document content. |
| `MeshData` (struct) | ~30 files total across framework + plugins; real construction/consumption, not just type mentions. |
| `mesh_from_indexed`/`mesh_from_indexed_with_face_groups` | `🧰️framework/🔨️modules/🧊️3d/📐️brep/🧰️kernel/🦀️component.rs` (tessellation bridge) + `✏️s/🔌️plugins/📐️cad/…/🚪️io/🗺️geometry-import/🦀️component.rs`. |
| `mesh_to_obj`/`mesh_from_obj`/`mesh_to_glb`/`mesh_from_glb`/`mesh_to_stl`/`mesh_from_stl`, `MeshExporter`/`MeshImporter`, `Obj/Glb/StlExporter`/`Importer`, `IoError` | **`🧰️framework/🔨️modules/🧊️3d/📐️brep/📦️mesh-io/🦀️component.rs` directly** (read verbatim, not grepped — see below) + framework's own mesh module + `📦️glue.rs` re-export. |

The `✏️s/🔌️plugins/🏗️fem/…/🚪️io/…` and cad's own `🚪️io` hits for `IoError`/format names turned out,
on read, to be **unrelated same-named types local to those artifacts' own codecs** — a pattern
match, not a real dependency on the framework's `mesh::IoError`. Counted out per the "grep finds
candidates, not a census" rule.

## The hard blocker (read, not inferred)

`🧰️framework/🔨️modules/🧊️3d/📐️brep/📦️mesh-io/🦀️component.rs:17-20` (read-only to me, W3a-owned,
413-passing-tests mandate):

```rust
use semio_framework::{
    dwg_drawing_to_mesh, dwg_from_bytes, dwg_to_bytes, mesh_from_obj, mesh_from_stl, mesh_to_obj, mesh_to_stl, GlbExporter, GlbImporter, MeshData,
    MeshExporter, MeshImporter,
};
```

This file uses **every one of**: `MeshData` (struct + `.vertex_count()`), `mesh_from_obj`,
`mesh_to_obj`, `mesh_from_stl`, `mesh_to_stl`, `GlbExporter`/`GlbImporter` (trait impls, called via
`.export()`/`.import()`), and the DWG bridge functions — at the exact import path
`semio_framework::{…}`, exercised by 5 of its own `#[cfg(test)]` cases (`glb_export_import_round_trip`,
`obj_export_import_round_trip`, `stl_ascii_round_trip_preserves_triangle_count`, etc. —
`🦀️component.rs:359-432`), which are presumably part of the 413 brep tests this ticket must not
regress. I cannot edit this file (hot-file table: `🧊️3d/**` → W3a lane, read-only for us), so its
import statement is fixed. That pins `MeshData`, the OBJ/STL free-function codec, and
`GlbExporter`/`GlbImporter` at their current `semio_framework::` path, unconditionally, this wave.

Separately, this wave's own scope explicitly keeps `mesh_to_dwg_drawing`/`dwg_drawing_to_mesh` "exactly
where they are, including their file location" (DWG carve-out) — and both bridge functions
(`🔺️mesh/🦀️component.rs:1911-1958`) construct/read `MeshData` directly, so `MeshData` must also stay
reachable from wherever those two functions end up, which per the explicit carve-out is the current
file.

## Why the doctrinal destination is unreachable via Cargo, not just inconvenient

Checked the real dependency graph, not assumed it:

- `✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/Cargo.toml:22-24` — `semio-s-plugin-stdio` depends on
  `semio-framework-os-kernel`, `semio-framework-plugin` (workspace), `semio-framework-schema`.
  `semio-framework-plugin`'s own `Cargo.toml` depends on `semio-framework` directly and re-exports
  `MeshData` (`🔨️modules/🔌️plugin/📦️packages/🦀️rust/📦️glue.rs` mounts it; confirmed live use at
  `✏️s/🔌️plugins/📐️cad/…/🚪️io/🗺️geometry-import/🦀️component.rs:18`:
  `use semio_framework_plugin::{ArtifactSerializer, MeshData};`).
- So `semio-s-plugin-stdio → semio-framework-plugin → semio-framework`. Moving `MeshData`/the
  Obj-Glb-Stl codec **into** stdio and making them private there would require
  `semio-framework`-layer crates (`semio-framework-os-kernel`, `semio-framework-os-infinite`,
  `semio-framework-plugin`, `semio-framework-os` (host), `semio-framework-os-flow`) to depend back
  on `semio-s-plugin-stdio` for `mesh_from_kind` — a direct cycle, not a refactor question. Verified
  by reading `🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/Cargo.toml`,
  `…/🔨️modules/♾️infinite/📦️packages/🦀️rust/Cargo.toml`,
  `…/🔨️modules/🔌️plugin/📦️packages/🦀️rust/Cargo.toml`,
  `…/🖥️host/📦️packages/🦀️rust/Cargo.toml`, `…/🔨️modules/🌊️flow/📦️packages/🦀️rust/Cargo.toml` — none
  depend on stdio, and cannot without the cycle above.

## The mesh artifact's own facet code needs none of this (a real, useful finding)

Read `✳️mesh/🧬️schema/💡️inferences/📦aabb/🦀️component.rs` in full: its `compute()` walks
`SemioPrimitive.positions: Vec<SemioPoint3>` (structured `f64` points) directly — a completely
different representation from `MeshData`'s flat `f32` buffers. It has its own honest, complete,
tested dependency-hash chain and needs no conversion through `MeshData` at all. Same story for the
17 mutation triads (SMO's lane, explicitly "do not touch" per `📌️important.md` line 40) and the
`🚪️io` facet's existing 5-format × 2-direction bridges (`✳️mesh/🚪️io/🦀️component.rs`'s
`io_bridge_entries()`), which already convert `SemioMeshSnapshot ↔ {Obj,Gltf,Stl,Ply,Las}Snapshot`
natively via `ArtifactSerializer`/`ArtifactDeserializer` — a different, already-complete mechanism
from the framework's raw-bytes `MeshExporter`/`MeshImporter`. **There is no live call site inside
`✳️mesh`'s own facet code that needs the framework's `MeshData`/primitive-constructor math at all.**
Adding a private, unused copy of that math into stdio "to satisfy the letter of the instruction"
would be dead-weight duplication with no real caller — against the no-example-files /
no-dead-code discipline this ticket holds everyone else to, so I did not add it.

## What I did NOT do, and why (the half-measure I rejected)

The `Primitives` region (`mesh_box`…`mesh_torus`, `mesh_from_kind`) has **no** live consumer inside
brep or the DWG region, so in isolation it *could* move without touching the read-only brep file.
I rejected doing this partial move because: (1) `MeshData` and the Obj/Glb/Stl codec still can't
move (brep blocks them), so the file `🔺️mesh/🦀️component.rs` continues to exist regardless —
moving only `Primitives` does not get closer to "the file shouldn't exist," it just removes the
smaller of the two blocked pieces; (2) `mesh_from_kind` has ~20 real call sites across 5
framework-layer files and ~15 plugin files, several outside my read boundary or requiring individual
design judgement (construction vs. legitimate ephemeral-marker use) that a rushed pass would get
wrong; (3) the same Cargo-cycle argument applies to the 5 framework-layer consumers as to `MeshData`
— they can't reach a stdio-private copy either. A half-migration that improves no crate boundary
while risking ~20 call sites felt like exactly the "forced completion" the ticket's own doctrine
warns against.

## The DWG scope boundary (reported prominently, per instruction)

`//#region Dwg` (`🔺️mesh/🦀️component.rs:~911` to EOF, ~1600 lines: `DwgDrawing`, `DwgBitWriter`,
`dwg_to_bytes`/`dwg_from_bytes`, plus the two mesh-bridge functions `mesh_to_dwg_drawing`/
`dwg_drawing_to_mesh`) is untouched, exactly as scoped — a complete, unrelated DWG binary
codec consumed by stdio's own `🖊️dwg` artifact and by 20 `cad`/`drawing` snapshot-serialization
consumers. It is not a mesh concern; it belongs in its own future wave. This wave changed nothing
in that region.

## A concrete path for a future wave

The blocker is that `MeshData` + the raw byte codec are needed on **both** sides of a dependency
edge that can only point one way (framework-layer → stdio, never the reverse). The standard fix is
a small new leaf crate (e.g. `semio-framework-mesh-data`) holding just `MeshData` +
`mesh_from_indexed`/`mesh_from_indexed_with_face_groups` + the Obj/Glb/Stl codec, that both
`semio-framework`(-plugin/-os-kernel/-os-infinite/-os/-os-flow) **and** `semio-s-plugin-stdio`
depend on. That breaks the cycle without brep, DWG, or any of the ~20 `mesh_from_kind` call sites
needing to change at all — but it means registering a new Cargo workspace member, which
`📌️important.md`'s hot-file table reserves for the W6 ratchet agent (`repo-root 🔣️taxonomy.json`
territory), so I did not attempt it this wave. Flagging it for the coordinator/W6 rather than
inventing a workspace change unilaterally.

## Files touched

**None.** No files were created, edited, or removed in `🧰️framework/**`, `✏️s/**`, or anywhere else.

## Verification commands run, with real output pasted

All three ran with the mandated flags (`RUSTC_WRAPPER=""`, `CARGO_TARGET_DIR` under this ticket,
`--all-targets`) to confirm the baseline I found is real and to leave an honest record, since I
made no changes to re-verify against.

```
$ RUSTC_WRAPPER="" CARGO_TARGET_DIR=".🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/DISSOLVE-KERNELS-AND-MODULES-INTO-EVENT-SOURCED-ARTIFACTS/🎯️target" cargo check -p semio-framework --all-targets
...
    Checking semio-framework v0.1.0 (/Users/ueli/Documents/semio/🧰️framework/📦️packages/🦀️rust)
    Finished `dev` profile [unoptimized] target(s) in 2.67s
```
Clean (warnings only, no errors).

```
$ RUSTC_WRAPPER="" CARGO_TARGET_DIR=".../🎯️target" cargo check -p semio-framework-3d --all-targets
...
warning: `semio-framework-3d` (lib) generated 15 warnings (run `cargo fix --lib -p semio-framework-3d` to apply 11 suggestions)
warning: `semio-framework-3d` (lib test) generated 9 warnings (4 duplicates)
    Finished `dev` profile [unoptimized] target(s) in 5.34s
```
Clean — brep (incl. `📦️mesh-io`'s tests) compiles and its test binary builds. Exit 0.

```
$ RUSTC_WRAPPER="" CARGO_TARGET_DIR=".../🎯️target" cargo check -p semio-s-plugin-stdio --all-targets
...
error[E0432]: unresolved imports `crate::artifacts::tiff::standards::v6_0::subsets::any::io::decode_tiff`, `...encode_tiff`
error[E0432]: unresolved import `crate::artifacts::binary::standards::v_raw::subsets::any::schema::snapshot::STDIO_BINARY_DOCUMENT_SCHEMA`
error[E0425]: cannot find value `STDIO_BINARY_DOCUMENT_SCHEMA` in module `...binary::standards::v_raw::subsets::any::schema::snapshot`
error[E0425]: cannot find value `STDIO_SVG_DOCUMENT_SCHEMA` in this scope (…svg/…schema/🦀️component.rs:805)
error[E0425]: cannot find function `register_schema_specs` in module `...deflate::standards::v_rfc1950::subsets::any::io`
error[E0308]: arguments to this function are incorrect (…gltf/…/🚪️io/🦀️component.rs:261 — two distinct `GltfComponentType` types with the same name colliding)
... (3 lib errors, 6 including tests total)
error: could not compile `semio-s-plugin-stdio` (lib) due to 3 previous errors; 605 warnings emitted
error: could not compile `semio-s-plugin-stdio` (lib test) due to 6 previous errors; 750 warnings emitted
```

## Concurrent-churn observations

`semio-s-plugin-stdio` was **already red at baseline**, before I touched anything (I made zero
edits). Every error is in `tiff`, `binary`, `svg`, `deflate`, and `gltf` artifact files — none in
`✳️mesh/**`, confirmed by reading each error location above. This is someone else's in-progress
work elsewhere in stdio (`gltf`'s error is a duplicate-type collision between
`…gltf/…/⚙️engine/🦀️component.rs:78` and `…gltf/…/🚪️io/🦀️component.rs:80`, both defining
`GltfComponentType` — looks like an in-flight rename/split). Per protocol I did not retry 3× at
60s intervals since this crate was never touched by my (zero) changes and I have nothing to
re-check — flagging as `blocked-churn`, not mine to fix, and unrelated to the mesh dissolution
task.

## sharedFileRequests

None filed. I did not need brep's file changed — I needed to *not* change it, which required no
request. If a future wave lands the leaf-crate fix above, `🧊️3d/📐️brep/📦️mesh-io/🦀️component.rs`'s
one `use semio_framework::{…}` line (`🦀️component.rs:17-20`) would need its source crate renamed to
the new leaf crate — a one-line patch, held here for whoever owns that file then:
`use semio_framework_mesh_data::{dwg_drawing_to_mesh, dwg_from_bytes, dwg_to_bytes, mesh_from_obj, mesh_from_stl, mesh_to_obj, mesh_to_stl, GlbExporter, GlbImporter, MeshData, MeshExporter, MeshImporter};`
(assuming the DWG bridge functions move to the same leaf crate at that time; if DWG stays in
`semio_framework` proper, split the `use` into two).

## Honest pass/fail

**Fail, honestly reported, as the ticket's own doctrine prefers over a forced pass.** Zero lines
of the in-scope mesh content were relocated. The blocker is real, verified by reading (not
grepping) the exact blocking file and the exact Cargo dependency graph, not assumed. The tree is
unchanged and was confirmed still green for `semio-framework` and `semio-framework-3d` (brep
still compiles, its tests still build — no regression, because no edit was made). `semio-s-plugin-stdio`
is red from unrelated concurrent work, not from anything in this lane. Recommended next step is
the leaf-crate split above, which is a W6/coordinator-scoped workspace change, not something to
attempt unilaterally from this lane.
