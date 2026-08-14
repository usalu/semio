# Wave FINISH — dissolve `📐️brep/` (last framework-3d directory)

## What moved

- `🧰️framework/🔨️modules/🧊️3d/📐️brep/⚙️engine/🦀️component.rs` → `🧰️framework/🔨️modules/🧊️3d/⚙️engine/🦀️component.rs` (plain `mv`, byte-identical content, sibling of `🥽️mesh`).
- `🧰️framework/🔨️modules/🧊️3d/📦️packages/🦀️rust/📦️glue.rs`: the `#[cfg(feature = "brep")] pub mod brep { #[path="../../📐️brep/⚙️engine/🦀️component.rs"] pub mod engine; }` wrapper was replaced with a direct mount:
  ```rust
  #[cfg(feature = "brep")]
  #[path = "../../⚙️engine/🦀️component.rs"]
  pub mod engine;
  ```
  Public path changed from `semio_framework_3d::brep::engine::X` to `semio_framework_3d::engine::X`. Top-of-file crate doc comment also fixed (it still said "B-Rep kernel", stale since an earlier wave already moved the kernel out).
- `📐️brep/` **no longer exists** — confirmed: `ls 🧰️framework/🔨️modules/🧊️3d/` now shows only `AGENTS.md`, `⚙️engine`, `📦️packages`, `🥽️mesh`. Before deleting, I `find`-listed `📐️brep/`'s contents: only the one `.rs` file was present (its twelve sibling foundation-math files had already been relocated to stdio by the concurrent wave PEEL4 before I started — visible in `git status` as pre-existing `D`/`??` pairs I did not create). No non-`.rs` files, no surprises.

## Consumer census (derived myself, not trusted from the brief)

`grep -rln "semio_framework_3d::brep::engine" --include="*.rs" .` (excluding `🎯️target`) found **13 files** with real code/doc references (the brief estimated ~8; actual count is higher because several stdio `✳️brep` facet files each have their own `use`):

1. `🧰️framework/🛍️products/💻️os/🦀️component.rs` (2 references, both `GeometryHandle` — see caveat below)
2. `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/📐️brep-geometry/🦀️component.rs`
3. `✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/🦀️component.rs`
4. `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🗺️geometry-import/🦀️component.rs`
5. `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️component.rs`
6. `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️brep/🧬️schema/🔺️diff/↔️offset/🦀️component.rs`
7. `.../✳️brep/🧬️schema/🔺️diff/🔀️boolean/🦀️component.rs`
8. `.../✳️brep/🧬️schema/⚙️engine/🦀️component.rs`
9. `.../✳️brep/🧬️schema/⚙️engine/📦️mesh-io/🦀️component.rs`
10. `.../✳️brep/🧬️schema/💡️inferences/🌳bounding-volume/🦀️component.rs`
11. `.../✳️brep/🧬️schema/💡️inferences/🏷classification/🦀️component.rs`
12. `.../✳️brep/🧬️schema/💡️inferences/🧩tessellation/🦀️component.rs`
13. `✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/benches/brep_kernel.rs`

Every occurrence of `semio_framework_3d::brep::engine::` (and the two bare doc-comment mentions `` `semio_framework_3d::brep::engine` `` / `` `semio_framework_3d::brep::*` ``) was repointed to `semio_framework_3d::engine::`. Post-edit repo-wide grep confirms **zero** remaining `brep::engine` or `📐️brep/⚙️engine` strings anywhere outside `🎯️target`/ticket scratch.

**Caveat, out of scope, not touched**: `💻️os/🦀️component.rs`'s `media_export_raster::SolidMediaExport` region (lines ~2744–2788, 3542–3545) references `semio_framework_3d::brep::kernel::{Brep, SolidExporter, SolidImporter}` and `semio_framework_3d::brep::engine::GeometryHandle`. `brep::kernel` was never mounted by framework-3d (it moved to stdio in wave DEDUP) and `GeometryHandle` was deleted from `⚙️engine` in that same wave — so this region was **already dead/broken before I touched anything** (its sibling file, `🖥️host/🦀️component.rs`, documents that wave IO1 already deleted the equivalent region there and left a comment pointing at "the wave's report" for who else still depends on it — this file is that who-else, apparently missed). Per my brief I must not touch `brep::kernel`, so I only repointed the `GeometryHandle` path prefix (`brep::engine::` → `engine::`, textually equivalent since `GeometryHandle` doesn't exist at either path) and left `brep::kernel::` untouched. This file belongs to package `semio-framework-os-kernel` (path `💻️os/📦️packages/🦀️rust`), which is a workspace member with **no dependency on `semio-framework-3d`** in its `Cargo.toml` — the crate's primary lib target does not appear to compile this dead region at all (see verification: `cargo check -p semio-framework-3d` pulls in `semio-framework-os-kernel` as its own dependency, at 0 errors, only pre-existing warnings). This crate/region is not on my gate list; flagging as an honest pre-existing remainder, not mine to fix.

## Single-source proof (six types)

`grep -rn "pub (type|struct|enum) <Name>"` across the whole repo (excluding `🎯️target`/ticket scratch), before-and-after:

