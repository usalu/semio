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

## Continuation Receipt — Original Vertex Mirrors and Native Pick Baseline

The original TypeScript mesh conversion mirror is actual GREEN 4/4 (326 ms Vitest, Nx 1.7 s), including the neutral original vertex fixture, full u64 labels and independent Three.js point distance. The separate native picking baseline is actual RED 0/1 (522 filtered, 0.01 s, Nx 3 min 27 s): the pointer chose group 4294967295, the explicit nonselectable surface sample sentinel, instead of original vertex group 0. Both original process handles are now absent and complete logs confirm terminal results; no duplicate gate was restarted. The same native cursors must skip that sentinel before geometric ranking/selection and overlay generation. Kernel conversion alone is not a native or Enduser interaction receipt.

## Original Vertex Consumer Continuation

Strict Ajv validation actually passes the native analytic vertex and new lower Scene neutral vertex-selection fixtures. The lower Scene law requires the same original paged owner to skip the absence sentinel for both rectangle and lasso, and to close its lease to terminal empty; its unchanged-production baseline remains live. The existing TypeScript target now includes an independent Three.js geometry/projected-point oracle against that same lower Scene fixture, with current replay live. Native authored-expansion law is also authored: the same original pick and marquee helper must succeed with face normals/color and corner UV/tangent data; production expanded pager is unchanged awaiting its actual baseline. Existing native/scene process descendants were verified live in canonical preparation; no observation timeout restart occurred.

The existing TypeScript conversion/geometry target with the added lower Scene oracle completed actual GREEN5/5 (Vitest1.71s,Nx7.9s). A new schema-first two-transfer fixture now declares original face/edge/vertex ranges, exact labels, points, offsets and independent area/segment length. The portable merge baseline is live; original merge production still drops those payloads and is unchanged until its actual receipt. Native same-fixture conversion law is authored separately to prove expected merged buffers remain admissible and exact; it is not yet run.

## Original Transfer Merge Baseline — Actual Red

The new neutral two-transfer portable law reached actual RED1/6 (five unchanged laws passed, Vitest253ms,Nx1.3s): existing merge output edges were empty rather than the original twelve numbers. Original points/groups/infos were likewise discarded by source inspection. The same merge function now concatenates original buffers and offsets each original face-index, edge-segment and point group; original labels/infos remain intact, and uniform color is retained. No new evaluator or renderer module was introduced. Changed-source portable replay is pending. This utility proof does not close mixed source lineage, heterogeneous materials, streaming/cancellation or native merged fixture acceptance.

The same original transfer merge portable replay is actual GREEN6/6 (Vitest228ms,Nx~1s), confirming buffer offsets, original face/edge/vertex label maps, original points, independent Three area1 and edge length2. Strict Ajv validates the neutral merge fixture. Native merged same-fixture gate is registered but pending; it proves converter acceptance rather than a native merge algorithm.

A separate original target formatter neutral boundary law is now authored for a256-byte render ID, vertex kind, ten-digit renderer group, u64-max label and two64-byte source identities. The exact canonical width is425bytes; current fixed owner allows424bytes. Its baseline is pending, production capacity unchanged. The authored face/corner expansion law and formatter law have launch entries in both canonical sources. Root remains in verified waits for native pick/marquee and lower Scene baseline through original live preparation descendants.

## Vertex Refusal Parity — Actual Portable Red

The schema-first original vertex fixture now also declares six malformed range/label cases. Portable baseline reached actual RED1/7 (six prior laws passed,284ms): standalone renderability admitted first group start1. The same existing group validator now checks original vertex groups as well as face/edge groups, with bounded128 Unicode scalar label length matching native conversion. No label narrowing or coordinate identity guess was added. Changed-source all-seven replay is pending. Native six-refusal same-fixture law is authored and registered alongside original topology and merged-buffer converter proof; it has no current runtime receipt.

Current source readback found the original vertex schema file absent at the expected canonical location. The canonical engine schema was saved and strict Ajv again actually validates both original shape cases; fixture outputs were preserved. The earlier reported strict check is not used to infer that a schema artifact existed in the current checkout.

## Current Portable Vertex Validation — Actual Green

The same standalone validator/converter replay is actual GREEN7/7 (Vitest774ms,Nx~3s), including original topology positions/labels, face/edge maps, Three geometry oracle, complete merge ranges and six malformed vertex refusals. After this receipt, the refusal test gained a direct independent Ajv2020 per-position/range/label oracle that must reject each same malformed source. That source change is under its own current all-seven replay; prior green does not prove the newly added oracle executed. Strict schema checks were repaired to use standard anyOf rather than a strict-Ajv union type; the current native vertex/refusal and formatter fixtures now actually validate.

