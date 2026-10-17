# Rust Mesh Crate Decomposition

## Actual Ownership and Dependency Plan

Read-only Source inventory confirms semio-framework-3d unconditionally mounts mesh, whose modeling file alone is266825 UTF8 bytes. Its true closure additionally mounts retained snapshot/input/plane/translation, surface, quality and eight tessellation children. The existing354249-byte three-file lower bound is therefore not the complete crate closure. The retained job test is223287 bytes and is cfg(test); it will need its own deliberate test-owner cut later. No Cargo metadata/compiler has been invoked.

The mesh root owns HalfedgeMesh and its seven private vectors/maps; modeling directly accesses those fields and adds inherent constructors. Root methods call retained jobs; quality adds inherent methods and depends on crate::inertia; surface also adds inherent methods. Moving HalfedgeMesh to a lower crate while leaving those impls above would violate Rust inherent-impl ownership. Adding a dependency from the lower document crate back to modeling would create a cycle. No such move or facade placeholder will be authored. The TypeScript3D owner has geometry/mesh contract types; retained jobs' existing TypeScript neutral oracle already compares fixture geometry with installed Three/BufferGeometry and fast-json-patch, but is not a runtime implementation of the full Rust state machine.

The first coherent production slice is deliberately narrower than a complete document-engine extraction:

1. `mesh/contract` owns actual neutral Vec3/id/error/operation-enum definitions, owned polygon-soup DATA and vertex provenance/ranked geometry values. Existing firstparty Value codecs/retirement contracts move with their types. The mesh public API intentionally reexports these canonical types; it does not preserve a duplicate implementation. HalfedgeMesh and its private mutation authority stay with their current defining crate in this slice.
2. `mesh/modeling/clipping` owns the existing actual retained solid half-plane clipping machine, private state, child buffers, advance and typed retirement. It depends only on the neutral contract plus existing firstparty Value/MeshEngine, never on the3D facade or HalfedgeMesh. The actual Bevel parent consumes its constructor, advance and closed provenance transfer methods. It receives owned polygon-soup DATA, not a fabricated host/control. Ranked geometry values are shared with the real decimation owner through the contract rather than duplicated.
3. The current3D facade depends on both new real crates. Framework workspace membership is handcrafted adjacent to the existing3D member; only exact manifest hunks are edited, preserving peer changes. No new external runtime dependency or Cargo effect.

These edges are acyclic: 3D→clipping→contract, and3D→contract. Existing lower Value/MeshEngine dependencies do not point back at3D (Source census required by tests). New crate closures must satisfy262144bytes/32files/4000nonblank Source lines; no quota is increased. The existing full3D coordinator closure remains an explicit oversized frontier after this initial slice, never relabeled green. A subsequent coherent cut needs a true document type owner plus intentionally imported modeling/surface/quality extension traits or explicit owned algorithm services; that caller migration must precede any move of foreign inherent impls.

## API and Test Order

Before body moves, a schema/corpus describes clipping input/output geometry and the actual expected package edges/limits. A Source/manifest law must fail on the current same-crate Clip definition and absent consumed dependencies. Pure TypeScript executes the clipping DATA transitions against independent installed Three/Plane line-intersection and SQLite incidence oracles; the existing valid fixture outputs are preserved. Source checks compare retained machine body/phase/provenance/retirement ownership and ensure no private state fields escape. Normal-FS only; no fake renderer/Native/Original/store. Rust laws are authored, not compiled or run under the current HOLD. Existing finite120s ticket Source route will run BOTH modes and strict; no browser/GPU/Bun.build/Rust/Cargo/cache/provider/lease/image/network or build effect.

No significant production edit has occurred at this planning checkpoint. All executable routes remain existing script.ts commands, and all generated evidence belongs to this ticket.

## Coherent First Boundary

The actual neutral owner is `semio-framework-3d-mesh-contract`, with lib Source at `mesh/contract/Rust`. It exclusively defines Vec3, VertexId/HalfEdgeId/FaceId/EdgeId, WeldMode/MirrorAxis, MeshKernelError/MeshResult, MeshPolygonSoup, MeshSourceVertex and MeshRankedEdge. Value codecs, deterministic ordering and controlled retirement moved with their defining types; no old definitions/foreign retirement impls remain in the facade/coordinator. The public mesh reexport is the intentional aggregate API over these canonical definitions. MeshVertex/HalfEdge/MeshFace/MeshTransfer/HalfedgeMesh and private document mutation remain with the original mesh owner in this initial cut. Existing consumers of the aggregate API therefore resolve the same one lower type owner without compatibility wrappers.

The actual clipping owner is `semio-framework-3d-mesh-clipping`. It owns all twelve original dynamic clipping buffers and its retained advance machine. Bevel now imports MeshClipJob from that real dependency, calls its advance against the exact owned soup and consumes closed completed provenance accessors. Public mutable state is not exposed. A private completed flag closes further advance; three separate transfers are each allowed only once after completion, while vertex provenance remains indexed by the actual parent's current origin mapping. Parent Work snapshot/retirement/cancel/grant pathways remain present. The clipping retirement ledger still covers every original dynamic buffer; new scalar terminal bits need no owned-buffer retirement.

