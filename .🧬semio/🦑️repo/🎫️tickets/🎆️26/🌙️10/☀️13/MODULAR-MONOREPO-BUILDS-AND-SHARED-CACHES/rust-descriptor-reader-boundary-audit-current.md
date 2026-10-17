# Rust Descriptor Reader Boundary Exploration

Read-only Source exploration for the next Root cut; publication and actual Native acceptance remain HOLD. No production edits, new authority/registrar, physical image, compiler or fake Native positive. This is not the final planning-reader freeze audit and does not expand its static inventory.

## P1 — Descriptor Input Precedes Genuine Owner Admission

Current U/📜️script.ts resolves caller descriptor path, lstatSyncs it, unbounded readFileSyncs and JSON.parses it before compiler phase creation, storage/image admission or closed collection validation. The initial pathname file check neither caps allocation nor binds later bytes to an opened descriptor. The selected producer publishes 4MiB-bounded descriptor bodies, but the consumer accepts arbitrary provided paths and the producer checks its encoding only after stringify. Its current raw consumer cannot inherit publication guarantees from a caller pathname.

Mode is available before descriptor DATA: this.invocation is the genuine routed Original. Authenticate/check it, then readNativeCallerAuthorityV1(this.invocation.capabilities).original.buildMode and verify its repository root. Environment SEMIO_BUILD_MODE and the descriptor unit mode are comparisons against that Original, not sources of admission authority. scriptCompilerAdmissionControlV1 can be created using this Original and a bounded descriptor-operation name without first trusting a unit ID from descriptor bytes.

The actual existing compiler owner is withRepositoryCompilerStoreV1(workspace, mode, control, environment, body), whose callback has a genuine retained same Rust image/store and narrower accepted control. It opens tooling/toolchain and the Rust partition, recovers/reserves actual compiler workspace, registers a private WeakMap owner, and holds withCompilerWorkspaceLifetimeV1 through callback settlement. requireRepositoryCompilerStoreV1 checks exact store identity, workspace/invocation/control refinement and actual reservation. Do not fabricate that object or replace the retained parent by an equivalent pathname.

No withRepositoryCompilerArtifactStoreV1 definition or reference was present in the inspected current framework tree (including an explicit ignored-file search). Current related exports are withRepositoryCompilerArtifactCacheV1 and requireRepositoryCompilerStoreV1. The cache factory adds artifact-cache writer operation/custody; it is not a descriptor-reader API and should not be invoked merely to mint read authority. If another lane adds a genuine read-only artifact-store owner later, it must be re-inspected rather than assumed present here.

## Exact Namespaces And Lower Reader Limits

Compiler layout fixes captureDirectory to imageRoot/cargo/{Original mode}/captures. Selected producer publishes captureDirectory/units/{64 lowercase hex SHA256}/descriptor.json under an exclusive rust-descriptor:<file> lease; descriptor bytes hash to that directory key. Consumer should admit only this exact namespace under the actual retained store and take the matching shared descriptor lease before opening/through parse. requireNativeStoragePathV1 alone is path confinement, not retained custody or content identity. Cross-mode, arbitrary image-root and arbitrary repository paths must refuse.

Existing ordinary withExactSourceReadBatchV1 authenticates issued phase and Source root, checks physical ancestors/no-follow single-link leaf/opened descriptor/EOF and settles accepted work. It does not itself authenticate compiler image ownership. Its use inside the genuine live store callback must preserve that owner around all reads and validate namespace explicitly. Managed Graph batch additionally rechecks its actual Graph store but admits only build-input-closures and build-lock-closures plus 64-hex keys; it is the wrong namespace/owner for Rust captures and must not be stretched to these descriptors.

readExactSourceJsonV1 currently has no reducing maximum argument. Its JSON role admits up to 134217728 bytes, with 268435456 aggregate metadata bytes, 262144 nodes, depth 64, and 65536-byte chunks. Source role is 536870912 bytes per file. Therefore an unchanged JSON call is not a 4MiB descriptor boundary. Minimal lower change is the same numeric reducing-cap path already used by readExactSourceFileV1, applied to JSON and checked for fresh and cached facts before retention/parse. Keep JSON role accounting/structure bounds; streaming a Source hash then separately raw-reading JSON would lose exact opened-byte binding. Use <=4194304 for descriptor, then validate exact closed version/units shape, readRustUnitV1 records and plans and rederive directory/content hash identity before selected-unit use. This preserves the descriptor shape; compact DAG encoding remains future work.