## Original Lower Scene Absence Baseline — Actual Red

The original lower Scene neutral rectangle/lasso law reached actual RED0/1 (161 skipped,.035s): rectangle selected [0,4294967295] rather than [0]. The same original scene math selector now excludes the absence sentinel before projection/inclusion and formats the resolved original group. No second selector, label guess or altered expected assertion was introduced. Current changed-source targeted replay is pending. The independent portable same-fixture Three geometry oracle and newly added Ajv six-refusal oracle now actually pass in the full7/7 portable gate (2.13s Vitest,Nx~6s); the oracle source change was replayed.

## Native Vertex Current Replay — Exact Prerequisite Floor

The original native pick+marquee changed-source run94417 is terminal PREASSERT exit1 after clearing original generation and both preparation stages. Sole compiler floor: durable-group557 initializer lacks existing CursorRevisionAccumulator fields indexed_edits and mutation_positions. No vertex assertion ran. Exact caller source readback and current original accumulator defaults must be checked before repair; this is source coherence and does not constitute a vertex green. Scene97783 remains separate current replay.

Fresh durable-group source already supplies both missing fields and calls the original index_applied_edit. Concurrent owner repair was preserved; root made no mutation there. The existing registered serial full World regression route now replays changed source: it covers the original vertex pick+marquee, authored expansion baseline, widest formatter baseline, the four ordinary interior-point fixture corrections and prior broad assertions together. Baseline-pending pager/capacity production remains unchanged. Earlier unrelated camera/grid/renderer failures remain open until actual current receipt.

## Original Lower Scene Absence Changed-Source Green

Actual GREEN1/1,161skipped,Nextest.012s,exit0. DEBUG proves both rectangle and lasso select only original vertex group0, skip the surface-sample sentinel and close the same lease to terminal empty. Receipt lower-scene-original-vertex-green.log. This is the generic Scene selector, not yet the native World authored pager or full Enduser flow. With this run terminal, the original native BRep three-law gate is dispatched separately: original topology fixture, neutral merged converter buffers and six vertex range/label refusals. World regression3 remains the sole current World lane.

## Original Vertex Receipts and Multi-Object Selection Baseline — 2026-10-06

The lower original Scene rectangle/lasso vertex selector is actual changed-source GREEN: 1/1, 161 skipped, 0.012 s; the preceding actual RED included the surface sentinel as a component. The repair skips only the established nonselectable sentinel before projection, in the same original selector.

The current original native BRep converter suite is actual GREEN: 3/3, 604 skipped, 0.016 s, Nx 5m52s. It covers original topology points, merged face/edge/vertex ranges and six malformed vertex-range/label refusals; it does not prove the native World pager or full Enduser runtime. Log: `🗑️generated/sol-2026-10-03/brep-original-vertex-merged-refusals-current.log`.

Original full serial World current3 is active, with no runtime result yet. It covers the repaired plain analytic vertex picker, the authored original-point expansion baseline, maximum encoded target capacity, and the newly authored original component-selection merge baseline. The neutral multi-object fixture uses two instances of the same source, both face group 0, and verifies replace/additive/subtractive/invertive full-source targets and surviving active-object projection. The production selection reducer remains unchanged until an actual RED receipt.

An independent test-only Lodash merge oracle consumes that same neutral fixture. The current portable full replay is pending; no oracle pass is claimed. The focused native multi-object merge command is registered in both launch source and projection at order 900.0365963.

## Original Wire and Object Overlay Baselines — 2026-10-06

The portable original geometry suite is actual GREEN 9/9, including test-only independent Three wire geometry/pointer projection and the Lodash exact multi-object merge oracle. Log: `🗑️generated/sol-2026-10-03/typescript-analytic-wire-independent-oracle.log`. Both launch files parse with zero errors; the new selection-merge entry appeared exactly once in each.

New neutral original wire fixture/schema and native law enter the actual Scene bridge with four original vertices/four original edges and zero triangle indices, then publish exact original-source vertex and edge picks. Current original inline pager rejects all zero-index meshes and its index phase assumes a positive count. Production remains unchanged pending the actual native RED baseline.

The original face overlay isolation law consumes the same four-mode multi-object fixture. Equal local group numbers on two instances must highlight only the original selected objects, including the survivor after subtraction. Current original overlay scanner still reads global numeric component projections. Production remains unchanged pending actual RED. Focused original wire and overlay commands are registered in both launch sources at orders 900.0365964 and 900.0365965; full original World current3 may cover them if source compilation begins after their authorship. No native wire/overlay pass is claimed.

