# Owned Snapshot Hash

`retained/🔢️hash/🟦️component.ts` now streams one exact captured owned-node index into the existing snapshot JSON/FNV-1a projection. There is no complete node array, snapshot stringify, whole UTF-8 buffer, recursive serializer or app-supplied digest. Insertion order comes from the persistent ordinal index. The snapshot field sequence is surface, revision, root, nodes and string-valued zero layoutEpoch; the existing empty-root wire projection remains zero.

The private JSON encoder uses explicit frames anchored to the currently captured node. Normalized native objects have at most 256 fields, so the dedicated key-metadata phase is at most 2,112 logical bytes; no unbounded generic JSON root is accepted. String code points and JSON escaping advance individually, including lone UTF-16 surrogates. Exact SurfaceDoc views emit their bytes as the canonical JSON numeric array, not as a view object's metadata. Each output allocation is 256 bytes; actual output is at most that size. Emission, hashing and frame accounting fit the unchanged one-item/4096-byte grant. Cancellation retires one frame before releasing its node/read/index owners. Error-after-prefix accounting preserves the produced prefix and charged work, then rejects without exposing a completed digest.

The strict neutral fixture includes insertion order with safe-53-bit IDs, an 8,193-byte SurfaceDoc, nested depth 80, numeric exponent/-zero forms, control/Unicode escaping, a lone surrogate in the surface identity and an own `__proto__` key. The test compares every emitted byte against Node JSON/Buffer for an Immer-produced snapshot, then compares the FNV digest, seven cancellation prefixes, zero-grant behavior and source retirement while hashing remains alive.

Canonical `@semio-tech/framework-renderer-react:test-long --args='--run -t OwnedHash'` history:

- R1: expected missing module during collection; no behavioral test ran.
- R2: 1 passed, 531 skipped, 532 total, five files; 17.83 seconds total and 1.29 seconds test time.
- R3: 1 passed, 531 skipped, 532 total, five files; 25.14 seconds total and 2.95 seconds test time. This includes the strengthened lone-surrogate and own-key fixture plus prefix-error accounting.
- Strict hash typecheck R1: exactly seven existing tutorial/local-interaction producer diagnostics; zero owned hash or test errors.

Complete logs: `🧪️renderer-owned-hash-{red-r1,r2,r3,typecheck-r1}-2026-08-27.txt`. The previous full-suite checkpoint is 531 tests before this hash test, not a full 532-test claim. This remains a private preparation cursor, not mounted live publication or a timing certificate.

## Next Consumer Boundary

The existing React interpreter already reads flat node IDs per component. The live PluginRuntime still expands a recursive BuiltNode before ShellHelpers flattens it again. That round trip must be removed in favor of an owned flat surface read. Graph validation permits shared DAG descendants, so recursively expanding every path can duplicate the same source node exponentially. The new transport/read path must preserve flat shared ownership and capture old byte roots for React-held reads; it must not introduce a recursively-owned built-tree disposal cascade. Actual wire admission, bounded notifications, exact publication ACK and per-instance aggregate close remain required.
