# Independent SDK Revalidation Source Audit

Read-only hostile review on 2026-10-15. Reviewed current capacity physical owner, derivative owner, new authentication tests, neutral corpus/schema, and actual lower SH/EX custody validators. No tests, compiler/image/store/Original acquisition, or Source/fixture edits ran. Parent reported genuine initial RED (`authStats=68`) and prototype 6 laws/61 assertions plus strict exit 0; those are retained handoff facts, not independently rerun results. Release/focused successors were running at assignment and are not certified here.

## Finding

No immediate authentication/effect split defect found in the inspected derivative success ladder. Production removal of repeated tree scans is narrowly bound to a private captured projection and its actual copier. No timestamp, cached validation bit, structural material claim or caller-provided copy implementation replaces an effect fence. Full-clone dispatch still defaults authentication to its physical verifier and retains its existing pre-body/post-body/drain behavior; this report proposes no full-clone refactor.

All paths below begin at `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools`.

## Private Authority And Effects

`📦️sdk/📥️source/🔗️derivative/🟦️.ts`, `authenticate(state)`: private WeakMap lookup establishes exact projection/workspace/store before invocation. Custody checks live state, retained lease, genuine storage reservation and original store control, then uses the lower private SH or EX issuance validator for exact resource/directory. The same checks run again after awaited `store.recheck()`. Dispatch on public lease mode cannot promote authority: each branch validates the lower WeakMap’s actual issued mode; actual base EX is needed during cold sealing. Lower SH/EX validators compare issued signal, private live record, resource and canonical lease path; exact store/control binding is supplied by the surrounding private state and storage admission, rather than claimed solely from public lease fields.

`requireNxSdkDerivativeProjectionV1` returns frozen captured counts and closures; it does no Source traversal. The closures retain private state and fail authentication after scope release. `requireNxSdkClonePublicationV1` authenticates exact quote/store and state before returning a private derived byte bound. Derivative `state.authenticate` calls the private projection authentication before/after await and checks quote liveness on both sides. No arbitrary material bound is admitted: `withNxSdkDerivativePublicationV1` gets counts from exact projection facts and input bytes from private captured inputs.

`📦️sdk/🎟️capacity/📥️physical/🟦️.ts`, admitted publication: registry EX is checked before the one-use claim, after fresh real statfs/store recheck, and after its physical success fence. Clone and input births require exact live AsyncLocal operation and quote/store; derivative clone requires `control===store.control`. `admitStage` additionally binds exact canonical owned stage and inode, with registry custody checks around awaited work. Accepted promises enter `operation.pending` synchronously; callback completion sets `operation.live=false` before drainage, preventing late descendant births. Already accepted copy work can finish under retained scopes.

The actual derivative copier gets private `derivativeCopies` custody, marks one-use synchronously, and stores its promise on the projection before awaiting. Copy pre/post checks and per-row guards remain. `accepted.live=false` occurs after the delegated copier settles, including rejection. Projection teardown drains its accepted copy before releasing the base loan. Escaped closures cannot mint fresh copy custody; they cannot replace the private copier.

## Five Separate Physical Boundaries

| Boundary | Current Source location/ordering |
| --- | --- |
| Copy before | `copyNxSdkDerivativeV1` starts `state.copy` with `await recheck(state)` before tree effects. |
| Copy after | Both rooted source/destination scopes and all layered row work finish, then `await recheck(state)` precedes successful copy result. |
| Admitted after drain | `withAdmittedNxSdkClonePublicationV1` closes births, waits `Promise.allSettled(operation.pending)`, propagates body/accepted failure, then `await quote.recheck()`. Registry EX remains required afterward. |
| Quote after drain | `withPublicationQuoteV1` closes `state.open`, waits actual publicationWork and cloneWork; derivative final `recheck()` occurs only after successful settlement. Quote remains live until its final finally. |
| Projection after drain | `withNxSdkDerivativeProjectionV1` waits state.copy, preserves failures, then rechecks before releasing SH in finally. |

These are five fences on the successful sealed single-copy path. Cold sealing retains extra checks: release SH, acquire actual nonwaiting EX, authenticate/recheck, seal using exact private capture, release EX, reacquire genuine SH and recheck. No bypass was introduced in this Source. The focused fixture starts already sealed, so it does not independently qualify the transition.

