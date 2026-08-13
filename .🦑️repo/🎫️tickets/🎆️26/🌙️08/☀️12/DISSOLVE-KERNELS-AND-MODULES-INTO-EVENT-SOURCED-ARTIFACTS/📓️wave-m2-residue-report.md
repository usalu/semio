# 🧱️ Wave M2 — Framework Residue Carved Out of `🧮️math`

Two new framework modules now hold everything in `🧰️framework/🔨️modules/🧮️math/` that a
stdio-forbidden crate names:

- `🧰️framework/🔨️modules/📐️geometry/` → crate `semio-framework-geometry`
- `🧰️framework/🔨️modules/🕸️graph/` → crate `semio-framework-graph`

All five ordered steps completed, including step 5 (deletion from `🧮️math`). The duplication
window is closed: nothing is mounted twice, nothing forwards.

---

## 1. Measured residue

Method (run at the repo root, `./target` and `🧮️math` itself excluded):

```
grep -rn "math::" --include="*.rs" "🧰️framework/"
grep -rn "math::geometry|math::graph|math::random|math::with_shape_ref|math::algebra::(Mat4|Vec3)" --include="*.rs" .
```

78 raw hits in `🧰️framework/`, of which **19 were false positives** and are NOT residue:

| False positive | Why |
| --- | --- |
| `🔨️modules/📚️compiler/**` (13 hits: `crate::math::MathBox`, `crate::math::layout`, …) | the compiler crate's OWN typesetting `math` module, nothing to do with `semio-framework-math` |
| `♾️infinite/🖼️canvas` `ui_styling::metrics::math::ICON_FONT_SIZE_PT` (3 hits) | a constants module inside `semio-framework-ui-styling` |
| `🌊️flow/🖥️host` `semio_s_plugin_flow_extension_math` (2 hits) | a plugin crate whose name contains "math" |
| `🗣️dsl/**` (5 hits) | prose-only doc comments naming `math::graph::dsl`; no code edge |

### Residue table (symbol → consuming crate → evidence → verdict)

| Symbol group | Consuming crate | Evidence (file:line, pre-move) | Verdict |
| --- | --- | --- | --- |
| `geometry::{Affine, ShapeRef, append_shape_to_path, geom_sel, Arc, BezPath, Circle, CubicBez, Line, PathEl, Point, Rect, RoundedRect, RoundedRectRadii, Vec2}` + `with_shape_ref!` | `semio-framework-os-infinite` | `♾️infinite/🖼️canvas/🦀️component.rs:21,223,233,273` | **residue** |
| `geometry::{clamp_f64, distance_between, distance_point_to_cubic_bezier, normalize_or_zero}` | `semio-framework-os-infinite` | `♾️infinite/🎲️board/🦀️component.rs:4` | **residue** |
| `geometry::Vec2` | `semio-framework-os-infinite` | `♾️infinite/🎲️board/➕️normal/↔️undirected/🦀️component.rs:7` | **residue** |
| `random::Rng` (14 `#[cfg(test)]` property suites) | `semio-framework-3d` | `🧊️3d/📐️brep/{➡️vector:387, 〰️polynomial:433/455, 📏️tolerance:221, ✂️curve-ops:449, ➰️curve:382, 🪢️bspline:374/392, 🏟️arena:234, ⚖️predicates:298/317/336, 🔢️matrix:336, 🪡️surface-ops:191}` | **residue** |
| `algebra::{Mat4, Vec3}` | `semio-framework-3d` **and** `semio-framework-ui` | `🧊️3d/🎬️scene/🦀️component.rs:3` — that file is `#[path]`-mounted into TWO crates (`🧊️3d` glue, and `🖱️ui/🎯️targets/🧊️wgpu/📦️glue.rs:222` as `kernel_3d_scene`, re-exported at `:226-229`) | **residue** |
| `graph::{Storage, GraphView, CoreEdge, NodeRecord, Node, Handle, NodeShape, ElementSemantics, Directed, Undirected, Directedness, Normal, Ported, EdgeId, NodeId, HandleId, HandleRole, GraphEdge, PortModel, Interner, Csr, *View, orient_endpoints, property_bag_from_json, property_bag_to_json, FlowNetwork}` | `semio-framework-os-infinite` | `♾️infinite/🎲️board/🦀️component.rs:5-6`; `🎲️board/🔌️ports/➡️directed/🦀️component.rs:160,191,325,340` | **residue** |
| `graph::algorithms::would_create_cycle_ids` (+ the whole algorithms module transitively) | `semio-framework-os-infinite` | `♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🦀️component.rs:1149` | **residue** |
| `graph::drawing::routing::*` | `semio-framework-os-infinite` | `♾️infinite/🎲️board/🦀️component.rs:8` | **residue** |
| `graph::drawing::force::*` | `semio-framework-os-infinite` | `♾️infinite/🎲️board/➕️normal/↔️undirected/🦀️component.rs:8` | **residue** |
| `graph::drawing::tidy_tree::buchheim_positions` | `semio-framework-os-infinite` | `♾️infinite/…/🕸️dag/🦀️component.rs:1154` | **residue** |
| `graph::manifest::{PropertyBag, PropertyValue}` | `semio-framework-os-flow` (8 files) | `🌊️flow/{📄️artifact:14, 🌉️bridge:14, 🖥️host:14, 📚️catalogue:14, 🌉️wasm:14, 🌿️vcs:14, 🖍️drawing:31, 📔️registry:14}` | **residue** |
| `graph::manifest::{ManifestValidator, manifest_by_id, flow_dag::flow_dag_manifest}` | `semio-framework-os-infinite` | `♾️infinite/…/🕸️dag:10,11`; `…/➡️directed/➕️normal:27` | **residue** |
| `graph::manifest::{PropertyBag, PropertyValue}` | `semio-framework-os-flow` (via the `🧠️neural/🦀️component.rs` mount) | `🧠️neural/🦀️component.rs:4` | **residue** |

