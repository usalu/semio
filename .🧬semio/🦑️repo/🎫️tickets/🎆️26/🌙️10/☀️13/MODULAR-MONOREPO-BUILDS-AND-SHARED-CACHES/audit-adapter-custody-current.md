# Adapter Custody Source Audit

Read-only Source review on 2026-10-16. No tests, compiler, host, images, download, SDK, publication, retirement, modifying Git, or lifecycle actions ran. This report is the sole written artifact. The initial adapter-invocation-custody-current.md describes an older implementation; actual Source was used below.

## Actionable Finding

The scenario deadline is checked before accepted-child drainage, not after it. In Core process invocation Source, withRequestedIssuedResourceControlV1 calls remaining(child) immediately after its callback returns. settle then drains accepted operations before retiring the child, but does not recheck the child's deadline or cancellation after drainage. Host runAdapter lines 107–108 awaits that wrapper and immediately passes its outcome to resultFor. Consequently, a handler can register a child that outlives the scenario budget, return promptly, and have its outcome materialized as passed when that child finally resolves, provided that child does not itself recheck its control. The parent Original can still have time remaining. This is a Source-derived gap; no runtime reproduction was authorized for this audit.

The appropriate owner-level repair is to validate the issued control after accepted children settle and before retiring it; checking only the broader host parent would lose the scenario restriction. Add a bounded regression with an accepted detached child crossing the narrowed deadline without self-checking, and require refusal before successful result hashing. Cancellation during drain has the same check-placement concern. Existing positive drainage coverage explicitly checks its control inside the child and therefore does not exercise this negative case.

## Confirmed Source Properties

- AdapterContext requires both invocation and phaseControl. makeAdapterContext authenticates their exact private issuance before directory effects, returns a frozen object retaining the exact objects, and checks each effectful accessor against the same pair. Copied and expired construction are covered in authored tests.
- runAdapter passes a genuinely narrowed scenario control to the context and wraps callback execution in canonical accepted-operation tracking. The canonical owner drains accepted children, including a rejected child whose rejection value is undefined: settle tracks failure with a separate boolean, and failures uses array length rather than truthiness.
- main and runAdapter require an explicit issued pair. Host entry receives process.env's actual parent envelope and issues its explicit host phase. It does not create a process envelope or reconstruct control objects.
- executeOne requires the pair and forwards the actual invocation envelope through scriptInvocationEnvironment. Preparation and scenario child processes use captureOwnedCommandResultV1 under requested issued controls, preserving the supplied parent control's restrictions on the parent side.
- runPhases is an explicit invocation-only Script-root entry creating its authored zero-request phase. Its four actual Script callers pass this.invocation. Its executeOne calls await and carry the issued pair, including subject-raw oracle execution.
- Actual constructor callers found are the host and subject-raw fixture. The vector-only Core fixture deliberately throws when unused authority is accessed. No new Original/exporter or broad ambient context lookup was found in these changed owners.
- The neutral schema declares the two required ports, explicit-issued construction, parent-envelope child birth and accepted-operation join. Source tests check actual protocol ports with the TypeScript AST and schema contract with the independent validator. These are reviewed test definitions, not an audit test-pass claim.

## Limits

Host resultFor/main output writes and coordinator planning/completion writes are broader existing materializers; they are not individually qualified by this adapter-context change. main has no fresh phase check after its dynamic import or before final JSONL mkdir/write; executeOne similarly has post-await read/completion effects. This audit does not establish whole-pipeline post-cancellation exclusion. Passing paths out of ctx.artifact cannot prevent arbitrary later direct filesystem writes by a handler; the demonstrated guard is on context methods and canonical accepted operations. Native descendant-tree settlement remains outside this review. No whole-repository strict-pass or runtime success is claimed.

## Reviewed SHA-256 Snapshot

Paths below are relative to the repository root. Concurrent edits can supersede this snapshot.

| Source | SHA-256 |
| --- | --- |
| Core test/adapter type, `🧰️framework/🔨️modules/🧪️test/🔌️adapter/🟦️.ts` | `52eaa365ee9eb424da1ac741c741ba39e55563510b93ea207f22d8835843c2b0` |
| Core adapter tests | `f3fb0f0503adefd7a87a81b6239020a3ab4408a4fb5446524ae4ec13feac658b` |
| Core test schema | `3bea63a6f8da0085c4972b9feaa6ec4cbf06e6d0a24d227a4961c1f7274b4294` |
| Repo test host | `f9a31b3e7a33b8e4e79733aecdc15f9b1ff2e2542436440059e40f400623b8d1` |
| Repo test execution/scenario | `cf56a7759359f03390597b78a28c8a2660f98a11c6cda1ea3a268cd09eeaee1e` |
| Repo test parity/orchestration | `7a6c97a9dcbeb1dbf05ef8bc376994e11b5515abadc3607643a099f7a539c24e` |
| Repo test context/coordinator owner | `809da9acd8725097747e5699370ae7b8e59037538f7cf9668c382af83bd598f4` |