## Canonical Camera Fixture Repair — 2026-10-06

The prior actual native World baseline had four camera-framing/diagnostic failures. Inspection of the existing schema found two stale fixture values: `perspective` is not a canonical `Viewport3dProjectionMode` (the current mode is `threePoint`), and strict `Orthographic {}` forbids the extra `fov` property. Only those fixture mode values and the initial diagnostic assertion were updated; all delivered positions, targets, FOVs and independent expected measurements remain unchanged. No compatibility alias or production camera fallback was added.

Current independent original React/Three camera-framing oracle is actual GREEN 4/4, Nx 13.8 s, log `🗑️generated/sol-2026-10-03/react-canonical-camera-framing.log`. The focused oracle command is registered in both launch sources at order 900.0365981. Native changed-source framing remains unverified until a current World replay.

The prior grid-only native assertion counted the fixture's ordinary two-vertex analytic edge as a finite grid. The test now supplies an empty mesh/instance payload to isolate the original procedural grid, retaining every scalar, camera projection, datum, fade and zero-line assertion. This is a fixture repair; original grid production is unchanged and current native proof remains pending. The prepared-reference input test already has a concurrent bounded abandonment drain; that existing repair was preserved rather than duplicated.

## World Current3 Build Frontier and Coherent Current4 Replay — 2026-10-06

Original World current3 is terminal PREASSERT exit 1: three E0277 borrowed-field prerequisites (`DagPreviewContent`, `DagExpandedPaths`, `PropertyBag`), Nx World target 17m30s, graph generation 5m31s. No plain vertex, authored expansion, formatter, wire, object projection or overlay assertion ran.

Fresh source readback already contains concurrent original repairs for every named type: the existing DAG tagged-field macro includes preview content and emits static borrowed statements, expanded paths delegates its original `Vec<String>` shape, and original Graph property bag supplies `Map(Value)`. These were preserved; no duplicate implementations or alternate modules were added. Original full serial World current4 is now dispatched against coherent current source, using the existing launch command with fresh graph and two Cargo build jobs. Log: `🗑️generated/sol-2026-10-03/native-wgpu-component-world-regressions-4.log`.

The existing six-token live renderer wake census also had two stale source expectations: removed `FrameWheelCursor` boundary and compact completion syntax before new typed fields. The test now ends the unchanged FrameBuildCursor census at current FrameBuildPhase, and checks each exact typed completion field inside the current completion variant independent of whitespace. Every original mutation/refusal and exact six-token census remains. This is a test-source repair following an actual prior RED; native changed-source result is still pending.


## Current Native World Runtime Baseline (2026-10-06)

The changed-source native World gate reached actual runtime: 236 laws passed and nine failed (284 filtered, 1.66 seconds of tests; Nx 13m48). Ordinary pointer selection, the original analytic vertex pick/marquee without authored expansion, isolated grid fixtures, renderer wake census, both live camera families, and delayed geometry framing passed. Six new feature laws failed in their original owners: remaining-object component projection, the 425-byte source target, object-scoped face overlay, analytic vertex retention during authored attribute expansion, pure-wire Scene publication, and clearing an empty Scene component/hover snapshot. Three existing failures remain: user-orbit camera ownership, the second camera fit's expected zoom, and prepared reference input admission. Native source-bound gumball law was authored after this compilation and is still pending its runtime baseline.

The current portable geometry gate passed nine laws, including independent Three gumball motion and Lodash component merges with strict Ajv neutral contracts. The React overlay component/interaction gate passed all 26 laws; its separate visible diagnostic law also passed and logged exact replace/add/subtract/invert instance masks and nine source refusals. These results establish those scopes only; full feature completeness remains open.


## Original Native World Repairs and Fresh Guest Law

After actual World4 REDs, the existing component formatter now has a derived425-byte maximum. The original Scene mesh owner admits nonempty points/wires with zero triangle indices; its original index writer handles zero items without emitting a fabricated triangle. During face/corner attribute expansion the same retained mesh cursor preserves the original selectable vertex tail using one source-tail scalar, and applies vertex-domain data or original point fallbacks instead of reading a nonexistent face/corner attribute for an unindexed point. No evaluator or geometry cache was added.

The existing Native World state and original Scene selection record now read the already-published Guest gumballSelectionIds lane. Guest ids stays the plain-object projection; component selection preview merges exact targets first and then derives the remaining active-object numeric projection. Original edge/face/vertex overlays resolve targets against each retained instance/source. Numeric ordinary-mesh selection is scoped to its active instance. Empty fresh Scene snapshots clear component and hover fields; unchanged digest preserves optimistic local preview. The previous omission-as-partial-patch law is now the full-snapshot clearing law, matching the existing Guest producer. The ordinary selected-edge fixture now supplies its required active object.