### Where the hypothesis was wrong

1. **`algebra::Mat2` is NOT residue.** Repo-wide grep for `Mat2` finds it only inside
   `➕️algebra`'s own test module and in unrelated identifiers (`wfc::symmetry`'s private
   `type Mat2 = (i32,i32,i32,i32)`, stdio-gltf's `AccessorType::Mat2`). Left in `🧮️math`.
   Residue from `➕️algebra` is exactly `Vec3` (58 lines) + `Mat4` (139 lines) + 4 + 91 lines
   of their tests.
2. **`🖱️ui` is a residue consumer and was not on the list.** `🧊️3d/🎬️scene/🦀️component.rs` is
   `#[path]`-mounted into `semio-framework-ui`'s wgpu target as well as into
   `semio-framework-3d`. `semio-framework-ui` is upstream of nearly everything, so a
   directory-scoped grep misses it. Its `semio-framework-math` optional dep (in the
   `wgpu-engine` feature array) has been replaced by `semio-framework-geometry`.
3. **`semio-framework-os-kernel-neural-engine` is NOT a consumer.** Its only two `math::`
   hits (`🧠️neural/⚙️engine/🦀️component.rs:1940-1941`) are a comment explaining why a
   migration was deliberately not done. No dep added; comment updated for the new names.
   The real neural consumer is `🧠️neural/🦀️component.rs`, which belongs to the
   `semio-framework-os-flow` crate (mounted at `🌊️flow/📦️packages/🦀️rust/📦️glue.rs:39`).
4. **`🕸️graph/🗣️dsl` IS reachable from framework code** — `♾️infinite/…/🕸️dag:1918,1933,1934`
   and `🧠️neural/🦀️component.rs:3` use `math::graph::dsl::{WireNode, WireEdge,
   wire_literal_from_dag}`. Per instruction it was left in `🧮️math`; those two crates
   therefore keep their `math` dep alongside the new `graph` dep. This is the reason
   step 4 could not be fully verified before step 5 (see §4).
5. **`🕸️graph/🛂️manifest` is much more than a vocabulary.** It `include!`s
   `🤖️generated/🦀️registry.rs`, produced by a `build.rs` + a 343-line `📜️script.ts` that walks
   the entire repo for `🛂️manifest.json` sources and also emits a TypeScript surface. The whole
   codegen machine had to move with it (see §3).
