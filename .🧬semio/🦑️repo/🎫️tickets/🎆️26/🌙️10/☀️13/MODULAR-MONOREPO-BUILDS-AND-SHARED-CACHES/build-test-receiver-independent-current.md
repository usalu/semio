# Independent Frozen Build Test Receiver Review

Read-only Source review on 2026-10-15. No tests, runtime, images, stores or Originals were started. Reviewed receiver/registrar, law schema, hostile child/parent tests and actual graph suite caller. Reported BOTH 14/172 and strict success remain retained handback evidence, not independent execution. Other moving storage Core work was excluded.

## Actionable Actual Graph Caller Gap

`L/⚡️caching/💾️storage/🔒️enforcement/🔌️nx/📦️graph/🧪️tests/📥️store/🟦️.ts`, withPrivatePoolV1, validates its incoming law invocation, then uses a captured suite AsyncLocal context to invoke `body(store,native)` without carrying that law invocation/signal forward. The pool/store/control were issued from the real receiver root. withNxGraphFixtureV1 then acquires namespaces with `owner.acquireNxGraphNamespaceV1(store,descriptor)` using that suite-root control. Thus per-law registrar cutoff or receiver callback abort does not itself cancel these actual graph namespace waits/effects; only root lifetime/deadline does. Joining the returned promise preserves drainage and prevents early cleanup, but does not restore per-law cancellation or reserve timing.

Root was notified promptly. Minimal next step is an explicitly issued per-law operation/control beneath the retained suite resource, preserving root store custody while checking/using the law signal for actual waits/effects. Do not restore same-object aliases or re-provision a pool per law. Demonstrate cancellation against this actual caller or a faithful real issued suite-store/wait seam before claiming graph per-law timeout authority. The current independent SQLite suite test does not exercise a graph resource wait issued through its root store.

`L` means `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library` throughout.

## Receiver And Registrar Source Assessment

`L/🏗️build/🧪️tests/📥️invocation/🟦️.ts` receives one actual process invocation. Private AsyncLocal snapshot retains the genuine root issuance; each enqueue registers its controller/work synchronously before promise callback effects, validates the real root and enters canonical withScriptInvocationSignalV1. Per-law aliases are private issued values, not structural copies. The pending set observes rejection independently of whether callers catch/ignore it, and the separate failed boolean preserves thrown undefined.

registerBuildTestLawV1 admits only schema plan keys and checks body+drain=total. It claims an active slot synchronously and registers callback(done), returning void to Bun. After actual root readiness it computes duration from elapsed runner time and current root remaining minus drain reserve, refusing nonpositive funding before body. It does not renew either deadline. This implements a shortened body when remaining root time is below the full authored tuple; it does not require full original nominal total to remain. Describe this as funding the capped body plus reserve, matching code, rather than promising every admitted body its full nominal budget.

The body cutoff records a receiver failure and aborts the real derived signal. Complete runs only after the accepted callback promise settles through the canonical issuer. Completion at/after total budget or after next-law/afterAll invalidation is not delivered to stale done. Next registered law refuses before its body while the active accepted scope remains unsettled. It does not silently reopen a FIFO deadline. done(undefined) on rejected undefined is represented by a runner-only Error while the exact actual receiver failure remains undefined. A thrown done callback is caught and recorded; completion handlers return normally rather than creating a rejected detached continuation.

Suite ownership is genuinely separate: registerBuildTestFixtureV1 enrolls before effects, opens under the real root’s issued control, publishes retain once and waits receiver close. Users register their promise synchronously and keep independent failure observations. Early owner return fails protocol. The fixture handle exposes run/settled, not a root lookup for a law alias. This correctly removes the prior cleanup-dependent law-owned suite cycle.

Shutdown order is coherent: close receiver admission and done port; abort pending callback controllers; join pending callbacks; join fixture users; close fixture retain scopes; settle fixture owners; physical cleanups in reverse; finish/join received root; throw retained primary. Cleanup failure does not overwrite the first Error or undefined. Already-failed scopes still drain. An uncooperative accepted callback can outlive reserve/afterAll timeout and ultimately the root deadline; no finite drain guarantee beyond cooperative/issued operations is claimed.

