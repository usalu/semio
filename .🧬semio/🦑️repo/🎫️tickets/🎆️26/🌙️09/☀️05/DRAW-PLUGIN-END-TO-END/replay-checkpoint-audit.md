# Replay Checkpoint Audit

Bounded source-only inspection on 2026-11-05. Read `native-preparation-observation-2026-11-05.md` first. No live product, lease, process or gate was queried or changed; no source or guard was edited and no tests were run. This audit establishes callgraph facts, not live timing or completion.

## Exact Source Anchors

Paths below are repository-relative, with line numbers from the inspected files.

- **S**: `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📦️dependencies/💾️storage/🟦️.ts`
- **C**: `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/🏪️store/🟦️.ts`
- **R**: `🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🟦️.ts`
- **L**: `🧰️framework/🔨️modules/🏃️process/🔒️leases/🟦️.ts`
- **D**: `🧰️framework/🔨️modules/📁️filesystem/🧾️observation/📁️descriptor/🟦️.ts`

## Checkpoint Cost And Guards

S:727 constructs the producer checkpoint as a synchronous call to `requireJavascriptDependencyStoreFrameV1`. S:477 checks the private live frame and workspace/store identity, invocation and control refinement, cancellation/deadline, issued Store ownership/read custody, the build EX lease, and an optional bindings EX lease. It does **not** invoke `store.recheck()`, Bun membership recheck or the async whole producer fence.

`requireResourceLeaseCustodyV1` at L:164 checks cancellation/deadline, private WeakMap issuance, held phase, mode, signal, resource and derived catalog path. L:45-49 computes that path with SHA-256 and materialized-path validation; it does not open SQLite, acquire a lease or replay a lease catalog. `requireMaterializedPathV1` at `🧰️framework/🔨️modules/🏃️process/🔒️leases/🎛️control/🟦️.ts`:45-49 performs path resolution, length and in-memory fenced-directory checks, without filesystem observation.

`requireRepositoryCacheStoreReadCustodyV1` at C:73 checks issued owner/reservation maps, original control invocation, Native reservation and cancellation/deadline. `requireNativeStorageReservationV1` at `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🟦️.ts`:66 checks private reservation maps and, when signals differ, issued control ancestry. Neither function replays the storage journal or image contents.

However, `requireRepositoryCacheStoreV1` at C:70 calls `requireOwner` at C:62, which calls `requireCurrentRepositoryCacheStoreV1` at C:24. That returns immediately while `implementationPinnedV1()` is true. Otherwise it authenticates current Store source and schema revisions synchronously. Its reservation/schema helper at `C`'s sibling `🎟️reservation/🟦️.ts`:21 similarly returns under a pin; otherwise it captures reservation source and Store schema revisions. `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🔏️revision/🟨️.mjs`:90-96 first permits a settled revision cache hit after a synchronous `lstatSync`; a miss captures source bytes. Thus unpinned checkpoints can observe caller implementation files, but claiming a full reread on every checkpoint would be incorrect. The live producer's pin state was not queried.

R:339-345 authenticates control issuance/invocation/refinement. `remaining` at R:260-335 walks retained control parents/follows and invocation deadlines synchronously, using per-call maps/sets and signal/time/progress checks. `checkResourceLeaseControl` at `🧰️framework/🔨️modules/🏃️process/🔒️leases/🎛️control/🟦️.ts`:15-20 calls `remainingMilliseconds`; these checks are not async scheduling waits. Refinement allocates a parent-ancestry set at R:345. Repeated control validation is concrete work, but no isolated timing or safe removal proof follows from this audit.

### Received Invocation Pin

The revision module at lines 74-80 shares `Symbol.for("semio.implementationPin.context.v1")` and `Symbol.for("semio.implementationPin.count.v1")` process-wide. Its predicate is an active async-context pin, or `processWide && open > 0`. R:251, `withReceivedImplementationPinV1`, uses those same symbols, creates an active pin, increments `open`, runs the callback in its AsyncLocalStorage context, and deactivates/decrements only in `finally`. A true `processWide` argument also enables the process-wide predicate. R:259, `receiveScriptProcessInvocation`, wraps the admitted invocation callback with this pin and passes `birth = !context.getStore()` as the process-wide argument. R:257 similarly pins received request invocations in their callback context. Therefore a callback running through these canonical receive functions has the Store revision guard's early-return condition by construction, including the process-wide birth case. Compiler-captured Sources are a separate mechanism: revision lines 31, 47-49 use captured images or authenticated physical reads; captured metadata alone is not the pin predicate. This audit did not prove the retained live producer's complete entrypoint-to-receive chain or query its actual pin state, so it does not attribute unpinned source-reading cost to that command.