6. **`🚶️traversal` / `🔧️operators` / `➕️normal` / `🔌️ports` are not dead** — they do have
   math-internal references (`crate::graph::{Storage, GraphView, …}`). They have zero
   *external* consumers, so they are NOT residue and stayed in `🧮️math`; they were repointed
   to `graph_core::` (see §3).

### Residue LOC — measured 6,594 vs the ~6,500 hypothesis

| New file | LOC |
| --- | --- |
| `📐️geometry/⚙️engine/🦀️component.rs` (kurbo facade 1,024 + Vec3/Mat4 197 + tests 134) | 1,355 |
| `📐️geometry/🎲️random/🦀️component.rs` | 632 |
| `🕸️graph/⚙️engine/🦀️component.rs` (root vocabulary + its test module) | 2,248 |
| `🕸️graph/🧮️algorithms/🦀️component.rs` | 913 |
| `🕸️graph/🖊️drawing/🦀️component.rs` | 843 |
| `🕸️graph/🛂️manifest/🦀️component.rs` | 603 |
| **subtotal (hand-written)** | **6,594** |
| `🕸️graph/🤖️generated/*.rs` (moved, machine-generated) | 1,550 |

Within 1.5 % of the hypothesis. No red flag: nothing dissolvable was smuggled in — the whole
of `➕️algebra`'s dense/sparse numerics, `🧮️cas`, `🧩️wfc`, `🔢️number`, `📈️polynomial`,
`🎲️probability`, `📊️statistics`, `🔗️causal`, `🎯️sampling`, `🎯️optimize`, `🗺️spatial`,
`📶️signal`, `📋️tabular`, `🌫️fuzzy`, `🎲️entropy`, `🔷️lie` and `🕸️graph/{🗣️dsl, 🚶️traversal,
🔧️operators, ➕️normal, 🔌️ports}` stayed put.

---

## 2. What the new modules look like

```
🧰️framework/🔨️modules/📐️geometry/
├── ⚙️engine/🦀️component.rs      kurbo facade + with_shape_ref! + Vec3/Mat4
├── 🎲️random/🦀️component.rs      seeded xoshiro256** Rng, alias table, distributions
└── 📦️packages/🦀️rust/{Cargo.toml, 📦️glue.rs}      semio-framework-geometry
                                                    deps: kurbo, serde

🧰️framework/🔨️modules/🕸️graph/
├── ⚙️engine/🦀️component.rs      Storage/views/Csr/Interner/Kinds/MaxFlow/PropertyJson
├── 🧮️algorithms/🦀️component.rs  Adjacency, IdIndex, bfs, dijkstra, scc, topo, mst, union-find
├── 🖊️drawing/🦀️component.rs     routing, force, tidy_tree
├── 🛂️manifest/{🦀️component.rs, 🟦️component.ts}
├── 🤖️generated/                 (moved wholesale from 🧮️math)
└── 📦️packages/🦀️rust/{Cargo.toml, 📦️glue.rs, build.rs, 📜️script.ts, 📋️project.json}
                                 semio-framework-graph
                                 deps: semio-framework-geometry, semio-framework-os-kernel
                                       (as `dsl`), neural_engine, serde, serde_json, thiserror
```

Notes on deps that were not in the plan but the code demanded:

- `semio-framework-graph → semio-framework-os-kernel` (aliased `dsl`): `🛂️manifest` implements
  `dsl::DslField for PropertyValue` (21 `dsl::` sites). Confirmed by a failing check, then added.
- `semio-framework-graph → neural_engine`: `🛂️manifest` converts `PropertyValue ↔
  neural_engine::Value`. This preserves the pre-existing one-way edge — `neural_engine` still
  must not depend on graph.
- `semio-framework-geometry` needs **no** `serde_json`/`thiserror`; only kurbo + serde.

The nx project `@semio-tech/framework-graph` now owns the `generate` target (manifest codegen);
`@semio-tech/framework-math`'s `generate` target and its `build.rs` are gone, and its remaining
targets no longer `dependsOn` it.

---

## 3. Ordered steps completed

