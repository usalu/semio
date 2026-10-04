# Native WGPU Analytic Selection Audit

Read-only source audit; no runtime acceptance claim.

The actual native selection producer is `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🦀️.rs`, not only the downstream interaction consumer. Its bounded marquee and pick cursors map original numeric renderer IDs once through `world_mesh_component_id` (4091–4093 and 5455–5457). `WorldComponentHit` carries only object token plus u32 ID. The terminal plan carries component ID in an f64 numeric slot (5499), and the scene instance record at 12103 has no `componentSource`.

The procedural native authoring admission in `generation3d/.../✏️editor/🎯️selection/🦀️.rs` requires the existing exact envelope `instance.domain.numericGroup~sourceHandle~fullDecimalLabel~canonicalRevision`. It confirms the captured source handle against the current cached output and its canonical authoring revision before editing. Numeric groups alone therefore cannot authorize analytic selected edits.

The renderer’s original paged `Mesh3dLease` owner (`🧰️framework/🔨️modules/🖱️ui/🎬️scene/📐️math/🦀️.rs`) presently retains only 14 numeric buffer fields; the native inline bridge moves `MeshData` into `WorldPlaceholderMeshCursor`. Analytic `component_references` are serialized in the digest but no borrowed native lookup was found. `Instance3d` and the scene draw snapshot must preserve the small original source handle and canonical revision, while the SAME paged mesh owner must retain the original moved reference labels through bounded admission and retirement. Adding a second label cache, rebuilding JSON on every pick, or inferring nearest geometry would violate ownership and selection authority.

Planned acceptance: neutral canonical fixture with labels above 2^53 and u64::MAX, scalar/list render instances, exact source and revision, repeated triangle groups, changed source refusal; independent Serde decoding and existing ray/marquee test oracle. Both bounded pick and marquee terminal actions must carry the same exact envelope as React. Original references must retain source ownership and retire within existing mesh closure. Runtime proof remains pending.

## Neutral Producer Baseline

Schema and three neutral scalar/list cases are authored before production: labels 9007199254740993 and u64::MAX, two triangles sharing one numeric face group, original scene mesh/source/canonical revision, independent Serde envelope oracle, bounded original pick and marquee owners. Both exact terminal actions must use existing interactionSelect and retain the canonical analytic ID. The registered infinite target uses cargo test directly; the initial attempt failed before compilation because Nextest -E was not accepted. The launch command is corrected to the actual cargo name filter plus --nocapture; no feature RED or GREEN is claimed from that dispatch failure.

The corrected cargo-filter baseline stopped before assertions on three existing JPEG appearance test API mismatches (descriptor field privacy twice and a required SceneRasterPool borrow; Nx 2 m 29 s). Those same existing test callers now use descriptor() and &pool, preserving all JPEG assertions. Current3 replay session94928 is pending; production native selection remains unedited.

### Canonical Formatter Prerequisite

Native analytic selection baseline current3 ended before its feature assertions at the OS Pack expression caller: controlled formatting returns ValueError while the old error mapping expected TextError. The canonical Surface owner repaired that exact caller using the existing refusal conversion. Baseline current4 is running against the changed source. No behavioral RED or native analytic picking success is claimed yet.

### Actual Cursor Baseline And Contract Readback

Baseline current4 reached the original cursor but the test turn ceiling was below the existing 1024-slot object authority. Corrected the test ceiling to the authority capacity plus eight fixed transitions, keeping one unit per turn. Read back the canonical native action contract: targets are JSON text inside the existing typed action arguments, so the independent Serde oracle now decodes that text rather than assuming an array. Current5 is replaying; native producer behavior remains unchanged.

### Current Derive Integration Frontier

Sole replay current6 cleared the shared artifact lock and ended PREASSERT at three missing OS DSL derive component expansion functions: expand_dsl_record, expand_dsl_scalar, expand_dsl_ops. Surface owns the canonical DSL mount and is inspecting this exact reached caller. The native producer remains unchanged; no analytic picking behavioral assertion has reached its target envelope yet.

### Lower Original Scene Owner Test First

