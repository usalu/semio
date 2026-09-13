# Native Runtime Graph

## Inspection And Intended Contract

The native renderer currently starts a nested Nx activation pipeline from NativeRunScript. That activation unnecessarily includes the browser renderer compiler, boot and worker generators. Its local asset server is created in the same process immediately before a synchronous binary invocation, which prevents that process from answering HTTP while the child runs.

The native loader reads plugin-ID directories and selects a .wasm component beside its JSON descriptor. The browser materializer instead publishes deployment-name directories containing JCO core modules. Reusing that directory as native input can select the wrong kind of WASM and does not satisfy the native loader's identity contract.

The replacement will select a concrete native variant/profile before the outer Nx graph is created. A finite, cacheable publication target will consume the matching completed component and descriptor, publish only selected native modules under one variant/profile owner, and depend on descriptor producers explicitly. The uncached native consumer will use the completed native directory and binary, serve declared assets asynchronously, and close its server and owned process tree on completion/cancellation. The descriptor producer currently uses JCO; retiring that dependency requires a separate native descriptor implementation and is not part of this boundary change.

## Concurrent Source Validation

The current-owner actual WGPU browser compiler retry failed after 11.2 seconds because semio-framework-pack refers to the absent semio_framework_deflate crate. This followed the earlier in-flight codec API change; no pack/codec changes are made by this task. Full repository contract validation is continuing independently.

## First Native Proof

The neutral graph contract failed before implementation (111 ms): native variants had no preparation owner. After implementation the graph-only proof passed. The extended native fixture passed in 12.2 seconds: both compilation profiles, native Nx cold/warm/deleted-output restoration, component-header and descriptor-digest checks, and a real Node child fetching HTTP from its still-responsive parent. Parent credentials were absent in the child. Cancellation reaped the owned child and closed the asset listener; invalid descriptor/component bytes preserved the previous publication.

The native publisher has a separate command file from the runtime consumer so asset-serving and compiler implementation imports do not enter the finite publication command closure. Generated editor profiles are being regenerated. Actual application compilation and painting remain unqualified while the separate pack/deflate source change is unresolved.

## Extended Proof And Actual Publication

The extended fixture passed in 25.0 seconds after selecting ESM output for its esbuild import-closure oracle. It additionally proves a leaf-profile component change invalidates only that profile, unrelated test bytes do not rerun publication, the other profile stays cached, and cancelling the native process kills its spawned grandchild as well. The publisher's esbuild closure excludes the root application, compiler and styling asset server.

The real Puzzle 3D native publication passed: one component with matching descriptor digest in the native plugin-ID directory (967 ms task duration, cache deliberately bypassed). It consumed already completed artifacts with task dependencies excluded; the preceding graph discovery also took time and this is not a fresh full pipeline or end-user startup measurement. No actual application compilation/painting claim follows.

The full existing repo:test suite passed in 2m28s before this native refactor. A new full run now includes the native runtime contract and editor/profile checks. New native run, smoke and preparation entries are generated for both profiles; scale entry points stay uncached with explicit native binary/fixture prerequisites.

## Further Storage Review

The new native runtime currently carries component bytes in each variant/profile publication. This gives the current native loader correct plugin-ID paths and complete Nx restoration, but variants that share a component duplicate its final bytes. A compact runtime manifest referencing separately owned component/descriptor outputs would avoid that duplication; it requires changing the Rust reader contract and qualifying it natively. This remains a storage optimization/ownership follow-up, and the complete Nx goal is not claimed finished.

The first full run with native graph assertions stopped after 2m24s on ordering in the new independent-closure assertion: the graph sorts component IDs, while the oracle sorted project IDs. Both are sets of task prerequisites. The assertion now compares sorted prerequisite identities while retaining the exact session prerequisite check; no production graph behavior changed for this test repair.