1. **COPY (additive).** Geometry/random/Vec3/Mat4 copied into `📐️geometry`; graph root split
   at the exact region boundaries (`⚙️engine` = lines 1–1275 + 2193–3165 of the old
   3,165-line file, `🧮️algorithms` = the inner body 1278–2190 of `pub mod algorithms`,
   dedented), drawing/manifest/generated copied into `🕸️graph`. `crate::` paths rewritten
   (`$crate::geometry::ShapeRef` → `$crate::ShapeRef`, `crate::graph::manifest::` →
   `crate::manifest::`, `crate::geometry::` → `geometry::`).
2. **Workspace members added ONE AT A TIME**, with a metadata gate between:
   - added `📐️geometry` member → `cargo metadata --no-deps` → `WORKSPACE_OK`
   - added `🕸️graph` member → `cargo metadata --no-deps` → `WORKSPACE_OK`
   - added `semio-framework-geometry` / `semio-framework-graph` to `[workspace.dependencies]`
     → `WORKSPACE_OK`
   The tree was never left with a dangling member.
3. **Standalone compile + tests**, both green (see §5).
4. **Consumers repointed, one crate at a time** (see §4 table).
5. **Deleted from `🧮️math` in the same change as the mount removal**: `📐️geometry/`,
   `🎲️random/`, `🕸️graph/🦀️component.rs`, `🕸️graph/🖊️drawing/`, `🕸️graph/🛂️manifest/`,
   `🤖️generated/`, `📦️packages/🦀️rust/build.rs`, and the `Vec3`/`Mat4` regions + their tests
   from `➕️algebra`. `📦️glue.rs` lost the four mounts and gained
   `extern crate semio_framework_geometry as geometry;` /
   `extern crate semio_framework_graph as graph_core;`. 42 math-internal files were rewritten
   (`crate::random::` → `geometry::random::`, `crate::graph::` → `graph_core::`, with
   `crate::graph::dsl` deliberately preserved). `Cargo.toml` lost `build`, `kurbo`,
   `neural_engine` and `[build-dependencies]`.

The `math` alias naming had to differ inside `🧮️math` itself: `pub mod graph { … }` still
exists there (dsl/traversal/operators/normal/ports), so the extern crate is aliased
`graph_core` to avoid an E0659 ambiguity. Every other consumer uses the plain `graph` alias.

---

## 4. Consumer repoint table

| Crate | Before | After | Verified |
| --- | --- | --- | --- |
| `semio-framework-3d` | `semio-framework-math` | `+ semio-framework-geometry`, keeps math (`number::Rational`) | ✅ check + 413/0 tests |
| `semio-framework-ui` | `semio-framework-math` (optional, in `wgpu-engine`) | `semio-framework-geometry` (optional, in `wgpu-engine`); math dep removed | ✅ `cargo check -p semio-framework-ui --features wgpu-engine` clean |
| `semio-framework-os-infinite` | `math` | `+ geometry`, `+ graph`, keeps `math` (for `math::graph::dsl`) | ⚠️ blocked-churn (§6) — all my symbols resolve |
| `semio-framework-os-flow` | `math` | `+ graph`, keeps `math` (neural mount uses `math::graph::dsl`) | ✅ lib clean; `--all-targets` blocked-churn (§6) |
| `semio-s-plugin-mathematical` | `math` | `geometry` + `graph` (math dropped) | ✅ check clean |
| `semio-s-plugin-animate` | `math` | `geometry` (math dropped) | ✅ check clean, 225/0 tests |
| `semio-s-plugin-sequence` | `math` | `graph` (math dropped) | ✅ check clean |
| `semio-s-plugin-architect` | `math` | `graph` (math dropped) | ⚠️ blocked-churn |
| `semio-s-plugin-dag` | `math` | `graph` (math dropped) | ⚠️ blocked-churn (stdio only) |
| `semio-s-plugin-cad` | `math` | `+ graph`, keeps math (`math::graph::dsl`) | ⚠️ blocked-churn (stdio only) |
| `semio-s-plugin-puzzle` | `math` | `geometry` + `graph` (math dropped) | ⚠️ blocked-churn (stdio only) |
| `semio-s-plugin-energy` | `math` | `geometry` (math dropped) | ⚠️ blocked-churn (stdio only) |
| `semio-s-plugin-trinity` | `math` | `+ geometry`, `+ graph`, keeps math | ⚠️ blocked-churn (stdio only) |
| `semio-s-plugin-remodel` | `math` | `+ geometry`, `+ graph`, keeps math (algebra/lie/optimize/signal/spatial) | ⚠️ blocked-churn |
| `semio-framework-os-kernel-neural-engine` | (no dep) | (no dep) — comment updated only | n/a |