## Generic And Replay Amplification

S:457-460 forwards every semantic `check` to the full checkpoint; `charge` also checks. S:468-470 performs full immutable-input verification: directory anchors, named and opened file stamps, complete chunked bytes, EOF, SHA-256 and final named identity. S:469 calls `check` on every `advance`; only crossing `progressWorkUnits` publishes `dependencies:store-input-replay` and awaits `yieldContinuation`, with checks after both awaits. This label has no percentage or completed-work suffix.

S:446-451 builds the parent trie, checks while adding paths/files, then visits each trie parent once. Each child is probed for actual same-volume identity and retained in an issued root-relative path proof. Visiting a parent uses `root.withDirectoryPath(current.path, visit)`, so "each parent once" does not mean each ancestor is physically opened only once across all paths.

Generic Windows adds synchronous physical observation beyond the caller checkpoint: D:280 wraps system calls with checkpoint-before/after; `facts` at D:281 observes type, basic/standard/file-ID/tag/legacy identity, security descriptor and stream information (plus reparse target for links). `internal` and `readEntry` at D:283 repeatedly compare current parent/child facts and invoke checkpoints. Those calls can multiply the synchronous caller checks substantially. `requireArtifactDescriptorClosureV1` at D:27 itself scans retained descriptor scopes for unresolved closure; `requireSynchronousArtifactBodyV1` at D:55 only checks function/proxy/async-function identity, not source text. POSIX D:153 also checks descriptor facts and checkpoints, with its own platform calls. No runtime measurement establishes either platform's share of the live elapsed time.

The expensive **whole fence** is separate: S:737 explicitly awaits `store.recheck()`, Bun `custody.recheck()`, current Source input/preservation observation, every retained immutable receipt replay, and preservation descriptor recheck. S:739-760 calls this fence across admission, staging, each installation, provider/singleton publication, ready verification and final checks. Consequently repeated `store-input-replay` and ownership-frontier progress can come from intentional fences over retained receipts; the tiny Generic checkpoint does not itself run this whole sequence.

## Result And Limit

There is demonstrable repeated synchronous deadline/issuance/path/facts work, and conditional unpinned implementation-file observation. There is no SQLite transaction or awaited physical ownership replay inside the inspected tiny checkpoint callgraph. Full async custody and immutable-input replay occurs at the explicit producer fence. These guards protect different identity, cancellation, lifetime and physical facts; this bounded audit proves no redundant mechanism that can safely be removed. It does not establish the live pin state, number of receipts/bytes/chunks, checkpoint frequency, syscall latency, remaining work, deadlock, failure or safe optimization.

## Root Follow-Up: Current Captured Runtime Entry

A bounded source lookup on 2026-11-05 found the current captured runtime entry chain. Relative to the build library root, 🏛️sealed/🚪️entry/🟦️.ts:42 selects the command kernel; ⌨️command/⚙️runtime/🟦️.ts:126 delegates to runCapturedRepositoryKernelV1. At ⌨️command/📥️entry/🟦️.ts:12, that function authenticates/adopts/rechecks the captured proof, then runs the body through receiveScriptProcessInvocation, an issued repository-command control/operation, and explicit withImplementationPinV1 around withRepositoryRuntimeEntriesV1 and the body. It rechecks the proof after completion. Thus the inspected current runtime entry explicitly pins its body as well as using the canonical received-invocation path. This supplies no missing-pin defect in that current source chain.

The retained process's captured entry bytes and live async-context state were not inspected. A current-source callgraph does not prove that an older captured process contains exactly those bytes. No guard/source change, benchmark, test pass, timing attribution or optimization follows. A conflicting cached process attribution and fresh census revalidation are separately recorded in producer-identity-revalidation.md; they must not be used to invent a pin or process failure.