Added the domain-neutral Scene component-source schema/fixture and a native law before production changes. Four label cases include a value above JavaScript integer precision and full u64 maximum, decoded independently by Serde; five malformed labels must refuse. The law requires original Vec/String pointers across the existing paged mesh owner, exact fixed source identity, stale-reader refusal, and terminal retirement. The existing Launch/seed register this law through the existing Bun/Nx target. Baseline is running; lower owner production remains unchanged until the reached API baseline. Root files: Scene `🧬️schema/🎯️component-source/🔣️.json`, `🧫️fixtures/🎯️component-source/🔣️.json`, existing `🧪️tests/🔬️math-unit/🦀️.rs`, both launch files.

## Original Scene Component Ownership — Native Receipt

Schema and neutral fixture preceded the lower scene law. Actual baseline refused to compile on the missing source/label ownership API. Production now moves the original component-reference map once into the existing mesh pager owner and retires it through the existing snapshot retirement protocol. The law confirms unchanged original vector and string pointers, exact positive u64 labels including 9007199254740993 and 18446744073709551615 against Serde, source handle/revision validation, malformed label refusal, bounded close, and stale lease refusal. Actual native result: **1/1 passed**, 160 skipped, 0.010 seconds; generated log `sol-2026-10-03/ui-scene-original-component-source-green.log`. The full scene regression and original Native WGPU producer law are next; this receipt does not claim renderer propagation.

The same session owner had two remaining private OS Flow ToValue paths. Both now name its already declared canonical Value trait directly; no behavior/assertion or retirement implementation changed.

## Native Producer Baseline and Source Propagation

Actual renderer baseline8 reached the intended assertion: `worldPick` versus required existing `interactionSelect`; 0/1 passed, 518 skipped, 0.01 seconds. Baseline9 was an accidental duplicate dispatched while8 remained active and cancelled through its own session before assertions. Full scene regression reached 110/111 passing before an existing Board2d neutral catalog payload omission stopped the run; 50 not run. No full-scene green claim.

The original mesh map is now moved before sealing the same mesh pager; source handle/revision crosses original instance wire, fixed-capacity snapshot spans, retained Instance3d and original interaction slot. Fixed-size identity is included in existing instance size admission. Ordinary existing instance literals explicitly carry no analytic source. Updated literal owners:
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🦀️.rs` (9 literals).
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🧪️tests/🔬️unit/🦀️.rs` (36 literals).
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🧪️tests/📜️journal-sequences/🦀️.rs` (2 literals).
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/⏯️tool-run-trace/🦀️.rs` (1 literals).
- `🧰️framework/🔨️modules/🖱️ui/🧪️tests/🔬️targets-wgpu-draw-unit/🦀️.rs` (2 literals).
- `🧰️framework/🔨️modules/🖱️ui/🧪️tests/🖼️raster-residency/🦀️.rs` (1 literals).
- `🧰️framework/🔨️modules/🖱️ui/🧪️tests/🔬️targets-wgpu-prepared-unit/🦀️.rs` (3 literals).
- `🧰️framework/🔨️modules/🖱️ui/🎬️scene/🧪️tests/🔬️math-unit/🦀️.rs` (12 literals).