## P1 — Dependency Receipts Need Their Own Held Reader

U currently reads each predecessor receipt by raw JSON.parse(readFileSync(...)) before registering its withReader shared lease. Those receipts are unbounded and may race replacement/retirement before parent artifact use. Existing receipt schema closes fields and validates identity against plan but gives no byte or array-count maximum. U's reusable own receipt path and captureRustUnitReceiptV1 also raw-read receipt JSON; charge files have only pathname size checks. Bound each role before allocation and authenticate same-image namespace. Parent receipt capture and artifact verification should happen under the genuine shared rust-unit:<parent directory> custody, retained for actual consumption, rather than attaching a future reader after an unprotected observation. Preserve bindRustUnitDependencyArtifactsV1 and actual receipt/artifact checks; a semantic digest is DATA rather than reader authority.

## Minimal Source-first Ordering

1. Schema-first descriptor collection/byte and receipt-role caps plus neutral exact-size/one-over, malformed/depth/node, reducing fresh/cached and duplicate/missing/cycle cases; preserve current descriptor shape and all invalidation laws.
2. Extend the actual lower JSON reader's numeric reducing cap, with independently validated genuine ordinary Source tests and failure/cancellation/settlement laws. These positives prove lower DATA capture only.
3. Authenticate Original and derive mode/control before descriptor reads. Enter existing genuine store owner (or re-review a later genuine artifact-read owner), validate exact captures namespace, shared descriptor custody and hash, then bounded same-opened-byte JSON capture/closed shape/plan selection. Keep accepted descendants inside same-image lifetime.
4. Move predecessor and reusable receipt capture into actual shared unit custody with role caps and receipt/artifact validation; retain write/publication/retirement ordering with existing output exclusion. Coordinate U state edits with funding lane.
5. Run Source/strict routes and retain separate negative clone/cancel/retired/escaping namespace tests. Actual Native and physical cross-platform qualification remain HOLD; do not invent Native positives to exercise the Source logic.

## Observed Identities

These are local observations, not a frozen complete executing closure. Shared files may drift while other lanes work.

|Path|Bytes|SHA256|
|---|---:|---|
|🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📦️units/📜️script.ts|11779|feb09094b9623605860cd03f0012e5c881d092fd73921abdf2881f16418ebc67|
|🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/🏗️compiler/🟦️.ts|13109|f1b7f111a123576ff68849fe7c95330869313752e8405116e130699b4f860a7e|
|🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/🏗️compiler/📏️capacity/🟦️.ts|8519|7741359351364a92a1437eff25b85dadcf1b5222fe639784dc0318ef0b491ac0|
|🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/🏗️compiler/📍️layout/🟦️.ts|1645|7f4435596d2083ab32fb78fa8049b1979f822b3d999b1a1a5ecb940f2e207822|
|🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📦️units/📤️receipt/🟦️.ts|6635|9a61a6923bce9bff1e3bb0be84dec0f12c6b13791d16a22c78bf06514d603999|
|🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📦️units/📤️receipt/🧬️schema/🔣️.json|1876|05c2291a33e10347b3e5e21d6141b01bd61a1fed96338b19e8ccdea88bef28a6|
|🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🧮️digest/📥️read/🟨️.mjs|14759|69f227276eb00730a52d288bd14bf263593d0c27462b0cf1a09208b31b6d3e7d|
|🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🧮️digest/📥️read/🧬️schema/🔣️.json|2627|3bb515be6be4314e43fd8026c9f8ccdcf97a33b2153a62970d00475203d7f54e|
|🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/📋️native-orchestration/📥️invocation/🟦️.ts|6905|0232b89f35b31e2968a30076238e3f4ce8601454f0f148149e994cfe58431b07|
