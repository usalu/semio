# Wave DEDUP report

Both windows closed. Full census, symbol-parity proof, and real verification output below.
A third, unrelated duplication was discovered and deliberately left untouched (out of the assigned
mandate) — see "Out-of-scope finding" at the end.

## JOB 1 — brep window: CLOSED

### Re-census (mount-resolved via `os.path.realpath` on every `#[path="…"]` in the repo, not substring grep)

Prior waves (G4, IO1) had already repointed **every real, compiled** consumer of the duplicated
symbols before this wave started. The re-census found:

| File | Symbols used | Status |
|---|---|---|
| `🧰️framework/🔨️modules/🧊️3d/📦️packages/🦀️rust/benches/kernel.rs` | `kernel::Brep`, `engine::{GeometryHandle,Vec3}` | Byte-identical duplicate of stdio's own `benches/brep_kernel.rs` → **deleted** |
| `🧰️framework/🛍️products/💻️os/🖥️host/🦀️component.rs` (the REAL, mounted `semio-framework-os` host file) | none (comments only) | Already migrated by wave IO1 — SolidMediaExport region already deleted |
| `🌊️flow/📐️brep-geometry`, `✏️s/🌊️flow/🧩️extensions/📐️brep`, cad `🗺️geometry-import`, cad `💡️inferences` (all **freshly re-verified** after the coordinator's clearance message) | `Vec3`/`ParamDomain`/`PointClassification`/`MeshTransfer` only | Non-duplicated types, kept in place — no change needed |
| cad's `register_host_io()` owner file (`✳️any/🚪️io/🦀️component.rs`) | `semio_s_plugin_stdio::…::engine::{block_on, BrepKernel, GeometryHandle}` | **Already** fully migrated to stdio — not touched, `register_host_io` untouched |
| gis's io component.rs | (none) | **Already** clean |
| stdio's own `⚙️engine/🦀️component.rs` | 25+ real algorithm modules via `semio_framework_3d::brep::*` | Legitimate "forward edge" — the 16k LOC not yet migrated. No change. |

**`🧰️framework/🛍️products/💻️os/🦀️component.rs`** (top-level, 4425 LOC, NOT the `🖥️host/🦀️component.rs`
file above) still contains live-looking `semio_framework_3d::brep::kernel::*` code, but a full
repo-wide `#[path]`-resolution walk (Python `os.path.realpath`, not string matching — the emoji
collision trap the ticket warned about) found **zero mounts anywhere** pointing at it. It is not
part of any compiled crate. Left alone — see "Out-of-scope finding" below.

**Conclusion: zero live consumers needed repointing.** Job 1's only remaining work was deleting
framework-3d's own dead/duplicate definitions.

### Symbol-parity proof (before deleting)

Extracted every `fn|struct|enum|trait|type|const` definition from OLD (`⚙️engine`+`🧰️kernel`, 1744
LOC) vs NEW (stdio's `⚙️engine/component.rs`, 1695 LOC): **243/249 symbols already present in the new
home.** The 6 missing were exactly the algorithm types the new file's own docstring says "stayed
behind": `Aabb, FaceGroup, MeshTransfer, ParamDomain, PointClassification, Vec3`. Confirmed these are
still used internally by framework-3d's own `bvh`/`spatial`/`offset`/`mesh_io`/`classify`/`tessellate`/
`boolean` modules via `crate::brep::engine::{…}` — **kept**, not deleted. `_sync` helper-method count
matched exactly (93/93) between old `Brep` and new `Brep`.

`⚙️engine/🖥️host/🦀️component.rs` (`BrepEngineHost`/`BrepDocumentOpEngine`/`BREP_ENGINE_ID`, 149 LOC):
repo-wide grep for the symbols AND for the literal engine-id string `"s.3d.brep"` found **zero** live
callers anywhere. Comments in cad's and process3d's inference files explicitly document that wave G4
already deleted the sole `static HOST: OnceLock<BrepEngineHost>` call site and replaced it with direct
`Brep::new()` ownership (`cad_brep_kernel()`, `ProcessKernelReplay`) — verified no cross-call registry
was load-bearing. Dead code with a verified live replacement already in production. Deleted.

`⚙️engine/🧮️compute/🦀️component.rs` (`block_on`, 13 LOC): duplicate of stdio's own `pub fn block_on`
(same `pollster::block_on` body). Zero external callers outside the two deleted files. Deleted.

### What was deleted / trimmed

- `🧰️framework/🔨️modules/🧊️3d/📐️brep/🧰️kernel/` — **deleted whole directory** (1482 LOC; 100% duplicate).
- `🧰️framework/🔨️modules/🧊️3d/📐️brep/⚙️engine/🧮️compute/` and `…/🖥️host/` — **deleted** (duplicate + dead code).
- `🧰️framework/🔨️modules/🧊️3d/📐️brep/⚙️engine/🦀️component.rs` — **trimmed** from 262 → 68 LOC, keeping only
  `Vec3, Aabb, ParamDomain, FaceGroup, MeshTransfer, PointClassification`.
- `🧰️framework/🔨️modules/🧊️3d/📦️packages/🦀️rust/benches/kernel.rs` — **deleted** (duplicate bench, verified byte-identical modulo import paths to stdio's `benches/brep_kernel.rs`); empty `benches/` dir removed.
- `🧰️framework/🔨️modules/🧊️3d/📦️packages/🦀️rust/📦️glue.rs` — removed `pub mod kernel;` mount.
- `🧰️framework/🔨️modules/🧊️3d/📦️packages/🦀️rust/Cargo.toml` — removed `[[bench]]` section; pruned
  `brep` feature + `[dependencies]` down to `dep:semio-framework-os-kernel` only (repo-wide grep across
  the **whole** 3d tree confirmed `async-trait`/`base64`/`blake3`/`pollster`/`semio-framework` became
  fully unused after the above deletions; `semio-framework-os-kernel` stays — `🕸️topology`'s `EngineRep`
  still needs it). `criterion` dev-dependency removed (was solely for the deleted bench).

## JOB 2 — DWG / `🔺️mesh` window: CLOSED (with one deliberate deviation from the literal instruction)

### Re-census

Sole framework-tier caller of `semio_framework::mesh_to_dwg_drawing` (+ `dwg_from_bytes`/`dwg_to_bytes`/
`dwg_drawing_to_mesh`): `🧰️framework/🔨️modules/🧊️3d/📐️brep/📦️mesh-io/🦀️component.rs` (440 LOC) — confirmed
via mount-resolved census, matching the ticket's own claim. This file is itself one of the
not-yet-migrated algorithm modules that stdio's `⚙️engine/component.rs` **already imports** via
`use semio_framework_3d::brep::mesh_io::{export_solid_dwg, export_solid_glb, …}` — i.e. mesh-io had
exactly one real consumer (stdio), and is brep↔mesh bridging IO code that belongs in an artifact under
the standing ruling.

**Resolution taken** (the ticket's own suggested option): moved `📦️mesh-io` into stdio as a sibling
submodule of `✳️brep/🧬️schema/⚙️engine/component.rs` (mounted the same way `⚙️engine` used to mount
`🧮️compute`/`🖥️host` — `#[path="📦️mesh-io/🦀️component.rs"] mod mesh_io;`). Its framework-3d algorithm
imports became external `semio_framework_3d::brep::*` (the same forward-edge pattern the parent file
already uses); its DWG calls became same-crate `crate::artifacts::dwg::{dwg_drawing_to_mesh,
dwg_from_bytes, dwg_to_bytes, mesh_to_dwg_drawing}` instead of crossing a framework→plugin edge (which
would have created a real cycle, since `stdio → semio-framework-3d` already exists for the algorithm
forward-edge). This dissolves the dependency entirely rather than inverting it.

DWG symbol/test parity (old `🔺️mesh/component.rs` 1648 LOC vs new stdio `…/🖊️dwg/…/🚪️io/component.rs`
2267 LOC): all 8 old functions/enums present in the new file, which additionally has a real R2004+
decode pipeline the old one never had. All 9 old DWG-specific tests present by exact name in the new
file's test module.

### Deviation from the literal instruction — read before trusting "delete `🔺️mesh/` entirely"

The ticket said to delete `🧰️framework/🔨️modules/🔺️mesh/` **entirely**. That directory holds two
unrelated files that happen to share a name:
- `🦀️component.rs` (1648 LOC) — the DWG codec, own docstring literally says *"a misplaced drawing/CAD
  format engine that used to share this file with the mesh geometry content… a known follow-up
  violation"*. **Deleted** (fully duplicated in stdio, proven above).
- `🟦️component.ts` (636 LOC) — **completely unrelated** component-scene-protocol payload types
  (`Canvas2dScene`, `World3dScene`, `UiMenuRef`, `ContextMenuItemSpec` TS twin, …), **actively imported**
  by `🧰️framework/📦️packages/🟦️typescript/🟦️glue.ts`. Deleting it would have broken the TS build for
  something with zero relation to DWG or mesh-io.

**Only the Rust file was deleted; the TypeScript file was left in place.** This is the "validate your
assumptions" rule in practice — a blind `rm -rf` on the named directory would have destroyed live,
unrelated code.

### What was deleted / created

- **Created** `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️brep/🧬️schema/⚙️engine/📦️mesh-io/🦀️component.rs` (relocated mesh-io, imports repointed as above).
- `✏️s/…/✳️brep/🧬️schema/⚙️engine/🦀️component.rs` — mounted the new submodule, changed the `mesh_io` import from `semio_framework_3d::brep::mesh_io::{…}` (external) to `mesh_io::{…}` (local).
- `🧰️framework/🔨️modules/🧊️3d/📐️brep/📦️mesh-io/` — **deleted** whole directory (moved).
- `🧰️framework/🔨️modules/🧊️3d/📦️packages/🦀️rust/📦️glue.rs` — removed `pub mod mesh_io;` mount.
- `🧰️framework/🔨️modules/🔺️mesh/🦀️component.rs` — **deleted** (DWG codec only; `🟦️component.ts` untouched).
- `🧰️framework/📦️packages/🦀️rust/📦️glue.rs` — removed `pub mod mesh;` (Rust mount only) and the
  `pub use mesh::{dwg_drawing_to_mesh, …}` re-export block, replaced with a provenance comment.

`cargo metadata --no-deps` run after every manifest edit → `WORKSPACE_OK` each time (no manifest
changes were actually required for Job 2 — no new crate dependencies needed, since stdio already
depended on `semio-framework-3d` and `semio-framework-mesh-engine` directly).

## Verification (real output, fresh — see note on measurement corruption below)

```
TD=".../🎯️target"
touch <edited file>; RUSTC_WRAPPER="" CARGO_TARGET_DIR="$TD" cargo check -p <crate> --all-targets
RUSTC_WRAPPER="" CARGO_TARGET_DIR="$TD" cargo test -p <crate> --lib
```

| Crate | check | `--lib` test | Baseline | Delta explanation |
|---|---|---|---|---|
| `semio-framework-3d` | 0 errors, 12 pre-existing warnings | **396 passed; 0 failed** | 413/0 | -17: 9 kernel.rs tests (dupe, already existed in stdio) + 2 host.rs tests (dead-code coverage, zero live callers) + 6 mesh-io tests (moved intact to stdio) |
| `semio-s-plugin-stdio` | 0 errors | **2957 passed; 5 failed; 4 ignored** | 2951/5 | +6, same 5 pre-existing failures by exact name (`artifacts::binary::…extent…`, `artifacts::dwg::v_ac1018::…fixture_honesty_law`, `artifacts::dxf::…bounds_matches…`, `artifacts::ifc::v2x3::…fixture_honesty_law`, `artifacts::zip::…entries…`) — confirmed byte-identical to `scratch-w0-baseline-failures-sorted.txt`. +6 = the moved mesh-io tests. |
| `semio-framework` | 0 errors | **98 passed; 0 failed** | 127/0 | -29: 9 DWG tests (now solely in stdio, verified present by exact name) + 20 mesh-engine tests (were already leftover duplicates of `semio-framework-mesh-engine`'s own suite from an EARLIER wave — that crate's own docstring says the mesh content was "already dissolved into semio-framework-mesh-engine"; verified 20/20 exact-name match against mesh-engine's test list) |
| `semio-framework-mesh-engine` | 0 errors | **20 passed; 0 failed** | 20/0 | unchanged (not touched) |
| `semio-s-plugin-process` | — | **158 passed; 0 failed** | 158/0 | unchanged (not touched) |
| `semio-s-plugin-flow-extension-brep`, `semio-s-plugin-cad`, `semio-s-plugin-gis` | 0 errors (`--all-targets`, checked together) | — | — | sanity check, unaffected |

### Sum invariant — fell by 11, fully reconciled (not a silent loss)

The ticket's gate says "framework-3d + stdio must not fall." Literal sum: baseline 413+2951=3364 →
now 396+2957=3353, a fall of **11**. Full reconciliation, not glossed over:
- **9**: the old `kernel.rs`'s 9 tests were *already* duplicated in stdio's engine.rs before this wave
  (wave G5's flip) — the baseline sum was double-counting them (once under framework-3d's dying copy,
  once under stdio's real copy). Removing the dying copy's 9 tests removes exactly that double-count;
  stdio's 9 equivalent tests were already running before my change and still are.
- **2**: `BrepEngineHost`'s 2 tests exercised code with zero live callers repo-wide (see above) — no
  production behavior lost, verified replacement pattern (`Brep::new()` direct ownership) already in
  production and already tested via `cad_brep_kernel()`/`ProcessKernelReplay`'s own call sites.
- The mesh-io move is net-zero on the sum (−6 framework-3d, +6 stdio) — already reflected above.

`413 + 2951 − 9(double-count removed) − 2(dead-code coverage removed) = 3353`, exactly matching the
measured post-change sum. **Zero unique test coverage was lost**; the fall is entirely explained by
removing tests that were either doubly-counted duplicates or covered dead code.

### Note on measurement freshness

The coordinator flagged a disk-full window (257 MB free at one point, now 199 GB free/confirmed
healthy) that could have produced phantom errors in measurements taken before ~19:30, and a separate
window where stdio was transiently uncompilable during their own brep-flip work. **All census greps
and all verification numbers in this report were taken fresh, after both windows closed, with disk
confirmed at 199 GB free.** One `cargo test -p semio-framework-3d --lib` run did hit an apparent
target-dir stall (timed out at 2 minutes, no `.cargo-lock` file present) — per protocol I did not keep
polling; a second fresh attempt immediately after completed in 1.97s with an identical 396/0 result,
so it is trusted. A separate real bug was caught and fixed during this process: the first draft of the
relocated mesh-io file omitted `mesh_to_dwg_drawing` from its `use crate::artifacts::dwg::{…}` import
line, which `cargo check -p semio-s-plugin-stdio` immediately caught as `E0425`; a subsequent
`cargo check` after the fix confirmed 0 errors.

## Out-of-scope finding (flagged, not acted on)

`🧰️framework/🛍️products/💻️os/🦀️component.rs` (top-level, 4425 LOC) is a near-complete, **unmounted**
duplicate of the real `🖥️host/🦀️component.rs` (4537 LOC, the file actually compiled into crate
`semio-framework-os`). Confirmed via a full `os.path.realpath` walk of every `#[path="…"]` directive in
the repo: zero mounts resolve to the top-level file. It still contains the old, unmigrated
`semio_framework_3d::brep::kernel::*` SolidMediaExport region that the real (mounted) file already had
deleted by wave IO1. Because it isn't compiled, it caused no build breakage and needed no repoint for
Job 1 — but it's dead weight and a whole-file duplication outside this wave's two named jobs (not a
brep-kernel or DWG-codec window). Left untouched to stay inside the assigned mandate; worth a
dedicated follow-up ticket.

## Files touched

**Created:**
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️brep/🧬️schema/⚙️engine/📦️mesh-io/🦀️component.rs`

**Edited:**
- `🧰️framework/🔨️modules/🧊️3d/📐️brep/⚙️engine/🦀️component.rs` (trimmed to 6 kept types)
- `🧰️framework/🔨️modules/🧊️3d/📦️packages/🦀️rust/📦️glue.rs` (removed `kernel`, `mesh_io` mounts)
- `🧰️framework/🔨️modules/🧊️3d/📦️packages/🦀️rust/Cargo.toml` (removed bench + pruned deps)
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️brep/🧬️schema/⚙️engine/🦀️component.rs` (mounted `mesh_io` submodule, repointed its import)
- `🧰️framework/📦️packages/🦀️rust/📦️glue.rs` (removed `pub mod mesh;` + DWG re-export block)

**Deleted:**
- `🧰️framework/🔨️modules/🧊️3d/📐️brep/🧰️kernel/` (whole dir)
- `🧰️framework/🔨️modules/🧊️3d/📐️brep/⚙️engine/🧮️compute/`, `…/🖥️host/` (whole dirs)
- `🧰️framework/🔨️modules/🧊️3d/📦️packages/🦀️rust/benches/kernel.rs` (+ now-empty `benches/` dir)
- `🧰️framework/🔨️modules/🧊️3d/📐️brep/📦️mesh-io/` (whole dir, moved to stdio)
- `🧰️framework/🔨️modules/🔺️mesh/🦀️component.rs` (Rust file only — `🟦️component.ts` kept, unrelated live code)