Source propagation public mount now explicitly exposes the original snapshot page byte bound. The current Board2d neutral catalog now includes its existing required packed `highlightedIdsJson` lane; no Board behavior or optional decode policy changed. Native original scene fixture additionally includes an ID with quote, newline and Unicode to exercise exact target JSON escaping. Canonical component picks/hover use the original interaction plan and existing generic publishers; component marquee uses its original retained job with exact source labels and original shared merge command. Numeric-only component branches were removed. Selection target string credits use actual escaped bytes. Renderer behavior remains unverified until fresh native receipt. Additional explicit ordinary literal owners:
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🧪️tests/🖱️pointer-gestures/🦀️.rs` (1 literals).

## Full Scene Regression Receipt

Actual full UI scene native replay after current Board catalog lane completion: **161/161 passed**, 0 skipped, 2.217 seconds; `sol-2026-10-03/ui-scene-original-component-source-regressions-2.log`. Original source ownership, all existing scene codec, math, pager and snapshot laws executed. Renderer production compiled far enough to reach only existing ObjectRegistry test callers needing the explicit new source field; no renderer behavior pass inferred. Exact source-less current test callers updated:
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🧪️tests/🔬️unit/🦀️.rs` (7 callers).
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🧪️tests/🖱️pointer-gestures/🦀️.rs` (1 callers).

## Current Component Contract — Additional TDD

Strict neutral schema/fixture and three native laws authored before modifying the remaining existing behavior: marquee dedup must include original object identity, shared world domain must admit vertex/edge/face and existing lasso/subtractive gestures, and optimistic preview must read the original JSON-encoded target text and source envelope. Current production still has numeric-only marquee dedup, two-granularity domain and array-only preview, so actual RED remains required before fixes. Dependency lock prerequisite currently prevents execution; no pass claim.

## Original Native WGPU Producer — Actual Green Receipt

Fresh native source replay: **1/1 passed**, 518 skipped, 0.07 seconds; Nx successfully ran original Infinite target. Four fixture rows exercised exact 9007199254740993 and 18446744073709551615 analytic labels, collection render index1, and quote/newline/Unicode ID escaping in both actual native pick and marquee publication. DEBUG output confirms canonical source-bound targets through the original scene bridge. Generated log `sol-2026-10-03/native-wgpu-analytic-component-picking-green-2.log`. Full scene161 also passed. Additional authored component contracts and broad World regressions remain required; this receipt does not claim enduser assembly, all gestures or large selections complete.

Additional component-contract baseline actually reached **0/3**, 519 skipped, 0.01 seconds: original marquee pairs collapsed4→2; shared domain lacked vertex; preview read encoded targets as an array and returned no selected group. Production now deduplicates original object-token/group pairs, declares five localized granularities/three methods/four merges in the existing domain, and previews only canonical JSON text against the original retained instance source and mesh label. Legacy component action branches removed. Fresh green replay and broad World regression still required. Existing WGPU generator metadata declares the exact frame worker output/producer; fresh Nx graph reached native contracts, so no speculative renderer metadata edit was made for the earlier cascaded discovery failure.

## Additional Native Component Contracts — Actual Green Receipt

Fresh original object-pair/domain/encoded-preview replay: **3/3 passed**, 519 skipped, 0.02 seconds; Nx success confirmed. The original four marquee object/group pairs remain distinct with exact duplicate dedup, shared localized domain admits five granularities/three methods/four merges, and original source-validated encoded text updates both local component hover and selection. `sol-2026-10-03/native-wgpu-component-contract-green.log`. Broader current World regression dispatched because component action/preview behavior changed; old test-only action helpers/expectations still need current contract checks. Tessellation conversion source currently emits face/edge reference maps but no vertex references; selectable analytic vertices and large component-selection paging remain open gates. No blanket feature completion claim.

## Current Native Component Contract Receipts

The registered native component contract gate actually failed all three authored laws before the fix (0/3, 0.01 s), then passed all three after the fix (3/3, 0.02 s; 519 filtered). Equal numeric groups on different original objects now remain distinct, the shared domain admits five granularities/three methods/four merges, and preview validates encoded target text against the original analytic source and label. The broader World regression gate is running; no broad pass is claimed.

Root graph collection current 7 and Generation2D owned-input backtrace current 7 both ended before behavior assertions: canonical Stdio source mounts/type references were still incomplete at their captured source revisions. Surface owns the exact captured source repairs and subsequent Semio IO canonical references. These receipts do not change previous behavior outcomes.

The remaining native selection acceptance work includes original topology vertex provenance, component marquee capacity/paging, and multiple object preview highlighting. The original generic marquee publisher also needs its per-page replace semantics checked; independent atomic action batching has sixteen slots and 4 KiB per ordinary string. Existing retained string action ownership has a separate 16 MiB source budget and must be considered through the same action queue before introducing any new output mechanism.

## Broader Native World Regression — Actual Red

The registered World regression gate reached its behavior tests: 200 passed, 38 failed, 284 filtered in 0.78 s. This is an actual regression receipt, not a compile floor. Original target-text preview tests still supplied obsolete array/numeric actions; an old component-marquee test expected premerged numeric IDs; the domain fixture still expected only two granularities; and the fixed-slot budget recorded 400 bytes rather than the measured 528 after original 128-byte analytic provenance was added.

Many bridge tests exhausted the original eight-slot global snapshot fixture owner because old test states did not retire their bridge leases. The isolated four-case analytic source test and three component laws previously passed. Fixture ownership must be closed through the original close ladder, with serial scheduling for the shared fixed pool; production capacity must not be increased to conceal this. Remaining unrelated grid/camera/renderer source-pattern failures are retained as separate evidence and are not declared fixed by the selection changes.

## Original Vertex Provenance — Schema and Test First

A new neutral vertex-provenance fixture covers eight original box vertices and four original wire vertices, with exact expected positions and an explicit nonselectable surface-sample sentinel. Ajv 2020 strict validation actually passed both cases. An authored native law compares original deconstructed topology and label resolution in the same BRep owner, checks exact positions using independent Parry point distances, and rejects all triangle surface samples as vertex targets. Production is unchanged for this law until its baseline receipt.

The planned implementation extends the original TessellationJob edge-packing ownership to carry original endpoint VertexIds and labels into the existing MeshTransfer points/groups, then the existing mesh/pager/interaction path. It will not infer identity from coordinates or proximity. The original component references, original renderer IDs, and original source/revision remain the authority.

World regression current 2 updates test-only obsolete selection payloads to the canonical encoded target text, shared reducer merge semantics and current projection spec. The same original scene bridge close ladder retires fixture-owned leases during test cleanup; shared eight-slot fixture scheduling is serial. The existing boxed fixed-slot fixture is reconciled to the actual 528-byte slot (including 128-byte analytic provenance). Broad current 2 is still running and no pass is claimed.

## Current World Regression 2 — Ownership and Contract Results

Actual current gate: 227 passed, 11 failed, 284 filtered in 3.27 s. No original snapshot `Unavailable` failures remain after fixture close ownership and serial shared-pool scheduling. The original exact analytic pick/marquee cases, new three component contracts, typed snapshots, source bridge rendering, component branch preview, unchanged-scene optimistic preview, shared domain admission and measured 528-byte slot budget all passed.

Four ordinary object-pick oracle tests still aimed at a floating-point triangle corner; they are being changed to an interior point while preserving the exact domain, target ID, merge and hover-clear assertions. The two camera-fit failures, camera row expectation, cursor-wake source-pattern assertion, grid projection assertion and prepared-input fixture exhaustion remain distinct open regression evidence. No broader green is claimed.

## Original Vertex Baseline — Actual Red

The registered BRep vertex-provenance gate reached the intended assertion and failed 0/1 (0.015 s, 604 skipped): `original topology vertex references` was missing from the original converted mesh. Ajv strict neutral schema and native test compilation had succeeded. Production vertex provenance changes are now authorized by this concrete baseline.

The same TessellationJob will emit each original edge endpoint once by VertexId, carrying its current PersistentLabel and position. The existing MeshTransfer points obtain explicit vertex groups. The existing MeshData appends these original points to its vertex buffer and marks every surface sample nonselectable with the neutral u32 sentinel. Existing triangle indices/face groups remain unchanged. Exact label references and current source/revision propagate through the original pager and interaction cursor. Inner tessellation metadata cancellation/allocator accounting is still an open acceptance item; this step does not claim it closed.

## Original Vertex Kernel — Actual Green

The authored original topology law now actually passes 1/1 (0.019 s, 604 skipped) after the same TessellationJob/Transfer/Mesh changes. Both box and wire resolve each preview point to the original current Vertex handle and PersistentLabel; all triangle surface samples carry the neutral nonselectable sentinel; exact positions match independent Parry point distances. The original triangle index/face/edge buffers remain the existing buffers.

The TypeScript mirror test is authored first with the same neutral fixture, full u64 labels and independent Three.js geometry/point distance oracle. Its baseline is pending. Native pick/marquee must separately prove they skip nonselectable surface samples before resolving an original vertex label; kernel conversion success alone is not an Enduser vertex-pick receipt.
