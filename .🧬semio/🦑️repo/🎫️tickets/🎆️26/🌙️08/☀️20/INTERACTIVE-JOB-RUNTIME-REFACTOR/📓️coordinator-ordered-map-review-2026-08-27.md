# Retained Ordered Map Review

## Native Gate And Adoption Decision

The coordinator's expanded source rerun also passes: shared-owner fixture 1 / hostile rejections 2, ordered-set fixture 1 / hostile rejections 2, original map fixtures 3 / lookup cases 2 / hostile rejections 8, grants 1/64/4096, independent fast-json-stable-stringify oracle. Source checks explicitly claim no runtime result. Log: `🧪️coordinator-ordered-map-source-r2-2026-08-27.txt`.

Expanded native r2 now **passes 16 tests**, 0 failed, 192 filtered, 1.46 s runtime after 9.97 s compilation. The five additional laws cover shared/final root release, eight-worker last-owner racing, allocation-preserving shared upsert of a non-Clone payload, and two ordered-set wire/retirement laws. The coordinator reviewed the exact Arc::into_inner final-frontier transfer and fixed pointer-only shared input handoff. No recursive payload destruction occurs in those API entries. Both executors may use the new methods and set wrapper. Domain adoption remains separately unverified. Log: `🧪️coordinator-ordered-map-native-r2-2026-08-27.txt`.

Coordinator canonical native run **passed all 11 tests**, 0 failed, 192 filtered, 0.84 s runtime after 20.21 s compilation. Exact route: `@semio-tech/framework-replication-rs:test --args='--lib value::ordered -- --nocapture'`. The negative guard test intentionally catches four ownership violations; those panic messages are expected and the test passed. Log: `🧪️coordinator-ordered-map-native-2026-08-27.txt`.

With source/oracle and native gates now executed, the coordinator approved the Flow and Dictionary executors to adopt this primitive. Approval covers the primitive, not its unimplemented app integration. Domain-aware nested-value retirement and explicit cold boundaries remain required; a run-to-completion Dictionary Drop is not acceptable.

The coordinator read all 306 lines of the initial owned persistent AVL map. Its fixed-node path-copy rotations and bytewise update-key comparison address the long-key comparison problem in BTreeMap-backed interactive copy. This is source review, not native verification or adoption approval.

Before Dictionary or Flow adopts it, the executor must resolve these gaps:

1. Make last-owner lifecycle explicit. OrderedMap, UpdateCursor, and Retirement currently default-drop recursive roots; all abandoned, successful, and cancelled paths need terminal guards and exact-owner retirement.
2. Add bytewise retained lookup. Ordinary get/contains still compare a complete long key synchronously.
3. Bound and account for rank-based iteration depth with a fixed AVL height invariant and invariant tests.
4. Keep whole-loop convenience constructors and serializers explicitly cold-only, and retire displaced owners without hidden recursive cleanup.
5. Test key, node, entry, and payload aliases through completion/cancellation, including last-Arc payload transfer under item/byte grants.
6. Add a language-neutral fixture and an independent third-party oracle; std::collections::BTreeMap alone does not meet the requested oracle gate.

AVL remains an acceptable implementation choice if these guarantees hold. No interactive collection or all-app completion credit is granted at this stage.

## Revised Source Review

Coordinator rerun: `NX_DAEMON=false bun x nx run @semio-tech/framework-replication-rs:test-source --skip-nx-cache` **passed**, executing three fixtures, two lookup cases, and eight hostile rejections at grants 1/64/4096 with the fast-json-stable-stringify oracle. Exact console: `🧪️coordinator-ordered-map-source-2026-08-27.txt`. The gate itself explicitly reports `runtimeClaims=0`.

The coordinator read the revised 430-line implementation and all eleven authored Rust laws. The map, update cursor, lookup cursor, and retirement queue now use guarded ManuallyDrop ownership; cold conveniences explicitly drain displaced and malformed-input roots. Lookup uses the same byte-accounted comparator as updates. Rank iteration documents a fixed `2 * usize::BITS` metadata frontier enforced at node construction. Tests cover all rotation/successor paths against BTreeMap, sorted-height invariants, failed serde decoding, long-key lookup, cancellation, no payload destruction before transfer, guard violations, and cross-worker continuation. The executor reports the language-neutral source gate passed with fast-json-stable-stringify as its independent byte oracle. Native execution is still queued, and neither Dictionary nor Flow has adopted the map yet.