Two collision fixes were needed in `♾️infinite/…/🕸️dag/🦀️component.rs`: that file already
aliases the board's directed-port module as `graph` (`pub use crate::infinite::board::ports::
directed::{self as graph, …}`), so the four migrated imports use the absolute
`::graph::{manifest,algorithms,drawing}` form, and two `#[cfg(test)]` imports at :6742/:6761
became `use super::graph::…` to resolve E0659.

Repo-wide sweep confirms no stale reference remains in the live tree:

```
grep -rn "math::geometry|math::random|math::algebra::Vec3|math::algebra::Mat4|
          math::with_shape_ref|semio_framework_math::{geometry,random,graph}" \
     --include="*.rs" 🧰️framework ✏️s     → NONE_RS
grep -rn "🧮️math/{🤖️generated,🕸️graph/🛂️manifest,📐️geometry,🎲️random}" \
     --include="*.ts|*.tsx|*.json|*.toml"  → (no hits)
```

---

## 5. Verification — real commands, real output

All runs used the mandated form:
`TD=…/🎯️target`, `touch <glue.rs>`, `RUSTC_WRAPPER="" CARGO_TARGET_DIR="$TD" cargo check -p <crate> --all-targets`.

```
$ RUSTC_WRAPPER="" cargo metadata --no-deps --format-version 1 >/dev/null && echo WORKSPACE_OK
WORKSPACE_OK

### cargo check -p semio-framework-geometry --all-targets
    Finished `dev` profile [unoptimized] target(s) in 0.33s
### cargo check -p semio-framework-graph --all-targets
    Finished `dev` profile [unoptimized] target(s) in 0.66s
### cargo check -p semio-framework-math --all-targets
    Finished `dev` profile [unoptimized] target(s) in 1.67s
### cargo check -p semio-framework-3d --all-targets
    Finished `dev` profile [unoptimized] target(s) in 2.03s
### cargo check -p semio-framework-os-flow --all-targets
error: couldn't read …/📚️examples/🌊️default.flow: No such file or directory (os error 2)   ← foreign
error: couldn't read …/📚️examples/🌊️default.flow: No such file or directory (os error 2)   ← foreign
error[E0425]: cannot find function `assert_dsl_round_trip` in module `store::test_support`  ← foreign
error[E0425]: cannot find function `assert_dsl_pack_equivalence` in …                        ← foreign
(`cargo check -p semio-framework-os-flow` — lib only — is clean, zero errors)

$ cargo check -p semio-framework-ui --features wgpu-engine
(zero errors)
```

Tests:

```
semio-framework-geometry:      57 passed;   0 failed
semio-framework-graph:        113 passed;   0 failed
semio-framework-3d:           413 passed;   0 failed   ← matches the required 413/0 baseline
semio-framework-math:        1568 passed;  15 failed
semio-s-plugin-animate:       225 passed;   0 failed
```

`semio-framework-math`'s 15 failures are **byte-identical to the recorded baseline set**
(`scratch-w0-baseline-failures-sorted.txt`), all in `cas::*` / `polynomial::*` /
`graph::dsl::*`; the sorted list is in `scratch-m2-math-failures-sorted.txt`. The pass count
moved 1738 → 1568 because exactly 170 tests emigrated: 57 to `semio-framework-geometry` +
113 to `semio-framework-graph` = 170. 1568 + 170 = 1738. Nothing was lost.

---

## 6. blocked-churn (other sessions' in-flight work — not touched, not fixed)