Failure state uses an explicit `failed` boolean separately from `failure:unknown` at quote, admitted and projection boundaries. Thus both Error identity and `throw undefined` survive; the code never interprets undefined as success. All accepted promises are settled before propagating a body failure. A failed operation skips success-only final physical verification, as expected; it still drains accepted work and closes scopes. The public frozen quote method is an owned closure, so the admitted call to `quote.recheck()` cannot be structurally substituted.

## What New Tests Actually Establish

`📦️sdk/📥️source/🔗️derivative/🧪️tests/🛂️.ts` has six laws. Neutral Ajv/SQLite validates the corpus and counts five effect names/three authentication names. The production-shaped sealed tiny path delegates actual metadata reads and lstat unchanged; its observer counts marker reads for full recheck entries and independently checks zero node_modules lstat during authentication. Source SHA and destination shared-versus-owned inode checks cover the three fixture files. This is substantially stronger than counting an internal helper counter, but marker reads are still a proxy for fence entry, not a complete syscall/event-order oracle.

The mutation law exercises owned first-file byte/size/mode disturbance before copy, just after its delegated copyFile, admitted-body, quote-body and projection-body. All expect failure; before/during-copy also forbid successful clone completion. During-copy deliberately mutates an already copied file; it does not cover a future row or shared inode. It does not assert public generation cleanup or visibility behavior.

The authority law rejects copied projection, cloned store, foreign workspace and escaped projection, and uses an independent SQLite BEGIN EXCLUSIVE attempt to verify actual retained SH. It does not directly assert copied/forged/escaped quote, wrong control, changed lease lifetime during awaited recheck or zero writes on each rejected authority case.

The detached failure law checks body/copy Error and undefined identity, accepted work existence, no active copy remaining, and both owned copies completing on body failure. The detached success law verifies the last three marker observations follow accepted copy settlement and body return. This directly covers the final quote fence moving after drainage. It catches errors inside the law, however: the enclosing Original itself succeeds when the expected inner rejection is caught. It is not independent evidence of an uncaught failure propagating through the whole Original.

## Remaining Hostile Cases To Preserve Or Add

These are evidence gaps, not newly observed production vulnerabilities. Keep the change bounded and retain existing hostile suites; do not claim this six-law fixture alone exhausts the design.

- Change/release actual SH, cancellation/control or store admission while `store.recheck()` is suspended; verify post-await authentication refuses before any effect. Exercise foreign same-signal control objects and exact store/control binding explicitly.
- Forged/copied/foreign-store/escaped quotes, escaped fact.authenticate/recheck closures, late AsyncLocal descendants after body completion, and rejected writeInputs accepted without awaiting. Verify zero late effects and real drainage, not only a caught error.
- File replacement with same size, symlink target changes, namespace addition/removal, ancestor swap, shared-file mutation, not-yet-copied-row mutation and changed pinned metadata/index. Current mutation case is only the first owned regular file.
- Independent reader blocks cold-seal EX; mutation across EX release/SH reacquisition; failed EX acquisition does not reopen effects. Existing cold-seal code remains, but new tests do not exercise it.
- Explicit whole-root Original failure propagation, cancellation during pending input/copy, conflicting multiple failures and ignored rejected promises; current tests verify primary inner identity and drainage.

Corpus schema weakness: `authentication` lacks maxItems/uniqueItems and `failures` lacks unique combinations. Runtime SQL/count assertions constrain today's authentication corpus, but the four failure rows could later duplicate one case while retaining length four. Add unique tuple coverage if these become the stable normative contract; current fixture does contain all four combinations.

Visibility limit remains explicit: actual publisher may rename within its callback before outer admitted/quote/projection fences. These checks withhold success after mutation; they do not prove no visible artifact exists after a later outer refusal. This was an existing stated scope boundary, not a reason to broaden this optimization. Initial full capture/hashes and metadata stamp ABA limits are unchanged.

## Handback

The bounded Source split is coherent on inspection. Accept the reported focused prototype result only at its actual scope, await collected release/strict successors, and retain separate full-size prototype/release qualification under existing Originals. Prioritize post-await authority invalidation and late/escaped quote tests before describing hostile authority coverage as complete. No additional physical operation was started by this audit.
