# Current Native World Causal Audit 1

Observed: 2026-10-07T00:58:07.237545+00:00. This is read-only diagnosis over actual physical current files and the surviving epoch-5 Board snapshot. It makes no historical reconstruction, production write, runtime success claim, or assertion change. The earlier controlled-cargo interface authority remains absent; dependent interface model/core commands remain held.

## Evidence custody

The original whole log is `🗑️generated/current-native-origin/epoch-5/board-whole5.log`; the retained assertion audit is `continuation-current-board5-whole-assertion-failure-audit.md`. That run reported 543 executed, 537 passed, six failed. The two World failures examined here are the existing live guest refresh assertion at native unit line 6573 and original zero-index GPU upload assertion at line 6735. This audit did not run a compiler or repeat those tests.

Current versus held full-body SHA-256 comparisons, measured immediately before authoring the proposal:

| Source | Current SHA-256 | Held epoch-5 identity |
| --- | --- | --- |
| `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🦀️.rs` | `f43c0d3a97047f7f64c37f3d93d4c1ffb18535df13dfc87673902f750e875c7d` | same full bytes |
| `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🧪️tests/🔬️unit/🦀️.rs` | `07d5d3c38fe40f59d74173d55fd776fdb2855c2a7ddd28a64a87feb52378660f` | same full bytes |
| `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🧫️fixtures/🎯️component-selection-merges/🔣️.json` | `ad9217a83ddaf4ef3f113d40e8e1b68c5237776c2a67a194c0a3a5d83ec9f2eb` | same full bytes |
| `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🧫️fixtures/🎯️analytic-wire-picking/🔣️.json` | `53ddcb04060e746c634d3afb3cdb5a32d3cc9d97cbbf1e4fa50deed4dfbd432c` | same full bytes |
| `🧰️framework/🔨️modules/🖱️ui/🎬️scene/📐️math/🦀️.rs` | `03c806c4ecb0d0120fe3a3a8d1cc68e841ff56b815d89fa93628d8a2a6b68bc3` | same full bytes |
| `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🖍️draw/🦀️.rs` | `7431cf848bbfc04be2290d1543dfa4aa3886406d5d09d22177aa69d4e8bbe204` | same full bytes |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🌐️World3dHost/🟦️.tsx` | `bda624f34cd004179be8d03ac5a95d6bcbb5dd30b59d241adda954ab13e2e56e` | same full bytes |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🌐️World3dHost/🧪️tests/🧩️component/🟦️.tsx` | `aeff62e78ab4460607eb6d4de8dbd2dc900b67cf79e1f74b3275255513489865` | same full bytes |
| `🧰️framework/🔨️modules/🧊️3d/📐️brep/⚙️engine/🧫️fixtures/🎯️vertex-provenance/🔣️.json` | `4d396c2b6a670c9d8530de639b8a7292dd5567ff60c51e45098a9e0f1f52e3bb` | same full bytes |
| `🧰️framework/🔨️modules/🧊️3d/🧪️tests/🧪️semio-tech-geometry-brep-js/🟦️.ts` | `51445b8646fe8f8516b6618797be5b609ebeef5855ac713eeb5474135b8d769f` | same full bytes |

All identity rows marked “same full bytes” were independently read from both paths. Equality qualifies the causal reading to the tested held source; it is not a new runtime result.

## Live guest refresh: content publication does not advance view authority

The language-neutral `component-selection-merges` fixture supplies `gumball.liveRefresh`: refresh after the first stream, real position offset 0.25, a changed source revision, unchanged captured start targets, and expected stream 0.25 / stream 0.5 / commit 0.25 totaling 1. The native original law uses the actual scene bridge under one-unit grants. Its geometry generation assertion succeeded before the subsequent view revision assertion failed.

In World source:

1. `stage_world3d_scene_bridge` (12882) hashes mesh, instance and camera bodies and stages changed data; equal digests do no work.
2. Parse/Meshes (12969–13056) decode actual producer bodies, compute mesh digests, retire and replace changed resident leases, and include the new instance component source in snapshot pages. `retire_world_mesh` (11687) and `publish_world3d_mesh_lease` (11626) advance geometry generation only.
3. Snapshot publication (13272–13275) advances bridge generation but constructs `World3dSnapshotDescriptor { revision: state.interaction_revision, ... }`. The changed content does not receive a successor view revision.
4. Snapshot apply (11896–11915) computes `max(state.interaction_revision, lease.revision)`, stamps the rebuilt draws with that revision and installs it. An unchanged selection/document/camera lane supplies no separate increment during this fixture refresh. Taking max of the same revision cannot advance it.

The source therefore directly explains the observed first failure. The existing max/re-stamping rule prevents an in-flight delivery from rolling the surface revision backwards; it should remain. A successor bridge content revision must be carried into the sealed descriptor without lowering any concurrent view revision on application. Merely replacing the installed max rule or removing the assertion would hide the defect.