The actual current-source WGPU browser compiler retry passed in 42.0 seconds (1/8 Nx hits). Cargo reported its dev compilation finished in 2.20 seconds, followed by canonical two-file Trunk publication. The earlier InputGeneration formatting failure and temporary missing deflate crate no longer prevented this invocation.

## Compact Native Runtime Revision

The real Puzzle 3D native runtime measured 108,758,059 apparent bytes and 108,769,280 allocated bytes in four files: 103,361,437 WASM bytes and 5,396,327 descriptor bytes, plus 295 bytes of metadata. This confirms that variant copies would materially grow retained state. The language-neutral schema and consumer regression are being changed to require a small portable manifest pointing at the existing component/descriptor producers. The native Rust reader will consume those exact references. The copying publication is an intermediate implementation, not the final storage contract.

## Compact Reader Qualification

The compact TypeScript/Nx fixture passed in 12.3 seconds. A native Rust oracle then passed using the production manifest reader, native Serde and SHA-256: both restored profiles resolved to their original component/descriptor outputs, including JSON split across three-byte/seven-byte pages and empty input/output page cases. The strengthened neutral fixture first failed in 23.7 seconds because the schema accepted absolute paths. After aligning the schema, publisher and Rust path rules and enforcing the 1 MiB serialized manifest bound, it passed in 28.6 seconds. Boundary cases include exactly-at-limit JSON, oversized whitespace, hidden trailing data, duplicate modules, unsupported versions and unknown fields. No variant copies of component or descriptor bytes are produced.

The previous full suite stopped after 6m20s at the duplicate-project assertion (not a native dependency-edge assertion). During a graph implementation reload the plugin returns a promise. The test used synchronous assert.throws and immediate array reads; it now awaits the plugin and uses an async rejection assertion. The actual duplicate-project guard remains intact. A fresh full suite is running.

A subsequent actual browser compiler invocation passed in 7m10s with 1/8 Nx hits. Concurrent source changes caused many crates to recompile, so this is not evidence of a warm repeat. The structural audit passed in 15.9 seconds with 704 projects, 7,676 commands and 8,148 artifact records, with zero automated findings. Structural coverage does not establish retention safety or cross-platform runtime behavior.

## Actual Compact Publication

Puzzle 3D compact publication passed with dependencies excluded and caching bypassed (739 ms). The completed native directory now contains 558 apparent bytes and 8192 allocated bytes in 2 files, compared with the previous 108,758,059 apparent bytes. Independent Python hashlib verified that its component reference and descriptor match the manifest SHA-256. This measures the runtime directory only; separately owned component/descriptor outputs and compiler state remain retained.

## Actual Native Integration

The actual current native renderer build passed in 3m06s with 0/5 Nx hits, including the compact manifest reader and ProgramBridge integration. The subsequent actual smoke target did not finish: it emitted process-owner progress through seven intervals and was interrupted (exit 130). Source inspection identifies a headless scheduling gap: run_smoke awaits run_renderer_io, which submits a retained I/O session; only the native frame path calls pump_renderer_io_sessions, while the headless entry point uses the generic block_on executor. No successful shell boot or GPU behavior is claimed. The independent reader/restore fixture remains green. This runtime gap must be resolved or independently qualified before claiming the native entry point complete.

## Full Current Contract Suite

The complete current repo:test target passed in 4m14s with 0/2 Nx hits. It includes the native Rust compact reader, neutral manifest validation, actual Nx restoration/invalidation, process-tree cancellation, editor entries, native Cargo/Trunk/Vite oracles, graph reload behavior and source/lifecycle contracts. git diff --check also passed. This suite does not exercise the headless native shell boot path that stalled in the separate actual smoke invocation. Process-table inspection after cancellation found no surviving process from that smoke invocation.

The attempted actual native repeat stopped before native compilation on a new shared Flow taxonomy validation error: owned flow-browser-package lacks previewTarget. It ran 637 ms and does not establish a warm native hit. The Flow publication follow-up report records the observed output/preview ownership issues without modifying concurrent Flow work.