A new original face-scan law reuses the shared neutral four merge rows and the React analytic target fixture's nine source refusals after a fresh Guest-like Scene publication. It preserves plain ids, tests exact full target overlays, and adds duplicate-current-label ambiguity. Production duplicate label refusal is still pending its actual Native baseline. Production gumball capture/publication remains unchanged while its actual source-bound baseline is pending. Full current World5 replay52380 is live, with capture native-wgpu-component-world-regressions-5.log. No GREEN claimed for these changed Native World owners yet.

The original pure-point BRep extension is separately actualGREEN: Native original3laws3/3 plus portable Ajv/Three1/1, existing box8/wire4/point1. It uses the original TessellationJob, one-unit/zero-budget/cancel laws, and no source mirror.


## Current Verification Prerequisite and Camera Sequences

World5 replay52380 is terminal PREASSERT, exit1, Nx4m28 target. Its five originating diagnostics are current Store/retained hydration/config mandatory ActorId boundaries; no Native World assertion ran and no changed Native owner qualification follows. Mesh and Document carry only the original genuinely opened ActorId through their affected caller owners, preserving concurrent core actor/genesis work. Read-only Codex app state identified the external core owner as the active chat Add editable mutation history; its coordination report is ticket26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING/📓️2026-10-06-coordination.md. No message was sent to the external chat and no core authority semantics were overridden. Subsequent shared core genesis migration adds further compile prerequisites.

The existing camera neutral fixture now explicitly declares delivered preserveCamera and delayedContent content policies. A new closed outer schema validates these sequence policies; the existing independent React/Three camera cohort is actualGREEN4/4 with strict Ajv validation (31ms assertions, Nx7.3s). All existing numeric expected poses/zooms remain exact: direct delivered7.7595 and raw-content→loaded88.5185185185185. Native test inputs now declare those separate canonical policies instead of implicitly requesting content framing for both sequences. The original Native fit owner schedules the already-existing owed content framer after accepted snapshot/draw gates, before fitting loaded geometry, and the original framer consumes owed debt on genuine user navigation. A new original pending-content/user-orbit law is authored; Native runtime of these changes is still pending.

The original Native source-bound gumball law now supplies the required existing window action declaration before actual handle capture. Its production capture/commit remains unchanged pending baseline. Readback also found the existing gumball_target state field is never assigned from published selection JSON; a nonzero-pivot law and original-field repair remain open.


## Current Native Prerequisite and Selection Lifecycle Qualification

World6 replay29969 ended PREASSERT with exit1 (Nx1m35 target plus39.9s graph); the current Store/ArtifactGenesis compiler wave blocked all World assertions. Its approximately39 originating Genesis errors do not qualify changed Native selection, framing, wire or attribute owners. Concurrent core owners subsequently released canonical verified Genesis and full-envelope Pack transfer; Document preserves those owners and has changed-source G3 six-law replay96861 live. Mesh Scope/Policy replay21 is also live after current compiler corrections.

The read-only Native selection audit is native-world-selection-lifecycle-audit-20261006.md. It found that production Scene selection sync changes exact nonactive targets without advancing interaction_revision, although the retained face overlay cursor uses that revision to become stale and rebuild. No production caller of apply_world_action_preview was found; its tests alone do not prove Shell preview integration. A closed neutral overlayLifecycle fixture now holds plain ids, active object and active group constant while switching the second object's full source face. The original Native law checks change-sensitive revision, unchanged publication stability, exact per-object groups and stale cursor retirement. Production revision repair remains pending actual baseline.

The same original gumball fixture now contains both origin and nonzero delivered pivots with independent Three ray/projection pointers. Original portable3D replay7765 is actualGREEN9/9 (81ms tests, Nx1.3s), including strict Ajv, independent Lodash merges, pure point and the two pivot rows. The new original Native delivered-pivot law is authored; gumball_target production assignment and exact source target capture/mode remain pending actual Native baseline. World7 changed-source full original replay3179 is live at native-wgpu-component-world-regressions-7.log and includes those laws and the nonactive overlay lifecycle.

Enduser original mounted inspector now is actualGREEN6/6 (English/German scalar blur/Enter and ordered collection cases); broader keyboard/control suite is127/127. Original full creation fixture validation is333 input,551 creation,3951 terminology assertions plus strict TypeScript, with154 widgets including141 BRep. Native metadata/shell/restoration qualification is still separate and pending. Original controlled Record semantic restoration oracle is separately actualGREEN1/1 with independent serde_json; it does not prove retained parser yielding or full G3 import restoration.