No additional receiver defect found in this bounded review. The graph caller cancellation gap above is distinct from receiver promise/cleanup ordering.

## Actual Test Coverage And Limits

The neutral law validates schema through Ajv/repository subset and compares phase order using independent SQLite. Real children cover normal aliases, timeout, funded cooperative cutoff, Error/undefined, registered Error/undefined, detached rejection, suite owner failures, ignored user failure and early owner return. Actual Python SQLite verifies EX blocked while owned; terminal probes verify availability after child settlement. Parent requires genuine negative exit1 and no unhandled-between-tests diagnostic. This is meaningful actual lifecycle evidence and more than an expected event table.

Timeout child deliberately keeps issued gate work beyond the runner timeout, next registered law refuses before effects, gate settles and cleanup follows. The focused tests establish done-port expiry/next-law invalidation for that actual Bun behavior. They do not directly force done itself to throw, initialization/receive failure, root funding exhausted before callback effects, every plan’s boundary arithmetic, fixture availability failure racing a queued run, or actual graph store cancellation. Those remain targeted evidence gaps, not proof of receiver failure.

Root/suite signal roles must remain distinct. Root-owned suite resources deliberately survive individual law stops; actual accepted law operations using those resources must still honor their own alias signal. Retaining root resources is not permission for per-law effects to ignore cancellation.

Adoption remains scoped: old withBuildTestInvocationV1 registrations and immediate async wrappers do not automatically gain registrar body timers or done-port protection. The retained lower-bound inventory of 29 immediate wrappers across 12 files is not corrected by BOTH receiver 14/172. Actual graph caller is Source/strict integrated; no large native graph lifetime result follows from tiny SQLite children. SDK fence/copy runtime successors after registrar conversion remain separate required gates.

## Frozen Source Identity Check

Actual SHA256 values read during this review match the handback:

- Receiver: `cb95f6ca684c00dd056c4ddc178b0db965a376d1a728797ea57ac90bf7b040f7`
- Law schema: `c4a90875a5ef915cdfa49603fabf71101d5dcd19d0b404c494037cda3f3c18c0`
- Graph fixture: `00585056a14d9025a42bd5fb5f2056920c343976a01fb183986b7279ed42b839`

These identify current read Source bytes, not a claim that any prior child loaded those exact bytes. Only this Markdown report changed.

## Exact Graph Effect Trace

- Graph fixture `📥️store/🟦️.ts:18–21`: law original is checked, suite root opens privatePool, suite AsyncLocal snapshot invokes callback with root-owned store/native; no law signal argument is retained.
- Same file `:25–26`: withNxGraphFixtureV1 calls acquireNxGraphNamespaceV1(store,descriptor), passes that same store/environment to consumer body, tracks child rechecks and releases namespace in finally. Consumers are not automatically re-issued into the incoming alias context.
- Actual graph owner `📦️graph/🟦️.ts:54`: store.control and invocation are parent.control/parent.invocation from root-owned suite; `:78` check validates that root control. `:82` actual namespace inventory continuation uses store.control; `:92` pool queue admission uses the captured frame control/signal. `:98–101` namespace acquisition performs actual mkdir/writeFileSync marker and opens native backing with control:store.control. These are concrete waits/effects unaffected by only aborting the law alias.
- Suite root physical retirement in fixture finally deliberately uses its root-issued control; that is appropriate cleanup custody, and should remain distinct from cancellable per-law acquisition/consumer effects.

The generic receiver fixture.run itself queues its user callback in the caller’s current async context and enrolls its returned promise; it does not independently supply or validate a per-law invocation parameter. It is therefore a retained-resource/user-drain API, not a guarantee that arbitrary operations on the published value carry law cancellation. The graph-specific captured snapshot is the concrete context replacement identified in this audit. No other suite owner context substitution was found in the inspected tiny receiver fixture; that fixture intentionally opens its root suite lease separately and law gate work uses the actual supplied alias. No global census or global cancellation parity is claimed.
