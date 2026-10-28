# glTF Source Ownership Independent Audit

## Confirmed P2: caller source remains live during IFC import

The public actual `gltfToIfc4x3` adapter in io/gltf.ts:162 captures the original control, but aliases snapshot.document immediately (164) and keeps source nodes and buffers live through awaited semantic/accessor checkpoints (170, 178–179). Accessor decoding reads snapshot.buffers directly after progress callbacks. There is no completed private source capture boundary comparable to the BIM/FEM external-source path. A caller progress callback can therefore replace subsequent source metadata or bytes and change an accepted output.

The ticket input gltf-source-mutation-probe.ts uses the actual readIfcGltfGlb parser and checked-in official Khronos Box GLB, not a hand-built surrogate. box.provenance.json pins Khronos commit 5bad5aaa0bbb5d0f9cdc934e626f27d0df1e79b8 and SHA256 ed52f7192b8311d700ac0ce80644e3852cd01537e4d62241b9acba023da3d54e.

Two deterministic actual endpoint results:

- At the first gltf-import callback, rename caller nodes. Output proxy Name changes from Mesh to CALLER-MUTATED-AFTER-IMPORT-START; fidelity remains geometry.
- At gltf-accessor completed=1 with batchSize=1, replace the last POSITION vertex x float at buffer byte offset 564 with 123.25. Output maximum changes from [0.5,0.5,0.5] to [123.25,0.5,0.5]; fidelity remains geometry.

Evidence: generated/gltf-source-mutation-probe.json and generated/gltf-source-byte-mutation-probe.json. Independent Three.GLTFLoader parses the untouched physical Box GLB and yields bounds approximately [-0.5,+0.5] on all axes (minor rotation floating error around 1e-16); generated/gltf-source-independent-three.json records its expected IFC-oriented maximum beside the actual baseline and mutated results. The mutation discrepancy is large and unrelated to float tolerance. The probe was executed with the available Bun runtime; Node's default TS loader cannot load this repository's JSON imports unchanged.

The expected ownership regression should permit caller mutation after completed capture, including every semantic/accessor checkpoint, while the emitted result continues to match the admitted private source and independent Three geometry. Caller stability remains required during cooperative capture itself, as documented by captureIfcExternalSource; no atomic-start snapshot guarantee is proposed.

## Shared capture constraints

The actual parser-owned snapshot buffer is Array<number>, confirmed by the probe; this public schema representation does not require a typed-array binary hook. Its document uses plain arrays/objects, exact BigInt ordinals and bit-word objects. A schema-owned binary hook is required only if a different real input boundary accepts typed buffers; this audit does not propose widening plainDataOnly or runtime dependency types.

The current IfcIoControl has maxEntities/maxValues/maxDepth rather than a separate maxBytes. GLB physical IO already uses maxValues as a byte ceiling, while accessor import checks count*width per accessor. External own-data copying additionally charges every numeric byte value/property and container depth, so adopting it needs explicit tested limit semantics; it must not silently reinterpret only decoded vertex count as total owned source size. Domain-record counting must define which glTF document records/buffer records count against maxEntities. Getter/descriptors/prototype/symbol/function rejection and safe __proto__ handling should stay behind the shared copier policy. Initial cancellation and completed capture/index refusal should precede private IFC output graph birth.

## Separate materialization frontier

After accessor completion, gltf.ts:178–185 performs eager full position transformation, index extraction/default creation, index validation, winding reversal and IFC point/triangle value construction without per-batch checkpoints. Export likewise serializes all index words at line 30, concatenates full chunks at 39 and creates Array.from(bytes) at 41 outside byte-batch checkpoints. These are source-visible work/copy bursts distinct from the confirmed source-alias defect. No latency threshold or native receipt failure was measured here; they remain a separate bounded-materialization follow-up. Fixing capture does not by itself establish those loops as cooperative.

This report establishes TypeScript adapter behavior only. It does not duplicate native geometry-only Pack ownership review, prove native Rust execution, or claim a new normative IFC geometry mapping defect. Root owns production correction and neutral/registered regression tests.

## Source-capture correction independently rechecked

Current follow-up: reviewed new gltf-source.ts and both loaded-source endpoint rebindings plus raw GLB byte capture. Reran the original gltf-source-mutation-probe.ts, including its actual Three.GLTFLoader comparison. The generated probe JSON files now contain corrected after-results: caller node rename leaves output Name=Mesh, and caller POSITION byte mutation at accessor completed=1 leaves output maximum=[0.5,0.5,0.5], matching the independent untouched physical GLB within the existing 1e-16 transform tolerance. Earlier values in the finding above record the observed pre-fix counterexample; the regenerated files should not be mistaken for retained red outputs.

The loaded path captures the entire plain own-data snapshot through the shared copier, validates fixed document families/asset/mesh primitive record containers, indexes the finite record budget, validates every byte including unused source bytes in bounded batches, then rebinds the private artifact before interpretation or writer encoding. Byte validation and record indexing use the captured original control. No third-party runtime dependency or typed-array allowance was added to the plain-data copier. The current record policy counts asset/family objects and mesh primitives, while numeric byte payload is charged through the central value/property budget; it does not claim that buffer arrays are identity-bearing domain records.

The raw path uses built-in TypedArray slot getters for exact kind, byte length, underlying buffer and offset; it bypasses caller shadow getters and accepts genuine Uint8Array/Buffer views while rejecting proxy, signed-byte, wider and DataView inputs. A new fixed-extent view borrows those admitted slots for cooperative copying; no caller property lookup is used in the byte loop. Extent loss/detachment yields undefined bytes and explicit refusal before parse, or constructor refusal for an already detached source. The original byte limit is checked before owned allocation, cancellation is checked before that allocation and at each copy boundary, and only the completed owned array reaches GLB framing. Source stability remains required while copying; neither SharedArrayBuffer concurrency nor cooperative input mutation becomes an atomic-start snapshot guarantee.

Reviewed neutral/actual tests cover loaded semantic mutation, accessor noninvocation, capture/index/byte cancellation with original control rebinding, limits, raw private parsing, shadow slot/tag accessors, exact subarray extent, proxy/brand refusal and invalid unused bytes. Parent owns execution/strict-check evidence for that suite. This auditor reran only the existing independent endpoint/Three probe, without duplicating a full sweep. No new confirmed defect in this source-capture correction. The separate post-accessor/export materialization frontier above remains open, and native execution limits are unchanged.