World7 replay3179 is terminal PREASSERT exit1 after shared core/DAG source release: three Root test-caller compilation errors (two private PreparedRenderInput terminal accessors and one InputState close_step Result<bool> mistaken for bool), no Native assertion reached. Those exact callers now use the original public close-step completion/Result contract; no production input API was changed. World8 replay30696 is live with those fixes. Original portable overlayLifecycle schema/Lodash gate22001 is actualGREEN9/9 (117ms tests, Nx4.2s).

The neutral gumball now has four actual authority cases (local release, live commit, blur abort, capture-loss abort). New Native law enqueues original pointer/cancel events and drives step_world3d_interaction under one-unit grants, checks source IDs/mode/tail, and retires original authority/input/Scene owners. Production currently excludes component mode before GumballPick in that original authority branch; the law captures this separate gap without test-only direct publication. The direct real-pick/job law now also checks captured nonzero pivot, all four tails and bounded surface retirement. Current Native production remains unchanged pending an actual assertion baseline.


## Actual Native World8 Baseline and Same-Owner Repairs

World8 replay30696 is ACTUAL runtime RED:233 passed/18 failed,284 filtered,1.94s assertions,Nx16m38s. Compiler prerequisites and Root test cleanup callers were clear for this capture. Distinct REDs include: legacy untyped zero-index Scene payloads now staged and refused Capacity; original retained Mesh schema refuses real zero-index wire; old ordinary overlay fixtures omit required active object and old raw marquee projection omits its canonical retained target; resident content-perspective inputs compare against delivered preserveCamera expectation; exact nonactive source selection leaves revision unchanged; duplicate current source label remains selectable; delivered pivot is absent; direct and event-authority component gumball capture do not obtain a handle. Several later Scene Capacity failures share the zero-index admission boundary; they are not counted as independently fixed features.

The original Scene selection owner now compares incoming effective fields before assignment, advances its existing view revision only on change, and reads/clears its existing finite gumball_target. Equivalent differently formatted selection text must preserve the revision; actual old cursor owned a physical writing mesh before supersession in the new law. Changed Native qualification is pending. The Scene inline filter now distinguishes actual triangles, edges or explicit selectable vertex IDs from unlabeled zero-index degenerate data, preserving old accepted geometry. Mesh owns the separate original math schema wire/point and bounded provenance uniqueness/capacity laws. Its matching portable Three/Ajv authority oracle is separately actualGREEN; production math remains pending its baseline.

The original fixture bridge performs the next original Scene sync after retained draw publication so real selection flags reach the same registry the render loop uses. Old ordinary face-overlay fixture inputs now explicitly name their active object; original ordinary marquee publication fixture supplies its original full target lane. All previous output assertions are retained. Source-bound gumball production remains unchanged while World9 replay50256 runs its baseline. A new neutral80-target law uses one source object, exact u64 labels and the existing action byte budget; original event/pick/publication owners must preserve every target, not the old64 object cap. Both existing gumball launch prefixes select this appended law.

Independent UI camera readback confirmed the raw-to-content React test is orthographicTop, while delivered perspective is preserveCamera; intentional perspective content framing follows the existing perspectiveThreePoint oracle. The original Native resident law now retains all existing numeric oracles and adds the third content-perspective lane, rather than comparing content and delivered policies to the same pose. Each retained camera surface retires after the law. Native runtime pending.

Document original quoted scanner actualRED consumed a whole token under one grant; same-reader cursor repair is actualGREEN2/2 plus full97/97 original DSL regression, and independent JSONPatch/schema oracle8/8/70 is GREEN. Full G3 capacity/import restoration remains pending original seven-law replay54345. Metadata14 remained PREASSERT glTF55/SVG38; exact canonical caller releases now justify metadata15 session1812. No metadata/shell/feature-complete claim follows from portable/frontend results.


## Native World 9 Actual Runtime and Original Gumball Repair — 2026-10-06

The original full World cohort actually executed: **237 passed, 15 failed, 284 filtered**, runtime1.43s, Nx7m50. Original generated log is `🗑️generated/sol-2026-10-03/native-wgpu-component-world-regressions-9.log`; retain while this ticket is open. This is incomplete and the goal/ticket remain active.

Actual GREEN now includes original pivot delivery for both neutral pivots with fresh empty selection clearing it; nonactive source-overlay revision invalidation, differently formatted equivalent JSON stability and stale partial writer retirement; the prior two face-overlay fixtures with their canonical active object; and the original component-marquee merge fixture with its canonical full source selection. Native DEBUG receipts were emitted by those new laws.

