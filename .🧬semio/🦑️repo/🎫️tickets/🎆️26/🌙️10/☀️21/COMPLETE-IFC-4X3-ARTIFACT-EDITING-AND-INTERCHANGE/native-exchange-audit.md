# Native IFC4x3 Geometry Exchange Independent Audit

Read-only production audit of native-exchange-implementation.md, native-exchange-plan.md, native_exchange.rs, native_gltf.rs, new Rust/native-oracle tests, IFC root declaration/assembly and definition claims, shared geometry reader and native glTF codec/framework IO contracts. No source/configuration changes, compiler-ingress changes, worktrees or git modifications. Direct reads/Bun fallback follows the documented cached repo CLI limitation. Authentic native compiler execution remains unavailable and was not bypassed.

Production paths below use IFC IO root `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🚪️io` unless stated otherwise.

## Findings

### P2 — Valid small-scale transforms are refused as singular

`native_gltf.rs:246` refuses `determinant.abs()<1e-15`. Determinant magnitude depends on coordinate scale, so this is not a nonsingularity test. Take the valid Khronos Box source and set its root scale to `[1e-6,1e-6,1e-6]`. Matrix and transformed positions are finite and representable; determinant is approximately1e-18. The static control flow necessarily returns exchange.transform before accessor processing. Expected supported static geometry with dimensions1micrometre, or an explicit documented minimum-scale boundary; actual implementation mislabels it singular. Use a scale-aware degeneracy test or admit nonzero finite determinants, consistent with the admitted affine subset. This is a static dataflow finding, NOT an executed native result.

Retained `native-exchange-audit-repro.ts` generates the concrete tiny-scale.glb input under ticket/🗑️generated/native-audit-inputs for future authentic native tests. It preserves all other official Box source data. No source fixture was modified.

### P2 — Strict GLB admission accepts malformed padding

`native_exchange.rs:81–101`, especially line98, verifies only padding length, then relies on shared native glTF `🚪️io/📥️import/🦀️.rs:80`, which explicitly trims NUL bytes at the JSON chunk tail. Thus a JSON chunk padded with NUL rather than required ASCII spaces passes the wrapper framing checks and is stripped before JSON parsing. Similarly, trailing BIN padding bytes are not validated as zero: shared decode_glb strips them, and line98 only checks the number discarded. Expected atomic malformed-frame refusal; actual static dataflow accepts these frames. Validate actual raw padding before delegating; do not infer padding validity from the already-trimmed snapshot.

The retained repro generator creates `nul-json-padding.glb` with at least one invalid NUL padding byte, exact header/chunk lengths, and unchanged Box BIN payload. This is prepared input plus source reasoning; native acceptance has not been executed.

## API, Registration and Fidelity Review

- ArtifactComposer payload dispatch and snapshot types match the actual repository glTF parse/decode signatures; parse_gltf_document returns GltfSnapshot, decode_accessor returns components as Vec<f64>, buffer/index fields are native usize. No obvious additional Rust type mismatch was established statically. Rustfmt evidence is not type-check evidence.
- Root declaration includes both composers; the artifact definition declares the exact IFC4x3→GLTF foreign output route and dependency. Controlled entries are all explicitly IoFidelity::Lossy, and successful IoOutcome returns the projection diagnostics. Pure composition confidence is identification confidence, not fidelity.
- Shared IFC snapshot representation is used with ADD2 header/graph validation; strict IFC4 STEP parser ingress is not invoked here. Existing IFC4 geometry reader intentionally supplies the shared source-unit projector; admitted face-set attributes match this limited subset. No full IFC4x3 infrastructure evaluator is inferred from that reuse.
- Export scales all evaluated source-unit world positions to metres and rotates `(x,y,z)`→`(x,z,-y)`. Import matrix multiplication, quaternion signs, reverse axis conversion and determinant-based winding swap are internally consistent. Accessor index range/type/alignment/stride/buffer-view extent checks are concrete. Unsupported source semantics produce errors before publishing conversion output; successful geometry conversion reports broad semantic omissions.
- IO registration during assembly happens before the later plugin transaction. This is already documented as a framework limitation; no claim that these entries have transactional rollback was validated here.
- Controlled receiving and final native encoding use the caller's original native controls. RawGLB final bytes use original copy_bytes. Projection/domain vectors and synchronous GLB/JSON/accessor codec temporaries remain outside a resumable funded cursor, as documented. Callback checkpoints do not prove fully owned-job cancellation/allocation/retirement conformance.
- Pure composer native-pack branches at native_exchange.rs:53/74 call decode_pack before projection's entity/node/buffer limits, unlike explicit64MiB rawGLB/JSON preadmission. Therefore the stated fixed projection bounds should not be read as a verified64MiB source-pack allocation bound. No new native decoder bound was established by this audit.

## Tests and Evidence Limits

Executed `bun test <IFC IO>/🧪️tests/native-oracle.test.ts`: **0passed,2skipped,0failed** because native-output environment/artifacts are absent. This correctly avoids substituting TS outputs for native proof.

Executed retained input generator with Bun; it writes only generated native audit inputs and reports determinant≈1e-18. It does not call Rust APIs, emulate native success, or assert native output validity. New Rust tests describe expected host/composer behavior but cannot yet establish it while authentic ingress refuses compilation. Existing TS/IfcOpenShell/three.js geometry evidence remains evidence for those implementations and source fixture expectations only.

No native Rust type check, route execution, native emitted pack/GLB, or independent native-output oracle result exists from this audit. Compilation through authentic repository ingress and real native differential checks remain required before marking the native routes operationally verified.
