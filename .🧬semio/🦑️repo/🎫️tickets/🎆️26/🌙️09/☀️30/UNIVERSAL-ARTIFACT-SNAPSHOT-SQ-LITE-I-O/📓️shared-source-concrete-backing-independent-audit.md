# Shared Source Concrete Backing Independent Audit

Read-only source review on 2026-10-03. No runtime gates, Cargo, production edits, or Git mutations were performed. Existing GREEN reports are context, not evidence that these additional boundaries ran.

## Confirmed Mechanisms

In `🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🟦️.ts`, operation byte-array allocation funnels through `SqliteAllocationControl.allocateBytes` (40–43). Exact byte lengths are charged cumulatively before `new Uint8Array`; retired construction buffers are not refunded. Export varints, numeric bytes, UTF-8 bytes, joins, overflow pointers, pages, and final output use that allocator (113–159,355–381,420–531). Import page visitation backing and copied record payload use it (564,676). Subarray views do not create additional byte backing. BLOB field decoding retains views into the admitted copied payload (603), rather than retaining the caller's original file. Projection clones BLOB input through admission and chunked copying (artifact/🟦️.ts:172–196). TEXT references are immutable JS strings, not new encoded copies at projection insertion. Projection ordering uses in-place heap sort with checkpoints (artifact/🟦️.ts:76–115).

Every production `new Uint8Array` and `TextEncoder`/`encode` path in this subtree was searched. The remaining `encoder.encode` is the fixed 16-byte module magic (69), outside operation ownership; IEEE helpers retain one fixed 8-byte ArrayBuffer (ieee754/🟦️.ts:8). These are module constants, not growing artifact-owned buffers. No VM object/array overhead estimate is needed to explain this concrete-byte contract.

## Remaining Actionable Boundaries

### 1. Physical Unicode measurement and decoding remain synchronous

Physical export preflight invokes `sqliteValueByteLength` for each field (409), whose string path calls synchronous `textByteLength` (98–112,129). Export `utf8` repeats the same whole-string scan before allocation (114). Later encoding yields, but those scans do not. Import `Reader.fields` invokes `text` over the entire UTF-8 field (603); `text` calls synchronous `decoder.decode` (95–96), and import then measures the returned string synchronously again (737). Overflow page copying checkpoints do not bound the later text decode or measurement. Projection's controlled long-text measurement does not cover these physical paths.

Individually scoped neutral gate proposal: one long Unicode TEXT field with mixed BMP/non-BMP scalars and an independently measured `length(CAST(value AS BLOB))` from Bun SQLite. Require physical measurement and reconstruction progress inside that single field, cancel at an interior frontier, and assert no later decode/encode work. Existing encoder `encodeInto` cancellation gate starts after measurement and therefore does not prove this boundary. Chunked physical UTF-8 decoding must preserve split scalar handling and exact literal text.

### 2. Injected control can override tighter operation allocation limits

`create`, `exportSqliteDatabase`, and `importSqliteDatabase` accept an independently configured optional control (artifact/🟦️.ts:18; physical:420,712). An existing control's maximum and signal are captured at its constructor (32–34), while call options are not reconciled with that control before allocation. Passing a permissive existing control with call options `maxAllocationBytes:0` therefore routes concrete allocations through the permissive ledger. This is a source-derived API boundary, not runtime proof of the result. Semantic/file limits remain separately validated.

Individually scoped neutral gate proposal: reuse one nonempty control, call each physical/projection entry with a smaller remaining operation grant, and assert `ownershipLimit` before any newly requested byte backing. Also use differing signals and verify cancellation at the same boundary. Define one authoritative operation owner explicitly; do not silently let a second options object express an unenforced byte cap.

### 3. Actual artifact Source composition does not expose a shared operation owner

The shared primitives permit manually sharing the third control argument. Actual Workflow and Run Source projectors create `ArtifactSqliteProjection` with `(sql,options)` only (workflow snapshot SQLite:30; run snapshot SQLite:24). Note Source likewise creates its projection without an injected owner. Their public projector interfaces return a database, leaving physical export to a separate call; no first-party production TypeScript caller of the physical export/import functions was found outside package reexports/shared implementation. First-party tests compose separate default owners, for example Workflow snapshot tests:20–21 and Run snapshot tests:15–16. These examples demonstrate separate composition, not an end-to-end cumulative artifact operation contract. The shared primitive explicit-owner test proves its own manually wired composition only.

Individually scoped neutral gate proposal: choose one real domain Source artifact with owned BLOB projection, expose/thread the operation owner through that domain projector and physical writer, and select a grant that admits each stage alone but refuses their sum. Assert exact cumulative requested byte lengths and retained debt after cancellation/replacement. Keep this separate from the shared primitive gate and from zero-grant gates.

## Authority and Scope Limits

TEXT import allocates a runtime-owned decoded string in `TextDecoder.decode`, while the concrete byte ledger charges the temporary encoded payload. Projection TEXT retains an existing immutable string. These are different ownership cases. The present byte-array mechanism must not be described as measuring all JS heap storage; exact UTF-16/string/object backing is runtime-dependent, and guessed VM sizes would not strengthen this proof. If the acceptance contract includes those retained allocations, a separately specified runtime-supported accounting mechanism remains required. This review does not claim a new typed-array allocation bypass for those strings.

The allocator also exposes an async `stage` ledger settlement API (51–55), but no first-party production Source use was found. Sequential reused-control settlement retains debt; concurrent/reentrant stages should not be claimed safe without their own gate or explicit restriction. No assertion about concurrency runtime behavior was tested here.

Conclusion: concrete operation byte-array admission is materially present, including positive exact-grant shared tests in source. Bounded physical Unicode work, mixed-owner option enforcement, and real domain-to-physical cumulative composition remain unproven or structurally incomplete at the identified boundaries.