Fifteen failures remain: eight older bridge cases are refused at Capacity, two camera laws (delivered projection debt and resident delayed content policy), duplicate current analytic source label, zero-index analytic wire, and three original component gumball capture/authority laws. These are real runtime failures, not compiler-only diagnostics.

Follow-up source read corrected the previous test-driver assumption: `sync_world3d_state` does not apply published draw selection flags. The renderer calls `apply_runtime_draw_flags` after its sync/fit steps. The fixture bridge now invokes that exact owner step after draw publication/retirement. Its extra terminal `sync_world3d_state` was removed because it consumed the accepted projection content-frame debt before the original debt assertion. No selected bit is synthesized in the test. The three-pane camera law now explicitly qualifies delivered perspective preservation and independent content perspective against their unchanged existing numerical oracles, alongside content orthographic top. This edit was after World9 dispatch and requires a fresh runtime.

Original retained gumball pick/motion/commit sources now capture canonical component addresses through one object-registry hash probe per grant and validate against the original mesh/source labels. Captures retain the exact object generation token plus index into the revision-owned `gumballSelectionIds`; they do not copy source IDs or geometry. Update and publication revalidate the same original target. The commit emits exact mode and source IDs through its existing bounded action builder. Primary-pointer authority now offers the same gumball in component modes. Selected capacity derives from original256 action-node credits minus11 worst-case fixed nodes (245 maximum), with the existing16KiB physical string claim enforcing actual source bytes. Counters are u16 and retirement still removes one captured item/turn. Ordinary mesh gestures retain the same token-based path. The canonical address parser is shared with original overlay admission.

These source repairs are **authored, not runtime-qualified yet**. Original full World10 was dispatched as session79762 with unchanged full filter, source assertions and DEBUG receipts, strict fresh Nx graph/daemon settings, offline Cargo and two build jobs. Mesh source admission/schema changes are owned by the mesh agent and only authored after its actual1/3 native RED. Component-local geometry preview, native event local selection preview, original source graph replay and broader Enduser journeys remain open.


### Edge-Only Source and Actual Preview Ownership Follow-Up

Source-current original scene-bridge neutral fixture contains an edge-only profile (edgePositions36, no positions), a vector (positions6+edgePositions6, no indices), and the solid. Its older expectations explicitly discard both wire records. Full wire support requires replacing that triangle-only expectation and consuming real edge-only samples in the existing packed render positions stream without adding topology vertex IDs. New shared analytic-wire schema/fixture edgeOnly contract expects8 render samples,0 topology vertices,4 real edges, exact Box3 bounds and unchanged canonical max-u64 edge target. The new Native law reuses the original actual bridge/pick/publication helper with source mesh and exact bounds, then retires the original surface; production edge-only sampling is unchanged pending actual nativeRED. Portable source uses existing Three geometry and strict Ajv, its fresh original9-law run84755 is pending. World10 may or may not sample the newly appended law; exact terminal count is authoritative.

Actual React G3 qualification: the producer always delivers gumballLiveDispatch=true. The mounted live lane bypasses local whole-instance pose preview, streams the exact original target/mode, and waits for original inference. Root original Native gesture likewise excludes live gestures from local whole-instance matrix preview. The reusable local component path remains separately open; no actual G3 whole-object visual defect is claimed. Native typed Select publication is a documented possible original preview seam, but no production local selection repair is claimed from its pure helper tests.

UI source audit found a separate required Enduser issue: default object-mode pure Point3d has no existing base paint branch (shaded needs indices, line needs edges, base points gated by vertex target, overlay points need vertex selection). This is source-only finding. UI execution lane is assigned an actual mounted neutral point law and same original visual owner repair after confirmedRED; Root will qualify the Native original visual owner in conjunction. No fabricated triangles/edges or new geometry evaluator is authorized.


Portable original wire edge-only replay84755 actually GREEN9/9 (227ms test runtime,769ms Vitest,Nx2.2s). Strict Ajv accepts the closed new shared edgeOnly contract. Existing own transfer preserves real edge samples and0 topology vertex IDs, existing Three edge Geometry independently has8 render samples and exact bounds. This validates the neutral/view oracle only; Native higher edge-only producer remains unchanged pending actualRED. Original document seven-law54345 is still live per agent's scoped receipt; its High agent yielded without stopping the process so Light native wire/point paint audit can use the fourth fleet slot, and will resume after audit. Root cannot poll that agent-scoped unified-exec session (Unknown process id), so filesystem diagnostics are the read-only monitoring seam until High resumes.


