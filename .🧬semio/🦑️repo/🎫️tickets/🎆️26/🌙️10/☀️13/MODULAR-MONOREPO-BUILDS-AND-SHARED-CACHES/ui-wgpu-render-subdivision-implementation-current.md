# UI WGPU Subdivision Source Implementation

## Status

The approved real production body subdivision is staged. Native execution remains held. This is **not a Rust compilation, GPU runtime, production-readiness, or throughput result**. The retained actual first native attempt91733 still failed before compiler admission at Replication; it has not been retried.

Independent Source review found an incomplete submission lifetime: current actual draw/readback callers discard `Submission` while the backend callback retains only its completion cell. Therefore current private Session retention is not proved through actual completion after caller/context abandonment. The proposed external accepted-work scope and caller drain closure are still being designed. Do not publish this provider as complete until that finding is corrected and qualified. A second finding, public `GrowBuffer::upload<T: bytemuck::Pod>`, has a genuine independent AST RED and a narrow Source correction to private visibility; the successor pure qualification below is green.

## Concrete Owners

`ComponentReferenceTable` is a real canonical owner with its original ordered backing, Value conversions, canonical JSON tree implementation, controlled retirement and native allocation laws. Mesh-engine reexports that exact concrete type; scene mesh and BREP construct it directly. The canonical JSON implementation moved with the type to obey Rust orphan rules. Its direct `pack-json` dependency is itself a pure Value consumer, not the Pack/Replication aggregate.

The UI owners contain actual bodies: neutral descriptors/device contract; opaque WGPU device, resources, conversion, commands and platform ingress; exact prepared shaders; real pipeline/atlas/target factory; original mesh loan/provenance and source return; bounded raster admission/staging/retirement; actual draw encoding; and upper frame presentation/readback/Canvas orchestration. The former300kB pipeline unit was removed. No owner mounts implementation outside its own Source root. Original shared JSON schema/corpus inputs remain data inputs.

The maximum complete per-owner Source closure below includes all declared owner features and tests for each of native macOS/Linux/Windows and wasm browser/WASI cfgs. Values are maxima across those five actual canonical scanner calls. The ordinary global limits remain262144 bytes/32 files/4000 active lines; every listed owner declares exactly those limits.

| Actual Package | Bytes | Files | Active Lines |
|---|---:|---:|---:|
| semio-framework-ui-wgpu-render | 130356 | 11 | 1975 |
| semio-framework-ui-render-prepared-shaders | 56422 | 2 | 1152 |
| semio-framework-ui-wgpu-device | 43309 | 6 | 345 |
| semio-framework-ui-wgpu-pipelines | 94731 | 2 | 1394 |
| semio-framework-ui-wgpu-draw-execution | 131357 | 4 | 2214 |
| semio-framework-ui-wgpu-mesh | 31338 | 3 | 593 |
| semio-framework-ui-wgpu-raster | 182067 | 9 | 2929 |
| semio-framework-ui-render-device | 17577 | 3 | 211 |
| semio-framework-mesh-component-references | 10516 | 4 | 119 |

The former upper canvas/testkit/native-law closure629030B/25files/10336lines is now129821B/11files/1971lines at the reported checkpoint. This is subdivision, not a measured build-speed improvement.

## Qualified Source Checkpoint

The unchanged finite120s authored Original/child and1MiB/8192-line transport produced terminal collected results:

- Prototype45352:16 pass/1375 expectations;1 separately routed census skipped;12.75s.
- Release64326:16 pass/1375 expectations;1 separately routed census skipped;12.84s.
- Strict prototype43118 and release50923: exit0.
- Full selected dependency Source census prototype23044 and release57540: exit0;1 test/536 expectations each; all495 rows written under ticket generated output.

The later public-API AST law failed on the actual public `Pod` bound (75501, retained `ui-wgpu-public-api-bound-red.log`), then `upload` became private. Successor prototype81074 and release28889 both completed with16 pass/2694 expectations,1 separately routed census skip (16.15s/15.91s); strict prototype90062 and release33816 completed exit0. All four handles were collected. This successor qualifies the private upload correction and stronger independent public-API AST checks, not the proposed accepted-work scope.

