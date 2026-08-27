# Rooted Borrowed Canonical Maps Handoff

## Status And Scope

The reusable Store map traversal passed its coordinator-owned native gate: all eleven sealer/map laws passed in the broader canonical_ run (18 passed total). The log is 🧪coordinator-store-canonical-native-r4-2026-08-27.txt. No full app interactivity, Miri result, or arbitrary trait-implementation proof is claimed here.

Seven implementation/test-source files comprise this extension: Store parent exports; canonical-edit parent; new borrowed Rust component; new borrowed native test component; strict map schema; language-neutral map fixture; and existing root script. No runtime dependency or executable script was added.

## Public API And Concrete Flow Integration

ArtifactCanonicalJson now requires Sync and optionally supplies canonical_json_borrowed_root returning Result<Option<ArtifactCanonicalJsonValue>, String>. Values are Scalar, Source, Array, or Object. Array::new accepts a Send iterator of borrowed typed values; Object::new accepts a Send iterator of borrowed key/value pairs. Both wrappers implement the standard Iterator trait for safe external typed-oracle traversal. No public API accepts encoded JSON, a digest, raw pointers, or an unsafe lifetime promise.

The Store supplies the complete Edit/MutationMeta wrapper in exact serde order. Concrete app visitors use native slice/BTreeMap/Dictionary iterators, retaining iterator positions without ordinal nth or repeated range/key comparison. Indexed fixed-field visitors remain an alternative for Config values. Flow's owner accepted and is integrating the borrowed API for recursive Widget/Tree/Dictionary values; its app adoption and native serde parity are separate gates.

## Private Lifetime And Authority Audit

The sole lifetime extension is private to ArtifactCanonicalEditEncoder::bind. It projects references obtained from the exact sealer-owned boxed immutable Edit, then stores the exact allocation address. Every subsequent chunk verifies the same address before dereferencing retained frames. The edit is never exposed mutably by the public sealer, and no projected frame or reference escapes its private encoder.

The encoder field precedes edit/post ownership in the sealer declaration. Thus explicit close and Rust field unwinding destroy every retained frame before the root. A completed encoder has no borrowed frames before the edit moves into a prepared candidate. reset rejects nonempty frames. Depth-limit/error paths preserve remaining frames for close; rejected child frame values drop while the root remains alive.

No unsafe Send implementation is used. Source requires Sync; iterator objects require Send; the existing sealer advance requires Send mutation and Send+Sync snapshot. Native worker-transfer tests exercise moving live iterator state and replaying on another worker. Sync does not by itself prove semantic determinism of interior-mutable/custom implementations: concrete visitor review and serde parity remain required.

Private publication authority remains unchanged: exact Store authority Arc, boxed edit address, post Arc address, and Store-computed digest are checked before commit. No full serialization was reintroduced into publication validation.

## Exact Work Bounds

Each advance emits/copies at most min(grant bytes,256) actual bytes. Keys and values are borrowed and emitted one UTF-8 byte at a time; escape expansion retains its offset. A 4,407-byte Unicode/quote/backslash/newline key therefore spans many one-byte or seven-byte turns rather than being copied or compared whole.

The frame vector reserves capacity64 once and initializes/reuses slots incrementally. Each next-byte operation has at most64×8 zero-output structural transitions. Native iterator advancement does not rescan prior keys or compare long keys. The combined borrowed/indexed depth is64: an indexed fallback receives64 minus its borrowed parent count. The adversarial review found and fixed the earlier independent-budget loophole.

A positive close grant retires at most one frame before domain edit/root/string retirement. Borrowed frames own iterator state, not the map payload. Completed bytes count both canonical passes, exact digest framing, and identity copies; no accumulated credits authorize a later monolith. Fixed scalar formatting remains a maximum64-byte serde buffer. Fixed frame/metadata transitions are bounded control work, not falsely reported as payload bytes.

Custom iterator construction/next/drop may hide arbitrary work despite satisfying Rust's type system. That is explicitly not certified by the API. Accepted concrete visitors must use reviewed bounded native iterator adapters and avoid owned recursive payloads in iterator state. BTreeMap traversal has native structural node transitions rather than a hard instruction-count guarantee.

## Checkpoint And Cancellation Coverage

Portable checkpoints contain operation/generation/base revision, authority fingerprint, phase/byte counts, and independently recomputed prefix digest, never native pointers or iterator/hash state. Fresh-owner restoration reconstructs iterators and replays actual bytes to the checkpoint. A checkpoint authenticates the observed prefix and authority, not an unobserved suffix of a different supplied payload; final sealing always binds and hashes the actual fresh immutable edit. A live encoder cannot rebind its allocation even if metadata is equal.

Cancellation before the first poll creates no iterator. Phase-by-phase cancellation, cancellation in the middle of the long Unicode key, completed-iterator cleanup, depth overflow, and explicit live-root rebind rejection all lead through frame-first retirement. Destructor counters assert no root destruction while a borrowed iterator exists and exactly one final tracked-root destruction.

## Tests And Evidence

The schema-first fixture contains nested sorted maps, an empty map, arrays, control characters, the 4,407-byte key, expected canonical JSON/digest, grants0/1/7/256/4096, and named hostile lifetime/checkpoint scenarios. Existing Ajv validates the schema strictly; JSON.stringify and Node crypto supply independent byte/digest oracles.

Map verification adds15 checks: four positive-grant byte-oracle cases, four strict schema hostiles, six concrete source-hostile substitutions, and one Node digest oracle. Together with the original21 checks, the exported sealer self-test has36. Source hostiles cover missing Sync, root retirement before frames, removed allocation binding, nth scanning, public lifetime capability, and independent indexed-depth allowance. These are source regressions, not compiler-negative/Miri proofs.

Four new native tests cover:

1. Exact second-pass canonical bytes versus serde_json and fixture digest under1/7/256/4096, with zero-grant no-poll/no-state-change assertions.
2. Cancellation at all seven phases plus mid-long-key cancellation; one-item/one-byte retirement and root/iterator destructor ordering.
3. Serialized checkpoint replay on a fresh allocation and another worker, plus moving a partially live retained owner across workers.
4. Rebound Box rejection before cached reference use, recursive-map depth overflow, and reduced indexed-depth budget rejection.

The original seven native tests remain required for stale operation/generation/revision/authority, forged prefix and prepared token, rebound post, maximum framing overhead,16/64KiB domains, all metadata origins, and Unicode authority retirement.

Canonical Nx verification before the final depth-budget guard passed690 checks with33 exact factory owners,254 custom rows,31 generic rows. The final canonical run failed before tests on an unrelated concurrent taxonomy error: generatorContracts["plugin-registry"].checkTarget "@semio-tech/plugin-registry:check-generated" is absent from its owner project. Its stack is discovery/🟦️component.ts:796 through .storybook/scopes.ts:199/226. No unrelated taxonomy was edited.

Supplemental isolated execution of the actual exported sealer helper passed all36 checks after the final depth guard. This does not substitute for the blocked canonical route. Targeted git diff --check passed with exit0. The coordinator subsequently reported all11 native sealer/map tests passed in its18-test broader canonical_ run.

## Remaining Gates

Flow must finish concrete recursive visitor integration and retained candidate/inverse/root preparation. Other apps still require their own bounded preparation/retirement adoption; this shared sealer does not fix existing whole-root clones or fixture serialization. Live Generator slider commands are the next separate packet, not included in this closure. No runtime browser result is claimed for Store maps.