## Original World10 Actual Runtime — 2026-10-06

Original registered World cohort terminal: 244 passed, 11 failed, 284 filtered out; actual runtime 1.52 seconds. Nx exited 1 after a 37 minute 29 second critical path, with original native target 31 minutes 52 seconds. All three new neutral point/wire laws compiled and ran. Actual RED: default unselected pure point emitted no primary lines; ordinary wire with showEdges=false emitted no lines; edge-only original source with no separate position array published no mesh. The old original wire with topology vertices still passed with runtime DEBUG and exact source picks.

Old camera ownership/admission cohorts and duplicate-source exclusion now passed. Three component-gumball capture laws still failed with no handle, leaving their actual native authority unresolved. Five older bridge assertions assumed surface-only draw ordering/count; one malformed two-index republication incorrectly reached the mesh writer and faulted. Those expectations are being hand-edited to retain original wire geometry and select the original solid by its authored mesh key while preserving its triangle, vertex, instance, camera, selection and source expectations.

Production source repair authored after actual RED: original WorldMeshSource reads packed edge endpoints directly only when the original positions stream is absent; the same constructor reserves real render endpoints without synthetic topology IDs or triangles. Original primary wire and pure-point paint use the existing LineList/cross primitives independently of picking/outlines. Existing original inline predicate again drops an incomplete two-index triangle. These repairs are UNQUALIFIED pending fresh original native execution.

Original shared ray_segment_distance source review found its denominator incorrectly used the ray-origin dot product rather than the ray/segment direction product. Eight neutral finite-segment/forward-ray cases and independent Three.Ray oracle were authored before touching that helper. Native baseline session49991 is live. No evaluator, geometry cache, source mirror or adapter was added.


### Original Ray Oracle Baseline Details

Initial native baseline49991 terminated PREASSERT: the original Nextest runner rejected --nocapture without the test-argument separator. Corrected original registered launch and changed rerun33089 now waits at the real Cargo artifact lock; the helper remains unchanged. Initial independent Three suite31233 ran all9 tests:8 passed/1 failed solely because square-rooting Three distanceSqToSegment residual3.55e-15 turned an exact-zero distance into5.96e-8 beyond an overprecise fresh oracle assertion. The new oracle now directly compares squared distance against the neutral squared expectation (12 decimal places), avoiding square-root amplification; original existing selection/merge/geometry expectations remain unchanged. Fresh portable rerun is live. This adjustment qualifies no native behavior.


Portable squared-distance oracle replay89940 ACTUAL GREEN:9/9 tests in1 file; original multi-object merge test independently validates all8 Three.Ray distance cases and retained pivot/merge/source/80-target expectations. Native helperbaseline33089 remains live on the original artifact queue; native behavior is not inferred from this oracle.


## Primary Object Interaction and Fleet Runtime Receipts

Root original WorldRayPickCursor source review confirms it exclusively advances indexed triangles for Instance/Hover, so a visible primary point/wire still cannot emit an object-domain select/hover target. A new actual native original-domain law uses existing neutral point and wire fixtures, drives real Scene publication and registry under one-unit grants, then drives original ray cursor/BoundedPlan publication for point, wire and edge-only wire. Expected exact authored object/granularity is retained; no RayPick production change was authored before that law's baseline. The independent original Three suite now also confirms the fixture pointer intersects the real wire segment and the original point's center ray. Its changed current run is live.

Fleet Mesh8 after2 ACTUAL GREEN:8/8,162 skipped,0.468-second assertion runtime, Nx22m3 including20m18 compilation/artifact wait. Runtime DEBUG records admission fence, full cancellation credit release, source16777217 ByteCapacity refusal, aggregate50013/32768 PageCapacity refusal, scratch4/48, unchanged Vec/String pointers, all three valid wire/point/edge-only domains, and allMAX pure-point Schema refusal with triangle/edge MAX surface samples preserved. The four original wasm producer regeneration lanes now run serially in session71090. Scope/Policy native runtime remains open.

Fleet UI metadata16 terminated PREASSERT5 original CommonMark SQLite namespace errors only (Cargo101, Nx35m4). Five physical owned_pack caller paths were corrected with exact whole-byte reversal; no codec/SQL/retirement/assertion changed. Source-current selected-shell original runtime25510 is live. Fleet document physical sparse Diff record provider closes the missing original Text/Binary macro binding only; no new domain vocabulary/parser/codec was added. Source-current original seven37246 is live and will precede focused new five-case Diff runtime. No selected-shell, full document capacity, actual GPU/browser or complete feature result is claimed.


## Actual Original Ray Baseline and Same-Owner Repair

