# FEM Numerical Owner Follow-Up

Native15's numerical fixture passes mesh construction and reaches PrepareAssembly, then returns FemError::Singular. Window/document laws pass5/5; visual laws pass11/13, with one test fixture grant mismatch corrected afterward. The mesh law selected0 due wrong package; its runner now targets FEM2D, where the shared engine is compiled. See fem-native-window-admission.md.

## Exact Missing Element Interface

The shared Element trait at ✏️s/🔨️modules/🏗️fem/⚙️engine/🏗️model/🦀️.rs defines mounted_node_id, mounted_node_id_count and mounted_stiffness_cell with None defaults. Only three 2D implementations override them. Bar3, Frame3 and Tet4 in ✏️s/🔨️modules/🏗️fem/⚙️engine/🧱️elements3d/🦀️.rs provide none. AssemblyJobConstruction::ValidateElementReferences immediately requires mounted_node_id_count and turns None into Singular. Thus the current native failure has a concrete source path unrelated to an actual matrix factorization. The full Elements enum delegates the trait through dyn_enum_close; the missing implementations belong to the element types.

Implement borrowed node identifiers/count and allocation-free stiffness cell access at the three concrete 3D element owners used by FEM3D. Reuse common scalar/fixed-size math so cold full-matrix and mounted cell calculations agree without invoking an allocating full matrix per cell. Existing Tet4::gradients allocates a 4×4 matrix and four VecD solves; a fixed-size geometry/gradient owner is needed. Bar3 scalar access is direct; Frame3 should reuse its local stiffness and coordinate transform formula in fixed-size form. Do not add a FEM-editor variant dispatch that duplicates Element ownership.

Add neutral scalar matrix/node-reference vectors, compare against an existing independent third-party numerical implementation when available, and execute through the FEM2D engine package. Native full mesh/numerical fixture remains required at2MiB. Preserve every expensive operation's progress/cancel/close contract.

## Mounted 3D Element Source Completion

The Bar3, Frame3 and Tet4 concrete owners now implement the three mounted interfaces directly. Each borrows node ids from its owned strings, publishes its fixed node count, validates context and cell bounds, and evaluates one stiffness cell without allocating a matrix. Their cold `stiffness_global` paths fill the result matrix through the same scalar/fixed-size cell functions, so mounted and cold formulas do not diverge. Frame3 resolves its rolled basis in fixed arrays and contracts only the three local components that can contribute to each transformed global cell. Tet4 replaces the allocating 4x4/four-solve gradient path with scalar triple products and fixed barycentric gradients, then contracts two six-component B columns through the isotropic constitutive tensor. No editor-side element switch or new runtime dependency is introduced.

Schema and fixture preceded the Rust implementation. The strict language-neutral fixture contains complete row-major matrices for a skew Bar3, a rolled skew Frame3 and a skew Tet4. NumPy 2.0.2 independently generated them with dense outer products, transformed Euler-Bernoulli matrix multiplication and affine tetrahedral `BᵀDB`; hashes and version are recorded in `🗑️generated/fem3d-mounted-stiffness-numpy.log`. Ajv 2020 rejects foreign fixture owners and validates every exact shape. fast-json-patch reconstructs every committed cell, and the neutral oracle also verifies symmetry and the three rigid-translation null modes. The isolated existing FEM3D Nx facade passes with strict TypeScript in `🗑️generated/fem3d-mounted-stiffness-neutral-1.log`.

The native law consumes the same fixture through the closed `Elements` enum. It checks every borrowed node id/count and bound, every mounted cell against the NumPy value, exact equality with the cold matrix cell, invalid context rejection, and row/column bounds. The existing FEM3D native facade now runs its focused filter in `semio-s-artifact-fem-2d`, which owns the shared engine. Cargo has not run for this source continuation; mounted-interface and full FEM3D runtime completion remain unverified.

Exact files:

- `✏️s/🔨️modules/🏗️fem/⚙️engine/🧱️elements3d/🦀️.rs`
- `✏️s/🔨️modules/🏗️fem/⚙️engine/🧱️elements3d/🧪️tests/🔬️unit/🦀️.rs`
- `✏️s/🔨️modules/🏗️fem/⚙️engine/🧱️elements3d/🧪️tests/🧫️fixtures/🧱️mounted-stiffness/🔣️.json`
- `✏️s/🔨️modules/🏗️fem/⚙️engine/🧱️elements3d/🧪️tests/🧬️schema/🧱️mounted-stiffness/🔣️.json`
- `✏️s/🔨️modules/🏗️fem/⚙️engine/🧱️elements3d/🧪️tests/🧱️mounted-stiffness/🟦️.ts`
- `📜️script.ts`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️08/CORRECT-COMMAND-CONFIG-AND-MUTATION-OWNERSHIP-LEVELS/validation/📜️script.ts`

The scoped diff check and `rustfmt --check` pass. Root's value-layer `PagedList` was consumed only after its separate native and neutral verification completed.

## Physical Backing Is Separate From Logical Partitioning

AssemblyPartitionBuffer in ✏️s/🔨️modules/🏗️fem/⚙️engine/🧮️analyses/🦀️.rs holds contiguous full/free triplet vectors. BuildPartitions reserves a dense maximum divided by logical worker partition count, but every vector is constrained to one4KiB owner. Several valid Tet4 elements exceed this gate with the caller's one logical partition. Raising stack, changing the fixture's partition count, or increasing the single-backing limit would not establish the proper physical ownership.

The reusable owner is now verified at `🧰️framework/🔨️modules/🌱️value/📋️list/🦀️.rs`. `PagedList<T, N>` separates fixed-fanout metadata and payload backing, admits at most one allocation through `reserve_capacity_one`, returns an unplaced producer from `push_reserved`, and releases one empty backing allocation under an exact byte grant. `framework-paged-list-native-2` passes four generic and eight UI laws on 2 MiB plus Ajv/JSON Patch and strict TypeScript; the extraction evidence is recorded in `framework-paged-list-ownership.md`. FEM can consume this owner from the replication value package without a UI dependency.

The retained assembly mapping needs no generic API extension. Each partition full/free list reserves toward its exact share of `maximum_triplets` one physical allocation per construction opportunity; the two merged lists reserve toward the full bound before element publication. Publication uses `push_reserved`, merge reads use `get`, and insertion sort can exchange two copied `AssemblyTriplet` values through sequential `get`/`get_mut`. Close first pops initialized payloads, then releases one empty backing allocation with its actual byte count. Logical partition count stays unchanged. The later continuation below pages the final CSR owner and its readers.

## Execution Handoff

Root's current source/oracle work is ready for FEM16: three Graphlib edge-adjacency cases, four full-page child-outcome cases with zero fuel/deadline/cancel preservation, proper StepOutcome retirement in all five numerical delegated stages, and mesh allocation after completed faces. Root should execute this when shared v17 channel/plugin changes are coherent. Temporary stage-qualified numerical failure in the fixture remains; the generic mesh temporary log is removed.

The 3D element and first assembly physical-owner source implementations are recorded below. This document remains a source audit, not a runtime pass.

## Paged Assembly Source Continuation

After the generic value-layer owner passed its independent abstraction laws, the retained assembly source was changed to consume it directly through the FEM2D engine package. Construction accumulates the exact worst-case triplet count for the actual `element_index % partition_count` assignment. Each partition full/free owner and each merged owner then admits at most one fixed-fanout metadata or payload allocation per opportunity toward that exact bound. One logical partition remains one logical partition. `assemble_cell` verifies both required destinations before moving either copied triplet, so a refusal cannot leave a full entry without its eligible free entry. The deterministic merge reads and publishes through the paged owner.

The shared cold `AssemblyJob::new` accepts a dynamic `AnalysisModel` and has no 128-element admission bound. Existing analysis coverage includes a 120-Hex8 model whose 24-by-24 cells already exceed a mounted-only logical total. The paged triplet type therefore uses the complete checked `usize` index space: actual requested capacity still comes from checked node-count, DOF-count, side-square and total additions, while `PagedList` rejects cumulative physical ownership beyond `isize::MAX`. This preserves the pre-existing dynamic surface without an arbitrary new element limit. The mounted route separately retains its 128-element model admission. Every metadata and payload allocation remains limited to 4 KiB, and logical partition count remains unchanged. The focused assembly-page law instantiates this maximum index space and therefore also covers its deeper fixed-fanout metadata path. A Bun BigInt cross-platform calculation in `🗑️generated/fem-assembly-max-index-space-neutral.log` confirms maximum radix-tree shifts of 24 bits on 32-bit targets and 56 bits on 64-bit targets, both below the native word width; native traversal is still unverified.

`AssemblyCsrBuild` now retains the moved paged triplet owner and exchanges copied values through indexed mutable access during its existing one-comparison insertion-sort opportunities. A separate unique-key counting stage precedes final CSR allocation. The first continuation still emitted contiguous compressed arrays; the follow-up below supersedes that source.

The focused `assembly_triplet_pages_cross_one_physical_page_without_extra_logical_partitions` law constructs fifty six-DOF beam owners. Its exact triplet bound crosses multiple 4 KiB payload pages while retaining one logical partition. It asserts that each construction opportunity changes paged backing by no more than 4 KiB, all four assembly lists exceed one physical page, and close retires at most one item/allocation per opportunity with no release above the byte grant. The registered FEM3D native facade selects it in the FEM2D engine package with `assembly_triplet_pages_` after the mounted-element law.

This continuation adds the following source files to the ledger:

- `✏️s/🔨️modules/🏗️fem/⚙️engine/🧮️analyses/🦀️.rs`
- `✏️s/🔨️modules/🏗️fem/⚙️engine/🧮️analyses/🧪️tests/🔬️unit/🦀️.rs`
- `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/📦️packages/🦀️rust/Cargo.toml`

Rustfmt parses the element, assembly and focused-test sources, and the scoped whitespace check passes. No Cargo command has run for either FEM continuation. The new element, assembly-page, correct mesh-package, full-page child-outcome and full visual laws all remain native-unverified.

## Downstream Sparse Ownership Audit

The production solid fixture's support and equation ownership is coherent before sparse construction. Its four document nodes are inserted first and their ids are retained in analysis order at `session/🦀️.rs:674-713`. Every extruded solid point then searches the model by coordinate before creating a node at `session/🦀️.rs:905-962`. Thus the base coordinates reuse `n0` through `n3`, while only upper-layer coordinates create new analysis ids. The committed independent `rect-unit-square` mesh oracle records eight vertices for the two-layer extrusion and therefore four footprint vertices; with one layer the mounted fixture has four new upper nodes. Tet4 exposes three translational DOFs per node, and the fixture fixes all three translations at the four base nodes, so the expected free order is twelve. This is a source and oracle derivation; the mounted runtime must still confirm it.

The immediate sparse stages fit their existing bounded owners for that order. `MountedScalarSlots` has 768 fixed scalar slots at `sparse/🦀️.rs:434-475`. `PcgJobConstruction` admits each of seven length-12 vectors independently and enforces a 4 KiB allocation at `sparse/🦀️.rs:2384-2489`. `ModalInputConstruction` counts the retained upper triangle before allocating stiffness arrays and builds a diagonal mass CSR under the 16 KiB owner gate at `sparse/🦀️.rs:2129-2303`. FEM3D rejects modal order above forty before LDLT at `session/🦀️.rs:1640-1659`; LDLT independently validates order and every input owner at `sparse/🦀️.rs:652-747`, and subspace validates the same order and sparse/factor owners at `sparse/🦀️.rs:2925-2967` and `4418-4445`.

The audit identified the contiguous `Csr` boundary before the following source continuation; the prior limitation is retained here as execution history rather than current source state.

## Merge Refusal and Final CSR Continuation

`advance_partition_merge` now retains the exact producer returned by `push_reserved`, leaves the source cursor unchanged on refusal, and clears the candidate plus advances its cursor only after successful placement. The schema-first two-lane fixture covers full and free destinations. Ajv rejects foreign fields, fast-json-patch removes one destination item, and an independent transition reducer proves that retry publishes the byte-identical candidate exactly once. Direct Bun and strict TypeScript pass in `🗑️generated/fem-assembly-merge-refusal-neutral-2.log` and `fem-paged-csr-tsc-1.log`. The Rust law fills the real destination to its admitted capacity, observes the retained candidate/cursor after refusal, retires one filler, retries, and closes under a 4 KiB grant.

Final compressed ownership is paged in source. `AssemblyCsrBuild` allocates row counts, row pointers, column indices, and scalar values through independent `PagedList<_, usize::MAX>` owners, one physical allocation per retained opportunity, and transfers the three final arrays directly to `Csr`. `Csr` no longer stores a contiguous runtime variant. PCG, modal input, subspace iteration, checkpoint publication/restoration, and close paths use indexed paged reads or one-page retirement. Cold Vec constructors and the existing value codec preserve the wire shape by moving values into pages; they do not become a second stored representation.

The language-neutral CSR fixture records a NumPy 2.0.2 dense oracle for a 513-order, 1,537-entry tridiagonal matrix. The neutral implementation reproduces row pointers, SHA-256 column/value bytes, selected matrix-vector action values, and the action L1 norm. It passes Ajv and strict TypeScript in `🗑️generated/fem-paged-csr-neutral-2.log` and `fem-paged-csr-tsc-1.log`; the generating NumPy output is `fem-paged-csr-numpy.log`. The native law builds the same CSR through the retained cursor, checks that both final entry arrays cross 4 KiB while every allocation opportunity remains at most 4 KiB, compares the committed NumPy action, and retires the construction and matrix under exact grants.

This continuation touches:

- `✏️s/🔨️modules/🏗️fem/⚙️engine/🧮️analyses/🦀️.rs`
- `✏️s/🔨️modules/🏗️fem/⚙️engine/🧮️analyses/🧪️tests/🔬️unit/🦀️.rs`
- `✏️s/🔨️modules/🏗️fem/⚙️engine/🧮️analyses/🧪️tests/🧫️fixtures/🔀️merge-refusal/🔣️.json`
- `✏️s/🔨️modules/🏗️fem/⚙️engine/🧮️analyses/🧪️tests/🧬️schema/🔀️merge-refusal/🔣️.json`
- `✏️s/🔨️modules/🏗️fem/⚙️engine/🧮️analyses/🧪️tests/🔀️merge-refusal/🟦️.ts`
- `✏️s/🔨️modules/🏗️fem/⚙️engine/🔢️sparse/🦀️.rs`
- `✏️s/🔨️modules/🏗️fem/⚙️engine/🔢️sparse/🧪️tests/🧫️fixtures/📚️paged-csr/🔣️.json`
- `✏️s/🔨️modules/🏗️fem/⚙️engine/🔢️sparse/🧪️tests/🧬️schema/📚️paged-csr/🔣️.json`
- `✏️s/🔨️modules/🏗️fem/⚙️engine/🔢️sparse/🧪️tests/📚️paged-csr/🟦️.ts`

Rustfmt and the scoped whitespace check pass. Root imported both neutral oracles into the common FEM3D facade. The common neutral facade passes both new oracles together with the existing FEM contracts in `🗑️generated/fem3d-engine-neutral-5.log`.

Three whole-route native attempts reached no selected native test. FEM17 stopped on the corrected `close_paged_owner_step` return type before compiling the test modules; FEM18 compiled the FEM engine production packages and stopped on a viewport fixture serializer; FEM19 stopped in a concurrently edited shared Store open-operation state machine before the FEM package. The exact logs are `🗑️generated/fem3d-window-config-native-17.log`, `fem3d-window-config-native-18.log`, and `fem3d-window-config-native-19.log`. The engine production surface is therefore compiler-verified through FEM18, while the merge-refusal, paged-CSR, mounted-element, assembly-page, and full numerical runtime assertions remain native-unverified.

The next coherent native run must retain the registered order: `fem3d_window_config_` in `semio-s-artifact-fem-3d`; `mesh_edge_authority_`, `mounted_3d_element_interfaces_`, and `assembly_triplet_pages_` in `semio-s-artifact-fem-2d`; then `live_visual::tests::` in `semio-s-artifact-fem-3d`. The existing ticket target is `abstraction-ownership-validation:fem3d-window-config-contract-native`; its runner owns that package/filter ordering. Run it from the ticket `validation` directory with `NX_DAEMON=false NX_ISOLATE_PLUGINS=false`, a fresh `NX_WORKSPACE_DATA_DIRECTORY=../🗑️generated/nx/<run>`, and `bun /Users/ueli/Documents/semio/node_modules/nx/dist/bin/nx.js run abstraction-ownership-validation:fem3d-window-config-contract-native`. No page or stack limit should change: the engine laws use 4 KiB allocation grants and the numerical fixture remains on its existing 2 MiB thread.

## Root Native21 Feedback and Native22

Native21 passed all seven FEM3D window ownership laws, including the renderer-origin camera command and exact scene echo. The following FEM2D test compile stopped at the new final CSR law: `Result::expect` required `AssemblyJob: Debug` from its ownership-preserving rejection type. Root changed only that assertion to `unwrap_or_else` with the same failure message. Production assembly ownership and numerical logic are unchanged by this fix. Native22 is running the full registered selected route; its result is not yet claimed.

Additional changed file: `✏️s/🔨️modules/🏗️fem/⚙️engine/🧮️analyses/🧪️tests/🔬️unit/🦀️.rs`.

## Root Native22 Runtime Findings

Native22 passed seven FEM3D window laws, one exact mesh-edge law, one full NumPy mounted-element law, and the merge refusal/retry law. The assembly page-close fixture failed because its cold dynamic model retained a contiguous 50-element allocation larger than the 4 KiB grant. The final CSR fixture then stalled inside the cold paged-capacity helper: `next_allocation_bytes` correctly returns zero when a placement slot already exists, but this helper was reserving additional empty capacity and repeatedly submitted that zero grant. Root terminated only its identified hung native test process. The route is failed, not green; full numerical tests did not run.

The cold capacity helper now submits the fixed physical page grant and returns an error if no allocation progress is made. The assembly fixture uses the actual mounted model owner for its 50 elements, preserving the four 4 KiB paged triplet owners and the exact close limit. The 513-order final CSR fixture retains its dynamic source model and plan; its cleanup grant is computed from those actual contiguous source allocations. Once the source assembly retires, all CSR construction pages and final matrix pages still close with the 4 KiB grant. This makes the test distinguish source ownership from final CSR page ownership. These corrections are authored but await the next native run.

Additional changed files: `✏️s/🔨️modules/🏗️fem/⚙️engine/🧮️analyses/🦀️.rs` and its `🧪️tests/🔬️unit/🦀️.rs`. Shared job physical close is independently green for all 27 job-crate laws; downstream FEM numerical validation remains queued.

## Source-Coherent Native23 Queue

The final CSR fixture now uses a 171-node BeamEb2 source model (513 DOFs), whose owned strings implement the existing bounded retirement contract. The previous synthetic axial spring helper is used by other cold numerical tests and does not implement that retirement contract; it was an unsuitable source owner for this close law. The CSR entries and NumPy matrix-action fixture remain unchanged. Its actual dynamic source-model/plan allocations receive their own computed close grant; all final CSR page owners retain the 4 KiB grant. Rustfmt parser checks accept both updated assembly files; scoped diff-check is clean.

The FEM numerical-child outcome law now uses a short `retained-child-state` payload. Its insufficient physical grant and exact release assertions are unchanged; the shared job crate has already independently proven empty, short and full page behavior.

The registered FEM3D route groups all selected filters by package using the native harness's verified `[FILTERS...]` interface. A read-only test listing confirms **9 selected FEM2D tests** (four window laws plus mesh, mounted, and three assembly laws) and **19 unique FEM3D tests** (seven window laws and the full live-visual module, with one overlapping window/visual law). Both dimension-neutral oracles run before these native commands. This includes the previously pending FEM2D window runtime proof and avoids switching back and forth between package feature graphs. These are expected selections, not passing native results. Native23 waits behind the coordinated app batch and recursive retry.

Additional changed files: root and ticket `📜️script.ts`; `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🧵️session/🧪️tests/🔬️unit/🦀️.rs`.

Combined neutral6 exits0 in 2.9s: both FEM dimension contracts, the two story laws, shared camera and exact command-owner contracts, merge refusal, NumPy CSR and mounted matrices, and strict TypeScript all pass. FEM23 is now running against shared Store/plugin source that passed recursive suite49 (5/5 with nonempty histories). The schema catalog regeneration proceeds separately; no full FEM native result is claimed yet.

## Native23 and Focused Numerical Follow-Up

Native23 exited 1 after **9/9 selected FEM2D tests passed** (947 filtered, 0.15s) and **18/19 selected FEM3D tests passed** (829 filtered, 0.07s). This proves both exact-window lifecycles, the mounted node/cell NumPy comparison, mesh edge authority, all three assembly page/merge/CSR laws, and short retained child-payload retirement. The full production solid/reaction/modal law failed on its single-step fuel assertion; its final numeric and close assertions were not reached. Log: `🗑️generated/fem3d-window-config-native-23.log`.

The permanent focused route `fem3d-numerical-child-native` is registered in root and ticket scripts/targets and both launch catalogs at 311.212. Focus1 exited 1 with **12/13 selected tests passed** (835 filtered, 0.01s). Its assertion identifies the failing child stage as Assembly and context stage as `fem.assembly.element-triplets`, with terminal=false and the one-unit grant unchanged. Log: `🗑️generated/fem3d-numerical-child-native-1.log`.

The shared AssemblyJob previously cleared checkpoint/preview flags before checking the grant, and terminal publication consumed no fuel. The common admission check and one-unit consumption now precede every control, publication, and work branch. A new schema-first twelve-case fixture checks checkpoint, preview, terminal, and normal work against zero fuel, an expired deadline, and an admitted opportunity. Ajv validates closed fixture records; fast-json-patch independently reconstructs each expected state. The native law uses real retained assembly construction and exact paged cleanup. It is selected by the existing `assembly_triplet_pages_` filter.

Combined neutral7 exits 0 in 2.6s, including all twelve new grant cases, both dimension contracts, the two story laws, CSR/mounted NumPy oracles, existing ownership vectors, and strict TypeScript. Rustfmt parses the two changed Rust sources. Native verification of this admission fix is pending behind the coordinated four-app batch; no corrected native result is claimed.

New changed paths for this continuation:
- `✏️s/🔨️modules/🏗️fem/⚙️engine/🧮️analyses/🦀️.rs`
- `✏️s/🔨️modules/🏗️fem/⚙️engine/🧮️analyses/🧪️tests/🔬️unit/🦀️.rs`
- `✏️s/🔨️modules/🏗️fem/⚙️engine/🧮️analyses/🧪️tests/🧫️fixtures/⛽️step-grant/🔣️.json`
- `✏️s/🔨️modules/🏗️fem/⚙️engine/🧮️analyses/🧪️tests/🧬️schema/⛽️step-grant/🔣️.json`
- `✏️s/🔨️modules/🏗️fem/⚙️engine/🧮️analyses/🧪️tests/⛽️step-grant/🟦️.ts`
- `✏️s/🔌️plugins/🏗️fem/🧪️tests/🪟️window-config-contract/🟦️.ts`
- root/ticket `📜️script.ts`, root `📋️project.json`, ticket validation `project.json`, and both root launch catalogs for the focused route.

### Adjacent Sparse Control Admission Finding

Source review of `sparse/🦀️.rs` finds that PCG emits a terminal, preview, or checkpoint after its work loop even when the loop performs no work because the context is already exhausted. Those branches can therefore publish and clear flags without an admitted opportunity. LDLT and subspace publication branches explicitly guard and consume their own grants. PCG requires a direct failing zero-fuel/deadline control test and a coherent publication-before-work repair, followed by existing convergence/checkpoint/preview laws. This is a source finding; no PCG runtime failure or correction has yet been claimed.

The adjacent PCG finding now has a dedicated schema-first nine-case fixture at `sparse/🧪️tests/🧫️fixtures/⛽️publication-grant/🔣️.json`, its strict JSON schema, independent Ajv/JSONPatch oracle, and a native real-PCG control law. It covers pending preview, checkpoint, and terminal publication with zero fuel, expired deadline, and one admitted grant. The native law records the exact work cursor and pending flag and drains all actual payload/model owners before asserting, including on the expected red path. PCG production remains unchanged for this first runtime proof.

Native24 has started through the canonical FEM route, now selecting eleven FEM2D laws (the original nine plus assembly admission and PCG publication) before the nineteen FEM3D laws. Its neutral stage passes all prior oracles plus twelve assembly and nine PCG grant vectors and strict TypeScript. Native results remain pending.

## Native24: Shared Assembly Green, PCG Publication Red

Native24 exited 1 with **10/11 selected native laws passing** (947 filtered, 0.24s). The new twelve-case assembly control-grant law passes alongside all nine earlier engine/window laws. The new PCG law fails its first zero-fuel checkpoint case: actual output is checkpoint with pending=false, while the neutral fixture requires yield with pending=true. The actual work cursor and fuel both remain zero. This is direct runtime evidence for the adjacent source finding. All fixture payload/model owners are drained before the red assertion. Log: `🗑️generated/fem3d-window-config-native-24.log`. The FEM3D package did not run because the first native package failed.

PCG now checks admission before publication and consumes one unit for a pending checkpoint, preview, or terminal result. Pending publication runs before new numerical work; work pauses when a publication flag becomes due, and the next admitted opportunity publishes it. This retains the numerical sequence while making the control boundary explicit. Its three existing deterministic-convergence, checkpoint-resume, and coarse-preview laws are included in the next native selection. Their harness now drains actual retained payloads instead of implicitly dropping nonempty outcomes. Both modified Rust sources pass rustfmt parser checks. The next route selects fourteen FEM2D tests plus nineteen FEM3D tests; it waits for the shared document hydrator's coherent integration.

The new sparse fixture/schema/oracle live under `✏️s/🔨️modules/🏗️fem/⚙️engine/🔢️sparse/🧪️tests/{🧫️fixtures,🧬️schema}/⛽️publication-grant/🔣️.json` and `🧪️tests/⛽️publication-grant/🟦️.ts`. This continuation also changes sparse production and native test sources, the common FEM contract oracle imports, and root/ticket FEM native filter registration.

### PCG terminal witness and independent package completion

The publication fixture now also requires `terminalEmpty: true` after explicit complete owner retirement. The source review found a likely stale PCG close-lane witness: the close ladder finishes after matrix plus seven scalar owners, while the InteractiveJob witness currently requires a lane above nine. This remains unchanged for the next native red proof. The fixture, strict schema, independent JSONPatch oracle, and native observed state all include the terminal witness.

The two FEM native package invocations now run sequentially and collect independent failures in an AggregateError after both finish. A failed engine law therefore no longer hides the 3D numerical result; a failed package still makes the entire route fail. This is registered in the same root/ticket script branches and requires no new launch command.

Shared recursive hydration suite52 now passes5/5 with684 filtered and0.43s runtime. NodeGraph's prepared shared UI type switch is the final source-coherence dependency before FEM25; root does not run Cargo during that short interval.

## Native25 Complete Package Results

Native25 exited1 in1m29s after both packages ran.

- FEM2D: **12/14 passed**,944 filtered,0.09s. All four window laws, all four assembly laws, mesh, mounted NumPy matrices, PCG checkpoint/resume, and coarse preview pass. The publication fixture's first zero-fuel case now correctly yields and preserves its pending checkpoint; only `terminalEmpty` differs after complete retirement. The batch-determinism law reaches the synchronous `pcg()` reference and fails because that driver implicitly drops a retained payload.
- FEM3D: **18/19 passed**,829 filtered,0.40s. The full numerical child passes its per-step fuel/time checks through the entire200,000-opportunity limit, but does not reach terminal. No final displacement/reaction/eigen/close completion is claimed.

Log: `🗑️generated/fem3d-window-config-native-25.log`. Both independent package failures are correctly collected by the updated runner.

PCG's terminal witness is corrected to the actual completed lane8 (matrix plus seven scalar owners). The cold synchronous driver now explicitly drains preview/checkpoint/commit/fault payloads using the existing shared batch-payload helper and retires its job owners before returning or failing. The direct publication/terminal fixture and batch-determinism test already provide the red evidence for these two corrections.

The FEM3D runtime law retains its200,000-opportunity limit and8ms step ceiling. Its failure diagnostic now includes the final numerical/context stages, actual PCG/subspace progress tuples, and pending-outcome state. The next focused/native run will distinguish a stalled cursor from an insufficient test opportunity bound without assuming either. Rustfmt parses both changed sources. Native verification of these corrections remains pending behind the corrected GIS/Remodel/Norm/Drawing batch.

UI Scene independently reports its quick native route green:117/117 tests, zero skipped, after the NodeGraph shared-type switch. This does not cover the viewport leaf's extended Serde corpus.

## Revalidated Continuation

The previous turn made concrete source and validation progress; this continuation does not restart completed tests. Current source still contains PCG lane8 terminal completion, explicit cold-driver payload/job retirement, and the full numerical-child stage/progress diagnostic. No FEM native process was live at resumption. Native26 has now started through the registered Bun/Nx route with14 FEM2D and19 FEM3D selected laws,2MiB configured stack, and both-package failure collection. Its combined neutral phase passes. Log: `🗑️generated/fem3d-window-config-native-26.log`.

Source review also found an adjacent exact-owner violation in `PcgJob::step_precondition`: after filling the preconditioned vector, the initial branch assigns `self.state.p = self.state.z.clone()`. Construction already admitted the search-direction vector, so this reallocates/replaces a complete vector during a scalar step. A subsequent direct pointer/capacity law should preserve the admitted p backing through initialization and compare scalar values against the existing numerical oracle. This finding is recorded for correction after the current native result; no fix or test pass is claimed.

Native26 has completed its first package: **14/14 selected FEM2D laws pass**,944 filtered,0.10s runtime. This includes all nine PCG publication/terminal fixture cases, batch-determinism versus the repaired synchronous reference, exact checkpoint resume, coarse preview, four assembly laws, four window laws, mesh and mounted NumPy matrices. The FEM3D package remains live; no complete-route pass is claimed.

## Native26 Final Result And Solver Cursor Review

Native26 exited 1 after 6m17s. FEM2D passed 14/14 and FEM3D passed 18/19 (829 filtered, 0.42s runtime). The full production numerical child consumed all 200,000 granted opportunities within its 8ms per-step ceiling. Its final diagnostic is Subspace/JacobiRotateCell, PCG iteration 11/512 with residual 5.243634533307156e-9 below 1e-8 and converged=true, subspace iteration 6/30 with relative eigenvalue change 0.06117360200046365 and converged=false, and no pending child outcome. This demonstrates ongoing numerical progress, not terminal completion. Final fields and bounded numerical close remain unreached.

The Jacobi cursor preserves its sweep counter across reset and permits 100 sweeps per subspace iteration. The fixed 200,000 whole-job test bound does not express that work budget. Before revising it, source inspection found a more concrete mathematical defect: forward substitution changes to diagonal scaling after the first factor column, and diagonal scaling then advances through every remaining row. A non-diagonal 3x3 LDLT corpus with three right-hand sides is being added to validate the actual retained cursor independently with NumPy 2.0.2. The same scalar-owner corpus covers the previously identified PCG search-direction backing replacement. Production fixes are pending runtime red evidence.

The current fleet is root FEM coordination, Sol app-window runtime completion, Sol NodeGraph/shared viewport and host completion, and Sol lower retained Pack physical ownership after Terra's completed architecture audit. The coordinated native queue is apps GIS13/Remodel9/Norm7, then NodeGraph renderer/host, then FEM27. Other developers' independent Cargo work is not treated as a reason to stop.

Neutral8 exited 0 through the registered Bun/Nx route in 9.7s. It includes the new strict scalar-owner schema, three exact non-diagonal right-hand-side solutions, three single-index precondition updates, all earlier oracles, both story tests and strict TypeScript. NumPy 2.0.2 independently reconstructed K=L D Lᵀ, multiplied all three known solution columns, and solved K X=B to the exact committed values. Rustfmt parses the new native laws. FEM27 now selects 16 FEM2D laws and the existing 19 FEM3D laws. Production remains unchanged for the native red proof; no solver correction pass is claimed.

FEM27 is live after the Pack source/callsite coherence repair and completed app batch. Its neutral phase passes and shared Pack/kernel/UI/plugin compilation is underway. The new native assertions compare typed floating-point values from the fixture, avoiding JSON integer-versus-float representation differences. Log: 🗑️generated/fem3d-window-config-native-27.log. GIS13, Remodel10 and Norm9 have passing selected app laws, but their runtime helpers explicitly use 8 MiB threads; those results are not evidence of normal 2 MiB runtime behavior.

FEM27's first package completed after 4m09s compilation: 14/16 laws pass, 944 filtered, 0.32s runtime. Both new regressions are red after exact job cleanup. PCG's first admitted precondition step leaves direction [0,0,0] rather than [3,0,0]. The retained subspace forward/diagonal/backward cursor produces [[1.17578125,-1.033203125,0.6015625],[-2.2109375,-0.16015625,-1.421875],[3.28125,2.046875,2.5625]], versus NumPy's exact [[1,-2,0.25],[-2,1,-1],[3,0.5,2]]. The factor cursor still reaches its next phase, changes at most one scalar per step and preserves its backing; the failure is mathematical ordering. FEM3D remains running against unchanged solver source.

The next run additionally selects all existing subspace laws: diagonal, non-diagonal, finite null-mode sentinel, checkpoint/resume, and cancellation/worker scheduling replay. The first package will select 21 laws. These are required numerical/checkpoint regressions for the forthcoming factor-step correction; no new passing claim is made.

## Native27 Completion And Corrections

Native27 exited 1 after both packages completed. FEM2D remains the clear mathematical/ownership red proof above (14/16). FEM3D passed 17/19 with 829 filtered in 0.79s after 5m00s compilation. Its maximum-plus-one refusal law exceeded the 8ms wall-clock assertion, and the full numerical child observed fuel=1 at BuildCsr with context stage still initial. That second result occurs before a child-stage grant and may be an already-expired deadline under scheduling contention; the original run did not record deadline state, so that cause is an inference. This run does not supersede Native26's later subspace-iteration diagnostic with a terminal numerical result.

The proven sparse defects are now corrected. Initial PCG preconditioning writes each computed scalar to both z and the already-admitted p backing, eliminating whole-vector clone/replacement. Retained subspace forward substitution increments through every lower-factor column, then explicitly transfers to diagonal scaling; backward substitution and per-column reset remain in their proper order. Both changed Rust sources parse with rustfmt and the focused diff check is clean.

The next full native run selects 21 FEM2D laws and 19 FEM3D laws with --test-threads=1. This removes contention between the native test cases while retaining the original 8ms per-step ceiling, real deadlines, one-unit fuel and 200,000 full-child opportunity limit. The unconsumed-grant assertion now also records expired-deadline state and elapsed microseconds. A borrowed static stage label replaces the test's diagnostic String. No threshold was raised and no corrected native pass is claimed yet.

Cargo is handed to NodeGraph renderer/host/WASM, then the lower Pack physical-owner laws. FEM28 follows. The source/callsite Pack repair compiled as a dependency in Native27, but that compilation alone does not validate its new allocation/release laws.

## Native28 Completion And Modal Publication Correction

Native28 exited 1 after independently completing both packages. FEM2D passes 18/21 with 939 filtered in68.09s, including both new scalar-owner laws, checkpoint/resume and the full cancellation/worker-scheduling replay. The three older modal laws now expose a second cursor defect: the diagonal case returns5 instead of1, the non-diagonal comparison fails, and the finite-null-mode case returns an empty eigenvalue list. FEM3D passes18/19 with829 filtered in0.41s. The full numerical child exceeds the unchanged8ms wall assertion; its fuel assertion passed first. The maximum-plus-one8ms law passes. Log: `🗑️generated/fem3d-window-config-native-28.log`.

Root traced the modal failures to convergence transferring into PublishIteration with first=p, which skips the requested eigenvalue prefix and fails to clear the previous values. The same transition sets converged before publication, allowing the next step's terminal branch to bypass the latest iteration. The source now resets the publication cursor and sets convergence only after all theta scalars and candidate vectors have been published.

The scalar-owner schema and fixture now contain diagonal[1,2,3], independently confirmed eigenvalues[1,2,3], requested mode counts1 and3, an old[91,92,93] buffer, and three prefix updates. A real one-fuel InteractiveJob test checks zero-fuel stability, exact backing preservation, each published scalar, and delayed terminal convergence. NumPy2.0.2 eigvalsh produced the committed values; log `🗑️generated/fem-modal-publication-numpy.log`. The older three native failures provide the runtime red evidence; the new direct transition law is not yet run.

Neutral9 passes in7.4s through Bun/Nx, including both story laws, strict TypeScript and the extended Ajv/JSONPatch scalar publication oracle. Both changed sparse Rust sources parse with rustfmt. The full numerical child retains its8ms/one-fuel/200,000 limits and now reports actual elapsed time and stage on wall failure. Native29 will select22 FEM2D laws and19 FEM3D laws after the lower Pack outer-owner acceptance run. No corrected modal or complete numerical-child native pass is claimed yet.

The read-only follow-up in `fem3d-step-ownership-follow-up-audit.md` traces actual remaining mesh multi-reserves, model-root allocation, PCG whole-preview/checkpoint encoding and solid identifier copying under one fuel. Root confirmed PCG's actual interactive branches still call `encode_value(&self.preview())` and `checkpoint_bytes()`, while retained LDLT/subspace use paged writers. These source gaps remain required corrections even if the small full-child fixture later passes. None is assigned as the cause of native28's wall failure without the enhanced diagnostic. PCG's existing cold from_checkpoint/checkpoint_bytes helpers and exact checkpoint-resume law must be kept coherent with a schema-first retained payload implementation; do not keep a synchronous interactive fallback.

Native29 is now live after the Pack outer native acceptance passed2/2. Its neutral phase passes. Log: `🗑️generated/fem3d-window-config-native-29.log`.

While native29 was still compiling upstream dependencies (no FEM package compile line yet), root added static PCG branch stage labels for complete, preview and checkpoint encoding. This changes diagnostics only and leaves admission, fuel and numerical state untouched. A PCG wall failure can now distinguish a control serializer from ordinary scalar iteration.

## Native29 First Package

FEM2D completed21/22 tests with939 filtered in184.42s after7m38s compilation. All three previously failing modal value laws now pass, along with the existing exact checkpoint/scheduling/cancellation replay and both scalar-owner laws. The sole failure is the new publication fixture at opportunity0 for requested modes1. Root inspected the constructor signature and found the test passed modes as matrix order, creating a rejected1x1 state around3x3 owners; this is a harness error, not another production numerical defect. The test now calls the constructor with order3 and the fixture mode count, and its assertion distinguishes yield/backing/terminal observations.

The second FEM3D package continues independently. Root registered `fem-scalar-owners-native` to rerun the three direct scalar tests plus their strict neutral oracle without repeating the already-passing expensive scheduling corpus after this test-only correction. The command is exposed in root and ticket scripts/Nx targets and both launch files at311.228. The focused rerun waits until the Pack catalog source switch is coherent.

Native29 completed exit1 in14m40s. FEM3D again passes18/19 with829 filtered in0.51s. Its full-child wall failure now records after-stage RecoverReaction, context initial, terminal=false, deadline exceeded, and elapsed8015µs. PCG/mesh bulk work is therefore not established as this failure's cause. The test recorded only the after-stage; source can reach it either by reading one node scalar and setting the reaction cursor or by recovering one full-CSR entry. Both inspected branches perform bounded scalar lookup/arithmetic without allocation on their success path. A scheduling interruption is possible, but not proven by the log.

The next diagnostic records before/after stage, node/axis/reaction cursor and pending-outcome presence, and captures elapsed time immediately after the child step rather than after the fuel assertion. Limits remain unchanged. This is test instrumentation only. Complete3D numerical output and bounded terminal close remain unproven.