The pure corpus covers Canvas raster clipping/DPR, Sharp readback bytes and padding, SQLite root ownership/failure accounting, exact shader fingerprints against independent Rust AST, private real-session Source, nineteen encoded attachments and hostile mutations, full disjoint owner closures, and graphlib SCC comparison. Former native Source-string inspection of whole production files moved to TS/NodeFS and independent AST. Actual native pixel, traversal, mesh/raster custody, readback, and real-device ownership laws remain native; shader anchor behavior moved into the real shader owner.

## Dependency Census And Provenance

The actual first-party declared path graph is98 owners, acyclic by independent graphlib. It contains no `semio-framework-ui` aggregate and no Replication. The graph includes selected upper root dev dependencies, every declared target/optional edge, and all declared features per owner for Source scanning; it is explicitly a conservative manifest union, not a Cargo feature-resolution claim. Each new owner is tested as an actual test root. Existing transitive owners are library/build roots, not their dev tests.

Only the existing `semio-framework-ui-contract` remains known oversized. All-feature/typegen union:756785B/37files/16135lines. Explicit no-feature/no-typegen comparison:756785B/37files/14919lines, identical across all five cfgs. Thus omitting typegen does not cure the ordinary Source refusal. Shortest declared chain is upper render→prepared→UI contract; a separate unconditional theme→UI contract path also exists. The selected native route requests upper `canvas-raster,testkit`; exact Cargo resolution remains unexecuted.

Value derive has an explicit canonical scanner refusal on all five cfgs: unsupported generated compile-token output requires a generated-input observer. This is a separate known gap, not a successful Source admission. No exception, Source cap increase or synthetic generated observer was added.

## API Audit

- Raw WGPU resources and external conversion functions occur only inside the concrete provider. Raw fields are private. No raw Device/Queue/Surface/Texture/View API, alias, reexport, generic bound or public `From` conversion is intentionally exported. Platform NativeWindow/TransferredCanvas ingress reexports retain the explicitly approved exception.
- Actual resource wrappers retain private issuing Session Arc; public work descriptors are data. Binding groups, pipeline shader/layout resources, render attachments/pass resources, copies, queues and Canvas interop check actual issuer identity before backend work.
- Borrowed buffer slices retain Rust buffer borrows; native render passes retain the actual encoder borrow. Submission status is callback-driven, never elapsed-time driven. **External accepted-operation retention/drain remains the review defect above.**
- Neutral borrowed resource descriptions are generic over first-party resource types. Canonical shader-contract VertexFormat/VertexStepMode/CompareFunction/PrimitiveTopology are reused. Full `PipelineSpec` consolidation is not claimed: the extended prepared pipelines retain exact finite descriptor states and exact WGSL, including affine/ellipse/section/emissive semantics absent in older generic shader strings.
- Upper logical Session Arc checks, exact Mesh3dLease/source-return provenance and raster Returned/Retained handback remain nominal. A Returned cancellation reinstalls the exact admission; Retained leaves it in the actual close cursor. No integer or serialized marker grants resources.
- New manifests, Framework/root workspace dependencies, owner catalog and editor/seed routes are registered. Existing optimizer profile rows were preserved. New editor rows carry both cache-disabling env fields. No SDK/Bazel/lease/cache-policy changes were made by this lane.

## Remaining Runtime Gaps

No WGPU29 provider typing/lifetime compilation, actual backend resource law, new owner native test, browser-target check, device completion/drop law or product/Infinite gate has executed after this subdivision. Correcting the accepted-work scope and all actual cancellation/drop/close paths precedes another Source freeze. UI contract subdivision and the honest Value-derive generated-input observer refusal also precede another native attempt. Any later manifest change needs Root's managed lock refresh; this lane must not run Cargo/store/compiler under the hold.