Original baseline33089 terminal ACTUAL RED:1 selected law failed,169 skipped; assertion runtime0.043s, Nx8m9 including original shared artifact wait. The real native forward ray through a finite segment reported4.131182 for neutral perpendicular-hit instead of0. No domain mutation/evaluator or higher-level approximation was substituted. Original shared ray_segment_distance now minimizes the same geometric squared gap over the valid interior and three boundary regions: forward ray, both finite endpoints and ray-origin boundary; nondegenerate tiny segments remain eligible. It uses existing internal Vec3 arithmetic only. Original empty-segment refusal remains. Fresh original native replay75555 selects the two retained old ray laws and new eight-case law under the registered launch. Runtime qualification is pending.

Portable original primary-object oracle98725 ACTUAL GREEN9/9 in onefile; the new real Three.Ray wire intersection/point center ray assertions and all prior geometry/selection/merge/pivot/metadata expectations pass. Native primary-object law remains an unexecuted baseline; original RayPick production is unchanged. UI test-only original live-refresh authority law now appended and retains exact captured start target IDs across actual Scene source replacement; its original full-prefix gate awaits RootWorld11 after ray helper qualification. No native live-refresh claim.


### Original Primary Object Style Baseline Prepared

The shared neutral primary-geometry-style fixture/schema now names real point/wire/edgeOnlyWire geometry and four object states with semantic line references; no new palette or hexadecimal paint table. Root appended an original Native law for all three geometries, all four states and two live themes. It derives expected existing theme fields from those semantic references, drives actual Scene selection/hover flags, and inspects real original submitted primary lines. Production primary color selection remains unchanged pending baseline. Both launch sources register the exact original primary interaction/style prefix. UI mounted point-hover baseline ACTUAL RED:neutral paint remained for hover; after that RED, the original point-only pointsMaterial now uses already-resolved style.lineColor while ordinary surface vertex grips keep their existing component color. UI combined31 runtime is live. The native source can reuse the existing MeshStyleState/resolve_mesh_style/mesh_style_paint owner after actual RED; no new palette API is required.


### Render Endpoints Are Not Topology Vertices

Original Native world_mesh_component_id falls back to packed position index when VertexIds is absent. Original component pick/marquee and glyph scans use schema.vertices as the candidate count, while the lower Scene vertex selection correctly requires VertexIds. This would make newly retained edge-only render endpoints look selectable even though the neutral fixture explicitly has topologyVertices0. Root strengthened the already-authored edge-only law to drive real one-unit vertex picking and primary-plus-component line submission, requiring no picked topology vertex and no synthetic vertex glyph. No component eligibility production change was authored before that new baseline. The original wire with authored VertexIds remains unchanged.


The strengthened edge-only baseline also retains rectangle and lasso no-topology requirements through the original bounded component marquee cursor. Existing native component-triangle oracle docstrings already require authored vertex/edge ID buffers, matching the lower Scene guard. This is a candidate eligibility issue, not a reason to introduce identity fallback or synthetic vertices. Runtime remains pending World11.


## Native Ray Qualification and World11 Dispatch

Original three-ray-law replay75555 terminal ACTUAL GREEN3/3,167 skipped, Nextest0.107 seconds; original target11m58 mostly artifact/graph preparation. Native DEBUG confirms all8 neutral finite-endpoint/forward-ray cases. The retained perpendicular-gap and degenerate-segment laws remain GREEN. This qualifies the original shared ray distance repair after actual perpendicular-hit RED4.131182 vs0; independent Three9/9 had already qualified the same neutral cases.

Fresh original full World11 now dispatched against the same original world:: filter, one test thread and nocapture; no second Root heavy lane. It includes current real edge-only sampling and default point/wire paint repair, preserved older surface/wire fixtures and exact solid assertions, the original three gumball captures, and new primary object pick/style/live guest refresh baselines. Component eligibility, original RayPick primary geometry, Native primary style and live-refresh production are unchanged before those baselines. No passing World11 assertion is claimed yet.

Fleet original four Wasm producers ACTUAL GREEN4/4 plus5 generation prerequisites, Nx23m57; generated Surface create(actor:string) and native JS actor forwarding verified. Original Scope/Policy five-law current23 session92384 now live. UI final mounted regression ACTUAL GREEN31/31,2 files/0 skip, Vitest13.85/Nx16.4, retaining previous29 plus real guest source refresh and12 point/wire/edgeOnlyWire style states. Original selected-shell second gate65701 remains native-live with no assertion result. Document original seven17685 remains live at the shared artifact lock after all three canonical fixture include paths were repaired; focused Diff5 follows its terminal.