The actual edge predicate now has one extracted Rust definition consumed by the actual machine; its signed distances, strict epsilon crossing, float32 interpolation and f64 provenance conversion preserve the original formula. Repeated intersections may compute one extra constant-size candidate point before the retained intersection map deduplicates it; there is no new unbounded loop/allocation or Source cache. The adjacent TypeScript implementation uses explicit float32 arithmetic and an authored three-coordinate result rather than unchecked array casts. Shared JSON gives seven finite crossing/inside/outside/on-plane/epsilon/axis cases.

Framework workspace membership and the3D/clip package dependency manifests are real edited owners; their `[lib]` paths point directly at those actual bodies, not placeholders. Existing Value/ValueDerive/MeshEngine dependencies are firstparty; serde/serde_json remain test-only where used. No external runtime dependency was added. No Cargo.lock was hand-edited or refreshed: actual Cargo metadata, managed lock resolution and Rust compilation remain gated.

## Measured Source Limits and Remaining Frontier

| Authored owner closure | Rust files | UTF8 bytes | Nonblank lines (conservative upper bound) |
| --- | ---: | ---: | ---: |
| New neutral contract root |1|6848|140|
| New clipping root including authored cfg(test) child |2|11726|153|

Both new manifests retain maxSourceBytes262144, maxSourceFiles32 and maxActiveLines4000. Tree-sitter parses both new roots/the authored test child plus the actual current parent mesh/modeling files. This is syntax and explicit Source/mount evidence only, not Rust type resolution, macro-expanded/cfg-selected input or successful compilation. No Cargo source-boundary guard was bypassed.

The current three-file unconditional old3D lower bound is still340210bytes (package root2780 +mesh79675 +modeling257755), above262144 by78066. The full old closure is larger because its other mounts remain. The standalone modeling file is now257755bytes; that fact does NOT make its whole crate eligible. The existing giant retained job test and plugin facade tests also remain unresolved. Full no-giant-Rust fulfillment is not claimed.

The next coherent scope is a true HalfedgeMesh/document owner with all seven private fields plus deliberately owned lower access/parts operations; move model-dependent inherent convenience/job constructors into an imported MeshModeling extension trait or explicit owned service before crossing the type boundary. Surface/quality need their own intentionally imported extension/service owners, and quality's crate::inertia dependency needs a neutral lower direction. Every explicit method caller must migrate together; a facade→modeling→facade cycle is forbidden. The newly consumed clipping family is already below that future coordinator and does not need a return dependency.

## TDD and Retained Failures

RED1 hit the unchanged output cap because a failing whole-string matcher printed the giant original Source. It remains a failed Original; no cap was raised. RED2 uses a bounded boolean observation and genuinely fails because the actual same-crate Clip owner is present. The geometry schema/Three/SQLite vector law passed at that RED checkpoint. Green1 passed2/44 after the real package/body handoff; Green2 passed3/71 after adding independent Rust grammar parsing and complete clipping retirement incidence.

Green3 stopped in the shared adapter prerequisite on a real coedited preparation custody line79 syntax error, before mesh tests. Root independently observed the later changed Source bytes parse clean (SHA873d2161bea7d20920366d4f196eb09379e26c7d4d1e038421d003ae1341a41d); no author/cause was inferred and no fix made here. Green4 exposed a test-only SQLite variable shadowed by an oracle distance number; the distinct names corrected it. Types1/2 exposed exact tuple-vs-JSON-array assertion contracts; the actual firstparty result now has an explicit dense tuple and its schema-validated expected JSON is compared byte-for-byte rather than cast. No case/assertion/root/strict flag was removed.

Current green5 BOTH is3tests/80assertions; the shared adapter independently passes10tests/216assertions in each wrapper invocation. Rust edge/provenance tests are authored only, not run. Exact current strict successors and hashes follow after terminal collection.

## Literal External Caller Census

This is repo-wide literal `semio_framework_3d::mesh` incidence in Rust Source, not resolved method dispatch or cfg-selected compilation. The newly private clipping family has exactly one production parent, the actual Bevel owner; no external caller can access the removed old private Clip definition. Existing aggregate API callers remain on the canonical reexport.

Found 46 literal caller files.

- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/📐️geometry/🌗️mesh-shading/🦀️.rs`
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/📐️geometry/🔢️math-values/🧪️tests/🎟️original/🦀️.rs`
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/📐️geometry/✅️analysis-check/🦀️.rs`
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/📐️geometry/🚀️original-field/🦀️.rs`
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/📐️geometry/🚀️original-field/🧪️tests/🦀️.rs`
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/📐️geometry/✏️mesh-edit/🦀️.rs`
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/📐️geometry/🧪️tests/🧰️oracle-support/🦀️.rs`
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/📐️geometry/💎️value/🦀️.rs`
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/📐️geometry/🔌️inputs/🦀️.rs`
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/📐️geometry/🎚️mesh-component/🦀️.rs`
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/📐️geometry/↔️mesh-transform/🦀️.rs`
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/📐️geometry/🥽️mesh-primitive/🦀️.rs`
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/📐️geometry/🧱️mesh-support/🦀️.rs`
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/📐️geometry/🧱️mesh-support/🧪️tests/🔬️unit/🦀️.rs`
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/📐️geometry/🧱️mesh-support/🧪️tests/🧰️mesh-harness/🦀️.rs`
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/📐️geometry/🔎️mesh-inspect/🦀️.rs`
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/📐️geometry/🗺️mesh-uv/🦀️.rs`
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/📐️geometry/⚙️compute/🎟️original/🦀️.rs`
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/📐️geometry/⚙️compute/🎟️original/📐️read/🦀️.rs`
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/📐️geometry/🔀️mesh-convert/🦀️.rs`
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📐️geometry/📼️mesh-interchange/🦀️.rs`
- `✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🦀️.rs`
- `✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🗑️object/🦀️.rs`
- `✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🗑️object/🧪️tests/🔬️unit/🦀️.rs`
- `✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🔷️mesh-edit/🦀️.rs`
- `✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📤️media/🧪️tests/🔬️unit/🦀️.rs`
- `✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧵️uv/🦀️.rs`
- `✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧲️transform/🦀️.rs`
- `✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧲️transform/🧪️tests/🔬️unit/🦀️.rs`
- `✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🌐️model/🦀️.rs`
- `✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📝️text/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs`
- `✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📝️text/📸️snapshot/🦀️.rs`
- `✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧭️view/🦀️.rs`
- `✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs`
- `✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔌️mounted/🦀️.rs`
- `✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️.rs`
- `✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🧪️tests/🔬️unit/🦀️.rs`
- `✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🗿️artifact/🧪️tests/🔬️unit/🦀️.rs`
- `✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🖌️session/🦀️.rs`
- `✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs`
- `✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧪️tests/🧭️normal-channels/🦀️.rs`
- `✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs`
- `✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧪️tests/🔬️export-concrete-forest-mesh/🦀️.rs`
- `✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧪️tests/🔬️unit/🦀️.rs`
- `✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🕸️mesh/🦀️.rs`
- `✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/🥽️mesh/🦀️.rs`

## Final Source Freeze

Final green6 prototype and release each collected terminal exit0:3tests/80assertions for the consumed mesh slice, plus the shared adapter's separate10tests/216assertions. Serialized types3 prototype and release each collected terminal exit0 with empty diagnostics, retaining the actual helper/test/wrapper import closure and strict/noUncheckedIndexedAccess flags. No validation deadline or output limit was raised. All owned handles are terminal.

Four matching Source test/type prototype/release launch entries are registered in both actual editor catalogs, with BUN_RUNTIME_TRANSPILER_CACHE_PATH=0 and NODE_DISABLE_COMPILE_CACHE=1 at parent birth. They invoke the existing ticket script.ts test/types-owned commands and its closed SEMIO_MESH_CRATE_SOURCE_ONLY selector. No new executable file or RuntimeConfig production quota override was introduced.

The final Source test-only serialized comparison retains all exact vector values while giving the JSON fixture and authored tuple their actual contracts; it does not suppress an unchecked Source read. The independent Rust grammar parser also parses the current mesh root/coordinator, but does not typecheck/link/expand derives or execute Rust. The two authored Rust laws cover shared edge vectors and retained completed/one-shot provenance; they remain uncompiled/unrun. Actual Rust behavior, real Cargo graph/lock refresh, complete3D source eligibility, compiler memory/performance, allocation and runtime remain unqualified.

The next task is the real document/extension-trait and retained-support/family split. This first coherent consumed boundary is frozen for independent review before that widening. The whole no-giant-crate goal is still active; old3D minimum340210bytes and the larger actual mounted closure are not a passing Source budget. No global writer/CPU/storage/capacity/floor/Original/Native authority owner was changed.

## Frozen Source Hashes

| Source | SHA-256 |
| --- | --- |
| `🧰️framework/Cargo.toml` | `ac530aa5668fd4524340cb3f2d4646ffdd8823d57f2379e247266f91ad616508` |
| `🧰️framework/🔨️modules/🧊️3d/📦️packages/🦀️rust/Cargo.toml` | `7ca87a03d766b95cc2d001b262a0967e2a04ae6283d69d171fb6d14b104a28a3` |
| `🧰️framework/🔨️modules/🧊️3d/🥽️mesh/🦀️.rs` | `0ee0b72c2a5873bee7dd68e95b7e58a579635059cf405912ede0201845a6fee1` |
| `🧰️framework/🔨️modules/🧊️3d/🥽️mesh/🛠️modeling/🦀️.rs` | `1dc05ff7fc3114e0ec869edf1b7439e1e433a8169e27856bc6cbe11007b1a563` |
| `🧰️framework/🔨️modules/🧊️3d/🥽️mesh/🧬️contract/🦀️.rs` | `6e0a60c777562b5e655a3d21750fe371110d42545e1c88eb87110133b5741732` |
| `🧰️framework/🔨️modules/🧊️3d/🥽️mesh/🧬️contract/📦️packages/🦀️rust/Cargo.toml` | `68ac7810849489612c2527029b726f869ecbe83131f4d08d1394e96c3a10da0f` |
| `🧰️framework/🔨️modules/🧊️3d/🥽️mesh/🛠️modeling/✂️clipping/🦀️.rs` | `e21618b7bb6c5351d220227413e01658c07465a96f61330b75b3abf326b86f8b` |
| `🧰️framework/🔨️modules/🧊️3d/🥽️mesh/🛠️modeling/✂️clipping/📦️packages/🦀️rust/Cargo.toml` | `694d62f9589a89da796d81e2e64ef6036067518d1b3bb66a8ca318cead364570` |
| `🧰️framework/🔨️modules/🧊️3d/🥽️mesh/🛠️modeling/✂️clipping/🟦️.ts` | `87a1f34b25e4ca15c6ac50cfa940c68434cf907f9cd14a92cf680a582139c76f` |
| `🧰️framework/🔨️modules/🧊️3d/🥽️mesh/🛠️modeling/✂️clipping/🧬️schema/🔣️.json` | `de3b05b64debf00be48c68df37e67c56261e5d3e07957b30a2b8279815aa1e11` |
| `🧰️framework/🔨️modules/🧊️3d/🥽️mesh/🛠️modeling/✂️clipping/🧪️tests/🔣️.json` | `3f5324ddcbf4d2c8e01db1fe3c7ba5bd540147652ab6c686de99353586567279` |
| `🧰️framework/🔨️modules/🧊️3d/🥽️mesh/🛠️modeling/✂️clipping/🧪️tests/🟦️.ts` | `0c9a79c5e2fce6c1a0aa71fa953485c9c128a9509373bd0e8c33a389b214cfc4` |
| `🧰️framework/🔨️modules/🧊️3d/🥽️mesh/🛠️modeling/✂️clipping/🧪️tests/🦀️.rs` | `a4ffb8c6008f61ab1f8293cdf5d6f765c22d83788a2c9caf23f1d7347c87ca1c` |
| `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️13/MODULAR-MONOREPO-BUILDS-AND-SHARED-CACHES/typescript-artifact/📜️script.ts` | `3a08d6d0310063c43cb932efac5920af524721e0f469d654843425a5398e9029` |
| `.vscode/launch.json` | `bb0bf04a644a3f71a55a69e74d162a9e7adfff9582f45d06d937accf95e53f40` |
| `.vscode/🧩️launch.seed.jsonc` | `e9efe9e0f8ff9d871af207fc11a13292a88ebeaccd812f63cc03ee6de457f85a` |

Shared Framework/workspace/editor/ticket files are exact observations and remain available for coordinated peer edits; hashes do not declare a global Source freeze.

## Retained Output Hashes

| Ticket output | SHA-256 |
| --- | --- |
| `🗑️generated/mesh-crate-green1-prototype.log` | `894dc864f9035fdcc71294e1955628b1f81b9e0891883af03a763102f8f53427` |
| `🗑️generated/mesh-crate-green2-prototype.log` | `0ffe2c24824d9453bf420627ac08181aa96c41b54bb60e124cc4d3caaa6dd0b1` |
| `🗑️generated/mesh-crate-green3-prototype.log` | `fabbfc595345aedcbfe4c6e8b7abe0431551af486431769be48f7b6c3606bff4` |
| `🗑️generated/mesh-crate-green4-prototype.log` | `0b5a2bd30d33d178fc7b4bc037bdf8b8b199d8d64b9c77a1504c80a9c00b99e0` |
| `🗑️generated/mesh-crate-green5-prototype.log` | `18410b33de9a2f439a956498ccd48e331b65c47214f7ca8c4dc16f696d4eafa2` |
| `🗑️generated/mesh-crate-green5-release.log` | `03e4f70103b89c6c9f3e351b9e8ad93f06e5b8106b69b72669cd8753af219072` |
| `🗑️generated/mesh-crate-green6-prototype.log` | `227312ee16d995750811c7f8f111120136a9e6865de518ab3c6a8fad90263a6e` |
| `🗑️generated/mesh-crate-green6-release.log` | `793765715750fd16a4e8ed911720d2ef9ad54fe9568c54e2cad2bfb401a504ed` |
| `🗑️generated/mesh-crate-red1-prototype.log` | `c57a8afe3d5979e51ed202de601e9c26e5df102e666b23761a11bd1362b0db7e` |
| `🗑️generated/mesh-crate-red2-prototype.log` | `316e12a728772d543cfdd1713598bb89cf411babb909f5806f799a082213c12a` |
| `🗑️generated/mesh-crate-types1-prototype.log` | `07da52f4aecd820c26e26e8046acf4878fc3a16b226a013540b4180b0762e875` |
| `🗑️generated/mesh-crate-types2-prototype.log` | `7df71369309dd0ec948fa4c03efee3a1e3b603504638aff25b55f3adddc119b9` |
| `🗑️generated/mesh-crate-types3-prototype.log` | `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855` |
| `🗑️generated/mesh-crate-types3-release.log` | `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855` |

## Next Consumed Document and Modeling Boundary — Design

The independent first-cut audit is clean. Its 340210-byte three-file lower bound remains a real refusal. The next cut moves the defining document, not another file mounted by the same crate. Production dependency direction is 3D facade → retained modeling → document → contract, with retained modeling → clipping → contract. Document never imports modeling or the facade. Vec3/id/error/soup/provenance remain single-owned by the existing contract.

The document crate will own HalfedgeMesh, MeshVertex/HalfEdge/MeshFace/MeshTransfer, literal owned document parts, direct topology edits, first-party codecs, tessellation/OBJ conversion, surface work and quality/inertia. Modeling-dependent inherent convenience methods move to the required MeshModelingOps extension trait in the actual retained modeling crate; all actual method callers import that trait. The primitive/affine/edit jobs stay their existing retained implementations. Shared scalar progress moves to contract; the result enum containing the actual HalfedgeMesh lives with document. No foreign inherent impl or document→facade edge is permitted.

Ordinary owned document fields are a deliberate explicit public representation: they contain only vectors, typed indices and first-party attribute DATA, with the existing from_owned_parts already admitting those exact values. They are neither process authority nor an authority constructor. Retained source projection and attribute remap move with their defining data types and existing retirement implementations, allowing their actual consumed modeling callers to use one canonical implementation. Private job internals remain private except existing cross-owner operations promoted to explicit owned-data operations.

Planned modules: mesh/📄️document/{🦀️.rs,📦️packages/🦀️rust/Cargo.toml}, existing mesh/modeling root becomes its package root with its own manifest. Existing tessellation/surface/quality children remain adjacent to their domain and are genuinely mounted only by the new document owner; no duplicate bodies. Inertia moves its actual defining mount to document and is intentionally reexported by the 3D public API. Existing source snapshot/remap algorithms become document children and are removed from modeling. The facade becomes intentional public reexports plus its higher-domain tests.

Tests remain with the defining private types. The document serde oracle and private tessellation tests run as authored document Rust unit tests; higher modeling/surface/quality tests retain all assertions with explicit extension-trait imports. No dependency-cycle test facade is used to fabricate matching Rust types. A language-neutral schema/corpus will exercise topology import/conversion boundary through the existing TypeScript twin and independent Three/SQLite oracles; Source tests independently parse real Rust mounts, crate DAG and extension-trait consumers. All new production closures must satisfy the unchanged 262144-byte/32-file/4000-line policy. Test-only Source totals and all uncompiled Rust remain reported separately.

Framework workspace membership, actual manifests, literal callers and both existing launch catalogs are updated together by narrow hunks. Existing four mesh test/types commands grow their cohort; the same strict safety roots remain. No Cargo metadata, compilation, native/provider/storage, image, browser, network or physical build action is authorized or claimed. This design is Source-only; Rust type resolution/macro expansion and runtime parity remain HOLD.

## Document Slice Implementation and Current Verification

The narrower first slice is now implemented. A new genuinely consumed `semio-framework-3d-mesh-document` crate owns the defining HalfedgeMesh, MeshVertex/HalfEdge/MeshFace/MeshTransfer, their first-party codecs/retirement and actual topology/conversion/surface/quality/inertia bodies. The original 3D mesh module intentionally reexports this public domain API and retains modeling. No second HalfedgeMesh/Vec3/id/error/soup implementation exists. Inertia has one defining mount in document and an intentional 3D reexport. Production document dependencies are contract, MeshEngine, Value/derive and PackJSON; none returns to 3D/modeling/clipping. Existing Parry/serde oracles are dev-only.

The actual higher modeling owner implements required `MeshModelingOps` (93 public operations) and private `MeshModelingInternals` (7 implementation helpers) for the lower document. The cold translation constructor is `MeshTranslationSource`. Concrete return/receiver/parameter/generic/where contracts are independently compared from actual Rust AST declarations and impls. No `pub` qualifier remains inside trait impls; declaration-only binding patterns are removed. Public signatures contain no private Work/Transform/PositionPolicy/SourceAttributeRemap/Affine3. Eighteen actual explicit caller imports are captured in document/tests/callers JSON; the wider literal current 47-consumer census is retained separately and consumed by the Source law. Inner module attributes remain before imports. Globs intentionally import the canonical extension trait. This is a literal Source census, not Rust name/type resolution.

The seven actual owned document collections are explicit public DATA; these are the same vectors/indices/first-party attributes already constructible with from_owned_parts, not process/storage authority. Cross-owner add_vertex and polygon_soup, canonical four vector arithmetic functions, polygon normal/triangulation functions, and tessellation selected/take_source are intentional first-party owned-data operations. Reconstruction, topology-remap refusal and unreferenced-compaction helpers remain private. Surface/tessellation job internals remain private.

Private tessellation tests and the actual serde document codec oracle now remain with their defining document type. The codec oracle uses a directly authored nonempty cube document instead of importing the higher primitive constructor: all equality assertions and refusal codec checks remain. Two private retained tessellation ownership laws move unchanged to a defining-owner test child with corrected fixture-relative paths. Higher modeling/unit/surface/quality assertions remain in the actual 3D facade test target with genuine trait imports. A remaining public-only tessellation cancellation assertion obtains its grant from actual public demand methods rather than a foreign private test helper. New Rust topology corpus tests are authored but UNRUN.

New language-neutral triangle/shared-diagonal vectors are schema-validated by Ajv, consumed by the ordinary first-party TS document projection and authored Rust document test, and independently checked against Three geometry normals/vertex counts and SQLite edge incidence. Source tests parse every literal cfg-possible document child, actual migrated callers, current original modeling inputs, both actual extension contracts and old clipping/contract invariants. Every literal include_str fixture path in the new defining closure is checked. These checks do not establish Rust compilation, macro expansion, native behavior or cfg-selected compilation.

Current literal counts: document unconditional 12 files / 175582 bytes / 2812 nonblank lines; conservative cfg-possible 25 / 242830 / 3181. Existing 3D unconditional 9 / 338902 / 3584; cfg-possible 16 / 689653 / 6435. Old three-file lower bound is now 281463 bytes (19319 over 262144), reduced from 340210; modeling alone is 277986 bytes. The overall no-giant-crate goal remains OPEN. Next actual cut should move retained source attribute/provenance remap and source projection into independently bounded document-dependent services before separating retained family crates; mounting more files in modeling cannot satisfy this limit. No bound was raised and no oversized modeling package was minted.

Verification progression: red1 genuinely failed the old facade owning HalfedgeMesh. Green1 failed only a newly mistyped relative TS fixture import; green2 exposed the prior first-cut Source assertion expecting a direct contract import after ownership moved (updated to assert actual document->contract plus facade->document). Green3 passed6/172; green4 passed7/376. Types1 caught an actual TS union-array reduce accumulator inference; explicit numeric accumulator typing corrected it, types2 prototype terminal0. Later green5 never reached mesh: shared adapter prerequisite failed9/196 on external viewport schema Source `export { default as 3dSchema }`, an invalid identifier. This failure is retained and reported to Root without attribution. Final current epoch verification follows after the actual external Source correction; no unchanged-luck retry or cap/deadline increase.

All existing four mesh test/types commands and both editor launch rows remain the actual launch surface; the same existing selected test root grows its cohort and the full previous strict safety root/wrapper is preserved. No new executable script exists. All new paths are at most101 characters absolute. No Rust/Cargo/metadata/compiler/Native/image/cache/browser/service/network action ran; Cargo.lock remains unrefreshed.

## Frozen Document Successor

Final green7 prototype and release each terminal exit0: **7 tests / 398 expectations**, plus the unchanged shared adapter prerequisite **10 / 216** each. Final types3 prototype (session32224) and release (session55605) were collected sequentially with **exit0**, empty diagnostics, preserving the prior actual test root and full shared wrapper strict safety root/flags. The parent and every validation child disabled Bun and Node compile caches at birth. No handles remain live.

Green6 is retained as a genuine Source fixture-path failure: the newly relocated two ownership laws used one too many parent segments. The consumed include_str filesystem law found it; both paths now resolve to the same actual existing owners fixture, and green7 covers every literal included fixture in the lower document closure. Green5 remains the distinct external prerequisite failure; the current coedited viewport schema now uses valid viewport3dSchema, enabling a substantive successor. Earlier results have not been relabeled. Types1 union accumulator error remains retained; types2 prototype0 preceded the final contract-law additions, so types3 is the final strict epoch.

Exact final literal Source closure (conservative, no Cargo/cfg execution):

| Owner | Selection | Rust files | Bytes | Nonblank lines |
| --- | --- | ---: | ---: | ---: |
| document | unconditional | 12 | 175582 | 2812 |
| document | cfg-possible | 25 | 242824 | 3181 |
| 3D | unconditional | 9 | 338902 | 3584 |
| 3D | cfg-possible | 16 | 689653 | 6435 |

Final old three-file lower bound **281463 bytes**, still **19319** above262144; retained modeling alone **277982**. These supersede the provisional pre-final byte counts above. The new document is bounded in production and cfg-possible test mounts; the existing higher aggregate remains oversized. No Rust typecheck, compilation, actual algorithm runtime, full target-resolved dependency graph or Cargo.lock update is qualified.

Next bounded design: a document-dependent retained provenance/remap owner can move the actual SourceAttributeRemap/SourceValueEquality state machines and their retirement incidence out of higher modeling without a return edge. The source snapshot family must likewise move its defining state with its retained preparation operations, not merely its file. That is a separate coherent algorithm boundary, with its own vectors and complete cfg-possible closure measurement before a new modeling crate is minted.

Applicable root/3D/s/Lowpoly/Flow/Procedural instructions were read; no AGENTS or Git state changed. No launcher/wrapper edit was needed this slice: the existing four registered mesh rows execute the growing actual cohort. The three no-longer-cross-owner reconstruction helpers remain private; no free authority constructor/Native port/control was introduced. Removed the now-unused direct 3D PackJSON edge; document retains its genuine codec dependency.

### Exact Source Inventory

Owned changes and creations (hashes are final observed bytes):

| Source | Bytes | SHA-256 |
| --- | ---: | --- |
| `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📐️geometry/📼️mesh-interchange/🦀️.rs` | 10619 | `68d527b2f910b33bee5ccdfe7d3f3fdafa7eb1cd56df31dfdf8670f52f2144be` |
| `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/📐️geometry/↔️mesh-transform/🦀️.rs` | 3458 | `f0f6189d3749bff94db339a27b45b698b1f12614f6adba7cb33f6f97b157b371` |
| `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/📐️geometry/✏️mesh-edit/🦀️.rs` | 6979 | `1a8e243fd696072061655a5ecc5b7a755a2f4f195eaff502d643a038af7c5894` |
| `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/📐️geometry/🎚️mesh-component/🦀️.rs` | 5453 | `8c710f83cefe7f762461df58d6c066779bed35b0877afa7ce394521852fd404e` |
| `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/📐️geometry/🔀️mesh-convert/🦀️.rs` | 7142 | `9862295a28413a0fcb3334a6eaa04fd7088b28494954b742c4e106e9c1572996` |
| `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/📐️geometry/🥽️mesh-primitive/🦀️.rs` | 4686 | `aa3aa1b123d78699576111e203fdf7af3041e30742d3a853cc1a774007c1ca56` |
| `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/📐️geometry/🧱️mesh-support/🦀️.rs` | 18499 | `c7bdf1f33b45126b6563384eac3f76e6f719eb63d117f1813285da60a8704bf3` |
| `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/📐️geometry/🧱️mesh-support/🧪️tests/🔬️unit/🦀️.rs` | 6057 | `0b31d118b88c49eb4dab691cfbb023e3e2b68aa22a6fff874e331fd949f593bf` |
| `✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/🥽️mesh/🦀️.rs` | 102443 | `f959f19ef8a34c24f6b11742ab56c98a09569ca9f0ae70539d07990bf39d19b6` |
| `✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/🥽️mesh/🧪️tests/🔬️unit/🦀️.rs` | 96442 | `ee1e71e246a62d818d216aa224c86848d7bed7ab747c302541b1384033fc87d5` |
| `✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🦀️.rs` | 25249 | `8cfc57f84714e170eb25a101a31d215da119509a4fc447509d0328977538e4c7` |
| `✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🔷️mesh-edit/🦀️.rs` | 16557 | `b15ca7ff77d4ac6c4f5d41434d9db49de1dfe2ae2b0f259d44ec02ab922fab07` |
| `✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🗑️object/🦀️.rs` | 7924 | `1b68d684103bf6c2ea490cd324172aecaae5d33d650f2027090ece54f1827e4a` |
| `✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🗿️artifact/🧪️tests/🔬️unit/🦀️.rs` | 11333 | `f23116eb81a2a2d559e8adc99646643aa2da46c6e918409e794f3ef7c384d675` |
| `✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs` | 35037 | `fab59035ad62a4ed59f7c5f169d4877142475affb843845683a96b0a23b0551c` |
| `✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📝️text/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs` | 5771 | `39c2601204957e1f83973b9394d3695071d72248bdce44975a541831a994fe5d` |
| `✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🧪️tests/🔬️unit/🦀️.rs` | 14260 | `e5171f43ba7a6807d0a462ebec2089a8a33add31db8ccaf37d84aac9cb643472` |
| `✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧪️tests/🔬️export-concrete-forest-mesh/🦀️.rs` | 4830 | `ffb9c2a5e0cb6f9f5cbf5ac4303d53c6e828b6514a81910979ef4755d09f0b26` |
| `🧰️framework/Cargo.toml` | 40765 | `e2e81fce0cd07e6c1e1632da2d67f8fd70419c7541cf54abe1cf810f232d4327` |
| `🧰️framework/🔨️modules/🧊️3d/📦️packages/🦀️rust/Cargo.toml` | 1989 | `dcc75508f2aa55c79da2e4555f256d015a9f24ac964bca9b8e0b9bfaa2d2e5af` |
| `🧰️framework/🔨️modules/🧊️3d/📦️packages/🦀️rust/🦀️.rs` | 2770 | `97f0db9506656cb9ac8a3839ce40736d6ce3659973aad56cbd017b3421b39f2e` |
| `🧰️framework/🔨️modules/🧊️3d/🥽️mesh/🎨️surface/🦀️.rs` | 31292 | `5384f76a1813e3007f99df779ea7eaa46b999dd19635b4b96fe53fe95698bb08` |
| `🧰️framework/🔨️modules/🧊️3d/🥽️mesh/📄️document/📦️packages/🦀️rust/Cargo.toml` | 1021 | `c5d8137dce1fc1403f13477608ff9abc35d07146bfc7ffdb918981ea3e28706a` |
| `🧰️framework/🔨️modules/🧊️3d/🥽️mesh/📄️document/🟦️.ts` | 2248 | `47864862002f501ddce70ae29cdf90d67488a593b0bb8e90835434cb2a811945` |
| `🧰️framework/🔨️modules/🧊️3d/🥽️mesh/📄️document/🦀️.rs` | 68069 | `068dcfee3641e663d926552e06c37ede582b3309914082048d51424abbc40961` |
| `🧰️framework/🔨️modules/🧊️3d/🥽️mesh/📄️document/🧪️tests/📚️consumers.json` | 9143 | `d7fac9f7e6bed1ae44030c61765fdd437857e9a8b1bfaceb9235d556d37307a1` |
| `🧰️framework/🔨️modules/🧊️3d/🥽️mesh/📄️document/🧪️tests/📥️callers.json` | 3404 | `4029eaa581b31f75515782193deb9273fce67b8a8cdaddf45ca1e6ac900c4780` |
| `🧰️framework/🔨️modules/🧊️3d/🥽️mesh/📄️document/🧪️tests/🔣️.json` | 918 | `59c5e892ddc5cf8fb51a0ab2934d22393af9658f3011269f9ef09f48f5ec83e5` |
| `🧰️framework/🔨️modules/🧊️3d/🥽️mesh/📄️document/🧪️tests/🦀️.rs` | 4005 | `800a9ddd62ee98b59faad0529e0d082d4d8ff8576c4e935aebfea9d3880b3363` |
| `🧰️framework/🔨️modules/🧊️3d/🥽️mesh/📄️document/🧬️schema/🔣️.json` | 1419 | `7f844c14eb36a698136e1b801cc96c9cbb7bda9f8c6bb6b297b20aaec4d99fc6` |
| `🧰️framework/🔨️modules/🧊️3d/🥽️mesh/🔎️quality/🦀️.rs` | 17353 | `dc060941254732d61b054fe29637dcd8b2d035ade1ee8d61086702e94b4db1f4` |
| `🧰️framework/🔨️modules/🧊️3d/🥽️mesh/🛠️modeling/✂️clipping/🧪️tests/🟦️.ts` | 15076 | `b2a757967cc205a6b1bce42f6513bed23e883e7101bf6b508985b26e67e1e650` |
| `🧰️framework/🔨️modules/🧊️3d/🥽️mesh/🛠️modeling/📥️input/🦀️.rs` | 4202 | `e1dc8d2e02ac007a06e2a5d0f2d92e7cfee5677dd0ba25e681a2552a095e5e66` |
| `🧰️framework/🔨️modules/🧊️3d/🥽️mesh/🛠️modeling/🦀️.rs` | 277982 | `d4b760e0520db1b51849954deb4639cf1e59fcccdba3449bd7b62d7bafe7c007` |
| `🧰️framework/🔨️modules/🧊️3d/🥽️mesh/🦀️.rs` | 711 | `6b77ed6cc4b26de318c07876f730d9480f6f1d936fb812ddf7787030d127b9ab` |
| `🧰️framework/🔨️modules/🧊️3d/🥽️mesh/🧪️tests/🔬️tessellation/📦️ownership/🦀️.rs` | 5571 | `ba4285bf560a46d7d1ff35fab4af695a2c3a21d021daa2506cabf69d72769086` |
| `🧰️framework/🔨️modules/🧊️3d/🥽️mesh/🧪️tests/🔬️unit/🦀️.rs` | 47729 | `121ba4f6aedd19998de90e4ad21de50d99eda51481507eb89039fb512a248b13` |
| `🧰️framework/🔨️modules/🧊️3d/🥽️mesh/🧬️contract/🦀️.rs` | 7108 | `52538cc8d390c01cc7e7e763ccb3206f8bd11997473807dd0380bfaeb380beec` |

Observed retained integration surfaces (not claimed authored in this slice):

| Source | Bytes | SHA-256 |
| --- | ---: | --- |
| `.vscode/launch.json` | 8735745 | `09e1b722459dc2f6e6317acfc3ac64cb9e0f75a481d68cb4ec83f3679ff3cac7` |
| `.vscode/🧩️launch.seed.jsonc` | 6624444 | `e9efe9e0f8ff9d871af207fc11a13292a88ebeaccd812f63cc03ee6de457f85a` |
| `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️13/MODULAR-MONOREPO-BUILDS-AND-SHARED-CACHES/typescript-artifact/📜️script.ts` | 95847 | `370cceee3f5e877d1d459db7e599140b067c7c6deae65a8c8d531fd29e7ba0cd` |
| `🧰️framework/🔨️modules/🧊️3d/🥽️mesh/🛠️modeling/✂️clipping/📦️packages/🦀️rust/Cargo.toml` | 709 | `694d62f9589a89da796d81e2e64ef6036067518d1b3bb66a8ca318cead364570` |
| `🧰️framework/🔨️modules/🧊️3d/🥽️mesh/🛠️modeling/✂️clipping/🦀️.rs` | 9777 | `e21618b7bb6c5351d220227413e01658c07465a96f61330b75b3abf326b86f8b` |
| `🧰️framework/🔨️modules/🧊️3d/🥽️mesh/🧬️contract/📦️packages/🦀️rust/Cargo.toml` | 607 | `68ac7810849489612c2527029b726f869ecbe83131f4d08d1394e96c3a10da0f` |

### Retained Successor Logs

| Log | Bytes | SHA-256 |
| --- | ---: | --- |
| `🗑️generated/mesh-document-green1-prototype.log` | 7376 | `b9ef77b981a10cbe93ebcd3ac831a71d1179ebbdaa6f1eec93c866963c1235d5` |
| `🗑️generated/mesh-document-green2-prototype.log` | 9760 | `a5515732e09d606b46b68f5f7c8ce53a18609fbe2b36a6c7b347e6759adf55c0` |
| `🗑️generated/mesh-document-green3-prototype.log` | 3345 | `70e7380bd977dcc9e4b6cc4493ef39148fd059a8b4ad50050de9ba3de1ab5fe8` |
| `🗑️generated/mesh-document-green4-prototype.log` | 3329 | `6bf40e7ade8092ad7a72f9cd6833dcb061a6a7aab7663d852316beb1631c5b73` |
| `🗑️generated/mesh-document-green5-prototype.log` | 3421 | `ad8081d793106e7d78089aeec5ea7e727c9b105eb9295920ec906c8ffc38a94c` |
| `🗑️generated/mesh-document-green6-prototype.log` | 10013 | `68904db62642d17445dbfb2ecd0369b27bab9049ecb29a00c8173ecbcaa4f201` |
| `🗑️generated/mesh-document-green7-prototype.log` | 3559 | `5ea066b6fbba2d55811d0d5d1eb3ea7442708f6199e558c0db6bfbd5d2c97847` |
| `🗑️generated/mesh-document-green7-release.log` | 3441 | `d950b41b3d4c20c9b3e0aea380c970a6fa630619bc61f71a0f7d7e5094635cbb` |
| `🗑️generated/mesh-document-red1-prototype.log` | 7810 | `84c5f0a8619d49c3c9d30b97d0f448b757b1e5f301d0501e959f12b2766efc17` |
| `🗑️generated/mesh-document-types1-prototype.log` | 2404 | `782badaabb7c737c5413038e2ef1cc149955133c5efa1d6a486b5494ebd63904` |
| `🗑️generated/mesh-document-types2-prototype.log` | 0 | `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855` |
| `🗑️generated/mesh-document-types3-prototype.log` | 0 | `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855` |
| `🗑️generated/mesh-document-types3-release.log` | 0 | `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855` |