| Crate(s) | Foreign breakage | Attribution |
| --- | --- | --- |
| `semio-framework-os-infinite` | 12 errors, all in `♾️infinite/🦀️component.rs` and `🌍️world/🦀️component.rs`: `include_bytes!("../../../../../../../🔨️modules/🖼️assets/…/🧊️capsule_J.glb")` resolves above the repo root, plus `E0608 cannot index into DslValue` | both files mtime `Aug 13 14:18`, `git diff HEAD` shows a wave-G1b mesh/GLB rewrite (`mesh_from_kind` → `placeholder_mesh`, `MeshData` → `WorldMeshBuffers`) that is not mine. Zero errors mention any residue symbol (classified mechanically over all error blocks). |
| `semio-framework-os-flow` | missing `🧰️framework/🛍️products/💻️os/📚️examples/🌊️default.flow` (dir contains only `♻️reuse`); `store::test_support::assert_dsl_{round_trip,pack_equivalence}` absent | `--all-targets` only; lib is green |
| `semio-s-plugin-{architect,dag,cad,puzzle,energy,trinity,remodel}` | `E0753 expected outer doc comment` ×many in `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖊️dwg/🏅️standards/🔖️ac1024/🪆️subsets/✳️any/🚪️io/🦀️component.rs:518+` | that file is uncommitted and mtime `Aug 13 14:38` (live) — the ENGINELESS-ARTIFACTS G2 relocation in progress |
| `semio-s-plugin-remodel` | `E0433 cannot find engine in v1` at `🗿️artifacts/📸️remodel/…/🚪️io/🦀️component.rs:14` | file committed `Aug 13 00:13`, untouched by me → pre-existing |
| `semio-s-plugin-architect` | ~10 `E0433 cannot find <mutation> in super` in `🗿️artifacts/🏛️program/…/🧬️mutations/🦀️component.rs` | file committed `Aug 12 15:50`, untouched by me → pre-existing |
| `semio-s-plugin-{mathematical,sequence}` | 1 and 2 mutation-inverse-law test failures in `🗿️artifacts/…/🧬️schema/🧬️mutations/**` | those files are staged-but-not-worktree-modified (`M ` in `git status`), i.e. another session's artifact-mutation restructuring; my only edits in those plugins are the app component + `Cargo.toml` |
| `semio-framework-ui` | `--all-targets` fails on pre-existing test-only bugs (`super::kernel_3d_scene` used from inside `draw::tests` where it should be `crate::wgpu::…`; `UiTreeActionPlacement` unimported; `Label: From<&str>`) | `🦀️draw.rs`/`🦀️engine.rs`/`🦀️label.rs` last committed Aug 6–12, `git status` shows only `Cargo.toml` modified under `🖱️ui/` (mine) |

---

## 7. Honest remainders

- **TypeScript packaging.** `🕸️graph/🛂️manifest/🟦️component.ts` and `🕸️graph/🤖️generated/*.ts`
  moved with the Rust, but the graph module has **no TS package of its own**;
  `@semio-tech/framework-math-js` still re-exports the manifest surface via a cross-module
  relative import (`../../../🕸️graph/🛂️manifest/🟦️component.ts`) and still exports
  `./🔣️manifest.schema.json` from the new location. TypeScript has no dependency-cycle law, so
  this does not affect the Cargo mandate, but a follow-up should give `🕸️graph` a
  `📦️packages/🟦️typescript` and move the export off `framework-math-js`.
- **`🕸️graph/🗣️dsl` (2,937 LOC) stays in `🧮️math`** as instructed, and is the only reason
  `♾️infinite`, `🌊️flow`, `📐️cad` and `🔱️trinity` still carry a `math` dep. Deciding it will
  free four crates.
- **`🎲️random` lives inside the `📐️geometry` module** per the wave spec. It is not geometric;
  if a third residue module is ever wanted, `🎲️random` is the natural first tenant.
- **`🧮️math` still owns the `graph` module name** (`math::graph::{dsl, traversal, operators,
  normal, ports}`) while the real graph vocabulary lives in `semio-framework-graph`. That is
  why the extern alias inside `🧮️math` is `graph_core`. It resolves itself when those five
  submodules dissolve.
- **`semio-framework-os-infinite` could not be given a green `--all-targets`** because of the
  live foreign edit in §6. Everything I changed in it resolves; re-run once that session lands.
- Repo root `📜️script.ts:294` still calls `runNx("@semio-tech/graph-manifest:generate")` — a
  project name that has not existed since before this wave. Pre-existing, left alone.