## Live gesture: a revision-only correction exposes another owner defect

There is no current captured string owner in `WorldGumballGesture`. Its selected roster (5610 onward) contains `WorldGumballTarget` entries holding old registry tokens and optional indexes into the current `state.gumball_selection_ids`. `WorldGumballTarget::text` (5530) resolves the current registry token and re-parses the current selection. `WorldComponentAddress::matches_source` (5570) requires the captured revision and current component source to be identical and then resolves the current mesh label.

Three independent guards then conflict with the required refreshed live gesture:

- Authority stepping (6769) starts bounded retirement whenever the stored gumball revision differs from the current surface revision.
- Motion stepping (6032) refuses the changed revision and re-validates each target against the current mesh/source.
- Commit stepping (6398–6416) refuses changed revision and emits IDs only by resolving those same current registry-dependent targets.

The refreshed source deliberately changes its source revision. Re-stamping old tokens or allowing stale tokens would bypass current pick authority and still fail source equality; retaining the old revision would leave the stale interaction registry undiscovered. A correct implementation must separate the current view/pick registry authority from the already-admitted gesture's own captured command targets.

The existing React source is an independent implementation authority for this scope, not an inferred compatibility requirement: `worldGumballStep` at 2957 copies `event.targets.ids` at start and later drag/release/cancel dispatches use that owned start roster. Its mounted component law at 279 uses the same fixture, actually rerenders a mesh with the new source revision, expects the exact three dispatches, and compares projection/deltas through real Three. This audit read that law; it did not run it.

Required complete World correction proposal:

1. Add a bounded captured target-text owner whose count/byte limits remain the existing action limits. At capture, first validate against the current registry/source and copy the exact accepted bytes with fuel and byte grants; refuse overflow. The start IDs include the original 64-hex source handle/revision and exact u64 decimal labels.
2. Transfer that owner into the gesture and into each command publication. Motion/stream/tail/cancel read owned IDs and retained start pivot/anchor/sent pose; they do not select from refreshed registry tokens or current selection.
3. Keep non-gesture picking/marquee/current registry guards. Admit successor bridge content revision and rebuild the current registry, while a current surface-owned captured live gesture retains its command authority across that publication. Host blur/capture loss/window closure still abort and retire the captured owner.
4. Capture, copy, command draft, clone/transfer, cancellation and retirement must preserve one-unit progress, physical byte credits, terminal emptiness and original wide-selection behavior. Preserve every existing fixture and native assertion.
5. Author a complete guarded current-origin World before/after pair only after these coupled cuts are implemented. No partial revision-only World pair is supplied here because it would immediately trigger the documented remaining guards.

## Original zero-index GPU upload: an extra triangle-only rejection

The sealed Mesh3d schema (UI scene math 832) permits zero indices when actual edge or topological vertex authority exists. The seal guard (1199) explicitly rejects a zero-index mesh only if it also lacks edges and a topological vertex. World inline decode (9594–9627) preserves index count independently from render vertex count, expands original edge-only endpoints into render vertices, and publishes the actual zero-index lease. The native GPU law already verified `lease.schema().indices == 0` before requesting upload.

The GPU table (UI target draw 675–679) computes exact vertex/index bytes then rejects `vertex_bytes == 0 || index_bytes == 0`. This contradicts the sealed schema and produces the observed `mesh upload schema was empty` before allocation or upload. The wire is valid and has vertices; it is rejected solely because indices are zero.

An authored current-origin full-body GPU proposal is saved at `cargo-inputs/📥️current-native-world/🧩️gpu-full-pairs-1.json`. It removes only the zero-index rejection and allocates a minimum u32-sized physical index buffer, retaining logical `schema.indices` and resident `index_count` as zero. The existing bounded index upload loop then executes zero writes. This buffer capacity is not an authored index, invented triangle, changed topology or widened semantic credit. Existing renderer call sites retain a valid index buffer, and the draw-owner path clamps to logical count and skips empty ranges. Existing upload close and resident retire owners remain intact.

The proposal has exact current full before/after bodies and SHA-256 values plus same-read current/held authority rows. It is explicitly compilerRun=false, runtimeVerified=false, publicationReady=false, sourceWrites=0. It is an input for Native's guarded stage, not a publication receipt.

## Required original verification

Use the exact registered native successor route after the Low current full-scope source gate. Preserve the whole canonical original Infinite target and its existing laws; no test filter should be used as closure. Retain original live refresh stream/commit/cancel/wide-source laws, raycursor2, original three actual GPU leases and bounded upload/resident retirement assertions. For multi-implementation evidence, rerun the existing mounted React live-refresh Three law and the analytic wire/point Three law through their registered routes where the full source plan requires them. No current success may be asserted until those executions and full postguards are retained.