- `Vec3`, `Aabb`, `ParamDomain`, `FaceGroup`, `PointClassification` — **exactly one** definition site each: `🧰️framework/🔨️modules/🧊️3d/⚙️engine/🦀️component.rs`.
- `MeshTransfer` — two hits: `⚙️engine/🦀️component.rs:49` (the type in scope here) and `🥽️mesh/🦀️component.rs:143`. Confirmed these are **different structs that happen to share a name in different modules** — `mesh::MeshTransfer` has fields `positions/normals/indices/edge_positions/face_ids/...` (plural, mesh-editing shape) vs `engine::MeshTransfer`'s `position/normal/index/edges/face_groups` (singular, transfer-wire shape). `🥽️mesh/` is sibling wave MESH's territory (explicitly off-limits to me) and predates this wave — not a duplication I introduced or need to resolve.

`#[test]` count: the moved file has zero `#[test]` items before and after (it is pure type definitions) — trivially conserved.

## Verification (all run once, live)

```
RUSTC_WRAPPER="" cargo metadata --no-deps --format-version 1 >/dev/null && echo WORKSPACE_OK
→ WORKSPACE_OK

cargo check -p semio-framework-3d --all-targets   → Finished (0 errors, pre-existing warnings only)
cargo test  -p semio-framework-3d --lib           → 62 passed; 0 failed   (matches gate exactly)

cargo check -p semio-s-plugin-stdio --all-targets → Finished (0 errors, pre-existing warnings only)
cargo test  -p semio-s-plugin-stdio --lib         → 3379 passed; 5 failed
  failing tests (name-for-name identical to scratch-w0-baseline-failures-sorted.txt):
    artifacts::binary::standards::v_raw::subsets::any::schema::inferences::extent::component::tests::inference_default_law
    artifacts::dwg::standards::v_ac1018::subsets::any::schema::component::tests::conformance_laws::fixture_honesty_law
    artifacts::dxf::standards::v_r12::subsets::any::schema::inferences::bounds::component::tests::bounds_matches_hand_built_entity_extent
    artifacts::ifc::standards::v2x3::subsets::any::io::component::tests::conformance_laws::fixture_honesty_law
    artifacts::zip::standards::v2_0::subsets::any::schema::inferences::entries::component::tests::inference_default_law
  → matches gate (3379/5) exactly, same 5 names — pre-existing, unrelated to this wave.

cargo check -p semio-s-plugin-cad --all-targets   → Finished (0 errors, pre-existing warnings only)
cargo test  -p semio-s-plugin-cad --lib           → 140 passed; 0 failed; 1 ignored   (matches gate exactly)

cargo check -p semio-framework-os-flow --all-targets → 158 error[...] / 160 "previous errors" total (known-red, gated on diff not zero)
  Diff method: captured full stderr (redirected correctly this time — `>file 2>&1`, not `2>&1 >file`), sorted every `^error` line after stripping the `file:line:col:` prefix, and diffed against the most recent pre-this-wave baseline in the ticket folder (`scratch-w5-2d-osflow-check2.txt`, 158 error[...] / "160 previous errors", same crate).
  Result: `diff` on both the error-code histogram and the full normalized error-message set → IDENTICAL, zero lines of difference (161 lines each, incl. the summary line). The one file I touched in this crate's dependency graph (`📐️brep-geometry/🦀️component.rs`) has zero errors of its own, just two pre-existing unrelated warnings (unused import, unnecessary qualification). Saved to `scratch-finish-osflow-errors-full.txt` / `scratch-finish-osflow-errors-full-sorted.txt` in this ticket folder.
```

## Honest remainders

- `💻️os/🦀️component.rs`'s dead `SolidMediaExport` region still references `semio_framework_3d::brep::kernel::*` (never resolves — `brep::kernel` isn't mounted anywhere in framework-3d, hasn't been since wave DEDUP). Per brief, `brep::kernel` is explicitly not mine to touch; flagging as a pre-existing gap in the IO1/DEDUP cleanup that a future wave should pick up (delete the region, mirroring what wave IO1 already did in `🖥️host/🦀️component.rs`).
- `semio-framework-3d`'s `Cargo.toml` `description` field ("native B-Rep kernel, half-edge mesh, and BVH spatial index") is stale — the B-Rep kernel and BVH left this crate in earlier waves. Left untouched as out-of-scope cosmetic drift, not something this wave's instructions asked me to fix.

## Files touched

Moved: `🧰️framework/🔨️modules/🧊️3d/📐️brep/⚙️engine/🦀️component.rs` → `🧰️framework/🔨️modules/🧊️3d/⚙️engine/🦀️component.rs`
Deleted: `🧰️framework/🔨️modules/🧊️3d/📐️brep/` (directory, now empty/gone)
Edited:
- `🧰️framework/🔨️modules/🧊️3d/📦️packages/🦀️rust/📦️glue.rs`
- `🧰️framework/🛍️products/💻️os/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/📐️brep-geometry/🦀️component.rs`
- `✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/🦀️component.rs`
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🗺️geometry-import/🦀️component.rs`
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️component.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️brep/🧬️schema/🔺️diff/↔️offset/🦀️component.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️brep/🧬️schema/🔺️diff/🔀️boolean/🦀️component.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️brep/🧬️schema/⚙️engine/🦀️component.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️brep/🧬️schema/⚙️engine/📦️mesh-io/🦀️component.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️brep/🧬️schema/💡️inferences/🌳bounding-volume/🦀️component.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️brep/🧬️schema/💡️inferences/🏷classification/🦀️component.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️brep/🧬️schema/💡️inferences/🧩tessellation/🦀️component.rs`
- `✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/benches/brep_kernel.rs`

New scratch: `scratch-finish-osflow-errors-full.txt`, `scratch-finish-osflow-errors-full-sorted.txt` (both in this ticket folder).
